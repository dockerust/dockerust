use clap::Parser;
use tracing::info;

use crate::container::container;

#[derive(Parser, Debug)]
pub struct Init {}

impl Init {
    pub unsafe fn exec(&self) {
        info!("{:?}", self);
        container::run_container_init_process().expect("Failed to run container");
    }
}
