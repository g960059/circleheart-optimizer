use crate::model::parameters::HemodynamicParams;
use rayon::prelude::*;
use rand::prelude::*;
use std::f64::consts::PI;
use num_cpus;
use rayon::ThreadPoolBuilder;
use fxhash::FxHashMap;

#[inline]
fn e(t: f64, Tmax: f64, tau: f64, HR: f64) -> f64 {
    let t_ = t % (60000.0 / HR);
    let base = (-(60000.0 / HR - 3.0 * Tmax / 2.0) / tau).exp() / 2.0;
    if t_ < Tmax {
        ((PI * t_ / Tmax - PI / 2.0).sin() + 1.0) / 2.0 * (1.0 - base) + base
    } else if t_ < 3.0 * Tmax / 2.0 {
        (-(t_ - Tmax) / tau).exp() * (1.0 - base) + base
    } else {
        base
    }
}

#[inline]
fn P(V: f64, t: f64, Ees: f64, V0: f64, alpha: f64, beta: f64, Tmax: f64, tau: f64, AV_delay: f64, HR: f64) -> f64 {
    let x = alpha * (V - V0).clamp(-700.0, 700.0);
    let Ped = beta * (x.exp() - 1.0);
    let Pes = Ees * (V - V0);
    Ped + e(t - AV_delay, Tmax, tau, HR) * (Pes - Ped)
}

#[inline]
fn calculate_valve_flow(grad: f64, R_open: f64, R_open_var: f64, R_close_var: f64) -> f64 {
    let (r, v) = if grad > 0.0 { (R_open, R_open_var) } else { (R_open + R_close_var, R_close_var) };
    if v == 0.0 || (grad < 0.0 && R_close_var == 100000.0) {
        grad / r
    } else {
        let sign = if grad > 0.0 { -1.0 } else { 1.0 };
        (sign * r + (r.powi(2) + sign * 4.0 * v * grad).max(0.0).sqrt()) / (2.0 * v)
    }
}

#[inline]
fn calculate_pressures_and_flows(
    t: f64, state: &[f64; 11], params: &HemodynamicParams
) -> ([f64; 6], [f64; 11]) {
    let [Qvs, Qas, Qap, Qvp, Qlv, Qla, Qrv, Qra, Qas_prox, Qda, Qap_prox] = *state;

    let Plv = P(
        Qlv, t, params.parameters["LV_Ees"].value, params.parameters["LV_V0"].value, params.parameters["LV_alpha"].value, 
        params.parameters["LV_beta"].value, params.parameters["LV_Tmax"].value, params.parameters["LV_tau"].value, 
        params.parameters["LV_AV_delay"].value, params.parameters["HR"].value
    );
    let Pla = P(
        Qla, t, params.parameters["LA_Ees"].value, params.parameters["LA_V0"].value, params.parameters["LA_alpha"].value, 
        params.parameters["LA_beta"].value, params.parameters["LA_Tmax"].value, params.parameters["LA_tau"].value, 
        params.parameters["LA_AV_delay"].value, params.parameters["HR"].value
    );
    let Prv = P(
        Qrv, t, params.parameters["RV_Ees"].value, params.parameters["RV_V0"].value, params.parameters["RV_alpha"].value, 
        params.parameters["RV_beta"].value, params.parameters["RV_Tmax"].value, params.parameters["RV_tau"].value, 
        params.parameters["RV_AV_delay"].value, params.parameters["HR"].value
    );
    let Pra = P(
        Qra, t, params.parameters["RA_Ees"].value, params.parameters["RA_V0"].value, params.parameters["RA_alpha"].value, 
        params.parameters["RA_beta"].value, params.parameters["RA_Tmax"].value, params.parameters["RA_tau"].value, 
        params.parameters["RA_AV_delay"].value, params.parameters["HR"].value
    );

    let Ida = (Qas_prox / params.parameters["Cas_prox"].value - Qda / params.parameters["Cda"].value) / params.parameters["Rda"].value;
    let Ias = (Qda / params.parameters["Cda"].value - Qas / params.parameters["Cas"].value) / params.parameters["Ras"].value;
    let Ics = (Qas / params.parameters["Cas"].value - Qvs / params.parameters["Cvs"].value) / params.parameters["Rcs"].value;
    let Ivs = (Qvs / params.parameters["Cvs"].value - Pra) / params.parameters["Rvs"].value;

    let Ivp = (Qvp / params.parameters["Cvp"].value - Pla) / params.parameters["Rvp"].value;
    let Iap = (Qap / params.parameters["Cap"].value - Qvp / params.parameters["Cvp"].value) / params.parameters["Rap"].value;
    let Icp = (Qap_prox / params.parameters["Cap_prox"].value - Qap / params.parameters["Cap"].value) / params.parameters["Rcp"].value;

    let Itv = calculate_valve_flow(Pra - Prv, params.parameters["Rtv"].value, params.parameters["Rtvs"].value, params.parameters["Rtvr"].value);
    let Imv = calculate_valve_flow(Pla - Plv, params.parameters["Rmv"].value, params.parameters["Rmvs"].value, params.parameters["Rmvr"].value);
    let Iasp = calculate_valve_flow(Plv - Qas_prox / params.parameters["Cas_prox"].value, params.parameters["Ras_prox"].value, params.parameters["Ravs"].value, params.parameters["Ravr"].value);
    let Iapp = calculate_valve_flow(Prv - Qap_prox / params.parameters["Cap_prox"].value, params.parameters["Rap_prox"].value, params.parameters["Rpvs"].value, params.parameters["Rpvr"].value);

    ([Plv, Pla, Prv, Pra, Iasp, Iapp], [
        Ics - Ivs, Ias - Ics, Icp - Iap, Iap - Ivp, Imv - Iasp, Ivp - Imv, 
        Itv - Iapp, Ivs - Itv, Iasp - Ida, Ida - Ias, Iapp - Icp,
    ])
}

