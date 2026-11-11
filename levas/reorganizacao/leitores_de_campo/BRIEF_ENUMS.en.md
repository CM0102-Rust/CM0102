# Enum reader: brief

Project: Championship Manager 01/02 (3.9.68) rewritten in Rust, routine by routine, as a DLL loaded by the original exe.
Repo: D:\Sistemas\CM0102. READ ONLY: never edit a file of the repo; the only file you write is your output JSON.

## Task
Fields of the game's records are now named (rust/src/motor/fields.rs, typed getters in rust/src/motor/records.rs), but
many fields hold enum values that the code compares with bare numbers: `row.status() == 0xf`, `p.job_for_club() == 0xb`,
`fixture.category() == 8`. Your census (TSV: getter, file:line, line text) lists those sites. For each (record type,
getter) that really is an enum (a small set of codes with distinct meanings), name its values. Skip fields whose numbers
are quantities, limits, attributes, years or sizes (club_count, goalkeeper 20, year 0x7d2): those are not enums.

## Sources
- the Rust code around each site (which record the receiver is: `x: Offer`, `LeagueTableRow(..)`, the method chain);
- rust/src/motor/fields.rs (the field's comment often lists the values) and records.rs (which record has the getter);
- pacotes/0x<address>/ (disassembly and pseudocode of the original routine; strings in the exe that the code prints for
  each value are the best evidence), docs/*.en.md, referencias/cm0102-function-map (community names).

## Rules
- One entry per (record type, getter). The same field reached through two record types (e.g. a base-class field) gets
  the same enum_name in both entries.
- Confidence: `verified` with concrete evidence (an exe string, a community name, two independent uses agreeing); `read`
  when the code's use makes it clear; `inferred` for a guess. A wrong name is worse than none: leave out values you cannot
  tell (sentinels like 0xff "none" are fine to name: NONE).
- Names: English UPPER_SNAKE_CASE for each value; enum_name lower_snake_case (e.g. `offer_status`, `job_kind`).

## Output
A JSON object:
{"offer_status": {"record": "Offer", "getter": "status", "enum_name": "offer_status", "values": {
   "7": {"name": "REJECTED", "meaning": "...", "confidence": "verified|read|inferred", "evidence": "one line"}, ...}}, ...}
`record` is the exact Rust type name of records.rs (`Offer`, `LeagueTableRow`, `Staff`, `Fixture`...), `getter` the exact
method name. Plain ASCII. Finish with a short summary in Portuguese: which enums, how many values per confidence, doubts.
