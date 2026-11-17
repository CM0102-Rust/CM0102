# Batch 303 — the Turkish and Russian cup entrants; the rules of the Polish divisions and the Turkish second division B

**Routines:**
- `tur_cup_collect_entrants()` = 0x8fcb90 (tur_cup.cpp, thiscall, ret 0): rule: the Turkish Cup takes 64 clubs, all class 1: 26 of the second division category B (the first 30 in database order, sorted by their last division, taken from the 26th backwards), then the first 20 of category A in reverse order, and the top-flight clubs from the end of the list backwards;
- `rus_cup_select_entrants()` = 0x7e99a0 (rus_cup.cpp, thiscall, ret 0): rule: the Russian Cup takes 48 clubs: the First Division clubs (class 2; the cup holder among them class 1) from entry 31 backwards, then the second division zone clubs (sorted by strength, the best 42 shuffled) until 32 are in, class 2; then the Premier League clubs (class 1) from entry 32 on; it succeeds when the list is full;
- `pol_first_set_up_rules()` = 0x7c9690 (pol_first.cpp, thiscall, ret 0): rule: the Polish First Division: 2 legs, 3 points a win and 1 a draw; tie-breaks head to head, goal difference and goals for; 4 in the promotion band, nobody goes down directly (relegation comes from the final stage); the Second Division below; 7 substitutes and 3 substitutions; rule bits 8;
- `pol_second_set_up_rules()` = 0x7cbf40 (pol_second.cpp, thiscall, ret 0): rule: the Polish Second Division: as the First Division, with 2 going up directly and 2 to the promotion play-off, 6 going down in 2001 (20 clubs) and 4 afterwards; the First Division above; rule bits 3;
- `tur_second_b_set_up_rules()` = 0x900460 (tur_second_b.cpp, thiscall, ret 0): rule: the Turkish second division category B: 3 points a win and 1 a draw; tie-breaks goal difference and goals for; 2 to the play-offs; category A above; 7 substitutes and 3 substitutions; rule bits 2;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (3):** 2 caught. `mwd27_classe_da_categoria_b` escaped: the mutation changes the class written for the 26 category B entrants of the Turkish second division (`tur_cup_collect_entrants`, `TurCup::collect_entrants` 0x8fcb90: `set_cup_entry(this, k, lower[0x19 - k], 1)` now writes 2). `set_cup_entry` writes the club at +0, status 0 at +5 and the class into byte +0x4 of the 6-byte entry of the entrant table (`cup_team_entry::SEED_LEVEL` of `cup::TEAM_TABLE`). It does not change the career because nothing reads that byte in this family of cups: the generic round draw (`Cup::draw_round` 0x51c410) indexes the table at +0 only, the club pointer (`mov ebx, [esi+0xa7]` with `eax = i*3` and `cmp dword ptr [ebx + eax*2], 0`, and `mov ecx, [edi + eax]` in the text calls), and the only ported routines that read +0x4 are those of the European competitions, which keep the draw class there. Unlike the Croatian cup seed (batch 301), there is not even a draw involved here: the class is a constant, so the generator stream stays identical. **The comparison does catch it:** the comparer `compare_tur_cup_collect_entrants` uses `compare_entrants_with_seeds` with the predicate `|_| true`, that is, it compares the class of **every** one of the 64 entries, and the mutation changes 26 of them. Not a gap: the career does not see it, the comparison does.. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (72df841b).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 de3fb984, original b0e4088f (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 302's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 303`

**Fields touched:**
- cup +4, +0x3e, +0xa7 (6 bytes: club, class, status); club +0x57 division
- league +0x3c, +0xc2..+0xc8, +0xab, +0xdb, +0x42, +0xbe..+0xc1, +0x1c, +0x20, +0xd9, +0x52, +0x49, +0x4a, +0xba
