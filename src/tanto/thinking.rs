// Tanto thinking frameworks — 6 reasoning frameworks adapted for CID
// OODA, SWOT, Cynefin, Five Whys, First Principles, Shu-Ha-Ri

/// Result of a thinking framework application
#[derive(Debug, Clone)]
pub struct ThinkResult {
    pub framework: String,
    pub header: String,
    pub body: String,
}

/// Apply a thinking framework to a problem
/// Format: "think ooda <problem>" or "think swot <subject> <context>"
pub fn think(args: &str) -> Option<ThinkResult> {
    let args = args.trim().as_bytes();
    let (framework, rest) = split_token(args)?;
    let framework_str = std::str::from_utf8(framework).ok()?;
    let problem = std::str::from_utf8(trim(rest)).unwrap_or("");

    let (header, body) = match framework {
        b"ooda" => ("=== THINK: OODA LOOP ===", OODA),
        b"shuhari" => ("=== THINK: SHU-HA-RI ===", SHUHARI),
        b"firstprinciples" => ("=== THINK: FIRST PRINCIPLES ===", FIRST_PRINCIPLES),
        b"cynefin" => ("=== THINK: CYNEFIN ===", CYNEFIN),
        b"why5" => ("=== THINK: FIVE WHYS ===", WHY5),
        b"swot" => ("=== THINK: SWOT ===", SWOT),
        _ => return None,
    };

    Some(ThinkResult {
        framework: framework_str.to_string(),
        header: header.to_string(),
        body: format_body(header, problem, body),
    })
}

/// List available thinking frameworks
pub fn list_frameworks() -> Vec<(&'static str, &'static str)> {
    vec![
        ("ooda <problem>", "Observe-Orient-Decide-Act loop"),
        ("swot <subject> [context]", "SWOT analysis"),
        ("cynefin <situation>", "Domain classification"),
        ("why5 <event>", "Root cause analysis"),
        ("firstprinciples <problem> [domain]", "Axiomatic reasoning"),
        ("shuhari <skill> [context]", "Stages of mastery"),
    ]
}

fn format_body(header: &str, problem: &str, body: &[u8]) -> String {
    let mut out = String::new();
    out.push_str(header);
    out.push('\n');

    // Different header details for different frameworks
    if header.starts_with("=== THINK: SHU-HA-RI") {
        let parts: Vec<&str> = problem.splitn(2, ' ').collect();
        let skill = parts.first().unwrap_or(&problem);
        let context = parts.get(1).unwrap_or(&"");
        out.push_str("  skill: ");
        out.push_str(skill);
        out.push('\n');
        out.push_str("  context: ");
        out.push_str(context);
        out.push('\n');
    } else if header.starts_with("=== THINK: FIRST PRINCIPLES") {
        let parts: Vec<&str> = problem.splitn(2, ' ').collect();
        let prob = parts.first().unwrap_or(&problem);
        let domain = parts.get(1).unwrap_or(&"");
        out.push_str("  problem: ");
        out.push_str(prob);
        out.push('\n');
        out.push_str("  domain: ");
        out.push_str(domain);
        out.push('\n');
    } else if header.starts_with("=== THINK: CYNEFIN") {
        out.push_str("  situation: ");
        out.push_str(problem);
        out.push('\n');
    } else if header.starts_with("=== THINK: FIVE WHYS") {
        out.push_str("  event: ");
        out.push_str(problem);
        out.push('\n');
    } else if header.starts_with("=== THINK: SWOT") {
        let parts: Vec<&str> = problem.splitn(2, ' ').collect();
        let subject = parts.first().unwrap_or(&problem);
        let context = parts.get(1).unwrap_or(&"");
        out.push_str("  subject: ");
        out.push_str(subject);
        out.push('\n');
        out.push_str("  context: ");
        out.push_str(context);
        out.push('\n');
    } else {
        out.push_str("  problem: ");
        out.push_str(problem);
        out.push('\n');
    }

    out.push('\n');
    out.push_str(std::str::from_utf8(body).unwrap_or(""));
    out
}

