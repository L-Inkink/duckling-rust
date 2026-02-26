#!/bin/bash
# Direct parsing benchmark (no HTTP overhead)

echo "=== Direct Parsing Benchmark ==="
echo ""

# Compile benchmark
echo "Compiling benchmark..."
cat > /tmp/bench_parse.rs << 'EOF'
use std::time::Instant;
use rustling_ontology::parser::Parser;
use rustling_ontology::locale::Locale;
use rustling_ontology::-resources::DefaultResource;

fn main() {
    let resource = DefaultResource::new().unwrap();
    let locale = Locale::EN;
    let parser = Parser::new(&resource, locale);

    let queries = vec![
        "tomorrow",
        "today",
        "yesterday",
        "now",
        "3pm",
        "in 5 minutes",
        "3 days ago",
        "tomorrow at 3pm",
        "next monday at 9am",
        "every monday",
    ];

    let iterations = 1000;

    for query in &queries {
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = parser.parse(query);
        }
        let elapsed = start.elapsed().as_nanos() as u64 / iterations;
        println!("{}: {}ns ({}μs)", query, elapsed, elapsed / 1000);
    }
}
EOF

echo "Running direct parsing benchmark..."
cd /Users/link/Project/duckling-rust
cargo run --release --example bench_parse 2>/dev/null || echo "Example not found, using alternative method"
