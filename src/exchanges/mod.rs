use std::error::Error;

pub mod mexc;

pub trait Exchanges {
    fn new_client(&self) -> Self;
}
