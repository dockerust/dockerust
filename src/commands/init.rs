use crate::container::container;
use clap::Parser;
use tracing::info;

#[derive(Parser, Debug)]
pub struct Init {
    image: String,
}

impl Init {
    pub fn exec(&self) {
        info!("{:?}", self);
        container::run_container_init_process(&self.image).expect("Failed to run container");
    }
}
