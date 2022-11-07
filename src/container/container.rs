use nix::errno::Errno;
use nix::{mount, unistd};
use std::cell::RefCell;
use std::ffi::CString;
use std::process::Command;
use tracing::info;

pub struct Container {}

/// equivalent to `dokerust init [image]`
/// fork a new process with a new namespace
pub fn new_parent_process(tty: bool, image: &str) -> RefCell<unshare::Command> {
    let mut cmd = unshare::Command::new("/proc/self/exe");
    cmd.args(&["init", image]);
    let namespaces = vec![
        unshare::Namespace::Net,
        unshare::Namespace::Uts,
        unshare::Namespace::Pid,
        unshare::Namespace::Mount,
        unshare::Namespace::Ipc,
    ];

    cmd.unshare(&namespaces);

    if tty {
        cmd.stdin(unshare::Stdio::inherit())
            .stdout(unshare::Stdio::inherit())
            .stderr(unshare::Stdio::inherit());
    }

    return RefCell::new(cmd);
}

/// mount a proc fs
pub fn run_container_init_process(cmd: &str) -> Result<(), Errno> {
    let mount_flags =
        mount::MsFlags::MS_NOEXEC | mount::MsFlags::MS_NOSUID | mount::MsFlags::MS_NODEV;

    mount::mount(
        Some("proc"),
        "/proc",
        Some("proc"),
        mount_flags,
        None::<&str>,
    )
    .expect("Failed to mount proc");

    info!("Mount procfs to /proc");

    let output = Command::new("ps")
        .args(["-ef"])
        .output()
        .expect("Failed to execute ps");
    info!("ps -ef: {}", String::from_utf8_lossy(&output.stdout));

    let args = [&CString::new(cmd).unwrap()];

    unistd::execv(args[0], &args)?;

    Ok({})
}

pub fn mount_proc() {
    let mount_flags =
        mount::MsFlags::MS_NOEXEC | mount::MsFlags::MS_NOSUID | mount::MsFlags::MS_NODEV;

    mount::mount(
        Some("proc"),
        "/proc",
        Some("proc"),
        mount_flags,
        None::<&str>,
    )
    .expect("Failed to mount proc");

    info!("Mount procfs to /proc");
}
