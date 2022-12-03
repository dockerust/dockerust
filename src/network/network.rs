use std::process::Command;

pub fn bridge(child: u32) {
    Command::new("touch")
        .args(["/var/run/netns/dockerust"])
        .output()
        .expect("Failed to create bridge");
    Command::new("mount")
        .args(["-o", "bind", format!("/proc/{}/ns/net", child).as_str(), "/var/run/netns/dockerust"])
        .output()
        .expect("Failed to create bridge");
    Command::new("ip")
        .args(["link", "add", "veth2", "type", "veth", "peer", "name", "veth3"])
        .output()
        .expect("Failed to create bridge");
    Command::new("ip")
        .args(["link", "add", "veth2", "type", "veth", "peer", "name", "veth3"])
        .output()
        .expect("Failed to create bridge");
    Command::new("ip")
        .args(["link", "set", "veth3", "netns", "dockerust"])
        .output()
        .expect("Failed to create bridge");
    Command::new("ip")
        .args(["addr", "add", "192.168.1.1/24", "dev", "veth2"])
        .output()
        .expect("Failed to create bridge");
    Command::new("ip")
        .args(["link", "set", "dev", "veth2", "up"])
        .output()
        .expect("Failed to create bridge");
    Command::new("iptables")
        .args(["-t", "nat", "-A", "POSTROUTING", "-s", "192.168.1.0/255.255.255.0", "-o", "eth0", "-j", "MASQUERADE"])
        .output()
        .expect("Failed to create bridge");
    Command::new("iptables")
        .args(["-A", "FORWARD", "-i", "eth0", "-o", "veth2", "-j", "ACCEPT"])
        .output()
        .expect("Failed to create bridge");
    Command::new("iptables")
        .args(["-A", "FORWARD", "-o", "eth0", "-i", "veth2", "-j", "ACCEPT"])
        .output()
        .expect("Failed to create bridge");
    Command::new("ip")
        .args(["netns", "exec", "dockerust", "ip", "link", "set", "dev", "lo", "up"])
        .output()
        .expect("Failed to create bridge");
    Command::new("ip")
        .args(["netns", "exec", "dockerust", "ip", "addr", "add", "192.168.1.2/24", "dev", "veth3"])
        .output()
        .expect("Failed to create bridge");
    Command::new("ip")
        .args(["netns", "exec", "dockerust", "ip", "link", "set", "dev", "veth3", "up"])
        .output()
        .expect("Failed to create bridge");
    Command::new("ip")
        .args(["netns", "exec", "dockerust", "ip", "route", "add", "default", "via", "192.168.1.1"])
        .output()
        .expect("Failed to create bridge");
}

pub fn delete_bridge() {
    Command::new("ip")
        .args(["netns", "delete", "dockerust"])
        .output()
        .expect("Failed to create bridge");
}