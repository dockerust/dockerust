use crate::container::container;
use clap::Parser;
use std::process;
use tracing::info;

#[derive(Parser, Debug)]
pub struct Run {
    /// Allocate a pseudo-TTY
    #[arg(short, long)]
    tty: bool,
    /// Keep STDIN open even if not attached
    #[arg(short, long)]
    interactive: bool,
    /// name of the container instance
    image: String,
}

impl Run {
    pub fn exec(&self) {
        info!("{:?}", self);
        container::new_parent_process(self.tty && self.interactive, &self.image);
        process::exit(-1);
    }
}
