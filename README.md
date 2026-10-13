# Plan-Net provider directory — Rust

Small library and CLI that sanity-check **Da Vinci Plan-Net** **`PractitionerRole`** JSON before directory feeds publish: resource type plus **`practitioner.reference`** and **`organization.reference`** — the links network directories use to place a clinician at a site.

Synthetic fixtures only. Not ONC certification, not a substitute for Plan-Net validator tooling or Inferno, and not legal/clinical advice.

Maintained by [Klarity Labs](https://klaritylabs.ai/) as open reference code for U.S. healthcare FHIR interoperability work.

**Exhibit 18** — [FHIR exhibit catalog](https://github.com/klaritylabs-us/klarity-fhir-exhibits) · [klaritylabs-us](https://github.com/klaritylabs-us)


## How this relates to other tools

| Tool class | Role |
|------------|------|
| **HL7 FHIR Validator**, **HAPI**, **Firely** | Full validation, servers, SDKs, IG packages |
| **Inferno**, **Touchstone**, vendor cert suites | **Conformance** against test plans |
| **This exhibit** | **Pre-flight construction** on synthetic fixtures—catch shape mistakes before wire traffic |

Conformance tools test whether your **API behaves**; exhibits focus on whether your **payloads and pipes are built** to survive real partners.

## Specifications cited

- [Da Vinci Plan-Net](http://hl7.org/fhir/us/davinci-pdex/plan-net/)

## What it checks

- `resourceType` is **`PractitionerRole`**
- **`practitioner.reference`** required
- **`organization.reference`** required

| Problem | What breaks |
|--------|-------------|
| **Practitioner without org** | Directory search shows orphan provider |
| **Inline only, no references** | Payer directory importers drop the row |
| **Wrong resource type** | Plan-Net consumer skips the entry |

## Run it

Needs [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo test
cargo run --bin validate-plan-net -- examples/valid-practitioner-role.json
cargo run --bin validate-plan-net -- examples/invalid-plan-net.json
```

Exit `0` if no errors, `1` if validation failed, `2` for bad usage or JSON.

## Sample output

One command:

```bash
cargo run --bin validate-plan-net -- examples/valid-practitioner-role.json
```

**Pass** (`examples/valid-practitioner-role.json`) — exit code `0`:

```json
[]
```

**Fail** (`examples/invalid-plan-net.json`) — exit code `1` (issues array; more entries may appear):

```json
[
  {
    "severity": "error",
    "path": "practitioner",
    "detail": "PractitionerRole.practitioner required",
    "rule_id": "practitioner-ref"
  }
]
```


## Layout

- `src/validate.rs` — rules
- `examples/` — valid and broken directory rows for tests and demos

## Inferno vs this repo

Plan-Net testing exercises **directory queries** on live FHIR servers. This repo is **row shape** — PractitionerRole resources your roster ETL emits before bulk upload.

## License

MIT — see [LICENSE](./LICENSE).

**Need this on your payloads or API?** 20-minute FHIR Scan — sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

## Contact

sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)
