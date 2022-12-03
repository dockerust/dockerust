use nix::mount::umount;
use std::fs;
use std::path::Path;
use std::process::Command;
use tracing::info;

pub fn new_workspace(root_url: &Path, mnt_url: &Path, image_url: &Path) {
    fs::create_dir_all(&root_url).expect("Failed to create lower dir");
    let output = Command::new("mount")
        .arg("-t")
        .arg("tmpfs")
        .arg("tmpfs")
        .arg(format!(
            "{}",
            root_url.to_str().unwrap(),
        ))
        .output()
        .expect("Failed to mount overlay");
    if !output.status.success() {
        output.stderr.iter().for_each(|&b| print!("{}", b as char));
        panic!("Failed to mount tmpfs");
    }
    create_lower_dir(root_url, image_url);
    create_upper_dir(root_url);
    create_work_layer(root_url);
    create_mount_point(root_url, mnt_url);
}

fn create_lower_dir(root_url: &Path, image_url: &Path) {
    let lower_dir = root_url.join("lower");
    fs::create_dir_all(&lower_dir).expect("Failed to create lower dir");
    let output = Command::new("tar")
        .arg("-xvf")
        .arg(image_url)
        .arg("-C")
        .arg(lower_dir)
        .output()
        .expect("Failed to unzip image");
    if !output.status.success() {
        panic!("Failed to unzip image");
    }
}

// UPPER dir: r/w
fn create_upper_dir(root_url: &Path) {
    let upper_dir = root_url.join("upper");
    fs::create_dir_all(&upper_dir).expect("Failed to create upper dir");
}

// WORK dir: r/w
fn create_work_layer(root_url: &Path) {
    let work_dir = root_url.join("work");
    fs::create_dir_all(&work_dir).expect("Failed to create work dir");
}

fn create_mount_point(root_path: &Path, mnt_path: &Path) {
    fs::create_dir_all(mnt_path).expect("Failed to create mount point");
    let output = Command::new("mount")
        .arg("-t")
        .arg("overlay")
        .arg("-o")
        .arg(format!(
            "lowerdir={},upperdir={},workdir={}",
            root_path.join("lower").to_str().unwrap(),
            root_path.join("upper").to_str().unwrap(),
            root_path.join("work").to_str().unwrap()
        ))
        .arg("none")
        .arg(mnt_path)
        .output()
        .expect("Failed to mount overlay");
    if !output.status.success() {
        output.stderr.iter().for_each(|&b| print!("{}", b as char));
        panic!("Failed to mount overlay fs");
    }
    info!("mount overlay fs success");
}

//TODO
pub fn delete_workspace(root_url: &Path, mnt_url: &Path) {
    umount(mnt_url).expect("Failed to umount");
    umount(root_url).expect("Failed to umount root");
    fs::remove_dir_all(root_url).expect("Failed to remove mount point");
}
