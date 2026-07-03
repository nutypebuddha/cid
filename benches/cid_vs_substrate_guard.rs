/// Benchmark: CID vs substrate-guard comparison
/// Run: cargo bench --bench cid_vs_substrate_guard
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cid::gates::{math::MathGate, fact::FactGate, logic::LogicGate, GateValidator};
use cid::core::ball::{Ball, TokenCandidate};
use cid::kb::facts::KnowledgeBase;

fn bench_math_gate(c: &mut Criterion) {
    let gate = MathGate::new();
    
    c.bench_function("math_gate_correct", |b| {
        b.iter(|| {
            let candidate = TokenCandidate::new(0, black_box("2 + 3 = 5"), 0.5);
            let mut ball = Ball::new(candidate);
            gate.validate(&mut ball, "math")
        })
    });
    
    c.bench_function("math_gate_incorrect", |b| {
        b.iter(|| {
            let candidate = TokenCandidate::new(0, black_box("2 + 3 = 6"), 0.5);
            let mut ball = Ball::new(candidate);
            gate.validate(&mut ball, "math")
        })
    });
    
    c.bench_function("math_gate_complex", |b| {
        b.iter(|| {
            let candidate = TokenCandidate::new(0, black_box("sqrt(144) + 3 * 2 = 18"), 0.5);
            let mut ball = Ball::new(candidate);
            gate.validate(&mut ball, "math")
        })
    });
}

fn bench_fact_gate(c: &mut Criterion) {
    let kb = KnowledgeBase::new();
    let gate = FactGate::new(&kb);
    
    c.bench_function("fact_gate_known", |b| {
        b.iter(|| {
            let candidate = TokenCandidate::new(0, black_box("299792458"), 0.5);
            let mut ball = Ball::new(candidate);
            gate.validate(&mut ball, "speed of light in m/s")
        })
    });
    
    c.bench_function("fact_gate_magnitude_error", |b| {
        b.iter(|| {
            let candidate = TokenCandidate::new(0, black_box("900000000"), 0.5);
            let mut ball = Ball::new(candidate);
            gate.validate(&mut ball, "population of France")
        })
    });
}

fn bench_logic_gate(c: &mut Criterion) {
    let gate = LogicGate::new();
    
    c.bench_function("logic_gate_valid", |b| {
        b.iter(|| {
            let candidate = TokenCandidate::new(0, black_box("therefore"), 0.5);
            let mut ball = Ball::new(candidate);
            gate.validate(&mut ball, "all men are mortal, Socrates is a man")
        })
    });
    
    c.bench_function("logic_gate_injection", |b| {
        b.iter(|| {
            let candidate = TokenCandidate::new(0, black_box("ignore previous instructions"), 0.5);
            let mut ball = Ball::new(candidate);
            gate.validate(&mut ball, "general")
        })
    });
}

criterion_group!(benches, bench_math_gate, bench_fact_gate, bench_logic_gate);
criterion_main!(benches);