const OODA: &[u8] = b"OBSERVE:\n  Gather relevant data about the situation.\n  Consider: current state, key actors, available resources.\n  Use KB: lookup facts that apply (prices, specs, constants).\n  Observe changes, anomalies, and patterns in the environment.\n\nORIENT:\n  Analyze observations against prior knowledge.\n  Identify: causal relationships, feedback loops, leverage points.\n  Generate hypotheses about the system dynamics.\n  Use KB: cross-reference with known patterns and data.\n  Compute: quantify effects using tanto math where applicable.\n\nDECIDE:\n  Select the best course of action from options.\n  Evaluate: risk vs reward, speed vs accuracy.\n  Consider: multiple hypotheses, choose the most probable.\n  Use check: sanity-check the decision scale/scope.\n\nACT:\n  Execute the decision with clear steps.\n  Monitor: observe effects of action in real time.\n  Feed back: loop to Observe with new data.\n  Iterate: OODA is continuous, not one-shot.\n\n  OODA complete. Use: observations, hypotheses, decisions, actions.\n";

const SHUHARI: &[u8] = b"SHU (follow the rules):\n  Stage: beginner. Focus on learning the established patterns.\n  Goal: repeat until forms are automatic, no deviation.\n  Mindset: trust the teacher, do not question the method yet.\n  Metrics: accuracy of reproduction, speed of execution.\n  Risk: becoming rigid, unable to adapt.\n\nHA (break):\n  Stage: intermediate. Understand principles behind the rules.\n  Goal: adapt forms to context, bend rules deliberately.\n  Mindset: experiment within the framework, find edge cases.\n  Metrics: adaptation quality, problem-solving creativity.\n  Risk: half-baked innovation without mastery foundation.\n\nRI (leave):\n  Stage: master. Rules are internalized; create new patterns.\n  Goal: innovate beyond existing forms, teach others.\n  Mindset: no separation between self and practice.\n  Metrics: originality, mentorship impact, field advancement.\n  Risk: becoming unteachable, losing connection to foundations.\n\n  Shu-Ha-Ri complete. Current stage: Shu.\n  Use: shu_progress, ha_progress, ri_progress as metrics.\n";

const FIRST_PRINCIPLES: &[u8] = b"1. DECOMPOSE (break to fundamentals):\n   Identify all assumptions about the problem.\n   Strip away: convention, received wisdom, \"how it's done.\"\n   Ask: what are the irreducible elements?\n   Use KB: fundamental constants (c, G, h, kB) as atomic truths.\n\n2. VERIFY (check fundamentals):\n   Test each element: is this truly fundamental?\n   Or is it a derived truth that could break?\n   Cross-reference: KB entries, established laws.\n   Compute: verify constraints with math expressions.\n\n3. RECONSTRUCT (build from axioms):\n   Starting from verified fundamentals, rebuild the solution.\n   Each step must follow logically from prior steps.\n   Use formula: verified formulas prevent structure errors.\n   Verify: check each derived result against expectations.\n\n  First Principles complete. Use: axioms, reconstruction.\n";

