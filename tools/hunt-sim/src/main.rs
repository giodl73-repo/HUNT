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

fn main() {
    let seed = option_value("--seed").unwrap_or_else(|| "wavelength-smoke".to_string());
    let result = simulate_wavelength(&seed);
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

fn simulate_wavelength(seed: &str) -> HuntResult {
    let run = SimulationRun::new("hunt-sim", "wavelength", seed);
    let mut rng = run.rng();
    let mut team = SolverTeam {
        id: "casual-music-lovers",
        skill: 3,
        parallelism: 3,
        trace: ActorTrace::new("casual-music-lovers", "solver-team"),
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
        let left = simulate_wavelength("fixed");
        let right = simulate_wavelength("fixed");

        assert_eq!(left.total_minutes, right.total_minutes);
        assert_eq!(left.solved[0].minutes, right.solved[0].minutes);
    }

    #[test]
    fn wavelength_sim_solves_all_feeders_and_meta() {
        let result = simulate_wavelength("coverage");

        assert_eq!(result.solved.len(), 6);
        assert!(result.meta_minutes > 0);
        assert!(result.team.trace.actions >= 7);
    }
}
