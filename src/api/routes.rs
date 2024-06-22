use crate::model::simulation;
use crate::model::parameters::HemodynamicParams;

use axum::{
    extract::Json,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

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

pub async fn optimize(Json(params): Json<OptimizationParams>) -> impl IntoResponse {
    let OptimizationParams {
        target_metrics,
        param_updates,
        num_repeats,
    } = params;

    let (best_params, best_fitness) = simulation::run_optimization(
        &target_metrics,
        param_updates,
        num_repeats,
    );

    Json(OptimizationResult {
        best_parameters: best_params,
        best_fitness,
    })
}