use log::info;
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::prelude::*;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting measurement");
    let output = get_output().await;
    let output = std::str::from_utf8(&output.stdout).unwrap();
    info!("Finished Measurement");

    let data: Value = serde_json::from_str(output).unwrap();
    let output = format!(
        "{}, {}, {}, {}\n",
        data["ping"]["latency"],
        to_mb(data["download"]["bandwidth"].as_f64().unwrap()),
        to_mb(data["upload"]["bandwidth"].as_f64().unwrap()),
        data["timestamp"].as_str().unwrap(),

    );
    println!("{}", output);
    log_file("main_log.csv", &output);
}

async fn get_output() -> std::process::Output {
    std::process::Command::new("speedtest")
        .arg("--format=json")
        .output()
        .expect("Speedtest failed!")
}

fn log_file(fname: &str, output: &str) {
    let path = format!("logs/{}", fname);
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .open(path)
        .expect("Failed opening file");
    file.write_all(output.as_bytes()).expect("Failed writing");
}

fn to_mb(x: f64) -> f64 {
    x / 1_000_000.
}
