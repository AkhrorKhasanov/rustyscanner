use clap::Parser;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::time::{timeout, Duration};

#[derive(Parser, Debug)]
#[command(author, version, about = "A lightning-fast port scanner in Rust")]
struct Args {
    /// Target IP address
    #[arg(short, long)]
    ip: String,

    /// Port range (e.g., 1-1024)
    #[arg(short, long)]
    range: String,

    /// Number of concurrent connections
    #[arg(short, long, default_value_t = 500)]
    concurrency: usize,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let ports: Vec<&str> = args.range.split('-').collect();
    let start_port: u16 = ports[0].parse().expect("Invalid start port");
    let end_port: u16 = ports[1].parse().expect("Invalid end port");

    println!("Scanning {} on ports {}-{}...", args.ip, start_port, end_port);

    // Concurrency cheklovi (semaphor)
    let semaphore = Arc::new(Semaphore::new(args.concurrency));
    let mut tasks = Vec::new();

    for port in start_port..=end_port {
        let ip = args.ip.clone();
        let permit = semaphore.clone().acquire_owned().await.unwrap();

        let task = tokio::spawn(async move {
            let _permit = permit; // Task tugagach permit bo'shatiladi
            scan_port(&ip, port).await;
        });
        tasks.push(task);
    }

    for task in tasks {
        let _ = task.await;
    }
}

async fn scan_port(ip: &str, port: u16) {
    let address = format!("{}:{}", ip, port);
    match timeout(Duration::from_secs(1), TcpStream::connect(&address)).await {
        Ok(Ok(_)) => println!("Port {}: OPEN", port),
        Ok(Err(_)) => {}, 
        Err(_) => {},     
    }
}