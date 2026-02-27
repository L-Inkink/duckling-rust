//! gRPC Performance Benchmark
//!
//! Starts an in-process gRPC server, then hammers it with concurrent clients.
//! Measures latency (P50 / P95 / P99) and throughput (req/s).
//!
//! Run:
//!   cargo run --release --features grpc --bin grpc_bench

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::net::TcpListener;
use tokio::sync::Barrier;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::{Channel, Server};

use rustling::grpc_proto::{
    parser_client::ParserClient,
    parser_server::ParserServer,
    BatchParseRequest, ParseRequest,
};
use rustling::grpc::{GrpcAppState, ParserService};

// ── helpers ──────────────────────────────────────────────────────────────────

/// Compute percentile (0-100) from a sorted Vec<f64> of microsecond latencies.
fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let idx = ((p / 100.0) * (sorted.len() - 1) as f64).round() as usize;
    sorted[idx.min(sorted.len() - 1)]
}

/// Start a gRPC server on a random port; return the address string.
async fn spawn_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind failed");
    let addr: SocketAddr = listener.local_addr().unwrap();

    let service = ParserService::new(GrpcAppState::new());
    tokio::spawn(async move {
        Server::builder()
            .add_service(ParserServer::new(service))
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await
            .ok();
    });

    // Give server a moment to be ready
    tokio::time::sleep(Duration::from_millis(50)).await;
    format!("http://{}", addr)
}

/// Create a connected gRPC client channel.
async fn make_client(addr: &str) -> ParserClient<Channel> {
    ParserClient::connect(addr.to_string())
        .await
        .expect("connect failed")
}

// ── benchmark scenarios ───────────────────────────────────────────────────────

/// Scenario A: Sequential single-client latency baseline
async fn bench_sequential(addr: &str, n: usize) -> Vec<f64> {
    let mut client = make_client(addr).await;
    let mut latencies = Vec::with_capacity(n);

    for _ in 0..n {
        let t = Instant::now();
        client
            .parse(ParseRequest {
                text: "42".to_string(),
                locale: "en".to_string(),
            })
            .await
            .unwrap();
        latencies.push(t.elapsed().as_micros() as f64);
    }
    latencies
}

/// Scenario B: Concurrent clients, each sends `requests_per_client` parses.
/// Returns (total_requests, elapsed_secs, all_latencies).
async fn bench_concurrent(
    addr: &str,
    concurrency: usize,
    requests_per_client: usize,
) -> (usize, f64, Vec<f64>) {
    let barrier = Arc::new(Barrier::new(concurrency));
    let addr = addr.to_string();

    let handles: Vec<_> = (0..concurrency)
        .map(|_| {
            let addr = addr.clone();
            let barrier = Arc::clone(&barrier);
            tokio::spawn(async move {
                let mut client = make_client(&addr).await;
                barrier.wait().await; // all clients fire together
                let mut lats = Vec::with_capacity(requests_per_client);
                for _ in 0..requests_per_client {
                    let t = Instant::now();
                    client
                        .parse(ParseRequest {
                            text: "42".to_string(),
                            locale: "en".to_string(),
                        })
                        .await
                        .unwrap();
                    lats.push(t.elapsed().as_micros() as f64);
                }
                lats
            })
        })
        .collect();

    let start = Instant::now();
    let mut all_latencies = Vec::new();
    for h in handles {
        let lats = h.await.unwrap();
        all_latencies.extend(lats);
    }
    let elapsed = start.elapsed().as_secs_f64();

    (concurrency * requests_per_client, elapsed, all_latencies)
}

/// Scenario C: Batch parse — measure latency as batch size grows.
async fn bench_batch(addr: &str, batch_size: usize, repeats: usize) -> Vec<f64> {
    let mut client = make_client(addr).await;
    let texts: Vec<String> = (0..batch_size).map(|i| format!("{}", i * 7 + 1)).collect();
    let mut latencies = Vec::with_capacity(repeats);

    for _ in 0..repeats {
        let t = Instant::now();
        client
            .parse_batch(BatchParseRequest {
                texts: texts.clone(),
                locale: "en".to_string(),
            })
            .await
            .unwrap();
        latencies.push(t.elapsed().as_micros() as f64);
    }
    latencies
}

/// Scenario D: Sustained load — measure RPS over a fixed window.
async fn bench_sustained_rps(addr: &str, concurrency: usize, duration_secs: u64) -> (u64, f64) {
    let counter = Arc::new(std::sync::atomic::AtomicU64::new(0));
    let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));

    let handles: Vec<_> = (0..concurrency)
        .map(|_| {
            let addr = addr.to_string();
            let counter = Arc::clone(&counter);
            let stop = Arc::clone(&stop);
            tokio::spawn(async move {
                let mut client = make_client(&addr).await;
                while !stop.load(std::sync::atomic::Ordering::Relaxed) {
                    client
                        .parse(ParseRequest {
                            text: "5 minutes".to_string(),
                            locale: "en".to_string(),
                        })
                        .await
                        .ok();
                    counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
            })
        })
        .collect();

    let start = Instant::now();
    tokio::time::sleep(Duration::from_secs(duration_secs)).await;
    stop.store(true, std::sync::atomic::Ordering::Relaxed);

    for h in handles {
        let _ = h.await;
    }
    let elapsed = start.elapsed().as_secs_f64();
    let total = counter.load(std::sync::atomic::Ordering::Relaxed);
    (total, elapsed)
}

// ── formatting helpers ────────────────────────────────────────────────────────