#[inline]
fn pv_func(t: f64, state: &[f64; 11], params: &HemodynamicParams) -> [f64; 11] {
    let (_, flows) = calculate_pressures_and_flows(t, state, params);
    flows
}

pub struct SimulationResult {
    pub t: Vec<f64>,
    pub y: Vec<Vec<f64>>,
}

pub fn runge_kutta_4<F>(
  mut f: F,
  y0: [f64; 11],
  t: &[f64; 5001],
  params: &HemodynamicParams,
) -> SimulationResult
where
  F: FnMut(f64, &[f64; 11], &HemodynamicParams) -> [f64; 11] + Sync ,
{
  let n = t.len().min(5001); 
  let mut y = [[0.0; 11]; 5001];
  let mut t_result = [0.0; 5001];
  y[0] = y0;
  t_result[..n].copy_from_slice(&t[..n]);

  for i in 1..n {
      let dt = t[i] - t[i - 1];
      let dt_2 = dt / 2.0;
      let dt_6 = dt / 6.0;
  
      let k1 = f(t[i - 1], &y[i - 1], params);
      let mut y_temp = y[i - 1];
      for j in 0..11 {
          y_temp[j] += dt_2 * k1[j];
      }
      let k2 = f(t[i - 1] + dt_2, &y_temp, params);
      for j in 0..11 {
          y_temp[j] = y[i - 1][j] + dt_2 * k2[j];
      }
      let k3 = f(t[i - 1] + dt_2, &y_temp, params);
      for j in 0..11 {
          y_temp[j] = y[i - 1][j] + dt * k3[j];
      }
      let k4 = f(t[i], &y_temp, params);
  
      for j in 0..11 {
          y[i][j] = y[i - 1][j] + dt_6 * (k1[j] + 2.0 * (k2[j] + k3[j]) + k4[j]);
      }
  }

  SimulationResult {
      t: t_result,
      y,
  }
}


