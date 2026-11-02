//! HUNT's existing six-feeder forecast model, shared by native and browser adapters.
//!
//! Inputs are assumptions, not puzzle content or blind solver observations. The
//! model retains WAVELENGTH-specific P5/P6 relief, five-answer meta rule, 180-minute
//! target and native finding locations. It is not a generic graph scheduler.
//! Adapters admit bounded inputs; callers must provide positive parallelism and
//! numeric values whose arithmetic fits u32. This extraction adds no new formula.

use rally_core::{
    percent_of, ActorTrace, ComparisonDelta, ComparisonReport, RunSeed, SimulationMetric,
    SimulationRun, ValidationFinding, ValidationReport,
};

#[derive(Debug, Clone)]
pub struct Puzzle {
    pub id: &'static str,
    pub title: &'static str,
    pub difficulty: u32,
    pub answer_len: u32,
}

#[derive(Debug, Clone)]
pub struct SolverTeam {
    pub id: &'static str,
    pub skill: u32,
    pub parallelism: usize,
    pub trace: ActorTrace,
}

#[derive(Debug, Clone, Copy)]
pub struct TeamProfile {
    pub id: &'static str,
    pub skill: u32,
    pub parallelism: usize,
}

#[derive(Debug, Clone)]
pub struct SolvedPuzzle {
    pub id: &'static str,
    pub title: &'static str,
    pub minutes: u32,
    pub hints: u32,
}

#[derive(Debug, Clone)]
pub struct HuntResult {
    pub run: SimulationRun,
    pub solved: Vec<SolvedPuzzle>,
    pub meta_minutes: u32,
    pub total_minutes: u32,
    pub team: SolverTeam,
    pub metrics: Vec<SimulationMetric>,
    pub report: ValidationReport,
}

#[derive(Debug, Clone)]
pub struct BatchSummary {
    pub runs: usize,
    pub average_minutes: f64,
    pub p95_minutes: u32,
    pub pass_rate: f64,
    pub average_hints: f64,
    pub bottlenecks: Vec<(&'static str, u32)>,
}

#[derive(Debug, Clone, Copy)]
pub struct HuntVariant {
    pub name: &'static str,
    pub hard_feeder_relief: u32,
    pub hint_threshold_shift: i32,
    pub meta_base_reduction: u32,
    pub parallelism_bonus: usize,
}

pub fn summarize_feeders(puzzles: &[Puzzle], results: &[HuntResult]) -> BatchSummary {
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
    let mut bottleneck_counts = puzzles
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

pub fn compare_to_baseline(
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

pub fn simulate_feeders(
    seed: &str,
    puzzles: &[Puzzle],
    profile: TeamProfile,
    variant: HuntVariant,
) -> HuntResult {
    let run = SimulationRun::new("hunt-sim", &format!("wavelength-{}", variant.name), seed);
    let mut rng = run.rng();
    let mut team = SolverTeam {
        id: profile.id,
        skill: profile.skill,
        parallelism: profile.parallelism + variant.parallelism_bonus,
        trace: ActorTrace::new(profile.id, "solver-team"),
    };

    let mut solved = puzzles
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

pub const BASELINE: HuntVariant = HuntVariant {
    name: "baseline",
    hard_feeder_relief: 0,
    hint_threshold_shift: 0,
    meta_base_reduction: 0,
    parallelism_bonus: 0,
};

pub const VARIANTS: &[HuntVariant] = &[
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
