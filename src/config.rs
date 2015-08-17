use std::collections::HashMap;

type FreezerState = String;
type Action = i64;
type Operator = i64;
type FileMode = u32;
type NamespaceType = String;
type Namespaces = Vec<Namespace>;

struct Cgroup {
    name: String,
    parent: String,
    allow_all_devices: bool,
    allowed_devices: Vec<Device>,
    denied_devices: Vec<Device>,
    memory: i64,
    memory_reservation: i64,
    memory_swap: i64,
    kernel_memory: i64,
    cpu_shares: i64,
    cpu_period: i64,
    cpu_rt_runtime: i64,
    cpu_rt_period: i64,
    cpuset_cpus: String,
    cpuset_mems: String,
    blkio_throttle_read_bps_device: String,
    blkio_throttle_write_bps_device: String,
    blkio_throttle_read_iops_device: String,
    blkio_throttle_write_iops_device: String,
    blkio_weight: i64,
    blkio_weight_device: String,
    freezer: FreezerState,
    hugetlb_limit: Vec<HugepageLimit>,
    slice: String,
    oom_kill_disable: bool,
    memory_swappiness: i64,
    net_prio_ifpriomap: Vec<IfPrioMap>,
    net_cls_classid: String,
}

struct Rlimit {
    limit_type: i64,
    hard: u64,
    soft: u64,
}

struct IDMap {
    container_id: i64,
    host_id: i64,
    size: i64,
}

struct Seccomp {
    syscalls: Vec<Syscall>
}

struct Arg {
    index: i64,
    value: u32,
    op: Operator,
}

struct Syscall {
    value: i64,
    action: Action,
    args: Vec<Arg>,
}

struct Config {
    no_pivot_root: bool,
    parent_death_signal: i64,
    pivot_dir: String,
    rootfs: String,
    readonlyfs: bool,
    privatefs: bool,
    mounts: Vec<Mount>,
    devices: Vec<Device>,
    mount_label: String,
    hostname: String,
    namespaces: Namespaces,
    capabilities: Vec<Capabilities>,
    networks: Vec<Network>,
    routes: Vec<Route>,
    cgroups: Vec<Cgroup>,
    apparmor_profile: String,
    process_label: String,
    rlimits: Rlimit,
    additional_groups: Vec<String>,
    uid_mappings: Vec<IDMap>,
    gid_mappings: Vec<IDMap>,
    mask_paths: Vec<String>,
    readonly_paths: Vec<String>,
    sysctl: HashMap<String, String>,
    seccomp: Vec<Seccomp>,
}

struct Device {
    device_type: rune,
    path: String,
    major: i64,
    minor: i64,
    permissions: String,
    file_mode: FileMode,
    uid: u32,
    gid: u32,
}

struct HugepageLimit {
    page_size: String,
    limit: u64,
}

struct IfPrioMap {
    interface: String,
    priority: i64,
}

struct Mount {
    source: String,
    destination: String,
    device: String,
    flags: i64,
    data: String,
    relabel: String,
    premount_cmds: Vec<Command>,
    postmount_cmds: Vec<Command>,
}

struct Command {
    path: String,
    args: Vec<String>,
    env: Vec<String>,
    dir: String,
}

// TODO https://github.com/opencontainers/runc/blob/master/libcontainer/configs/namespaces_syscall.go

struct Namespace {
    namespace_type: NamespaceType,
    path: String,
}

struct Network {
    network_type: String,
    name: String,
    bridge: String,
    mac_address: String,
    address: String,
    gateway: String,
    ipv6_address: String,
    ipv6_gateway: String,
    mtu: i64,
    txqueuelen: i64,
    host_interface_name: String,
    hairpin_mode: bool,
}

struct Route {
    destination: String,
    source: String,
    gateway: String,
    interface_name: String,
}
