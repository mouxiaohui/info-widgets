use serde::Serialize;
use systemstat::{saturating_sub_bytes, Duration, Platform, System};
use tokio::time::sleep;

#[derive(Serialize)]
struct Memory {
    percentage: f64,
    total: String,
    used: String,
}

#[derive(Serialize)]
pub struct SystemInfo {
    cpu_load: Option<f64>,
    memory: Option<Memory>,
    battery_remaining_capacity: Option<u32>,
}

impl SystemInfo {
    pub async fn get() -> Self {
        let sys = System::new();

        let cpu_load = match sys.cpu_load() {
            Ok(delay_cpus) => {
                // You need to wait some time (about a second is good)
                // before unwrapping the DelayedMeasurement with .done().
                sleep(Duration::from_secs(1)).await;

                let mut idles = 0f32;
                if let Ok(cpus) = delay_cpus.done() {
                    for (i, cpu) in cpus.iter().enumerate() {
                        if i == 0 {
                            idles = cpu.idle
                        } else {
                            idles = (idles + cpu.idle) / 2.0
                        }
                    }
                }

                Some(round_2_decimal((1.0 - idles).into()))
            }
            Err(_) => None,
        };

        let memory = match sys.memory() {
            Ok(mem) => {
                let total = mem.total;
                let used = saturating_sub_bytes(total, mem.free);
                let per = used.as_u64() as f64 / total.as_u64() as f64;
                Some(Memory {
                    percentage: round_2_decimal(per),
                    total: total.to_string(),
                    used: used.to_string(),
                })
            }
            Err(_) => None,
        };

        let battery_remaining_capacity = match sys.battery_life() {
            Ok(battery) => {
                let cap = battery.remaining_capacity * 100.0;
                Some(cap as u32)
            }
            Err(_) => None,
        };

        // println!("{}", cpu_load.unwrap());

        Self {
            cpu_load,
            memory,
            battery_remaining_capacity,
        }
    }
}

fn round_2_decimal(number: f64) -> f64 {
    (number * 100.0).round() / 100.0
}