pub fn calculate_hemodynamic_metrics(sol: &SimulationResult, params: &HemodynamicParams) -> FxHashMap<String, f64> {
  let t = &sol.t;
  let states = &sol.y;
  let dt = t[1] - t[0];

  let cycle_duration = 60000.0 / params.parameters["HR"].value;
  let end_time = t[t.len() - 1];
  let start_time = end_time - cycle_duration;

  let mut cycle_indices = [0; 5001];
  let mut cycle_index_count = 0;

  for (i, &time) in t.iter().enumerate() {
      if time >= start_time && time <= end_time {
          cycle_indices[cycle_index_count] = i;
          cycle_index_count += 1;
      }
  }

  if cycle_index_count == 0 {
      panic!("No cycle indices found. Check the cycle duration and timing.");
  }

  let mut stroke_volume = 0.0;
  let mut central_venous_pressure = 0.0;
  let mut pulmonary_capillary_wedge_pressure = 0.0;
  let mut systolic_arterial_pressure = f64::MIN;
  let mut diastolic_arterial_pressure = f64::MAX;
  let mut systolic_pulmonary_arterial_pressure = f64::MIN;
  let mut diastolic_pulmonary_arterial_pressure = f64::MAX;

  let mut LVEDV = f64::MIN;
  let mut LVESV = f64::MAX;

  let cas_prox = params.parameters["Cas_prox"].value;
  let ras_prox = params.parameters["Ras_prox"].value;
  let cap_prox = params.parameters["Cap_prox"].value;
  let rap_prox = params.parameters["Rap_prox"].value;
  
  for &i in cycle_indices.iter().take(cycle_index_count) {
      let (pressures, _) = calculate_pressures_and_flows(t[i], &states[i], params);
      let [Plv, Pla, Prv, Pra, Iasp, Iapp] = [pressures[0], pressures[1], pressures[2], pressures[3], pressures[4], pressures[5]];
  
      stroke_volume += Iasp.max(0.0) * dt;
  
      central_venous_pressure += Pra;
      pulmonary_capillary_wedge_pressure += Pla;
  
      let Aop = states[i][8] / cas_prox + Iasp * ras_prox;
      systolic_arterial_pressure = systolic_arterial_pressure.max(Aop);
      diastolic_arterial_pressure = diastolic_arterial_pressure.min(Aop);
  
      let PAP = states[i][10] / cap_prox + Iapp * rap_prox;
      systolic_pulmonary_arterial_pressure = systolic_pulmonary_arterial_pressure.max(PAP);
      diastolic_pulmonary_arterial_pressure = diastolic_pulmonary_arterial_pressure.min(PAP);
  
      let Qlv = states[i][4];
      LVEDV = LVEDV.max(Qlv);
      LVESV = LVESV.min(Qlv);
  }

  let cycle_len = cycle_index_count as f64;
  central_venous_pressure /= cycle_len;
  pulmonary_capillary_wedge_pressure /= cycle_len;

  let left_ventricular_ejection_fraction = (LVEDV - LVESV) / LVEDV * 100.0;

  let mut metrics = FxHashMap::default();
  metrics.insert("stroke_volume".to_string(), stroke_volume);
  metrics.insert("central_venous_pressure".to_string(), central_venous_pressure);
  metrics.insert("pulmonary_capillary_wedge_pressure".to_string(), pulmonary_capillary_wedge_pressure);
  metrics.insert("systolic_arterial_pressure".to_string(), systolic_arterial_pressure);
  metrics.insert("diastolic_arterial_pressure".to_string(), diastolic_arterial_pressure);
  metrics.insert("systolic_pulmonary_arterial_pressure".to_string(), systolic_pulmonary_arterial_pressure);
  metrics.insert("diastolic_pulmonary_arterial_pressure".to_string(), diastolic_pulmonary_arterial_pressure);
  metrics.insert("left_ventricular_ejection_fraction".to_string(), left_ventricular_ejection_fraction);

  metrics
}


pub fn evaluate(params: &HemodynamicParams, target_metrics: &[(f64, String, f64)]) -> f64 {
  let default_data = [
      params.parameters["Qvs_initial"].value,
      149.3527787113375, 405.08061599015554, 135.97317102061024,
      144.32186565319813, 75.34345155268299, 117.70495107318685,
      73.76400781737635, 68.42882775454605, 42.75963410693713,
      20.28639894876003,
  ];
  let default_time = 954.931700000081;
  let dt = 2.0;
  let mut t_eval = [0.0; 5001];
  for i in 0..t_eval.len() {
      t_eval[i] = default_time + i as f64 * dt;
  }

  let sol = runge_kutta_4(pv_func, default_data, &t_eval, &params);

  let metrics = calculate_hemodynamic_metrics(&sol, params);
  let mut error = 0.0;
  for (target, key, weight) in target_metrics {
      if let Some(&value) = metrics.get(key.as_str()) {
          error += weight * (target - value).powi(2);
      }
  }
  error
}



pub fn generate_individual() -> HemodynamicParams {
  let mut rng = Pcg32::seed_from_u64(rand::thread_rng().gen());
  let mut params = HemodynamicParams::new();
  for (name, param) in params.parameters.iter_mut() {
      if param.fitting {
          param.value = rng.gen_range(param.range.0..param.range.1);
      }
  }
  params
}




