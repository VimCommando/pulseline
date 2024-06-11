use battery;
use psutil::cpu;
use psutil::memory;
use std::fmt::Write;
use std::{thread, time};

// Converts a floating point percent into an aproximate vertical block
fn percent_block(percent: f32) -> char {
    match percent as i32 {
        11..=21 => return '▁',
        22..=32 => return '▂',
        33..=43 => return '▃',
        44..=54 => return '▄',
        55..=65 => return '▅',
        66..=76 => return '▆',
        77..=87 => return '▇',
        88.. => return '█',
        _ => return ' ',
    }
}

// Poll system battery information and return battery percent and indicator string
fn get_battery_string() -> Option<String> {
    let battery_manager = match battery::Manager::new() {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Unable to create battery manager: {}", e);
            return None;
        }
    };

    // Get a batteries iterator
    let mut batteries = match battery_manager.batteries() {
        Ok(batteries) => batteries,
        Err(e) => {
            eprintln!("Unable to create batteries iterator: {}", e);
            return None;
        }
    };

    // Itterate to the first battery
    let battery = match batteries.next() {
        Some(Ok(battery)) => battery,
        Some(Err(e)) => {
            eprintln!("Unable to access battery information: {}", e);
            return None;
        }
        None => {
            eprintln!("Unable to find any batteries");
            return None;
        }
    };

    // Determine battery status indicator
    let heart = match battery.state() {
        battery::State::Charging => "♥",
        battery::State::Discharging => "♡",
        battery::State::Empty => "♡",
        battery::State::Full => "♥",
        battery::State::Unknown => "♥", // MacOS "charge hold" status lands here
        _ => "?",
    };

    // Return formatted battery String
    Some(format!(
        " {:.0}{}",
        battery.state_of_charge().value * 100.0,
        heart
    ))
}

fn main() {
    // Beware, this string will run out of room on a CPU with over 244 cores!
    let mut output = String::with_capacity(256);
    let mut cpu_percent_collector = cpu::CpuPercentCollector::new().expect("CPU percent collector");

    // Some time must pass to get CPU usage percentages
    let cpu_sample_pause = time::Duration::from_millis(500);
    thread::sleep(cpu_sample_pause);

    // Per-core CPU usage histogram
    let cpu_percents_percpu = match cpu_percent_collector.cpu_percent_percpu() {
        Ok(cpu) => cpu,
        Err(e) => {
            eprintln!("Unable to create cpu percent collector: {}", e);
            // Just return an empty vector if we can't create a per-CPU collector
            Vec::new()
        }
    };

    // Write the histogram blocks to the output
    for percent in cpu_percents_percpu {
        write!(output, "{}", percent_block(percent)).expect("write cpu histogram block");
    }

    // Write the normalized (max 100%) CPU usage to the output
    match cpu_percent_collector.cpu_percent() {
        Ok(cpu) => write!(output, "{:3.0}ℂ", cpu).expect("write cpu percent string"),
        Err(e) => eprintln!("Unable to determined cpu usage percent: {}", e),
    };

    // Write virtual memory usage to the output
    match memory::virtual_memory() {
        Ok(vm) => write!(output, " {:.0}ℝ", vm.percent()).expect("write memory percent string"),
        Err(e) => eprintln!("Unable to determine used memory percent: {}", e),
    };

    // Write the battery charge percentage and status indicator to the output
    match get_battery_string() {
        Some(batt_str) => write!(output, "{}", batt_str).expect("updated output string"),
        None => (),
    };

    // Finally print output
    print!("{}", output);
}
