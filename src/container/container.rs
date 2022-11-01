use nix;
use std::fs;
use std::io::Result;
use std::process::{Command, Stdio};
use tracing::info;

const STACK_SIZE: usize = 4 * 1024 * 1024;

pub struct Container {}

/// equivalent to `dokerust init [image]`
/// fork a new process with a new namespace
pub fn new_parent_process(tty: bool, image: &str) {
    let stack = &mut [0u8; STACK_SIZE];
    let path = fs::read_link("/proc/self/exe").expect("Cannot find self process");
    let mut binding = Command::new(path);
    let cmd = binding.args(["init", image]);

    if tty {
        cmd.stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
    }

    info!("Run Command: {}", format!("{:?}", cmd).replace("\"", ""));

    let cb = Box::new(|| {
        let exit_status = cmd
            .spawn()
            .expect("Failed to spawn process")
            .wait()
            .unwrap();
        match exit_status.code() {
            Some(code) => code as isize,
            None => -1,
        }
    });

    let flags = nix::sched::CloneFlags::CLONE_NEWUTS
        | nix::sched::CloneFlags::CLONE_NEWPID
        | nix::sched::CloneFlags::CLONE_NEWNS
        | nix::sched::CloneFlags::CLONE_NEWNET
        | nix::sched::CloneFlags::CLONE_NEWIPC;

    let pid = nix::sched::clone(
        cb,
        stack,
        flags,
        Some(nix::sys::signal::Signal::SIGCHLD as i32),
    )
    .expect("Failed to clone process");
    let _ = nix::sys::wait::waitpid(pid, None).expect("Failed to waitpid");
}

/// mount a proc fs
pub fn run_container_init_process(cmd: &str, args: &[&str]) -> Result<()> {
    Ok(())
}
