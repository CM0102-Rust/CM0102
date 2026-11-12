# nma_1: game_daily_staff_update (0x5ce2d0) stays out of this round

*English version of [FORA.md](FORA.md). Both are kept in sync; the Portuguese file is the original.*

The routine `game_daily_staff_update` (0x5ce2d0..0x5d5a1f, 30,544 bytes, game.cpp, cdecl(void)) was not ported in this
session (12/11/2026, porter `nmaA`). Nothing of it went into the queue: a partial copy cannot be hooked (the routine is
a single hook) and the item is only worth anything whole.

## Why

- **Size:** 9,229 instructions, 866 calls (395 `utils_random`, 54 `_ftol` 0x945640, 8 0x945ff0), a 0x888-byte frame
  with a local `LLIST` (0x68d020/0x68d040) and a `qsort` (0x945333) in the middle. It is the per-person body of a loop
  over the whole person table (`G_STAFF_TABLE`, 0x6e bytes, up to `G_STAFF_COUNT`), plus the tail (sorting 0xae2d24 by
  0x5d6220 and copying 0xae2d20 to +5 capped by +7, only when the local byte was set). A byte-faithful port (x87 order,
  each `_ftol`, every draw in order) is around 3,000 lines of Rust; it does not fit in one session with the faithfulness
  the queue requires.
- **Proof:** the routine cannot run twice (it touches every person, contract, dispute, news item, media story, list),
  so the comparer would be "original only", like the other day passes. Under the 31/10 criterion (the Rust ×2 careers
  compared with each other, the original only recorded), a porting error shows up in no test: only the mutations would
  cover it. The per-piece comparer asked for cannot run the original piece by piece (the per-person body is not a
  function). Before porting, how to prove it should be settled with Lucas: e.g. a comparer that snapshots the person's
  own fields and contract, runs the original on a one-person table (swapping `G_STAFF_TABLE`/`G_STAFF_COUNT` for a table
  of 1) and then the copy, with `STATE_REGIONS` and the news counted, and names the piece that diverged by its
  `covered!`; that needs to be shown feasible first (the called routines find the person by id in the table).

## Map of the pieces (for the next round)

Boundaries no short jump (up to 3000 bytes) crosses (script `mapa_pedacos.py` in this folder). The long jumps
crossing several pieces: 0x5ce400→0x5d3367 (a non-player skips to the staff part), 0x5cfa53/5e/85/91→0x5d2392,
0x5cfa91→0x5d352d, 0x5d111e..0x5d1155→0x5d28f8, 0x5d3938/0x5d3a8d→0x5d578d.

| piece | range | bytes | what it calls (summary) |
|---|---|---|---|
| entry | 0x5ce33c–0x5ce4b6 | 378 | filter (+0x1a; general.dat +0xb & 0x30 = 0x20 skips), contract (0x4cf5a0 → 0xae1080), B team, fatigue recovery, contract at the club |
| favoured team-mate | 0x5ce4b6–0x5ce5e7 | 305 | `staff_pick_favoured_team_mate`, fatigue, draws, human club |
| form and list | 0x5ce5e7–0x5ce9b9 | 978 | recent form, transfer listed |
| morale | 0x5ce9b9–0x5cf106 | 1,869 | 77 draws, contract flags, future transfer |
| dislikes and training | 0x5cf106–0x5cf391 | 651 | stats, dislike, injury, AWOL, staff rating, training report |
| disputes | 0x5cf391–0x5cf4c8 | 311 | `DISPUTE::*`, dispute news, wants to leave |
| contract grievances | 0x5cf4c8–0x5cfa64 | 1,436 | Comp_Stats, club likes, morale, flags and grievances, `Date::days_since` |
| injury and lists | 0x5cfa64–0x5cfc42 | 478 | injury, club shortlists, finances, contract events |
| grievances 2 | 0x5cfc42–0x5cfd53 | 273 | grievances, flags |
| media story | 0x5cfd53–0x5cff5e | 523 | `Media::create_player_media_story` |
| performance | 0x5cff5e–0x5d0e27 | 3,785 | 11 stats, rating, appearances, praise story, national team, thanks the manager |
| value and likes | 0x5d0e27–0x5d10e5 | 702 | wants to leave, player value, likes of clubs |
| retirement and free | 0x5d10e5–0x5d1327 | 578 | wealth band, retirement, free transfer, seeks assurances |
| contract | 0x5d1327–0x5d29de | 5,815 | 33 draws, contract offers, wants a new contract, deserved status, loan, offers |
| staff dislikes | 0x5d29de–0x5d2d9f | 961 | dislikes, contracts, ban, deserved status |
| grievances 3 | 0x5d2d9f–0x5d2fb3 | 532 | grievances, position training |
| languages and absence | 0x5d2fb3–0x5d377e | 1,995 | languages, events, AWOL, staff normalisation |
| fitness | 0x5d377e–0x5d38f9 | 379 | training, injury, match condition, national duty, poor fitness |
| sorting | 0x5d38f9–0x5d3a13 | 282 | 0x944e46 ×2, `qsort` |
| likes of clubs | 0x5d3a13–0x5d3d09 | 758 | mean rating, error box, likes of clubs |
| staff attributes (5 pieces) | 0x5d3d09–0x5d4320 | 1,559 | draws, `pow_double`, stats, fatigue |
| injury and attributes | 0x5d4320–0x5d4800 | 1,248 | injury, human club, staff ratings |
| fatigue | 0x5d4800–0x5d4996 | 406 | fatigue |
| playing time | 0x5d4996–0x5d4ad2 | 316 | playing-time mood, rotation, selection score, training report |
| player attributes (2 pieces) | 0x5d4ad2–0x5d5771 | 3,231 | 91 draws, `clamp_char_sum` ×26, training |
| person's tail | 0x5d5771–0x5d5864 | 243 | normalise, fade relationships, club shortlists |

The piece names come from the calls, not from reading the code [inference]. The whole disassembly:
`python engenharia-reversa/desmonta.py jogo/cm0102.exe 0x5ce2d0 0x5d5ae0`. The routine is the same in cm0102-1080.exe
(the 30,544 bytes checked). In the ENG career to 2002-06-01 it runs 322 times (`docs/cobertura_simulacao.txt`).
