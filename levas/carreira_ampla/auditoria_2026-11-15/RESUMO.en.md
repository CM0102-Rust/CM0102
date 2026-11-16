# The 8-league audit (night of 15 to 16/11/2026)

Leagues: `ENG ESP ITA FRA BRA JPN AUS USA`; the main copy's DLL with batch 300 published (2,490 routines).

## Careers to 2003-01-31
- Rust ×2: identical (fingerprint `23ede44babb08db7`, 99,398 matches, database `518b3070cb17e55c`, 15,523,980,521
  draws), determinism **OK**, garbage detector **equal**.
- Original (recorded only): stopped on the same date.

## Comparing to 2002-06-01
Date 0 and random 0 divergences; engine: **1 divergence**, in 100 billion comparisons.

| Routine | Lines | Region | Batch |
|---|---|---|---|
| `add_cup_winners_record` | 1 | `0xae2470` (`G_DAYS_SINCE_SCRATCH_DATE`) | 294 |

No game boxes.

## Comparing `ENG SCO IRL ITA ESP` to 2002-06-01
Date, random and engine with **0 divergences**.

## The divergence and the fix

**Verdict: a comparer fault; the Rust copy was right.** The only differing region: `0xae2470`, the static date of
`exe::date_days_since` (0x54cdb0), with the same day (306) and a different year — original 1983 (the age arithmetic
inside the renewal), copy 2000 (the last day count it made).

`compare_champion_update` runs the copy as a dry run, without the renewal, and marks `DRY_RUN_STRAY_SCRATCH_DATE` so
the comparer excludes that address. But `offer_renewal_if_due` (`rust/src/motor/staff_records.rs`) cleared the mark
unconditionally, while `exe::date_days_since` only rewrites `0xae2470` **when the two years differ**: for a player
whose contract started in the current year the mark was cleared and the comparer checked an address that still held
the original's renewal residue.

The year guard has now gone in, making the clearing conditional:

```rust
if read_u16(G_CURRENT_DATE_YEAR) != k.date_started_year() {
    DRY_RUN_STRAY_SCRATCH_DATE.store(false, Relaxed);
}
```

**Process note:** this is the same fix that commit `f90bc84e1` (11/11/2026) announced in its message without having it
in the diff — that commit's 13 lines in `staff_records.rs` were only mechanical `padronizar.sh` cleanup, and
`git log -S DRY_RUN_STRAY_SCRATCH_DATE` returns no fix at all. The diagnosis of 11/11 was right; it was the patch that
got lost, and the divergence came back here. Details in [correcao.md](correcao.md).

## Proof of the fix
The 8-league comparison to 2002-06-01 repeated with the fixed DLL (bench 2, 24,032 s): it reached 2002-151, 63,373
matches, fingerprint `a60f8fb6966b0913`; **date, random and engine with 0 divergences** and no `DIVERGENCE` line in the
DLL log. Audit closed.

## Sweep of the same defect class
The five comparers that call `exe::crt_qsort` (`goldcup`, `bra_nat_third`, `usa_mls`, `world_cup`, `scout_manager`)
sort local `Vec`s, not the game's tables: no shared state moved. The class is closed for now.
