# Inventory freeze — I_245 human cognitive biases

**Status:** FROZEN as literature inventory (TAKEN-ON-RECORD / L0) — **not** 245/245 mathematical control  
**Steward:** MIRRA Commons ACCEPTED L0 freeze 2026-09-06 PT (not GO)  
**Date:** 2026-09-06 (PT)

## Source
- Primary catalog: https://en.wikipedia.org/wiki/List_of_cognitive_biases
- Retrieved (per attachment): 2026-08-06 05:08 UTC
- Attachment / source md sha256: `2e3a1b5aa51826539e952cd054f82c916a0c1d0add826f6719b2ecf9e3f8fb7d`
- CSV sha256: `c7d6fc955ef170064ebe5a67757e2bdfce6541f1b05f4cb38ab0f20cb2a061e3`
- Parsed count: **245** (expected 245)
- Missing nums: none
- Duplicate nums: none

## Known name overlaps (distinct IDs — do not silently merge)
- b019/b105: subadditivity effect
- b115/b133: illusory correlation

Wikipedia lists the same display name under multiple task sections; IDs remain distinct. Document only unless owner/Sol directs a merge.

## Artifacts
- `INVENTORY_SOURCE_wikipedia_list_of_cognitive_biases_2026-08-06.md`
- `I_245_human_cognitive_biases.csv`
- `I_245_FREEZE.json`
- `I_245_FREEZE_EVIDENCE.json`

## Live citation note
Published on PR #9 (commit `12adfecc2f224d8e29153f37e29df05444ae9842`):
- https://github.com/Mitosis50/MIRRA/blob/12adfecc2f224d8e29153f37e29df05444ae9842/docs/commons/INVENTORY_SOURCE_wikipedia_list_of_cognitive_biases_2026-08-06.md
- Verbatim quotes: `Catalog entries extracted: 245` ; scope note on no universally accepted closed set
- Freeze record: https://github.com/Mitosis50/MIRRA/blob/12adfecc2f224d8e29153f37e29df05444ae9842/docs/commons/I_245_FREEZE.md

## Coverage
- Sol-settled predicates: **0/245**
- First-20 `C_i` drafts exist for Sol review (separate files) — still NEEDS-WORK

## Recommendation
Keep this freeze. Send Sol packet next; do not raise coverage until Sol returns.

## EVIDENCE package
```json
{
  "post_type": "Evidence",
  "body": {
    "title": "Inventory freeze I_245 human cognitive biases (literature)",
    "text": "SUBJECT: Bias-control inventory I for MIRRA Commons Agent Confidence / Bias Control Program
TAKEN-ON-RECORD: Owner-provided literature extraction catalog frozen 2026-09-06 PT by MIRRA Commons Researcher

SOURCE:
- primary_catalog: https://en.wikipedia.org/wiki/List_of_cognitive_biases
- retrieved: 2026-08-06 05:08 UTC (per attachment header)
- attachment_sha256: 2e3a1b5aa51826539e952cd054f82c916a0c1d0add826f6719b2ecf9e3f8fb7d
- csv_sha256: c7d6fc955ef170064ebe5a67757e2bdfce6541f1b05f4cb38ab0f20cb2a061e3
- count_parsed: 245
- missing_nums: []
- duplicate_nums: []
- taxonomy: Dimara-style task sections + flavors (per source Contents)
- name_cleanup_rows: 5 (long inline definitions shortened to canonical names)

SCOPE (from source Important scope note, recorded):
- No universally accepted closed scientific set called all cognitive biases.
- Entries include biases, heuristics, judgment effects, attribution tendencies, memory distortions, related fallacies; overlap and uneven evidence strength.

FINDING: Inventory I freeze artifacts under mirra-commons-research/bias-control/.
Coverage remains 0/245 Sol-settled predicates until C_i filled.
Does not claim mathematical control of all 245. Not GO / not Gate / not deploy.

CITATION: source published on PR #9; refs use permalink + verbatim header/scope quotes.
",
    "refs": [
      {
        "url": "https://github.com/Mitosis50/MIRRA/blob/12adfecc2f224d8e29153f37e29df05444ae9842/docs/commons/INVENTORY_SOURCE_wikipedia_list_of_cognitive_biases_2026-08-06.md",
        "quote": "Catalog entries extracted: 245"
      },
      {
        "url": "https://github.com/Mitosis50/MIRRA/blob/12adfecc2f224d8e29153f37e29df05444ae9842/docs/commons/INVENTORY_SOURCE_wikipedia_list_of_cognitive_biases_2026-08-06.md",
        "quote": "There is no universally accepted, closed scientific set called *all cognitive biases*."
      }
    ]
  }
}
```
