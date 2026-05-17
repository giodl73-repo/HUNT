use rally_core::{
    percent_of, ActorTrace, RunSeed, SimulationMetric, SimulationRun, ValidationFinding,
    ValidationReport,
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
    if runs > 1 {
        let results = simulate_batch(&seed, runs);
        let summary = summarize_batch(&results);
        println!("HUNT simulator: WAVELENGTH batch");
        println!("seed: {seed}");
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

    let result = simulate_wavelength(&seed, TEAM_PROFILES[1]);
    println!("HUNT simulator: WAVELENGTH");
    println!("run_id: {}", result.run.run_id);
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

fn option_value(name: &str) -> Option<String> {
    let args = env::args().collect::<Vec<_>>();
    args.windows(2)
        .find(|pair| pair[0] == name)
        .map(|pair| pair[1].clone())
}

fn simulate_batch(seed: &str, runs: usize) -> Vec<HuntResult> {
    (0..runs)
        .flat_map(|idx| {
            TEAM_PROFILES.iter().map(move |profile| {
                simulate_wavelength(&format!("{seed}-{idx}-{}", profile.id), *profile)
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

fn simulate_wavelength(seed: &str, profile: TeamProfile) -> HuntResult {
    let run = SimulationRun::new("hunt-sim", "wavelength", seed);
    let mut rng = run.rng();
    let mut team = SolverTeam {
        id: profile.id,
        skill: profile.skill,
        parallelism: profile.parallelism,
        trace: ActorTrace::new(profile.id, "solver-team"),
    };

    let mut solved = WAVELENGTH
        .iter()
        .map(|puzzle| solve_puzzle(puzzle, &mut team, &mut rng))
        .collect::<Vec<_>>();
    solved.sort_by_key(|solved| solved.minutes);

    let feeder_wall_clock = solved
        .chunks(team.parallelism)
        .map(|batch| batch.iter().map(|puzzle| puzzle.minutes).max().unwrap_or(0))
        .sum::<u32>();
    let meta_ready_count = solved.len().min(5);
    let missing_penalty = if meta_ready_count >= 5 { 8 } else { 25 };
    let meta_minutes = 18 + rng.next_bounded(15) + missing_penalty;
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
        ],
        report: ValidationReport {
            subject: "wavelength".to_string(),
            findings,
        },
    }
}

fn solve_puzzle(puzzle: &Puzzle, team: &mut SolverTeam, rng: &mut RunSeed) -> SolvedPuzzle {
    let base = 12 + puzzle.difficulty * 12 + puzzle.answer_len;
    let noise = rng.next_bounded(18);
    let skill_discount = team.skill * 4;
    let minutes = base + noise - skill_discount.min(base / 2);
    let hints = if minutes > 70 {
        team.trace.record_blocked_turn();
        2
    } else if minutes > 50 {
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
        let left = simulate_wavelength("fixed", TEAM_PROFILES[1]);
        let right = simulate_wavelength("fixed", TEAM_PROFILES[1]);

        assert_eq!(left.total_minutes, right.total_minutes);
        assert_eq!(left.solved[0].minutes, right.solved[0].minutes);
    }

    #[test]
    fn wavelength_sim_solves_all_feeders_and_meta() {
        let result = simulate_wavelength("coverage", TEAM_PROFILES[1]);

        assert_eq!(result.solved.len(), 6);
        assert!(result.meta_minutes > 0);
        assert!(result.team.trace.actions >= 7);
    }

    #[test]
    fn batch_summary_reports_pass_rate_and_bottlenecks() {
        let results = simulate_batch("batch", 3);
        let summary = summarize_batch(&results);

        assert_eq!(summary.runs, 9);
        assert!(summary.average_minutes > 0.0);
        assert_eq!(summary.bottlenecks.len(), 6);
    }
}
