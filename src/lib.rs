/// # Flows Connector DSI
/// 
/// This is a library that wraps the inbound/outbound of 
/// [flows.network](https://flows.network)'s flow function into 
/// a Rust data structure.
/// 
/// You can find the example code for this library 
/// [here](https://github.com/second-state/flow-functions).

pub mod github;
pub mod gmail;
pub mod monday;
pub mod notion;
pub mod sendgrid;
pub mod twilio;

pub use serde::*;
pub use serde_json::*;

mod tests;
