use criterion::{Criterion, criterion_group, criterion_main};
use std::process::Command;

macro_rules! run_with_args {
    ($c:expr, $name:expr, $args:expr) => {
        $c.bench_function($name, |b| {
            b.iter(|| {
                Command::new("target/release/iti-lot-simulator")
                    .args($args)
                    .output()
                    .expect("Failed to execute command");
            })
        });
    };
}

fn defaults(c: &mut Criterion) {
    let mut group = c.benchmark_group("defaults");
    run_with_args!(group, "faithful", &["3", "--faithful"]);
    run_with_args!(group, "standard", &["3"]);
    run_with_args!(group, "continuous", &["3", "-c"]);
    run_with_args!(group, "standard_binary_search", &["3", "-b"]);
    run_with_args!(group, "continuous_binary_search", &["3", "-bc"]);
    group.finish();
}

fn fifty(c: &mut Criterion) {
    let mut group = c.benchmark_group("fifty");
    run_with_args!(group, "standard", &["50"]);
    run_with_args!(group, "continuous", &["50", "-c"]);
    run_with_args!(group, "standard_binary_search", &["50", "-b"]);
    run_with_args!(group, "continuous_binary_search", &["50", "-bc"]);
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = defaults, fifty,
}
criterion_main!(benches);