fn print_latency_table(header: &str, latencies: &mut Vec<f64>) {
    latencies.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mean = latencies.iter().sum::<f64>() / latencies.len() as f64;
    println!(
        "  {:<36}  mean={:>7.0}µs  P50={:>7.0}µs  P95={:>7.0}µs  P99={:>7.0}µs  max={:>8.0}µs",
        header,
        mean,
        percentile(latencies, 50.0),
        percentile(latencies, 95.0),
        percentile(latencies, 99.0),
        latencies.last().copied().unwrap_or(0.0),
    );
}

fn sep() {
    println!("{}", "─".repeat(90));
}

// ── main ──────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    let _ = env_logger::try_init();
    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║          Rustling gRPC Performance Benchmark                 ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    // ── Warm up ──────────────────────────────────────────────────────────────
    let addr = spawn_server().await;
    {
        let mut c = make_client(&addr).await;
        for _ in 0..50 {
            c.parse(ParseRequest { text: "42".to_string(), locale: "en".to_string() })
                .await
                .ok();
        }
    }
    println!("Server ready at {}  (50 warm-up requests done)", addr);
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // A. Sequential baseline (single client)
    // ═══════════════════════════════════════════════════════════════════════
    sep();
    println!("A. Sequential single-client latency  (1000 requests)");
    sep();

    let mut seq_int = bench_sequential(&addr, 1000).await;
    print_latency_table("parse integer \"42\"", &mut seq_int);

    // Duration parse
    {
        let mut client = make_client(&addr).await;
        let mut lats: Vec<f64> = Vec::new();
        for _ in 0..1000 {
            let t = Instant::now();
            client
                .parse(ParseRequest { text: "5 minutes".to_string(), locale: "en".to_string() })
                .await
                .unwrap();
            lats.push(t.elapsed().as_micros() as f64);
        }
        print_latency_table("parse duration \"5 minutes\"", &mut lats);
    }

    // Mixed text (multiple entities)
    {
        let mut client = make_client(&addr).await;
        let mut lats: Vec<f64> = Vec::new();
        for _ in 0..500 {
            let t = Instant::now();
            client
                .parse(ParseRequest {
                    text: "Meeting at 3pm for 45 minutes, invite 12 people".to_string(),
                    locale: "en".to_string(),
                })
                .await
                .unwrap();
            lats.push(t.elapsed().as_micros() as f64);
        }
        print_latency_table("parse mixed (multi-entity, 500 req)", &mut lats);
    }

    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // B. Concurrent clients — throughput
    // ═══════════════════════════════════════════════════════════════════════
    sep();
    println!("B. Concurrent client throughput  (requests per client: 200)");
    println!("   {:>12}  {:>12}  {:>12}  {:>12}  {:>12}", "Concurrency", "Total req", "Elapsed(s)", "Throughput", "P99 lat");
    sep();

    for concurrency in [1, 5, 10, 25, 50, 100] {
        let (total, elapsed, mut lats) = bench_concurrent(&addr, concurrency, 200).await;
        lats.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let rps = total as f64 / elapsed;
        println!(
            "   {:>12}  {:>12}  {:>12.2}  {:>10.0} r/s  {:>9.0}µs",
            concurrency,
            total,
            elapsed,
            rps,
            percentile(&lats, 99.0),
        );
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // C. Batch parse — latency vs batch size
    // ═══════════════════════════════════════════════════════════════════════
    sep();
    println!("C. Batch parse latency  (repeats: 200 per batch size)");
    println!("   {:>12}  {:>14}  {:>12}  {:>12}  {:>16}", "Batch size", "Total items", "P50/call", "P99/call", "Per-item (P50)");
    sep();

    for &bs in &[1usize, 4, 8, 16, 32, 64] {
        let mut lats = bench_batch(&addr, bs, 200).await;
        lats.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let p50 = percentile(&lats, 50.0);
        let p99 = percentile(&lats, 99.0);
        println!(
            "   {:>12}  {:>14}  {:>10.0}µs  {:>10.0}µs  {:>14.1}µs",
            bs,
            bs * 200,
            p50,
            p99,
            p50 / bs as f64,
        );
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // D. Sustained RPS (5-second window, various concurrency)
    // ═══════════════════════════════════════════════════════════════════════
    sep();
    println!("D. Sustained RPS over 5-second window");
    println!("   {:>12}  {:>12}  {:>14}", "Concurrency", "Total req", "RPS");
    sep();

    for concurrency in [10, 50, 100] {
        let (total, elapsed) = bench_sustained_rps(&addr, concurrency, 5).await;
        println!(
            "   {:>12}  {:>12}  {:>12.0} r/s",
            concurrency,
            total,
            total as f64 / elapsed
        );
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // E. Multi-locale comparison
    // ═══════════════════════════════════════════════════════════════════════
    sep();
    println!("E. Multi-locale latency comparison  (200 requests each)");
    sep();

    let locale_cases = [
        ("en", "42"),
        ("fr", "quarante-deux"),
        ("de", "vierzig"),
        ("zh", "四十二"),
        ("es", "cuarenta y dos"),
    ];
    for (locale, text) in locale_cases {
        let mut client = make_client(&addr).await;
        let mut lats: Vec<f64> = Vec::new();
        for _ in 0..200 {
            let t = Instant::now();
            client
                .parse(ParseRequest { text: text.to_string(), locale: locale.to_string() })
                .await
                .unwrap();
            lats.push(t.elapsed().as_micros() as f64);
        }
        let label = format!("locale={} text=\"{}\"", locale, text);
        print_latency_table(&label, &mut lats);
    }
    println!();

    sep();
    println!("Benchmark complete.");
    sep();
    println!();
}
