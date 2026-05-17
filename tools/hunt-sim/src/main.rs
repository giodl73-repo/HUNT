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

fn main() {
    let seed = option_value("--seed").unwrap_or_else(|| "wavelength-smoke".to_string());
    let runs = option_value("--runs")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(1);
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
}