fn crossover(parent1: &HemodynamicParams, parent2: &HemodynamicParams) -> HemodynamicParams {
  let mut rng = Pcg32::seed_from_u64(rand::thread_rng().gen());
  let mut child = parent1.clone();
  for (name, param) in child.parameters.iter_mut() {
      if param.fitting && rng.gen_bool(0.5) {
          param.value = parent2.parameters[name].value;
      }
  }
  child
}

fn mutate(individual: &mut HemodynamicParams, mutation_rate: f64) {
  let mut rng = Pcg32::seed_from_u64(rand::thread_rng().gen());
  for param in individual.parameters.values_mut() {
      if param.fitting && rng.gen_bool(mutation_rate as f64) {
          param.value = rng.gen_range(param.range.0..param.range.1);
      }
  }
}

fn tournament_selection(population: &Vec<(HemodynamicParams, f64)>, tournament_size: usize) -> HemodynamicParams {
  let mut rng = Pcg32::seed_from_u64(rand::thread_rng().gen());
  let mut best = &population[rng.gen_range(0..population.len())];
  for _ in 0..tournament_size-1 {
      let individual = &population[rng.gen_range(0..population.len())];
      if individual.1 < best.1 {
          best = individual;
      }
  }
  best.0.clone()
}
fn run_single(
  target_metrics: &[(f64, String, f64)],
  param_updates: Option<FxHashMap<String, (Option<f64>, Option<(f64, f64)>, Option<bool>)>>,
  threads_per_start: usize,
) -> (HemodynamicParams, f64) {
  let mut params = HemodynamicParams::default();

  if let Some(updates) = param_updates {
      for (name, (value, range, fitting)) in updates {
          params.update(&name, value, range, fitting);
      }
  }

  let population_size = 100;
  let generations = 200;
  let elite_count = 1;
  let tournament_size = 3;
  let initial_mutation_rate = 0.1;
  let final_mutation_rate = 0.01;

  let pool = ThreadPoolBuilder::new().num_threads(threads_per_start).build().unwrap();

  let mut population: Vec<(HemodynamicParams, f64)> = pool.install(|| {
      (0..population_size)
          .into_par_iter()
          .map(|_| {
              let individual = generate_individual();
              let fitness = evaluate(&individual, target_metrics);
              (individual, fitness)
          })
          .collect()
  });

  for generation in 0..generations {
      println!("Generation: {}/{}", generation + 1, generations);

      population.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
      let mut new_population = population.iter().take(elite_count).cloned().collect::<Vec<_>>();

      let mutation_rate = initial_mutation_rate - (initial_mutation_rate - final_mutation_rate) * (generation as f64 / generations as f64);

      let offspring = pool.install(|| {
          (0..population_size - elite_count)
              .into_par_iter()
              .map(|_| {
                  let parent1 = tournament_selection(&population, tournament_size);
                  let parent2 = tournament_selection(&population, tournament_size);
                  let mut child = crossover(&parent1, &parent2);
                  mutate(&mut child, mutation_rate);
                  let fitness = evaluate(&child, target_metrics);
                  (child, fitness)
              })
              .collect::<Vec<_>>()
      });

      new_population.extend(offspring);
      population = new_population;

      let mut best_fitness = population.iter().map(|(_, fitness)| *fitness).collect::<Vec<_>>();
      best_fitness.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
      let top_three_fitness = best_fitness.iter().take(3).cloned().collect::<Vec<_>>();
      println!("Fitness at generation {}: {:?}", generation, top_three_fitness);
  }

  population.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
  (population[0].0.clone(), population[0].1)
}

pub fn run_optimization(
  target_metrics: &[(f64, String, f64)],
  param_updates: Option<FxHashMap<String, (Option<f64>, Option<(f64, f64)>, Option<bool>)>>,
  num_repeats: usize
) -> (HemodynamicParams, f64) {
  let cpus = num_cpus::get();
  let threads_per_start = cpus / num_repeats;

  let results: Vec<(HemodynamicParams, f64)> = (0..num_repeats)
      .into_par_iter()
      .map(|repeat| {
          println!("Starting repeat: {}/{}", repeat + 1, num_repeats);
          run_single(target_metrics, param_updates.clone(), threads_per_start)
      })
      .collect();

  let (best_params, best_fitness) = results.into_iter().min_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap();
  (best_params, best_fitness)
}
