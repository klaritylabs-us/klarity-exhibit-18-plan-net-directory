use serde_json::Value;
use crate::outcome::{ValidationIssue, ValidationReport};

pub fn validate_plan_net_json(json: &str) -> Result<ValidationReport, String> {
    let v: Value = serde_json::from_str(json).map_err(|e| format!("JSON: {e}"))?;
    let mut r = ValidationReport::ok();
    if v.get("resourceType").and_then(|x| x.as_str()) != Some("PractitionerRole") {
        r.push(ValidationIssue::error("resourceType", "Plan-Net directory uses PractitionerRole", "plan-net-role"));
    }
    if v.get("practitioner").and_then(|p| p.get("reference")).is_none() {
        r.push(ValidationIssue::error("practitioner", "PractitionerRole.practitioner required", "practitioner-ref"));
    }
    if v.get("organization").and_then(|p| p.get("reference")).is_none() {
        r.push(ValidationIssue::error("organization", "PractitionerRole.organization required", "org-ref"));
    }
    Ok(r)
}
