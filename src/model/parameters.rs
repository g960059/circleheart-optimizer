use fxhash::FxHashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub value: f64,
    pub default: f64,
    pub range: (f64, f64),
    pub fitting: bool,
}

impl Parameter {
    pub fn new(default: f64, range: (f64, f64), fitting: bool) -> Self {
        Self { value: default, default, range, fitting }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HemodynamicParams {
    pub parameters: FxHashMap<String, Parameter>,
}

impl HemodynamicParams {
    pub fn new() -> Self {
        let mut params = FxHashMap::default();
        
        params.insert("Rcs".to_string(), Parameter::new(830.0, (400.0, 1000.0), true));
        params.insert("Rcp".to_string(), Parameter::new(10.0, (5.0, 150.0), true));
        params.insert("Ras".to_string(), Parameter::new(20.0, (10.0, 450.0), true));
        params.insert("Rvs".to_string(), Parameter::new(25.0, (15.0, 60.0), true));
        params.insert("Rap".to_string(), Parameter::new(13.0, (6.0, 50.0), true));
        params.insert("Rvp".to_string(), Parameter::new(15.0, (5.0, 30.0), true));
        params.insert("Cas".to_string(), Parameter::new(1.83, (0.5, 4.0), true));
        params.insert("Cvs".to_string(), Parameter::new(70.0, (50.0, 250.0), true));
        params.insert("Cap".to_string(), Parameter::new(20.0, (2.0, 30.0), true));
        params.insert("Cvp".to_string(), Parameter::new(7.0, (5.0, 15.0), true));
        params.insert("LV_Ees".to_string(), Parameter::new(2.21, (1.0, 3.0), true));
        params.insert("LV_alpha".to_string(), Parameter::new(0.029, (0.02, 0.04), true));
        params.insert("LV_beta".to_string(), Parameter::new(0.34, (0.18, 0.4), true));
        params.insert("LA_Ees".to_string(), Parameter::new(0.48, (0.2, 0.7), true));
        params.insert("LA_alpha".to_string(), Parameter::new(0.058, (0.04, 0.07), true));
        params.insert("LA_beta".to_string(), Parameter::new(0.44, (0.3, 0.6), true));
        params.insert("RV_Ees".to_string(), Parameter::new(0.74, (0.5, 1.5), true));
        params.insert("RV_alpha".to_string(), Parameter::new(0.028, (0.01, 0.035), true));
        params.insert("RV_beta".to_string(), Parameter::new(0.34, (0.15, 0.5), true));
        params.insert("RA_Ees".to_string(), Parameter::new(0.38, (0.2, 0.6), true));
        params.insert("RA_alpha".to_string(), Parameter::new(0.046, (0.03, 0.07), true));
        params.insert("RA_beta".to_string(), Parameter::new(0.44, (0.3, 0.5), true));
        params.insert("Qvs_initial".to_string(), Parameter::new(749.9842973712131, (200.0, 6000.0), true));


        // Add fixed parameters
				params.insert("Ras_prox".to_string(), Parameter::new(30.0, (10.0, 100.0), true));
        params.insert("Rap_prox".to_string(), Parameter::new(15.0, (5.0, 50.0), true));
        params.insert("Rmv".to_string(), Parameter::new(2.5, (2.5, 2.5), false));
        params.insert("Rtv".to_string(), Parameter::new(2.5, (2.5, 2.5), false));
        params.insert("Cas_prox".to_string(), Parameter::new(0.54, (0.54, 0.54), false));
        params.insert("Cap_prox".to_string(), Parameter::new(1.0, (1.0, 1.0), false));
        params.insert("LV_V0".to_string(), Parameter::new(5.0, (1.0, 50.0), true));
        params.insert("LV_Tmax".to_string(), Parameter::new(300.0, (300.0, 300.0), false));
        params.insert("LV_tau".to_string(), Parameter::new(25.0, (25.0, 25.0), false));
        params.insert("LV_AV_delay".to_string(), Parameter::new(160.0, (160.0, 160.0), false));
        params.insert("LA_V0".to_string(), Parameter::new(10.0, (5.0, 20.0), true));
        params.insert("LA_Tmax".to_string(), Parameter::new(125.0, (125.0, 125.0), false));
        params.insert("LA_tau".to_string(), Parameter::new(20.0, (20.0, 20.0), false));
        params.insert("LA_AV_delay".to_string(), Parameter::new(0.0, (0.0, 0.0), false));
        params.insert("RV_V0".to_string(), Parameter::new(5.0, (2.0, 30.0), true));
        params.insert("RV_Tmax".to_string(), Parameter::new(300.0, (300.0, 300.0), false));
        params.insert("RV_tau".to_string(), Parameter::new(25.0, (25.0, 25.0), false));
        params.insert("RV_AV_delay".to_string(), Parameter::new(160.0, (160.0, 160.0), false));
        params.insert("RA_V0".to_string(), Parameter::new(10.0, (3.0, 30.0), true));
        params.insert("RA_Tmax".to_string(), Parameter::new(125.0, (125.0, 125.0), false));
        params.insert("RA_tau".to_string(), Parameter::new(20.0, (20.0, 20.0), false));
        params.insert("RA_AV_delay".to_string(), Parameter::new(0.0, (0.0, 0.0), false));
        params.insert("HR".to_string(), Parameter::new(90.0, (90.0, 90.0), false));
        params.insert("Ravs".to_string(), Parameter::new(0.0, (0.0, 0.0), false));
        params.insert("Ravr".to_string(), Parameter::new(100000.0, (100000.0, 100000.0), false));
        params.insert("Rmvr".to_string(), Parameter::new(100000.0, (100000.0, 100000.0), false));
        params.insert("Rmvs".to_string(), Parameter::new(0.0, (0.0, 0.0), false));
        params.insert("Rpvr".to_string(), Parameter::new(100000.0, (100000.0, 100000.0), false));
        params.insert("Rpvs".to_string(), Parameter::new(0.0, (0.0, 0.0), false));
        params.insert("Rtvr".to_string(), Parameter::new(100000.0, (100000.0, 100000.0), false));
        params.insert("Rtvs".to_string(), Parameter::new(0.0, (0.0, 0.0), false));
        params.insert("Rda".to_string(), Parameter::new(3.0, (3.0, 3.0), false));
        params.insert("Cda".to_string(), Parameter::new(0.52, (0.52, 0.52), false));

        Self { parameters: params }
    }
    
    pub fn update(&mut self, name: &str, value: Option<f64>, range: Option<(f64, f64)>, fitting: Option<bool>) {
        if let Some(param) = self.parameters.get_mut(name) {
            if let Some(value) = value {
                param.value = value;
            }
            if let Some(range) = range {
                param.range = range;
            }
            if let Some(fitting) = fitting {
                param.fitting = fitting;
            }
        }
    }
}

impl Default for HemodynamicParams {
    fn default() -> Self {
        Self::new()
    }
}