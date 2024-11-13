use battery;
use psutil::{cpu, memory};
use std::fmt::Write;

/// Converts a floating point percent into an aproximate vertical block
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

/// Poll system battery information and return battery percent and indicator string
fn get_battery_string() -> Result<String, String> {
    let battery_manager =
        battery::Manager::new().map_err(|e| format!("Failed to create battery manager: {e}"))?;

    // Get a batteries iterator
    let mut batteries = battery_manager
        .batteries()
        .map_err(|e| format!("Failed to create batteries iterator: {e}"))?;

    // Itterate to the first battery
    let battery = match batteries.next() {
        Some(battery) => {
            battery.map_err(|e| format!("Failed to access battery information: {e}"))?
        }
        None => {
            return Err(format!("Failed to find any batteries"));
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
    Ok(format!(
        " {:.0}{}",
        battery.state_of_charge().value * 100.0,
        heart
    ))
}

/// Prints a string with a CPU core histogram, CPU percent, RAM percent, and
/// battery percent. Sleeps for 500ms to collect CPU stats.
fn main() {
    let newline = std::env::args().any(|a| a == "-n");

    // Beware, this string will run out of room on a CPU with over 244 cores!
    let mut output = String::with_capacity(256);
    let mut cpu_percent_collector =
        cpu::CpuPercentCollector::new().expect("Failed to create CPU percent collector");

    // Some time must pass to get CPU usage percentages
    let cpu_sample_pause = std::time::Duration::from_millis(500);
    std::thread::sleep(cpu_sample_pause);

    // Per-core CPU usage histogram
    let cpu_percents_percpu = match cpu_percent_collector.cpu_percent_percpu() {
        Ok(cpu) => cpu,
        Err(e) => {
            eprintln!("Failed to create cpu percent collector: {}", e);
            // Just return an empty vector if we can't create a per-CPU collector
            Vec::new()
        }
    };

    // Write the histogram blocks to the output
    for percent in cpu_percents_percpu {
        write!(output, "{}", percent_block(percent)).expect("Failed writing CPU histogram block");
    }

    // Write the normalized (max 100%) CPU usage to the output
    match cpu_percent_collector.cpu_percent() {
        Ok(cpu) => write!(output, "{:3.0}ℂ", cpu).expect("Failed writing CPU percent string"),
        Err(e) => eprintln!("Failed to determine cpu usage percent: {}", e),
    };

    // Write virtual memory usage to the output
    match memory::virtual_memory() {
        Ok(vm) => {
            write!(output, " {:.0}ℝ", vm.percent()).expect("Failed writing memory percent string")
        }
        Err(e) => eprintln!("Failed to determine used memory percent: {}", e),
    };

    // Write the battery charge percentage and status indicator to the output
    match get_battery_string() {
        Ok(batt_str) => write!(output, "{}", batt_str).expect("Failed to update output string"),
        Err(_) => (), // Ignore not having any batteries
    };

    // Finally print output
    match newline {
        false => print!("{output}"),
        true => println!("{output}"),
    }
}
