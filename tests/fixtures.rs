use std::{fs, path::PathBuf};
use plan_net::validate_plan_net_json;

fn ex(n: &str) -> String {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("examples"); p.push(n);
    fs::read_to_string(p).unwrap()
}

#[test]
fn valid_fixture() {
    let s = ex("valid-practitioner-role.json");
    let r = validate_plan_net_json(&s).unwrap(); assert!(r.is_valid());
}

#[test]
fn invalid_fixture() {
    let s = ex("invalid-plan-net.json");
    let r = validate_plan_net_json(&s).unwrap(); assert!(!r.is_valid());
}
