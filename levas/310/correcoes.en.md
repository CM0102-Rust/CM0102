# The 310th batch: two comparer false alarms in the real game (19/11/2026)

The chain's real game (bench 3, ENG BEL HOL GER DEN to 2002-08-01) reported two divergences. Both came from the
comparers, not from the ports; both comparers were fixed and the real game was run again from scratch on bench 4:
**0 divergences** to 2002-08-01 (that is the result in `cadeia.txt`).

| Routine | What the real game showed | Cause | Fix |
|---|---|---|---|
| `hol_first_advance_stage` | `before=3 after=5` | On reaching stage 4, `HolFirst::create_play_off_groups` (0x5f6720) adds stages 4 and 5 (the virtual `+0x3c` with 4 and with 5) and the original ends on 5. The shared comparer (`compare_first_division_advance`) required a single step. It is the same false alarm as Finland's in the 304th batch. | The Dutch comparer accepts stage 5 when the new stage is 4; otherwise, the shared rule. |
| `ger_lge_cup_collect_entrants` | `list false` on the 6 entries | The cup holder comes from `comp_util_get_latest_placed_club` (0x4afd80), which re-sorts the global competition history table (`[0xae23d4]`, `[0xae237c]` records of 0x1a bytes) on every call with the CRT's qsort, which is not stable. Side by side, the routine runs twice, and the second run can see records of equal key in another order. | The copy runs on the history as the original saw it, and the original's history is put back afterwards; if the lists differ, the log shows both. In the rerun the routine ran and matched. |

Both ports were checked instruction by instruction against the disassembly and did not change.
