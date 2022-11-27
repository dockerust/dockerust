use std::cell::RefCell;
use std::ffi::CString;
use std::{fs::File, io::Read, os::unix::io::FromRawFd};

use nix::errno::Errno;
use nix::{mount, unistd};
use tracing::info;

pub struct Container {}

/// equivalent to `dokerust init [image]`
/// fork a new process with a new namespace
pub fn new_parent_process(tty: bool) -> RefCell<unshare::Command> {
    let mut cmd = unshare::Command::new("/proc/self/exe");
    cmd.args(&["init"]);
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

    // let (reader, writer) = pipe().unwrap();

    cmd.file_descriptor(3, unshare::Fd::ReadPipe);

    return RefCell::new(cmd);
}

fn read_user_command() -> Result<Vec<String>, Errno> {
    let mut f = unsafe { File::from_raw_fd(3) };
    let mut input = String::new();
    f.read_to_string(&mut input)
        .expect("Failed to read from pipe");
    let cmd_array = input.split_whitespace().map(|s| s.to_string()).collect();
    Ok(cmd_array)
}

/// mount a proc fs
pub fn run_container_init_process() -> Result<(), Errno> {
    let cmd_array = read_user_command().unwrap();
    info!("commands: {:?}", cmd_array);
    assert!(cmd_array.len() >= 1);

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

    let args = cmd_array
        .into_iter()
        .map(|s| CString::new(s).unwrap())
        .collect::<Vec<_>>();

    unistd::execv(&args[0], &args)?;

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
