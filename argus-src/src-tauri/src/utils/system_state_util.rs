use sysinfo::{Cpu, System};

/// 获取内存信息
/// 返回值第一个是总内存，第二个是剩余内存
pub fn get_memory() -> (u64, u64) {
    let mut system = System::new_all();

    system.refresh_memory();

    (
        system.total_memory() / 1024,
        system.available_memory() / 1024,
    )
}

/// 获取 CPU 信息
fn get_cpu_load() -> f32 {
    let mut sys = System::new();
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    // 因为 sys 虽然初始化，但只是数据结构的初始化，数据并不真实
    sys.refresh_cpu_usage();
    let total_load: f32 = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum();
    let avg_load = total_load / sys.cpus().len() as f32; // 平均 CPU 使用率
    avg_load
}

/// 获取占用百分比
pub fn get_memory_as_percentage() -> f32 {
    let (fm, am) = get_memory();
    (am as f32) / (fm as f32)
}

pub fn main() {
    let mut system = System::new_all();

    // 刷新系统信息
    system.refresh_all();

    // 获取 CPU 信息
    let cpus = system.cpus();
    println!("CPU 信息:");
    for (i, cpu) in cpus.iter().enumerate() {
        println!("核心 {}: {}%", i, cpu.cpu_usage());
    }

    // 获取总内存和可用内存
    println!(
        "总内存: {} MB, 可用内存: {} MB",
        system.total_memory() / 1024,
        system.available_memory() / 1024
    );

    // 操作系统信息
    // println!(
    //     "操作系统: {}, 版本: {}, 内核版本: {}",
    //     system.name().unwrap_or_else(|| "未知".into()),
    //     system.os_version().unwrap_or_else(|| "未知".into()),
    //     system.kernel_version().unwrap_or_else(|| "未知".into())
    // );
}

#[cfg(test)]
mod tests {
    use crate::utils::system_state_util::{get_memory, main};

    #[test]
    fn it_works() {
        // main()
        let (fm, am) = get_memory();
        println!("总:{},可用:{}", fm, am);

        println!("{}", (am as f32) / (fm as f32));
    }
}
