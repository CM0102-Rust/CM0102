*English version of [FORA.md](FORA.md). Both are kept in sync; the Portuguese file is the original.*

# gra_2: game_daily_staff_update (0x5ce2d0, game.cpp): out, for now

Reader gra, round 4, 2026-11-21. The routine is 30,544 bytes, 9,230 instructions, 630 of them x87, and 395 draws. It
could not be ported whole safely in one session; it was **not** hooked by halves. The pieces below are ported (in
[pedacos.rs](pedacos.rs), read from the disassembly byte by byte, compiled over HEAD 1b33a4990 with no new warning,
never run); the two big ones are missing. There is no `codigo.rs`: the item does not enter the queue (nor `ordem.txt`).

## The routine's map

| Range | What it is | State |
|---|---|---|
| 0x5ce2d0..0x5ce33c | prologue; a local LLIST (0x68d020, never used; destroyed at the end by 0x68d040); the loop over the staff table | missing (trivial) |
| 0x5ce33c..0x5ce400 | per person: skipped without a nation (+0x1a) or with the state +0xb & 0x30 == 0x20; club = the contract's (`get_contract`) or the club job for jobs 5 and 12; `club_get_linked_b_team(job, &local, 1)` (hidden in the pseudo-C; the local is read at 0x5d2392); the club's contract (0x4cf5f0) | missing (small) |
| 0x5ce400 | no player sheet: 0x5d3367 | — |
| 0x5ce406..0x5ce4b6 | player: `daily_wear` (0x62c2f0); the gate (3 days a year by id, or contract/division, or reputation ≥ 0x128e, or id % 7 + day == 0); outside it, the next person | missing (small) |
| 0x5ce4b6..0x5d2392 | the player's day: morale, grievances (+0x45 bit by bit), transfer requests, press, contract (~16 KB, dozens of `_ftol` and float compares) | **missing** |
| 0x5d2392..0x5d352d | the grievances with a club, the squad status (raised by a draw with `_ftol`), the wish to go back to the nation [inference] (contract days × morale in double) | **missing** |
| 0x5d3367..0x5d352d | the non-player | `staff_day_non_player` |
| 0x5d3539..0x5d377a | the unattached person loses reputation | `staff_day_unattached_decline` |
| 0x5d3786..0x5d38f9 | tired after training (pow of professionalism) | `staff_day_training_condition` |
| 0x5d38f9..0x5d393e | the 3-days-a-year gate (id % 365, +120, +240); outside: 0x5d578d | missing (trivial) |
| 0x5d393e..0x5d3a0c | the day's two ability tables (malloc, qsort) | `staff_day_build_ability_tables` |
| 0x5d3a13..0x5d3a23 | `game_average(G_PLAYER_STATS, 0)` kept at [esp+0x4c] (hidden in the pseudo-C; read by the development) | missing |
| 0x5d3a29..0x5d3a8d | the error box of line 0x1901 (no sheet): dead, the sheet was already tested | — |
| 0x5d3a92..0x5d3af0 | the age at which development turns, by position | `staff_day_development_age` |
| 0x5d3af0..0x5d3d09 | the personality drift (1 time in 10) | `staff_day_personality_drift` |
| 0x5d3d09..0x5d5771 | the development of ability and attributes (~6.8 KB, nearly all the x87: age in float, `clamp_char_sum` 0x5d61d0, `pow_double` 0x5d5a20, the new ability, the "moved" mark at [esp+0x28]) | **missing** |
| 0x5d5771..0x5d578d | attributes normalised (0x538bc0), relationships faded (0x547f90), the day's mark [esp+0x67] | missing (trivial) |
| 0x5d578d..0x5d5864 | the clubs' shortlists | `staff_day_shortlist_check` |
| 0x5d587a..0x5d58e0 | end of the day: the abilities handed back in order | `staff_day_hand_back_abilities` |

Also: `staff_day_add_morale`, the -100..100 clamp of the contract's morale the routine repeats some 60 times.

## Pseudo-C traps found

- hidden arguments: `club_get_linked_b_team` (0x5ce3cd), the `_ftol(random(days - 360) × 0.25)` (0x5d36a4),
  `game_average` (0x5d3a1b), `get_shortlist_manager` (0x5d5ab0), whose arguments stay on the stack for 0x842c00;
- in the development (0x5d3d09..0x5d5771) the calls of `clamp_char_sum` (0x5d61d0) and `random` come out with their
  arguments shuffled in the pseudo-C; only the disassembly will do.

## Defects of the original

1. The loop stops one record early: the end is base + (n − 1) × 0x6e with `<`, so the table's last person never gets
   the day.
2. The development age tests "defender ≠ 0" (not "≥ 15"): nearly every outfield player goes back to 31, undoing the
   midfield and attack ages.

## What keeps it from being hooked, besides the size

The pass-through comparer (only the original runs), like its game.cpp siblings
(`compare_game_update_squad_relationships`), checks nothing: with it a wrong copy passes the chain (Rust ×2 only proves
determinism). The routine calls dozens of routines with effects outside the people (news, contracts, shortlists,
`0x588cd0`, `0x744240`, `0x72c1f0`), so side by side would mislead too. Before hooking: a real comparer (e.g. the pure
pieces side by side with the person's state and the generator, the rest checked against a plan), or accepting the
pass-through with strong mutations.

## To continue

The two missing blocks, each with the disassembly at hand (`desmonta.py jogo/cm0102.exe 0x5ce4b6 0x5d352d` and
`0x5d3d09 0x5d5771`), one piece at a time; then the loop (0x5ce33c..0x5ce4b6, 0x5d38f9, 0x5d5771) and the hook. The
routine runs every day in the ENG career (the development only on each player's 3 days a year).
