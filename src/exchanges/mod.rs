use crate::exchanges::model::{Config, ServerTime};
use std::error::Error;
use std::io;

pub mod mexc;
mod model;

pub trait Exchanges {
    fn new_client(&self, cfg: Config) -> Self;
    fn ping(&self) -> Result<(bool), Box<dyn std::error::Error>>;

    fn time(&self) -> Result<(ServerTime), Box<dyn std::error::Error>>;
}
