# Batch 301: the three defects the chain found (16/11/2026)

This batch's window (ENG POL TUR GRE HRV RUS to 2002-08-01, `levas/301/LIGAS` and `levas/301/ATE`) goes past the
season turnover, and it is what found the first two; the third showed up on the parallel comparison's first use.

## 1. The rule bit set and then cleared (our own defect, in the code) — fixed

**Symptom:** the chain's base run (no mutation) died on 2002-170 (19/06/2002) with a game error box,
`Error | v3.9.68 gre_prm..cpp 483`. The mutations were cancelled ("base with no result").

**The path:** the Greek turnover calls the rules (the assert on line 476) and then the table `0x5ead60` (line 483).
The table walks the clubs and counts those whose division is the Greek first division id (`[0x9cf86c]`); if the total
is not 14 (`[esi+0x3e]`, which it writes itself), it returns 0 and the assert fires.

**Ruled out before finding it:** the simulation allocator (`grep "region exhausted"` in `ddraw.log` = 0: the 1 GB
arena never ran out) and the constructor template (`construct_division_league` matches `0x5ea410` step by step). The
field lists of `gre_prm_rules`/`gre_second_rules` also match `0x5eae90`/`0x5ebde0`, field by field.

**The cause:** `apply_league_rules_with_flags` (`rust/src/motor.rs`) set the bit and the list cleared it right after:

```rust
write_u8(this.addr() + league::LEAGUE_FLAGS, (this.league_flags() as u8) | flags); // sets 0x40
eng_prm_apply_league_rules(this, written)   // ...and the list writes (LEAGUE_FLAGS, 1, 0) over it
```

The original does `or byte ptr [esi+0xd9], 0x40` and **never** zeroes the field; the entry
`(league::LEAGUE_FLAGS, 1, 0)` is in the list only so the comparer knows which address to check. Worse: the original
sets the bit **before** the virtual call that builds the schedule (`comp_vtable::CREATE_SCHEDULE`, +0x3c), and bit
0x40 of `+0xd9` is the round order — the Greek schedule was being built with the wrong bit.

**The fix:** the applier filters the flags field out of the write list. All 5 callers belong to this batch (the
Croatian and Greek ones), so nothing published was affected.

**The proof:** the same career on the **original engine** reached 2002-212 with no error box at all
(`grep -c "msgbox 2002"` = 0), that is, the Greek turnover works in the original game with these leagues and the
failure was ours. With the fix the base run reached 2002-212 and the mutations ran: **14 of 14, 13 caught and 1
escaped** (explained in `escapadas.py`).

## 2. The Croatian entrants comparer does not restore the generator (a comparer fault)

**Symptom:** in the real game, 2 divergences, both the same line, on the only two occasions the routine runs (career
creation and the turnover):

```
engine: DIVERGENCE cro_cup_collect_entrants(0x21967710) returned 1 x 1 memory true list false (48 entries)
```

Return equal, memory equal, the **list** different.

**The cause:** `compare_entrants_with_seeds` (`rust/src/motor.rs`) passes as `side_by_side` regions
`[(this + cup::TEAM_COUNT, 2), (G_MOUSE_LEFT_BUTTON_DOWN, 4)]`. `side_by_side` restores those regions before running
the copy (`before.write_back()`), and the template's header promises "the random generator restored between them" —
but `G_MOUSE_LEFT_BUTTON_DOWN` is `0xb67a34`, another global (the error-box paths write to it, for example at
`0x5eade2`), **not** the generator's state. `utils_random` (0x90cfc0) works with `[0xde1f60]` (the pointer into the
fixed draw table, from 0xaa2454 to `[0xde2798]`) and `[0xde1f5c]` (the offset). The project already has the right
region:

```rust
// rust/src/rng.rs:56
pub const STATE_REGIONS: [(usize, usize); 2] = [(0xde1f5c, 8), (0xad6c30, 4)];
```

Without it the original consumes the 32 draws of the Second Division seeds and the copy consumes the **next 32**: the
seeds come out different by construction, and the list always differs. Hence the two divergences being the only two
calls.

**It is a false alarm:** the seeds do not change the career. `Cup::draw_round` (0x51c410) indexes the entrant table at
+0 only, the club pointer; no ported routine reads +0x4; and the original itself does not write +0x4 for the 16 first
division entrants, which keep the `crt_malloc` garbage. It is the same fact that made the mutation
`mwd8_semente_menor` escape (see `escapadas.py`): the byte is written and never read.

**The fix:** add `crate::rng::STATE_REGIONS` to the template's regions, keeping the two that are already there
(`0xb67a34` covers the routine's error-box path, line 0x12b). With the generator restored the seeds are the same in
both runs, and comparing them discriminates again.

## 3. The parallel comparison feature (`COMPARAR_BANCADA`) on its first use

`COMPARAR_BANCADA=K` (written on 14/11, at Lucas's request: "this is impractical") runs the real game's comparison on
a bench alongside, in parallel with the mutations. This batch was the first to use it, and it had three defects — two
of them only visible at run time:

1. **It synced the code but did not build.** `bancada.py sincronizar` copies `rust/src`, and `simular.py` does not
   copy the DLL into `jogo_testes` (the one that copies is `mutacoes.py`). The parallel comparison would have run with
   the bench's old DLL, giving a result that is not the batch's code. The block now builds and installs the DLL before
   comparing. (The same trap caught a short check of mine, which ran with the DLL from before the fix and repeated the
   divergence — 15 minutes wasted.)
2. **`wait` on a pid from another shell.** The parallel job is launched in the outer shell, but the `wait` was inside
   the subshell that writes `cadeia.txt`: `wait: pid 49797 is not a child of this shell`. The chain went on without
   waiting and the `cat` of the parallel file came out empty, leaving the real-game block with no result (and
   `gen_docs.cadeia` failing the batch, correctly). The job now writes to a `.parcial` and only renames it at the end,
   and the real-game step waits for the file to appear.
3. **The cleanup of the partial file came after the launch** (in the first attempt at the fix): it would delete the
   file the job had just opened, and the `mv` would fail. It moved to before.

This batch's `cadeia.txt` keeps the `wait` error line with a note, and the bench 2 comparison's output copied where
the `cat` would have put it — the same DLL, the same window. The record shows what ran, not a cleaned-up version.

**The batch's final result:** 14 mutations (13 caught, 1 escape proved), real game `2ce549afd0fa92fa` with 0
divergences on all three tapes, Rust careers ×2 identical (`be15cf32d7c80775`), detector equal, original recorded
(`1016b83c9995b595`). `gen_docs.cadeia(301)` = ok.
