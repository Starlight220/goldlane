use crate::types::{Inputs, Outputs};

use futures::executor::block_on;
use yew::prelude::*;
use yew::events::InputEvent;
use crate::backend;

pub struct Calculator {
    inputs: Inputs,
    outputs: Outputs
}

#[derive(Debug)]
pub enum Msg {
    LaunchVelocity(f64),
    LaunchAngle(f64),
    LaunchHeight(f64),
}

impl Component for Calculator {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            inputs: Inputs { v0: 0.0, theta0: 0.0, h0: 0.0 },
            outputs: Outputs { x: 0.0, v: 0.0, theta: 0.0 }
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LaunchVelocity(v0) => self.inputs.v0 = v0,
            Msg::LaunchAngle(theta0) => self.inputs.theta0 = theta0,
            Msg::LaunchHeight(h0) => self.inputs.h0 = h0
        };
        // TODO: cache client
        self.outputs = block_on(backend::CalculatorClient::new().calculate(self.inputs.clone())).unwrap();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let v0_listener = ctx.link().callback(
            |event: InputEvent| Msg::LaunchVelocity(unwrap_f64(event.data())));
        let theta0_listener = ctx.link().callback(
            |event: InputEvent| Msg::LaunchAngle(unwrap_f64(event.data())));
        let h0_listener = ctx.link().callback(
            |event: InputEvent| Msg::LaunchHeight(unwrap_f64(event.data())));

        html! {
            <div>
                <input
                    oninput={v0_listener}
                    type="number"
                    value={format!("{:.02}", &self.inputs.v0)}
                />
                <input
                    oninput={theta0_listener}
                    type="number"
                    value={format!("{:.02}", &self.inputs.theta0)}
                />
                <input
                    oninput={h0_listener}
                    type={"number"}
                    value={format!("{:.02}", &self.inputs.h0)}
                />
                <div>{format!("Displacement: {:.02}", &self.outputs.x)}</div>
                <div>{format!("Impact Velocity: {:.02}", &self.outputs.v)}</div>
                <div>{format!("Impact Angle: {:.02}", &self.outputs.theta)}</div>
            </div>
        }
    }
}

use std::str::FromStr;

/// Panics on error.
fn unwrap_f64(opt: Option<String>) -> f64 {
    f64::from_str(opt.unwrap_or("0.0".to_string()).as_str()).unwrap()
}
