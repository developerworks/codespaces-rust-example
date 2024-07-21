use libc::{getrusage, RUSAGE_SELF};
use std::mem::MaybeUninit;

fn main() {
    let cpu_time = get_cpu_time();
    println!("Cpu time: {:?}", cpu_time);
}

// Get process cpu time: user time + system time
/// # Returns
///
/// Nano seconds of sum of user time and system time
///
/// ru_utime.tv_sec: Seconds of user cpu time
/// ru_utime.tv_usec: Micro seconds of user cpu time
/// ru_stime.tv_sec: Seconds of system cpu time
/// ru_stime.tv_usec: Micro seconds of system cpu time
pub fn get_cpu_time() -> Option<u64> {
    let usage = unsafe {
        let mut usage = MaybeUninit::uninit();
        if getrusage(RUSAGE_SELF, usage.as_mut_ptr()) != 0 {
            return None;
        }
        usage.assume_init()
    };
    // Nano seconds
    let user =
        1_000_000_000 * (usage.ru_utime.tv_sec as u64) + 1_000 * (usage.ru_utime.tv_usec as u64);
    let system =
        1_000_000_000 * (usage.ru_stime.tv_sec as u64) + 1_000 * (usage.ru_stime.tv_usec as u64);
    Some(user + system)
}
