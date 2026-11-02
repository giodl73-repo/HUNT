//! Strict browser admission and report projection over HUNT's native feeder model.
use hunt_sim_core::{
    compare_to_baseline, simulate_feeders, summarize_feeders, HuntResult, Puzzle, TeamProfile,
    BASELINE, VARIANTS,
};
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;

pub const MAX_REQUEST_BYTES: usize = 65_536;
pub const MAX_REPORT_BYTES: usize = 2_097_152;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    seed: String,
    runs: u32,
    variant: String,
    team: Team,
    puzzles: Vec<Feeder>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Team {
    skill: u32,
    parallelism: u32,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Feeder {
    difficulty: u32,
    answer_length: u32,
}

fn failure(kind: &str, message: impl ToString) -> Value {
    json!({"kind":kind, "message":message.to_string()})
}

fn run_value(r: &HuntResult) -> Value {
    json!({"run_id":r.run.run_id,"adapter":r.run.adapter,"subject":r.run.subject,"seed_label":r.run.seed_label,
        "total_minutes":r.total_minutes,"meta_minutes":r.meta_minutes,
        "model_status":r.report.status(),
        "team":{"id":r.team.id,"skill":r.team.skill,"parallelism":r.team.parallelism,
            "actions":r.team.trace.actions,"blocked_turns":r.team.trace.blocked_turns},
        "solved":r.solved.iter().map(|s|json!({"id":s.id,"title":s.title,"minutes":s.minutes,"hints":s.hints})).collect::<Vec<_>>(),
        "metrics":r.metrics.iter().map(|m|json!({"name":m.name,"value":m.value})).collect::<Vec<_>>(),
        "findings":r.report.findings.iter().map(|f|json!({"severity":f.severity,"code":f.code,"location":f.location,"message":f.message})).collect::<Vec<_>>()})
}

fn evaluate(text: &str) -> Value {
    if text.len() > MAX_REQUEST_BYTES {
        return failure(
            "browser_refusal",
            "Request exceeds 65,536 UTF-8 bytes; native CLI limits are separate.",
        );
    }
    let request: Request = match serde_json::from_str(text) {
        Ok(r) => r,
        Err(e) => return failure("invalid_request", e),
    };
    if request.seed.len() > 128 {
        return failure("browser_refusal", "Seed exceeds 128 UTF-8 bytes.");
    }
    if !(1..=128).contains(&request.runs) {
        return failure("browser_refusal", "runs must be 1..128.");
    }
    if request.puzzles.len() != 6 {
        return failure(
            "browser_refusal",
            "This model requires exactly six feeder assumptions, in P1..P6 order.",
        );
    }
    if request.team.skill > 10 || !(1..=6).contains(&request.team.parallelism) {
        return failure(
            "browser_refusal",
            "Team skill must be 0..10 and parallelism 1..6.",
        );
    }
    if request
        .puzzles
        .iter()
        .any(|p| p.difficulty > 10 || p.answer_length > 32)
    {
        return failure(
            "browser_refusal",
            "Difficulty must be 0..10 and answer_length 0..32; no answer text is accepted.",
        );
    }
    let Some(variant) = VARIANTS.iter().copied().find(|v| v.name == request.variant) else {
        return failure("invalid_control", "Unknown native variant.");
    };
    let names = [
        ("P1", "Feeder 1"),
        ("P2", "Feeder 2"),
        ("P3", "Feeder 3"),
        ("P4", "Feeder 4"),
        ("P5", "Feeder 5"),
        ("P6", "Feeder 6"),
    ];
    let puzzles: Vec<Puzzle> = request
        .puzzles
        .iter()
        .zip(names)
        .map(|(p, (id, title))| Puzzle {
            id,
            title,
            difficulty: p.difficulty,
            answer_len: p.answer_length,
        })
        .collect();
    let team = TeamProfile {
        id: "reader-team",
        skill: request.team.skill,
        parallelism: request.team.parallelism as usize,
    };
    let simulate = |v| {
        (0..request.runs)
            .map(|i| {
                simulate_feeders(
                    &format!("{}-{i}-{}", request.seed, team.id),
                    &puzzles,
                    team,
                    v,
                )
            })
            .collect::<Vec<_>>()
    };
    let baseline = simulate(BASELINE);
    let candidate = simulate(variant);
    let base_summary = summarize_feeders(&puzzles, &baseline);
    let candidate_summary = summarize_feeders(&puzzles, &candidate);
    let summary = |s: &hunt_sim_core::BatchSummary| {
        json!({"runs":s.runs,"average_minutes":s.average_minutes,"p95_minutes":s.p95_minutes,
        "finding_free_rate_percent":s.pass_rate,"average_hints":s.average_hints,
        "bottlenecks":s.bottlenecks.iter().map(|(id,count)|json!({"id":id,"runs":count})).collect::<Vec<_>>()})
    };
    let comparison = compare_to_baseline(
        "wavelength",
        &base_summary,
        variant.name,
        &candidate_summary,
    );
    json!({"kind":"report","schema":"hunt.six-feeder-forecast.v1","request_sha256":format!("{:x}",Sha256::digest(text.as_bytes())),
        "model":"HUNT WAVELENGTH six-feeder heuristic","seed":request.seed,"variant":variant.name,
        "variant_parameters":{"hard_feeder_relief":variant.hard_feeder_relief,"hint_threshold_shift":variant.hint_threshold_shift,
            "meta_base_reduction":variant.meta_base_reduction,"parallelism_bonus":variant.parallelism_bonus},
        "scope":"Synthetic forecast only; not actual solving, blind playtest, fairness, delight, toolkit installation, stage approval or schedule certification.",
        "rng_contract":"Native run ID includes variant name, so variants use different random streams; comparison does not isolate a causal effect. Repeatability is within the exact request/build.",
        "model_assumptions":["Six fixed IDs; P5/P6 receive clue relief.","Solve times are sorted then chunked by parallelism, not an optimal schedule.","Meta readiness is fixed at five of six; no actual answers or extraction are checked.","Hint thresholds count simulated pressure; hints do not reduce modeled solve minutes.","Target is 180 minutes; finding-free rate is not a hunt pass rate."],
        "baseline":{"summary":summary(&base_summary),"runs":baseline.iter().map(run_value).collect::<Vec<_>>()},
        "candidate":{"summary":summary(&candidate_summary),"runs":candidate.iter().map(run_value).collect::<Vec<_>>()},
        "native_comparison":{"status":comparison.status(),"note":"Raw RALLY classifier calls zero strictly improved metrics regressed, including ties; this is not a recommendation.",
            "improved_count":comparison.improved_count(),"deltas":comparison.deltas.iter().map(|d|json!({"metric":d.metric,"baseline":d.baseline,"candidate":d.candidate,"direction":d.direction,"change":d.change(),"improved":d.improved()})).collect::<Vec<_>>()}})
}

#[wasm_bindgen]
pub fn forecast_request(text: &str) -> String {
    let output = evaluate(text).to_string();
    if output.len() > MAX_REPORT_BYTES {
        failure("browser_refusal", "Report exceeds 2 MiB.").to_string()
    } else {
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn example() -> Value {
        json!({"seed":"synthetic","runs":3,"variant":"guided-final-set","team":{"skill":3,"parallelism":2},"puzzles":[{"difficulty":2,"answer_length":5},{"difficulty":3,"answer_length":7},{"difficulty":2,"answer_length":4},{"difficulty":3,"answer_length":8},{"difficulty":4,"answer_length":6},{"difficulty":4,"answer_length":5}]})
    }
    #[test]
    fn shared_engine_projection_and_summary_are_exact() {
        let text = example().to_string();
        let r = evaluate(&text);
        assert_eq!(r["kind"], "report");
        assert_eq!(forecast_request(&text), forecast_request(&text));
        let candidate = &r["candidate"];
        let totals = candidate["runs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["total_minutes"].as_u64().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            candidate["summary"]["average_minutes"],
            json!(totals.iter().sum::<u64>() as f64 / 3.0)
        );
        assert_eq!(
            candidate["summary"]["bottlenecks"]
                .as_array()
                .unwrap()
                .iter()
                .map(|b| b["runs"].as_u64().unwrap())
                .sum::<u64>(),
            3
        );
        let direct = simulate_feeders(
            "synthetic-0-reader-team",
            &[
                Puzzle {
                    id: "P1",
                    title: "Feeder 1",
                    difficulty: 2,
                    answer_len: 5,
                },
                Puzzle {
                    id: "P2",
                    title: "Feeder 2",
                    difficulty: 3,
                    answer_len: 7,
                },
                Puzzle {
                    id: "P3",
                    title: "Feeder 3",
                    difficulty: 2,
                    answer_len: 4,
                },
                Puzzle {
                    id: "P4",
                    title: "Feeder 4",
                    difficulty: 3,
                    answer_len: 8,
                },
                Puzzle {
                    id: "P5",
                    title: "Feeder 5",
                    difficulty: 4,
                    answer_len: 6,
                },
                Puzzle {
                    id: "P6",
                    title: "Feeder 6",
                    difficulty: 4,
                    answer_len: 5,
                },
            ],
            TeamProfile {
                id: "reader-team",
                skill: 3,
                parallelism: 2,
            },
            VARIANTS[5],
        );
        assert_eq!(candidate["runs"][0], run_value(&direct));
    }
    #[test]
    fn ties_keep_raw_native_classifier_and_no_causal_claim() {
        let mut r = example();
        r["variant"] = json!("baseline");
        let out = evaluate(&r.to_string());
        assert_eq!(out["baseline"], out["candidate"]);
        assert_eq!(out["native_comparison"]["status"], "regressed");
        assert!(out["rng_contract"]
            .as_str()
            .unwrap()
            .contains("different random streams"));
    }
    #[test]
    fn browser_admission_refuses_unbounded_or_wrong_shape() {
        for (key, value) in [
            ("runs", json!(0)),
            ("runs", json!(129)),
            ("puzzles", json!([])),
            ("seed", json!("x".repeat(129))),
        ] {
            let mut r = example();
            r[key] = value;
            assert_eq!(evaluate(&r.to_string())["kind"], "browser_refusal");
        }
        let mut r = example();
        r["team"]["parallelism"] = json!(0);
        assert_eq!(evaluate(&r.to_string())["kind"], "browser_refusal");
        r = example();
        r["answer"] = json!("no answer input");
        assert_eq!(evaluate(&r.to_string())["kind"], "invalid_request");
        assert_eq!(
            evaluate(&" ".repeat(MAX_REQUEST_BYTES + 1))["kind"],
            "browser_refusal"
        );
    }
    #[test]
    fn admitted_maximum_is_bounded_and_inspectable() {
        let mut r = example();
        r["runs"] = json!(128);
        r["team"] = json!({"skill":10,"parallelism":6});
        for p in r["puzzles"].as_array_mut().unwrap() {
            p["difficulty"] = json!(10);
            p["answer_length"] = json!(32);
        }
        let text = forecast_request(&r.to_string());
        assert!(text.len() < MAX_REPORT_BYTES);
        let out: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(out["kind"], "report");
        assert_eq!(out["candidate"]["runs"].as_array().unwrap().len(), 128);
    }
}