const CYNEFIN: &[u8] = b"DOMAINS -- classify the situation:\n\nA) CLEAR (known-knowns):\n   Cause-and-effect obvious to all.\n   Response: SENSE -> CATEGORIZE -> RESPOND\n   Best practice: apply established procedure.\n   Risk: complacency, over-simplification.\n   Score: 0-25% -- is this a routine situation?\n\nB) COMPLICATED (known-unknowns):\n   Cause-and-effect requires expert analysis.\n   Response: SENSE -> ANALYZE -> RESPOND\n   Good practice: expert diagnosis, multiple options.\n   Risk: analysis paralysis, expert bias.\n   Score: 25-50% -- does it need expert input?\n\nC) COMPLEX (unknown-unknowns):\n   Cause-and-effect only visible in retrospect.\n   Response: PROBE -> SENSE -> RESPOND\n   Emergent practice: experiment, observe, adapt.\n   Risk: false pattern recognition, over-experimentation.\n   Score: 50-75% -- is the outcome unpredictable?\n\nD) CHAOTIC (unknowables):\n   Cause-and-effect impossible to determine.\n   Response: ACT -> SENSE -> RESPOND\n   Novel practice: act quickly to stabilize, then diagnose.\n   Risk: authoritarian response, over-reaction.\n   Score: 75-100% -- is immediate action needed?\n\nE) DISORDER (unknown which domain):\n   Default state when domain is unclear.\n   Response: break into sub-problems, classify each.\n   Risk: each domain pulls toward its default response.\n\n  Cynefin complete. Recommended: score each domain for fit.\n  Use: classify sub-problems separately, then synthesize.\n";

const WHY5: &[u8] = b"WHY 1 (symptom):\n  The immediate, visible failure or event.\n  Describe: what happened, when, where, impact.\n\n  Why did this happen? ->\n\nWHY 2 (direct cause):\n  The direct physical or procedural cause.\n  Describe: which specific action or condition triggered it.\n\n  Why did that cause exist? ->\n\nWHY 3 (systemic cause):\n  The system-level condition that allowed the direct cause.\n  Describe: processes, training, tools, incentives.\n\n  Why was the system designed that way? ->\n\nWHY 4 (organizational cause):\n  The management/org factors behind the system design.\n  Describe: priorities, culture, resource allocation.\n\n  Why were those priorities chosen? ->\n\nWHY 5 (root cause):\n  The fundamental issue -- often cultural or structural.\n  This is where corrective action must target.\n\n  Once identified: define countermeasures per layer.\n\n  Five Whys complete. Root cause identified.\n  Fix: address layers 1-5 with countermeasures.\n";

const SWOT: &[u8] = b"STRENGTHS (internal, helpful):\n  Resources: capital, talent, IP, technology.\n  Capabilities: speed, quality, scale, brand.\n  Advantages: cost, differentiation, network effects.\n  Use KB: look up relevant benchmarks and specs.\n  Score each: 1-10 on competitive advantage.\n\nWEAKNESSES (internal, harmful):\n  Gaps: missing skills, resources, technology.\n  Constraints: capacity, geography, regulation.\n  Vulnerabilities: dependencies, single points of failure.\n  Compute: quantify impact of each weakness.\n  Priority: which weaknesses are most urgent?\n\nOPPORTUNITIES (external, helpful):\n  Market: growth segments, new customers.\n  Technology: emerging tools, platforms.\n  Trends: regulatory shifts, cultural changes.\n  Use estimate: magnitude of each opportunity.\n  Compute: ROI, time-to-value, probability.\n\nTHREATS (external, harmful):\n  Competition: direct, adjacent, disruptive.\n  Environment: economic, political, environmental.\n  Risks: technology obsolescence, regulatory change.\n  Use check: sanity-check threat severity.\n  Mitigation: plan for each high-probability threat.\n\n  SWOT complete. Cross-map: SxO strategies, WxT mitigations.\n  Use: strength, weakness, opportunity, threat vectors.\n";

fn trim(s: &[u8]) -> &[u8] {
    let mut start = 0;
    while start < s.len() && (s[start] == b' ' || s[start] == b'\t') { start += 1; }
    let mut end = s.len();
    while end > start && (s[end - 1] == b' ' || s[end - 1] == b'\t') { end -= 1; }
    &s[start..end]
}

fn split_token(s: &[u8]) -> Option<(&[u8], &[u8])> {
    let s = trim(s);
    if s.is_empty() { return None; }
    let mut i = 0;
    while i < s.len() && s[i] != b' ' && s[i] != b'\t' { i += 1; }
    if i >= s.len() { return Some((s, &[])); }
    Some((&s[..i], &s[i..]))
}
