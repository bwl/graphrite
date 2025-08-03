use std::process::Command;

fn run_cli(args: &[&str]) -> (i32, String, String) {
    let output = Command::new("cargo")
        .args(["run", "-q", "-p", "graphrite-cli", "--"])
        .args(args)
        .output()
        .expect("run cli");
    (
        output.status.code().unwrap_or(1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

fn assert_dot_contains(path: &str, needles: &[&str]) {
    let (code, out, err) = run_cli(&["render", "--format", "dot", path]);
    assert_eq!(code, 0, "stderr: {}", err);
    for n in needles {
        assert!(out.contains(n), "missing: {}", n);
    }
}

#[test]
fn bluesky_golden_dot_stable() {
    assert_dot_contains(
        "/Users/bwl/Developer/aiaide/samples/valid/bluesky.mmd",
        &[
            "digraph G",
            "user_app [label=\"Mobile/Web Client\"];",
            "user_app -> pds_api;",
        ],
    );
}

#[test]
fn simple_flow_golden_dot() {
    assert_dot_contains(
        "/Users/bwl/Developer/aiaide/samples/valid/simple_flow.mmd",
        &[
            "start [label=\"Start\"];",
            "validate [label=\"Validate Input\"];",
            "queue [label=\"Queue\"];",
            "worker1 [label=\"Worker 1\"];",
            "worker2 [label=\"Worker 2\"];",
            "process [label=\"Process\"];",
            "store [label=\"Store\"];",
            "notify [label=\"Notify\"];",
            "archive [label=\"Archive\"];",
            "metrics [label=\"Metrics\"];",
            "end_success [label=\"Success\"];",
            "end_fail [label=\"Fail\"];",
            "start -> validate;",
            "queue -> worker1;",
            "queue -> worker2;",
            "worker1 -> process;",
            "worker2 -> process;",
            "process -> store;",
            "process -> notify;",
            "store -> archive;",
            "store -> metrics;",
            "notify -> end_success;",
            "archive -> end_success;",
            "validate -> end_fail;",
        ],
    );
}

#[test]
fn very_parallel_golden_dot() {
    assert_dot_contains(
        "/Users/bwl/Developer/aiaide/samples/valid/very_parallel.mmd",
        &[
            "ingest [label=\"Ingest\"];",
            "precheck [label=\"Precheck\"];",
            "router [label=\"Router\"];",
            "proc_a [label=\"Proc A\"];",
            "proc_b [label=\"Proc B\"];",
            "proc_c [label=\"Proc C\"];",
            "proc_d [label=\"Proc D\"];",
            "proc_e [label=\"Proc E\"];",
            "proc_f [label=\"Proc F\"];",
            "merge1 [label=\"Merge 1\"];",
            "merge2 [label=\"Merge 2\"];",
            "finalize [label=\"Finalize\"];",
            "out [label=\"Out\"];",
            "ingest -> precheck;",
            "precheck -> router;",
            "router -> proc_a;",
            "router -> proc_b;",
            "router -> proc_c;",
            "router -> proc_d;",
            "router -> proc_e;",
            "router -> proc_f;",
            "proc_a -> merge1;",
            "proc_b -> merge1;",
            "proc_c -> merge1;",
            "proc_d -> merge2;",
            "proc_e -> merge2;",
            "proc_f -> merge2;",
            "merge1 -> finalize;",
            "merge2 -> finalize;",
            "finalize -> out;",
        ],
    );
}

#[test]
fn maintenance_loop_golden_dot() {
    assert_dot_contains(
        "/Users/bwl/Developer/aiaide/samples/valid/maintenance_loop.mmd",
        &[
            "monitor [label=\"Monitor\"];",
            "alert [label=\"Alert\"];",
            "classify [label=\"Classify\"];",
            "triage [label=\"Triage\"];",
            "assign [label=\"Assign\"];",
            "fix [label=\"Fix\"];",
            "review [label=\"Review\"];",
            "verify [label=\"Verify\"];",
            "release [label=\"Release\"];",
            "postmortem [label=\"Postmortem\"];",
            "backlog [label=\"Backlog\"];",
            "monitor -> alert;",
            "alert -> classify;",
            "classify -> triage;",
            "triage -> assign;",
            "assign -> fix;",
            "fix -> review;",
            "review -> verify;",
            "verify -> release;",
            "release -> postmortem;",
            "postmortem -> backlog;",
            "backlog -> monitor;",
        ],
    );
}

#[test]
fn thought_pattern_golden_dot() {
    assert_dot_contains(
        "/Users/bwl/Developer/aiaide/samples/valid/thought_pattern.mmd",
        &[
            "observe [label=\"Observe\"];",
            "collect [label=\"Collect Data\"];",
            "orient [label=\"Orient\"];",
            "hypothesize [label=\"Hypothesize\"];",
            "decide [label=\"Decide\"];",
            "plan [label=\"Plan\"];",
            "act [label=\"Act\"];",
            "reflect [label=\"Reflect\"];",
            "improve [label=\"Improve\"];",
            "repeat [label=\"Repeat\"];",
            "observe -> collect;",
            "collect -> orient;",
            "orient -> hypothesize;",
            "hypothesize -> decide;",
            "decide -> plan;",
            "plan -> act;",
            "act -> reflect;",
            "reflect -> improve;",
            "improve -> repeat;",
            "repeat -> observe;",
        ],
    );
}

#[test]
fn us_highway_system_golden_dot() {
    assert_dot_contains(
        "/Users/bwl/Developer/aiaide/samples/valid/us_highway_system.mmd",
        &[
            "nyc [label=\"New York\"];",
            "bos [label=\"Boston\"];",
            "phl [label=\"Philadelphia\"];",
            "dc [label=\"Washington DC\"];",
            "chi [label=\"Chicago\"];",
            "den [label=\"Denver\"];",
            "phx [label=\"Phoenix\"];",
            "la [label=\"Los Angeles\"];",
            "sd [label=\"San Diego\"];",
            "sf [label=\"San Francisco\"];",
            "sea [label=\"Seattle\"];",
            "por [label=\"Portland\"];",
            "slt [label=\"Salt Lake City\"];",
            "atl [label=\"Atlanta\"];",
            "mia [label=\"Miami\"];",
            "no [label=\"New Orleans\"];",
            "hou [label=\"Houston\"];",
            "dal [label=\"Dallas\"];",
            "sa [label=\"San Antonio\"];",
            "nyc -> bos;",
            "nyc -> phl;",
            "phl -> dc;",
            "nyc -> chi;",
            "dc -> atl;",
            "chi -> den;",
            "den -> slt;",
            "slt -> sf;",
            "sf -> la;",
            "la -> sd;",
            "sea -> por;",
            "por -> sf;",
            "atl -> mia;",
            "atl -> no;",
            "no -> hou;",
            "hou -> dal;",
            "dal -> sa;",
            "sa -> phx;",
            "phx -> la;",
        ],
    );
}
