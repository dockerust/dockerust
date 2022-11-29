use crate::volume::volume::new_workspace;
use nix::errno::Errno;
use nix::mount::{mount, umount2, MntFlags, MsFlags};
use nix::unistd::{chdir, execv, pivot_root};
use resolve_path::PathResolveExt;
use std::cell::RefCell;
use std::ffi::CString;
use std::path::Path;
use std::{fs, fs::File, io::Read, os::unix::io::FromRawFd};
use tracing::info;

pub struct Container {}

/// equivalent to `dokerust init [image]`
/// fork a new process with a new namespace
pub fn new_parent_process(tty: bool, image: &str) -> RefCell<unshare::Command> {
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

    cmd.file_descriptor(3, unshare::Fd::ReadPipe);

    // TODO: move to somewhere else
    let mnt_url = Path::new("/dockerust/mnt");
    let root_url = Path::new("/dockerust");
    let img_abs_path = image.try_resolve().expect("Failed to resolve image path");
    new_workspace(root_url, mnt_url, img_abs_path.as_ref());
    cmd.current_dir(mnt_url);

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

pub fn run_container_init_process() -> Result<(), Errno> {
    let cmd_array = read_user_command().unwrap();
    info!("commands: {:?}", cmd_array);
    if cmd_array.len() < 1 {
        panic!("Please specify a command to run");
    }

    setup_mount();

    let args = cmd_array
        .into_iter()
        .map(|s| CString::new(s).unwrap())
        .collect::<Vec<_>>();

    execv(&args[0], &args)?;

    info!("execv success");

    Ok({})
}

fn setup_mount() {
    let pwd = std::env::current_dir().unwrap();
    info!("pwd: {:?}", pwd);
    mount_rootfs(pwd.as_path());
    pivot_rootfs(pwd.as_path());
    mount_procfs();
    mount_tmpfs();
}

fn mount_rootfs(rootfs: &Path) {
    /* https://man7.org/linux/man-pages/man2/pivot_root.2.html
    Ensure that 'new_root' and its parent mount don't have
    shared propagation (which would cause pivot_root() to
    return an error), and prevent propagation of mount
    events to the initial mount namespace.
     */
    mount(
        None::<&str>,
        "/",
        None::<&str>,
        MsFlags::MS_PRIVATE | MsFlags::MS_REC,
        None::<&str>,
    )
    .expect("disable propagation of mount events failed");

    mount::<Path, Path, str, str>(
        Some(rootfs),
        rootfs,
        None::<&str>,
        MsFlags::MS_BIND | MsFlags::MS_REC,
        None::<&str>,
    )
    .expect("mount rootfs failed");
}

fn pivot_rootfs(rootfs: &Path) {
    fs::create_dir_all(rootfs.join("pivot_root")).expect("Failed to create pivot_root dir");

    pivot_root(rootfs.as_os_str(), rootfs.join("pivot_root").as_os_str())
        .expect("Failed to pivot root");

    chdir("/").expect("Failed to chdir to root");

    umount2(
        Path::new("/").join("pivot_root").as_os_str(),
        MntFlags::MNT_DETACH,
    )
    .expect("Failed to umount pivot_root dir");

    fs::remove_dir_all(Path::new("/").join("pivot_root")).expect("Failed to remove pivot_root dir");

    chdir("/").expect("Failed to chdir to root");

    info!("Pivot rootfs success");
}

fn mount_procfs() {
    mount(
        Some("proc"),
        "/proc",
        Some("proc"),
        MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID | MsFlags::MS_NODEV,
        None::<&str>,
    )
    .expect("Failed to mount proc");

    info!("Mount procfs success");
}

fn mount_tmpfs() {
    mount(
        Some("tmpfs"),
        "/dev",
        Some("tmpfs"),
        MsFlags::MS_NOSUID | MsFlags::MS_STRICTATIME,
        Some("mode=755"),
    )
    .expect("Failed to mount tmpfs");

    info!("Mount tmpfs success");
}
