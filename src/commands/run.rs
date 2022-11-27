use crate::container::container;
use clap::Parser;
use std::io::Write;
use std::process::exit;
use tracing::{error, info};

#[derive(Parser, Debug)]
pub struct Run {
    /// Allocate a pseudo-TTY
    #[arg(short, long)]
    tty: bool,
    /// name of the container instance
    commands: Vec<String>,
}

impl Run {
    pub unsafe fn exec(&self) {
        info!("{:?}", self);
        let parent = container::new_parent_process(self.tty.clone());
        let mut child = match parent.borrow_mut().spawn() {
            Ok(child) => child,
            Err(e) => {
                error!("Failed to spawn child process: {}", e);
                exit(-1);
            }
        };

        // send_init_command(&self.commands, writer);
        let mut writer = child
            .take_pipe_writer(3)
            .expect("Failed to take pipe writer");

        writer
            .write_all(self.commands.join(" ").as_bytes())
            .expect("Failed to write to pipe");

        // explicitly close the writer
        drop(writer);

        child.wait().expect("Failed to wait child process");
    }
}
