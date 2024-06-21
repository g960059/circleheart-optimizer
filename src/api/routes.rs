use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::model::simulation;
use crate::model::parameters::HemodynamicParams;
use fxhash::FxHashMap;

#[derive(Deserialize)]
pub struct OptimizationParams {
    pub target_metrics: Vec<(f64, String, f64)>,
    pub param_updates: Option<FxHashMap<String, (Option<f64>, Option<(f64, f64)>, Option<bool>)>>,
    pub num_repeats: usize,
}

#[derive(Serialize)]
pub struct OptimizationResult {
    pub best_parameters: HemodynamicParams,
    pub best_fitness: f64,
}

async fn optimize(params: web::Json<OptimizationParams>) -> impl Responder {
    let OptimizationParams {
        target_metrics,
        param_updates,
        num_repeats,
    } = params.into_inner();

    let (best_params, best_fitness) = simulation::run_optimization(
        &target_metrics,
        param_updates,
        num_repeats,
    );

    HttpResponse::Ok().json(OptimizationResult {
        best_parameters: best_params,
        best_fitness,
    })
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/optimize")
            .route(web::post().to(optimize))
    );
}