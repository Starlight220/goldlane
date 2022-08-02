use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub(crate) struct Inputs {
    pub v0: f64,
    pub theta0: f64,
    pub h0: f64,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub(crate) struct Outputs {
    pub x: f64,
    pub v: f64,
    pub theta: f64
}
