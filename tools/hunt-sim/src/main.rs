use rally_core::{
    percent_of, ActorTrace, ComparisonDelta, ComparisonReport, RunSeed, SimulationMetric,
    SimulationRun, ValidationFinding, ValidationReport,
};
use std::env;

#[derive(Debug, Clone)]
struct Puzzle {
    id: &'static str,
    title: &'static str,
    difficulty: u32,
    answer_len: u32,
}

#[derive(Debug, Clone)]
struct SolverTeam {
    id: &'static str,
    skill: u32,
    parallelism: usize,
    trace: ActorTrace,
}

#[derive(Debug, Clone, Copy)]
struct TeamProfile {
    id: &'static str,
    skill: u32,
    parallelism: usize,
}

#[derive(Debug, Clone)]
struct SolvedPuzzle {
    id: &'static str,
    title: &'static str,
    minutes: u32,
    hints: u32,
}

#[derive(Debug, Clone)]
struct HuntResult {
    run: SimulationRun,
    solved: Vec<SolvedPuzzle>,
    meta_minutes: u32,
    total_minutes: u32,
    team: SolverTeam,
    metrics: Vec<SimulationMetric>,
    report: ValidationReport,
}

#[derive(Debug, Clone)]
struct BatchSummary {
    runs: usize,
    average_minutes: f64,
    p95_minutes: u32,
    pass_rate: f64,
    average_hints: f64,
    bottlenecks: Vec<(&'static str, u32)>,
}

#[derive(Debug, Clone, Copy)]
struct HuntVariant {
    name: &'static str,
    hard_feeder_relief: u32,
    hint_threshold_shift: i32,
    meta_base_reduction: u32,
    parallelism_bonus: usize,
}

#[derive(Debug, Clone)]
struct GameNightModule {
    id: &'static str,
    game: &'static str,
    answer: &'static str,
    author: &'static str,
    base_minutes: u32,
    quality: u32,
    communication: u32,
    rework_risk: u32,
}

#[derive(Debug, Clone)]
struct GameNightResult {
    run: SimulationRun,
    modules: Vec<ModuleResult>,
    meta_minutes: u32,
    total_minutes: u32,
    metrics: Vec<SimulationMetric>,
    report: ValidationReport,
}

#[derive(Debug, Clone)]
struct ModuleResult {
    id: &'static str,
    game: &'static str,
    answer: &'static str,
    author: &'static str,
    minutes: u32,
    quality: u32,
    reworked: bool,
    visible_to_admin: bool,
    trace: ActorTrace,
}

#[derive(Debug, Clone)]
struct GameNightSummary {
    runs: usize,
    average_minutes: f64,
    p95_minutes: u32,
    pass_rate: f64,
    average_reworks: f64,
    visibility_gap_rate: f64,
}

#[derive(Debug, Clone, Copy)]
struct GameNightVariant {
    name: &'static str,
    communication_bonus: u32,
    editorial_relief: u32,
    meta_clarity: u32,
    parallel_review: bool,
}

const BASELINE: HuntVariant = HuntVariant {
    name: "baseline",
    hard_feeder_relief: 0,
    hint_threshold_shift: 0,
    meta_base_reduction: 0,
    parallelism_bonus: 0,
};

const VARIANTS: &[HuntVariant] = &[
    BASELINE,
    HuntVariant {
        name: "stronger-hints",
        hard_feeder_relief: 0,
        hint_threshold_shift: -10,
        meta_base_reduction: 0,
        parallelism_bonus: 0,
    },
    HuntVariant {
        name: "p5-p6-clue-relief",
        hard_feeder_relief: 1,
        hint_threshold_shift: 0,
        meta_base_reduction: 0,
        parallelism_bonus: 0,
    },
    HuntVariant {
        name: "meta-prop-clarity",
        hard_feeder_relief: 0,
        hint_threshold_shift: 0,
        meta_base_reduction: 8,
        parallelism_bonus: 0,
    },
    HuntVariant {
        name: "team-parallelism",
        hard_feeder_relief: 0,
        hint_threshold_shift: 0,
        meta_base_reduction: 0,
        parallelism_bonus: 1,
    },
    HuntVariant {
        name: "guided-final-set",
        hard_feeder_relief: 1,
        hint_threshold_shift: -8,
        meta_base_reduction: 4,
        parallelism_bonus: 0,
    },
];

const GAME_NIGHT_BASELINE: GameNightVariant = GameNightVariant {
    name: "baseline",
    communication_bonus: 0,
    editorial_relief: 0,
    meta_clarity: 0,
    parallel_review: false,
};

const GAME_NIGHT_VARIANTS: &[GameNightVariant] = &[
    GAME_NIGHT_BASELINE,
    GameNightVariant {
        name: "editorial-pass",
        communication_bonus: 0,
        editorial_relief: 2,
        meta_clarity: 0,
        parallel_review: false,
    },
    GameNightVariant {
        name: "admin-standups",
        communication_bonus: 2,
        editorial_relief: 0,
        meta_clarity: 0,
        parallel_review: false,
    },
    GameNightVariant {
        name: "meta-briefing",
        communication_bonus: 1,
        editorial_relief: 0,
        meta_clarity: 2,
        parallel_review: false,
    },
    GameNightVariant {
        name: "parallel-review",
        communication_bonus: 1,
        editorial_relief: 1,
        meta_clarity: 0,
        parallel_review: true,
    },
    GameNightVariant {
        name: "ship-room",
        communication_bonus: 2,
        editorial_relief: 2,
        meta_clarity: 2,
        parallel_review: true,
    },
];

const WAVELENGTH: &[Puzzle] = &[
    Puzzle {
        id: "P1",
        title: "Side A",
        difficulty: 2,
        answer_len: 6,
    },
    Puzzle {
        id: "P2",
        title: "Notation",
        difficulty: 3,
        answer_len: 6,
    },
    Puzzle {
        id: "P3",
        title: "Deep Cuts",
        difficulty: 3,
        answer_len: 7,
    },
    Puzzle {
        id: "P4",
        title: "Chart Toppers",
        difficulty: 3,
        answer_len: 6,
    },
    Puzzle {
        id: "P5",
        title: "Between the Lines",
        difficulty: 4,
        answer_len: 5,
    },
    Puzzle {
        id: "P6",
        title: "Name That Band",
        difficulty: 4,
        answer_len: 6,
    },
];

const TEAM_PROFILES: &[TeamProfile] = &[
    TeamProfile {
        id: "speedster-duo",
        skill: 4,
        parallelism: 2,
    },
    TeamProfile {
        id: "casual-music-lovers",
        skill: 3,
        parallelism: 3,
    },
    TeamProfile {
        id: "methodical-newcomers",
        skill: 2,
        parallelism: 2,
    },
];

const GAME_NIGHT_MODULES: &[GameNightModule] = &[
    GameNightModule {
        id: "M1",
        game: "Chess",
        answer: "CASTLE",
        author: "The Methodical",
        base_minutes: 42,
        quality: 8,
        communication: 5,
        rework_risk: 18,
    },
    GameNightModule {
        id: "M2",
        game: "Settlers",
        answer: "TRADE",
        author: "The Speedster",
        base_minutes: 24,
        quality: 5,
        communication: 3,
        rework_risk: 55,
    },
    GameNightModule {
        id: "M3",
        game: "Risk",
        answer: "BORDER",
        author: "The Skeptic",
        base_minutes: 34,
        quality: 6,
        communication: 4,
        rework_risk: 42,
    },
    GameNightModule {
        id: "M4",
        game: "Pandemic",
        answer: "SPREAD",
        author: "The Social",
        base_minutes: 38,
        quality: 7,
        communication: 6,
        rework_risk: 25,
    },
    GameNightModule {
        id: "M5",
        game: "Codenames",
        answer: "CIPHER",
        author: "The Lurker",
        base_minutes: 29,
        quality: 7,
        communication: 1,
        rework_risk: 34,
    },
];

fn main() {
    let seed = option_value("--seed").unwrap_or_else(|| "wavelength-smoke".to_string());
    let runs = option_value("--runs")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(1);
    let scenario = option_value("--scenario").unwrap_or_else(|| "wavelength".to_string());
    if matches!(scenario.as_str(), "boardgames" | "game-night") {
        run_game_night_cli(&seed, runs);
        return;
    }
    if has_flag("--compare-variants") {
        let runs = runs.max(12);
        let baseline_results = simulate_batch(&seed, runs, BASELINE);
        let baseline = summarize_batch(&baseline_results);
        println!("HUNT simulator: WAVELENGTH variant comparison");
        println!("seed: {seed}");
        println!("runs_per_variant: {}", runs * TEAM_PROFILES.len());
        for variant in VARIANTS {
            let results = simulate_batch(&seed, runs, *variant);
            let summary = summarize_batch(&results);
            let comparison = compare_to_baseline("wavelength", &baseline, variant.name, &summary);
            let status = if variant.name == "baseline" {
                "baseline".to_string()
            } else {
                comparison.status().to_string()
            };
            let improved = if variant.name == "baseline" {
                "-".to_string()
            } else {
                format!(
                    "{}/{}",
                    comparison.improved_count(),
                    comparison.deltas.len()
                )
            };
            println!(
                "variant:{} status={} improved={} avg_minutes={:.1} p95={} pass_rate={:.1}% avg_hints={:.1} top_bottleneck={}",
                variant.name,
                status,
                improved,
                summary.average_minutes,
                summary.p95_minutes,
                summary.pass_rate,
                summary.average_hints,
                summary
                    .bottlenecks
                    .first()
                    .map(|(id, count)| format!("{id}:{count}"))
                    .unwrap_or_else(|| "-".to_string())
            );
        }
        return;
    }
    let variant = option_value("--variant")
        .as_deref()
        .and_then(find_variant)
        .unwrap_or(BASELINE);
    if runs > 1 {
        let results = simulate_batch(&seed, runs, variant);
        let summary = summarize_batch(&results);
        println!("HUNT simulator: WAVELENGTH batch");
        println!("seed: {seed}");
        println!("variant: {}", variant.name);
        println!("runs: {}", summary.runs);
        println!("average_minutes: {:.1}", summary.average_minutes);
        println!("p95_minutes: {}", summary.p95_minutes);
        println!("pass_rate: {:.1}%", summary.pass_rate);
        println!("average_hints: {:.1}", summary.average_hints);
        for (id, count) in summary.bottlenecks {
            println!("bottleneck:{}={}", id, count);
        }
        return;
    }

    let result = simulate_wavelength(&seed, TEAM_PROFILES[1], variant);
    println!("HUNT simulator: WAVELENGTH");
    println!("run_id: {}", result.run.run_id);
    println!("variant: {}", variant.name);
    println!("status: {}", result.report.status());
    println!("team: {}", result.team.id);
    println!("total_minutes: {}", result.total_minutes);
    println!("meta_minutes: {}", result.meta_minutes);
    for solved in &result.solved {
        println!(
            "{} ({}) solved_in={} hints={}",
            solved.id, solved.title, solved.minutes, solved.hints
        );
    }
    for metric in &result.metrics {
        println!("metric:{}={:.2}", metric.name, metric.value);
    }
    for finding in &result.report.findings {
        println!("{}:{}: {}", finding.severity, finding.code, finding.message);
    }
}

fn run_game_night_cli(seed: &str, runs: usize) {
    if has_flag("--compare-variants") {
        let runs = runs.max(12);
        let baseline_results = simulate_game_night_batch(seed, runs, GAME_NIGHT_BASELINE);
        let baseline = summarize_game_night_batch(&baseline_results);
        println!("HUNT simulator: Game Night variant comparison");
        println!("seed: {seed}");
        println!("runs_per_variant: {runs}");
        for variant in GAME_NIGHT_VARIANTS {
            let results = simulate_game_night_batch(seed, runs, *variant);
            let summary = summarize_game_night_batch(&results);
            let comparison = compare_game_night_to_baseline(&baseline, variant.name, &summary);
            let status = if variant.name == "baseline" {
                "baseline".to_string()
            } else {
                comparison.status().to_string()
            };
            let improved = if variant.name == "baseline" {
                "-".to_string()
            } else {
                format!(
                    "{}/{}",
                    comparison.improved_count(),
                    comparison.deltas.len()
                )
            };
            println!(
                "variant:{} status={} improved={} avg_minutes={:.1} p95={} pass_rate={:.1}% avg_reworks={:.1} visibility_gap_rate={:.1}%",
                variant.name,
                status,
                improved,
                summary.average_minutes,
                summary.p95_minutes,
                summary.pass_rate,
                summary.average_reworks,
                summary.visibility_gap_rate
            );
        }
        return;
    }

    let variant = option_value("--variant")
        .as_deref()
        .and_then(find_game_night_variant)
        .unwrap_or(GAME_NIGHT_BASELINE);
    if runs > 1 {
        let results = simulate_game_night_batch(seed, runs, variant);
        let summary = summarize_game_night_batch(&results);
        println!("HUNT simulator: Game Night batch");
        println!("seed: {seed}");
        println!("variant: {}", variant.name);
        println!("runs: {}", summary.runs);
        println!("average_minutes: {:.1}", summary.average_minutes);
        println!("p95_minutes: {}", summary.p95_minutes);
        println!("pass_rate: {:.1}%", summary.pass_rate);
        println!("average_reworks: {:.1}", summary.average_reworks);
        println!("visibility_gap_rate: {:.1}%", summary.visibility_gap_rate);
        return;
    }

    let result = simulate_game_night(seed, variant);
    println!("HUNT simulator: Game Night");
    println!("run_id: {}", result.run.run_id);
    println!("variant: {}", variant.name);
    println!("status: {}", result.report.status());
    println!("total_minutes: {}", result.total_minutes);
    println!("meta_minutes: {}", result.meta_minutes);
    for module in &result.modules {
        println!(
            "{} {} answer={} author={} quality={} minutes={} reworked={} visible={} actions={} blocked={}",
            module.id,
            module.game,
            module.answer,
            module.author,
            module.quality,
            module.minutes,
            module.reworked,
            module.visible_to_admin,
            module.trace.actions,
            module.trace.blocked_turns
        );
    }
    for metric in &result.metrics {
        println!("metric:{}={:.2}", metric.name, metric.value);
    }
    for finding in &result.report.findings {
        println!("{}:{}: {}", finding.severity, finding.code, finding.message);
    }
}

fn has_flag(name: &str) -> bool {
    env::args().any(|arg| arg == name)
}

fn option_value(name: &str) -> Option<String> {
    let args = env::args().collect::<Vec<_>>();
    args.windows(2)
        .find(|pair| pair[0] == name)
        .map(|pair| pair[1].clone())
}

fn find_variant(name: &str) -> Option<HuntVariant> {
    VARIANTS
        .iter()
        .copied()
        .find(|variant| variant.name == name)
}

fn find_game_night_variant(name: &str) -> Option<GameNightVariant> {
    GAME_NIGHT_VARIANTS
        .iter()
        .copied()
        .find(|variant| variant.name == name)
}

fn simulate_game_night_batch(
    seed: &str,
    runs: usize,
    variant: GameNightVariant,
) -> Vec<GameNightResult> {
    (0..runs)
        .map(|idx| simulate_game_night(&format!("{seed}-{idx}"), variant))
        .collect()
}

fn summarize_game_night_batch(results: &[GameNightResult]) -> GameNightSummary {
    let mut totals = results
        .iter()
        .map(|result| result.total_minutes)
        .collect::<Vec<_>>();
    totals.sort_unstable();
    let pass_count = results
        .iter()
        .filter(|result| result.report.status() == "pass")
        .count();
    let reworks = results
        .iter()
        .flat_map(|result| result.modules.iter())
        .filter(|module| module.reworked)
        .count();
    let visibility_gaps = results
        .iter()
        .flat_map(|result| result.modules.iter())
        .filter(|module| !module.visible_to_admin)
        .count();
    let module_count = results.len() * GAME_NIGHT_MODULES.len();
    let p95_index = ((totals.len() as f64 * 0.95).ceil() as usize).saturating_sub(1);

    GameNightSummary {
        runs: results.len(),
        average_minutes: totals.iter().sum::<u32>() as f64 / results.len().max(1) as f64,
        p95_minutes: totals.get(p95_index).copied().unwrap_or(0),
        pass_rate: percent_of(pass_count as u32, results.len() as u32),
        average_reworks: reworks as f64 / results.len().max(1) as f64,
        visibility_gap_rate: percent_of(visibility_gaps as u32, module_count as u32),
    }
}

fn compare_game_night_to_baseline(
    baseline: &GameNightSummary,
    candidate_id: &str,
    candidate: &GameNightSummary,
) -> ComparisonReport {
    let mut report = ComparisonReport::new("game-night", "baseline", candidate_id);
    report.add_delta(ComparisonDelta::higher_is_better(
        "pass_rate",
        baseline.pass_rate,
        candidate.pass_rate,
    ));
    report.add_delta(ComparisonDelta::lower_is_better(
        "average_minutes",
        baseline.average_minutes,
        candidate.average_minutes,
    ));
    report.add_delta(ComparisonDelta::lower_is_better(
        "average_reworks",
        baseline.average_reworks,
        candidate.average_reworks,
    ));
    report
}

fn simulate_game_night(seed: &str, variant: GameNightVariant) -> GameNightResult {
    let run = SimulationRun::new("hunt-sim", &format!("game-night-{}", variant.name), seed);
    let mut rng = run.rng();
    let modules = GAME_NIGHT_MODULES
        .iter()
        .map(|module| solve_game_night_module(module, &mut rng, variant))
        .collect::<Vec<_>>();
    let locked_modules = modules.iter().filter(|module| module.quality >= 6).count();
    let rework_count = modules.iter().filter(|module| module.reworked).count();
    let visibility_gaps = modules
        .iter()
        .filter(|module| !module.visible_to_admin)
        .count();
    let review_minutes = if variant.parallel_review {
        modules
            .iter()
            .map(|module| module.minutes)
            .max()
            .unwrap_or(0)
    } else {
        modules.iter().map(|module| module.minutes).sum::<u32>()
    };
    let meta_minutes = 34u32.saturating_sub(variant.meta_clarity * 5)
        + rng.next_bounded(18)
        + if locked_modules < 5 { 18 } else { 0 };
    let total_minutes = review_minutes + meta_minutes;
    let mut findings = Vec::new();
    if locked_modules < 4 {
        findings.push(ValidationFinding::error(
            "meta-blocked",
            "scenarios/boardgames/meta/META-DESIGN.md#80-rule",
            "fewer than four answer words were reliable enough for the 80% meta rule",
        ));
    }
    if rework_count > 2 {
        findings.push(ValidationFinding::warning(
            "author-rework-spike",
            "scenarios/boardgames/MODULE-LOG.md#multi-author-dynamics",
            "too many independently authored modules needed revision before integration",
        ));
    }
    if visibility_gaps > 1 {
        findings.push(ValidationFinding::warning(
            "admin-visibility-gap",
            "scenarios/boardgames/CLAUDE.md#the-5-author-personalities",
            "low-communication authors left the admin with insufficient handoff visibility",
        ));
    }
    if total_minutes > 180 {
        findings.push(ValidationFinding::warning(
            "over-target-time",
            "scenarios/boardgames/SCOPE.md#hunt-identity",
            "multi-author integration exceeded the 2-3 hour target window",
        ));
    }

    GameNightResult {
        run,
        modules,
        meta_minutes,
        total_minutes,
        metrics: vec![
            SimulationMetric::new("locked_modules", locked_modules as f64),
            SimulationMetric::new("rework_count", rework_count as f64),
            SimulationMetric::new("visibility_gaps", visibility_gaps as f64),
            SimulationMetric::new("target_window_used", percent_of(total_minutes, 180)),
            SimulationMetric::new("parallel_review", u32::from(variant.parallel_review) as f64),
        ],
        report: ValidationReport {
            subject: "game-night".to_string(),
            findings,
        },
    }
}

fn solve_game_night_module(
    module: &GameNightModule,
    rng: &mut RunSeed,
    variant: GameNightVariant,
) -> ModuleResult {
    let mut trace = ActorTrace::new(module.id, "module-author");
    trace.record_action();
    let communication = module.communication + variant.communication_bonus;
    let visible_to_admin = communication >= 3 || rng.percent_chance(communication * 14);
    if !visible_to_admin {
        trace.record_blocked_turn();
    }
    let relief = variant.editorial_relief;
    let rework_threshold = module.rework_risk.saturating_sub(relief * 12);
    let reworked = rng.percent_chance(rework_threshold);
    let quality_noise = rng.next_bounded(4);
    let quality = (module.quality + relief + quality_noise / 2 + u32::from(reworked)).min(10);
    let minutes = module.base_minutes + rng.next_bounded(22) + if reworked { 24 } else { 0 }
        - (variant.communication_bonus * 3).min(module.base_minutes / 2);
    if reworked {
        trace.record_action();
    }

    ModuleResult {
        id: module.id,
        game: module.game,
        answer: module.answer,
        author: module.author,
        minutes,
        quality,
        reworked,
        visible_to_admin,
        trace,
    }
}

fn simulate_batch(seed: &str, runs: usize, variant: HuntVariant) -> Vec<HuntResult> {
    (0..runs)
        .flat_map(|idx| {
            TEAM_PROFILES.iter().map(move |profile| {
                simulate_wavelength(&format!("{seed}-{idx}-{}", profile.id), *profile, variant)
            })
        })
        .collect()
}

fn summarize_batch(results: &[HuntResult]) -> BatchSummary {
    let mut totals = results
        .iter()
        .map(|result| result.total_minutes)
        .collect::<Vec<_>>();
    totals.sort_unstable();
    let sum_minutes = totals.iter().sum::<u32>();
    let hint_sum = results
        .iter()
        .flat_map(|result| result.solved.iter())
        .map(|solved| solved.hints)
        .sum::<u32>();
    let pass_count = results
        .iter()
        .filter(|result| result.report.status() == "pass")
        .count();
    let mut bottleneck_counts = WAVELENGTH
        .iter()
        .map(|puzzle| (puzzle.id, 0u32))
        .collect::<Vec<_>>();
    for result in results {
        if let Some(slowest) = result.solved.iter().max_by_key(|solved| solved.minutes) {
            if let Some((_, count)) = bottleneck_counts
                .iter_mut()
                .find(|(id, _)| *id == slowest.id)
            {
                *count += 1;
            }
        }
    }
    bottleneck_counts.sort_by(|left, right| right.1.cmp(&left.1).then(left.0.cmp(right.0)));
    let p95_index = ((totals.len() as f64 * 0.95).ceil() as usize).saturating_sub(1);

    BatchSummary {
        runs: results.len(),
        average_minutes: sum_minutes as f64 / results.len().max(1) as f64,
        p95_minutes: totals.get(p95_index).copied().unwrap_or(0),
        pass_rate: percent_of(pass_count as u32, results.len() as u32),
        average_hints: hint_sum as f64 / results.len().max(1) as f64,
        bottlenecks: bottleneck_counts,
    }
}

fn compare_to_baseline(
    subject: &str,
    baseline: &BatchSummary,
    candidate_id: &str,
    candidate: &BatchSummary,
) -> ComparisonReport {
    let mut report = ComparisonReport::new(subject, "baseline", candidate_id);
    report.add_delta(ComparisonDelta::higher_is_better(
        "pass_rate",
        baseline.pass_rate,
        candidate.pass_rate,
    ));
    report.add_delta(ComparisonDelta::lower_is_better(
        "average_minutes",
        baseline.average_minutes,
        candidate.average_minutes,
    ));
    report.add_delta(ComparisonDelta::lower_is_better(
        "p95_minutes",
        baseline.p95_minutes as f64,
        candidate.p95_minutes as f64,
    ));
    report
}

fn simulate_wavelength(seed: &str, profile: TeamProfile, variant: HuntVariant) -> HuntResult {
    let run = SimulationRun::new("hunt-sim", &format!("wavelength-{}", variant.name), seed);
    let mut rng = run.rng();
    let mut team = SolverTeam {
        id: profile.id,
        skill: profile.skill,
        parallelism: profile.parallelism + variant.parallelism_bonus,
        trace: ActorTrace::new(profile.id, "solver-team"),
    };

    let mut solved = WAVELENGTH
        .iter()
        .map(|puzzle| solve_puzzle(puzzle, &mut team, &mut rng, variant))
        .collect::<Vec<_>>();
    solved.sort_by_key(|solved| solved.minutes);

    let feeder_wall_clock = solved
        .chunks(team.parallelism)
        .map(|batch| batch.iter().map(|puzzle| puzzle.minutes).max().unwrap_or(0))
        .sum::<u32>();
    let meta_ready_count = solved.len().min(5);
    let missing_penalty = if meta_ready_count >= 5 { 8 } else { 25 };
    let meta_minutes =
        18u32.saturating_sub(variant.meta_base_reduction) + rng.next_bounded(15) + missing_penalty;
    team.trace.record_action();
    let total_minutes = feeder_wall_clock + meta_minutes;

    let total_hints = solved.iter().map(|puzzle| puzzle.hints).sum::<u32>();
    let hard_puzzle_max = solved
        .iter()
        .filter(|solved| matches!(solved.id, "P5" | "P6"))
        .map(|solved| solved.minutes)
        .max()
        .unwrap_or(0);
    let mut findings = Vec::new();
    if total_minutes > 180 {
        findings.push(ValidationFinding::warning(
            "over-target-time",
            "scenarios/wavelength/ROUNDS.md#target-solve-time",
            "simulated team exceeded the 2-3 hour target window",
        ));
    }
    if hard_puzzle_max > 75 {
        findings.push(ValidationFinding::warning(
            "hard-feeder-bottleneck",
            "scenarios/wavelength/PUZZLES.md#P5-P6",
            "one hard feeder dominated the wall-clock solve",
        ));
    }
    if meta_ready_count < 5 {
        findings.push(ValidationFinding::error(
            "meta-blocked",
            "scenarios/wavelength/meta/META-DESIGN.md#80-rule",
            "meta did not receive five feeder answers",
        ));
    }

    HuntResult {
        run,
        solved,
        meta_minutes,
        total_minutes,
        team,
        metrics: vec![
            SimulationMetric::new("target_window_used", percent_of(total_minutes, 180)),
            SimulationMetric::new("total_hints", total_hints as f64),
            SimulationMetric::new("team_actions", 7.0),
            SimulationMetric::new("hard_feeder_relief", variant.hard_feeder_relief as f64),
            SimulationMetric::new("parallelism_bonus", variant.parallelism_bonus as f64),
        ],
        report: ValidationReport {
            subject: "wavelength".to_string(),
            findings,
        },
    }
}

fn solve_puzzle(
    puzzle: &Puzzle,
    team: &mut SolverTeam,
    rng: &mut RunSeed,
    variant: HuntVariant,
) -> SolvedPuzzle {
    let difficulty = if matches!(puzzle.id, "P5" | "P6") {
        puzzle.difficulty.saturating_sub(variant.hard_feeder_relief)
    } else {
        puzzle.difficulty
    };
    let base = 12 + difficulty * 12 + puzzle.answer_len;
    let noise = rng.next_bounded(18);
    let skill_discount = team.skill * 4;
    let minutes = base + noise - skill_discount.min(base / 2);
    let first_hint_threshold = (50 + variant.hint_threshold_shift).max(25) as u32;
    let second_hint_threshold = (70 + variant.hint_threshold_shift).max(40) as u32;
    let hints = if minutes > second_hint_threshold {
        team.trace.record_blocked_turn();
        2
    } else if minutes > first_hint_threshold {
        1
    } else {
        0
    };
    team.trace.record_action();
    SolvedPuzzle {
        id: puzzle.id,
        title: puzzle.title,
        minutes,
        hints,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wavelength_sim_is_repeatable() {
        let left = simulate_wavelength("fixed", TEAM_PROFILES[1], BASELINE);
        let right = simulate_wavelength("fixed", TEAM_PROFILES[1], BASELINE);

        assert_eq!(left.total_minutes, right.total_minutes);
        assert_eq!(left.solved[0].minutes, right.solved[0].minutes);
    }

    #[test]
    fn wavelength_sim_solves_all_feeders_and_meta() {
        let result = simulate_wavelength("coverage", TEAM_PROFILES[1], BASELINE);

        assert_eq!(result.solved.len(), 6);
        assert!(result.meta_minutes > 0);
        assert!(result.team.trace.actions >= 7);
    }

    #[test]
    fn batch_summary_reports_pass_rate_and_bottlenecks() {
        let results = simulate_batch("batch", 3, BASELINE);
        let summary = summarize_batch(&results);

        assert_eq!(summary.runs, 9);
        assert!(summary.average_minutes > 0.0);
        assert_eq!(summary.bottlenecks.len(), 6);
    }

    #[test]
    fn guided_final_set_improves_pass_rate() {
        let baseline = summarize_batch(&simulate_batch("variant", 12, BASELINE));
        let guided = summarize_batch(&simulate_batch(
            "variant",
            12,
            find_variant("guided-final-set").unwrap(),
        ));

        assert!(guided.pass_rate >= baseline.pass_rate);
        assert!(guided.average_minutes <= baseline.average_minutes);
    }

    #[test]
    fn comparison_report_marks_guided_variant_improved() {
        let baseline = summarize_batch(&simulate_batch("comparison", 12, BASELINE));
        let guided = summarize_batch(&simulate_batch(
            "comparison",
            12,
            find_variant("guided-final-set").unwrap(),
        ));
        let report = compare_to_baseline("wavelength", &baseline, "guided-final-set", &guided);

        assert_ne!(report.status(), "regressed");
        assert!(report.improved_count() >= 2);
    }

    #[test]
    fn game_night_sim_tracks_author_handoffs() {
        let result = simulate_game_night("game-night", GAME_NIGHT_BASELINE);

        assert_eq!(result.modules.len(), 5);
        assert!(result.meta_minutes > 0);
        assert!(result.modules.iter().all(|module| module.trace.actions > 0));
    }

    #[test]
    fn ship_room_reduces_game_night_rework_pressure() {
        let baseline = summarize_game_night_batch(&simulate_game_night_batch(
            "game-night-variant",
            12,
            GAME_NIGHT_BASELINE,
        ));
        let ship_room = summarize_game_night_batch(&simulate_game_night_batch(
            "game-night-variant",
            12,
            find_game_night_variant("ship-room").unwrap(),
        ));

        assert!(ship_room.average_reworks <= baseline.average_reworks);
        assert!(ship_room.pass_rate >= baseline.pass_rate);
    }
}
