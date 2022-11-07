use crate::container::container;
use clap::Parser;
use std::process::exit;
use tracing::{error, info};

#[derive(Parser, Debug)]
pub struct Run {
    /// Allocate a pseudo-TTY
    #[arg(short, long)]
    tty: bool,
    /// name of the container instance
    image: String,
}

impl Run {
    pub fn exec(&self) {
        info!("{:?}", self);
        let parent = container::new_parent_process(self.tty, &self.image);
        let mut child = match parent.borrow_mut().spawn() {
            Ok(child) => child,
            Err(e) => {
                error!("Failed to spawn child process: {}", e);
                exit(-1);
            }
        };
        child.wait().expect("Failed to wait child process");
        exit(-1);
    }
}
