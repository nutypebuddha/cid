use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cid::core::ball::{TokenCandidate, Ball};
use cid::gates::{math::MathGate, logic::LogicGate, fact::FactGate, confidence::ConfidenceGate, formal::FormalGate, GateValidator};

fn benchmark_math_gate(c: &mut Criterion) {
    let gate = MathGate::new();
    let candidate = TokenCandidate::new(0, "42", 0.5);
    
    c.bench_function("math_gate_validate", |b| {
        b.iter(|| {
            let mut ball = Ball::new(black_box(candidate.clone()));
            gate.validate(&mut ball, "number")
        })
    });
}

fn benchmark_logic_gate(c: &mut Criterion) {
    let gate = LogicGate::new();
    let candidate = TokenCandidate::new(0, "therefore", 0.5);
    
    c.bench_function("logic_gate_validate", |b| {
        b.iter(|| {
            let mut ball = Ball::new(black_box(candidate.clone()));
            gate.validate(&mut ball, "because all men are mortal")
        })
    });
}

fn benchmark_fact_gate(c: &mut Criterion) {
    let gate = FactGate::new();
    let candidate = TokenCandidate::new(0, "3.14159", 0.5);
    
    c.bench_function("fact_gate_validate", |b| {
        b.iter(|| {
            let mut ball = Ball::new(black_box(candidate.clone()));
            gate.validate(&mut ball, "pi is approximately")
        })
    });
}

fn benchmark_confidence_gate(c: &mut Criterion) {
    let gate = ConfidenceGate::new(0.5);
    let candidate = TokenCandidate::new(0, "42", 0.5);
    
    c.bench_function("confidence_gate_validate", |b| {
        b.iter(|| {
            let mut ball = Ball::new(black_box(candidate.clone()));
            gate.validate(&mut ball, "number")
        })
    });
}

fn benchmark_formal_gate(c: &mut Criterion) {
    let gate = FormalGate::new();
    let candidate = TokenCandidate::new(0, "For all x, P(x) implies Q(x)", 0.5);
    
    c.bench_function("formal_gate_validate", |b| {
        b.iter(|| {
            let mut ball = Ball::new(black_box(candidate.clone()));
            gate.validate(&mut ball, "theorem proof")
        })
    });
}

fn benchmark_full_validation(c: &mut Criterion) {
    let candidate = TokenCandidate::new(0, "42", 0.5);
    
    c.bench_function("full_validation_pipeline", |b| {
        b.iter(|| {
            let mut ball = Ball::new(black_box(candidate.clone()));
            
            let math_gate = MathGate::new();
            let logic_gate = LogicGate::new();
            let fact_gate = FactGate::new();
            let confidence_gate = ConfidenceGate::new(0.5);
            let formal_gate = FormalGate::new();
            
            let result1 = math_gate.validate(&mut ball, "number");
            ball.add_result(result1);
            
            let result2 = logic_gate.validate(&mut ball, "number");
            ball.add_result(result2);
            
            let result3 = fact_gate.validate(&mut ball, "number");
            ball.add_result(result3);
            
            let result4 = confidence_gate.validate(&mut ball, "number");
            ball.add_result(result4);
            
            let result5 = formal_gate.validate(&mut ball, "number");
            ball.add_result(result5);
            
            ball
        })
    });
}

criterion_group!(
    benches,
    benchmark_math_gate,
    benchmark_logic_gate,
    benchmark_fact_gate,
    benchmark_confidence_gate,
    benchmark_formal_gate,
    benchmark_full_validation
);
criterion_main!(benches);
