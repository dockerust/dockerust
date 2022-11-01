use clap::Parser;
use tracing::info;

#[derive(Parser, Debug)]
pub struct Init {
    image: String,
}

impl Init {
    pub fn exec(&self) {
        info!("{:?}", self);
    }
}
