*Portuguese original: [FORA.md](FORA.md).*

# gra_2b: game_daily_staff_update (0x5ce2d0, game.cpp): out, for now

> **25/11/2026 (gra reader, round 6):** continued and delivered in [gra_2c](../gra_2c/rotinas.py): the whole routine hooked, the development and the loop ported, proven by the emulation of the whole routine (`check_routine.py`). This `pedacos.rs` stays only as history.

Reader gra, round 6, 25/11/2026, continuing [gra_2](../gra_2/FORA.en.md). The routine is **not** hooked and does not
go into the queue: the development (0x5d3d09..0x5d5771) is missing, and the hook is of the whole routine. There is no
`rotinas.py`, `codigo.rs`, `mutacoes.py` or `PRONTO.txt`.

## What moved on

The whole day of a contracted player, **0x5ce4b6..0x5d352d** (two thirds of the routine), is ported in
[pedacos.rs](pedacos.rs), read from the disassembly and proved by emulation. The gra_2 pieces are kept with it (the
non-player, the unattached decline, the training condition, the ability tables, the turning age, the personality
drift, the shortlist check and the abilities handed back), still without emulation.

| Range | Piece (Rust) |
|---|---|
| 0x5ce4b6..0x5ce9f9 | `staff_day_unrest_odds`: the favoured team mate, fatigue and form, the youngster's status, the odds of the unrest roll |
| 0x5ce9f9..0x5cf106 | `staff_day_grievance_unrest`: each grievance rolls against its own odds |
| 0x5cf106..0x5cf391 | `staff_day_unrest_roll`: the impatient chairman, the assistant's training report |
| 0x5cf391..0x5cf57c | `staff_day_disputes_and_morale`: disputes, the unrest falling, the transfer status, morale towards 10 |
| 0x5cf57c..0x5cfa3a | `staff_day_contract_moods`, `staff_day_european_flag`: flags 0x10, 0x1000, 0x200, 4 and the grievances that come and go |
| 0x5cfa97..0x5d0041 | `staff_day_grievance_fades`: the grievances that fade or start, going to the media (0x23) |
| 0x5d0041..0x5d0e27 | `staff_day_youngster_praise`, `staff_day_international_praise`, `staff_day_club_stories` (+ `_criticism`, `_praise`): the media stories |
| 0x5d0e27..0x5d10e5 | `staff_day_value_demand`: the value and flag 8 |
| 0x5d10e5..0x5d1448 | `staff_day_squad_status`: Bosman, retirement, jobs, assurances, the 0x70 player |
| 0x5d1448..0x5d16e8 | `staff_day_wants_move`: grievance 1 (wants a move) and settling back |
| 0x5d16e8..0x5d2392, 0x5d24e7..0x5d28f8 | `staff_day_contract_talks` (+ `_new_contract`, `_contract_reminder`, `_contract_running_out`, `_going_home`, `_relations`) |
| 0x5d28f8 | the end for the excluded jobs (grievances 0x10 and 2) in `staff_day_player` |
| 0x5d2392..0x5d352d | `staff_day_settling` (+ `staff_day_homesick`, `sd_learn_language`) |
| (order) | `staff_day_player`, `staff_day_part_b`: the pieces in sequence with the original's jumps |

Every call to an exe routine goes through the exe address (`exe::...` or `FN_` with `transmute`), hooked ones included,
so that the emulation catches it and the game goes through the hooks.

## The proof: emulation (unicorn)

[check_player.py](check_player.py) runs the original from 0x5ce4b6 (with its stack frame built: club at [esp+0x1c],
contract, sheet, id, the residue [esp+0x70], the B team at [esp+0x7c]) up to 0x5d352d/0x5d3539, and the copy
(`staff_day_player` through a test export, [apply_test_build.py](apply_test_build.py)) in **the same fake world**:
person, sheet, contract, 10 clubs, nations, continents, divisions, state tables, managers. The ~80 called routines are
stubbed by address, with values drawn in call order (the same on both sides if the copy calls the same), the floating
point ones returned in st0; the contract's flags and grievances really change (`clear_flags`, `set_flags`,
`clear_grievances`, `apply_event_flags`). It compares the list of calls with their arguments (byte or short ones only
at the width the callee reads) and every byte written to the world, and the residue [esp+0x70].

- **8,000 cases, 0 differences** (on the final build of `pedacos.rs`); 5,502 of the range's 6,262 instructions executed (the range includes the
  non-player, 0x5d3367..0x5d352d, which this test does not walk: about 94% of the player's day).
- Not walked by the emulation (checked only by reading): 0x5cf367..0x5cf38e, 0x5cf953..0x5cf962, 0x5cfbfe..0x5cfc3d, 0x5cff18..0x5cff58, 0x5d080a..0x5d0823, 0x5d09d4..0x5d0a07, 0x5d0a1f..0x5d0a69, 0x5d0b12..0x5d0b25, 0x5d0bb2..0x5d0bc7, 0x5d0d2f..0x5d0d91, 0x5d0db3..0x5d0e0c, 0x5d1025..0x5d1033, 0x5d16c3..0x5d16d0, 0x5d192c..0x5d19de, 0x5d19ef..0x5d1ae0, 0x5d1b1e..0x5d1b42, 0x5d1cda..0x5d1d07, 0x5d1d25..0x5d1d37, 0x5d1d91..0x5d1e2d, 0x5d1e45..0x5d1ebd, 0x5d1ece..0x5d1ee0, 0x5d236c..0x5d237b, 0x5d2499..0x5d24ae, 0x5d2591..0x5d25a0, 0x5d263a..0x5d2649, 0x5d269d..0x5d26ac, 0x5d2738..0x5d2745, 0x5d2840..0x5d287e, 0x5d29b7..0x5d29c9, 0x5d2d1c..0x5d2d7e, 0x5d2e7c..0x5d2e8b, 0x5d3183..0x5d31b1, 0x5d3235..0x5d3244, 0x5d32cd..0x5d3325.
- The days used deliberately open the routine's day gates ((day + id + k) % m), and 15% of the cases have id 0.
- **It discriminates:** the mutation `random(5)` → `random(6)` in one grievance roll shows a difference at case 0.
- Use: `python check_player.py DLL MAP [N] [first] [c.txt]`, with the test DLL built with `/MAP`; `c.txt` is the
  routine's compact disassembly ([compact.py](compact.py)), only for the coverage. `emu.py` is the engine (from sbB).
- Found with the tool itself: unicorn keeps translated code; the st0 return uses one fixed stub per `ret` size, never
  rewritten (rewriting it gave a wrong stack).

## The proposed comparer

Side by side in the game does not work: the routine touches everything, calls dozens of routines with effects (news,
contracts, lists, media) and draws hundreds of times. Proposal: a **pass-through comparer** (only the original runs,
like its game.cpp siblings) in the chain, plus the **emulation of the whole routine** as the item's proof (extending
`check_player.py` to the loop and the development) and **strong mutations** that the ENG career catches. Pure pieces
side by side are not worth it: nearly all of them call exe routines with effects.

## What is missing

1. **The development, 0x5d3d09..0x5d5771** (6.8 KB, nearly all x87). Read up to 0x5d50a0; the map:
   - 0x5d3d09..0x5d3eba: the age factor x = (turning age - age), × 0.25 when positive (float at [esp+0x20]); below 0,
     the reputations fall: past 45 all three become 1; past 41, × 0.875/0.875/0.75; with x < -3 (`0x96afc0`),
     × 0.925/0.925/0.825; else × 0.975^-x and 0.95^-x (the CRT `pow`); with random(natural fitness) > 14, x becomes
     random(ftol(x × 10)) × 0.1.
   - 0x5d3eba..0x5d3fd7: stats 6/1, 6/0x11, 1/0x11 and 1/1, a share t = (x × 25 + 50) × 0.01 × (1/1) held to 0..100,
     and a weighted rating (the game's average - 0.25 without games; `game_average` at [esp+0x4c]).
   - 0x5d3fd7..0x5d4384: growth: the marks [esp+0x48] (byte, counter) and [esp+0x58] (byte), reputation up 1.025/1.01
     before 22, the growth at [esp+0x28] (short) from the 13-short table at `0x9cd21c` (-15..14) indexed by two
     ratings held to 0..12, fatigue (`injury_get_fatigue`) and injury.
   - 0x5d4384..0x5d46b1: with growth above 0, training: the club factor (training +0x7f, or reputation), the human
     manager or the youth coach, **the loop over the assistant and the 5 coaches** (cl from -1 to 4), the division,
     held to 0.1..3, and the final growth held to growth ± 5 and to 2..25.
   - 0x5d46b1..0x5d4996: professionalism shrinks or stretches the growth; with fatigue above 1050 the fall is limited
     to 5% of the ability; the ability rises by random(growth); at a club in a weak division, a rise above a ceiling
     (division reputation² × 25 + club reputation, × 0.01) is cut (`0x96af50` = -0.9), never below the old one.
   - 0x5d4996..0x5d4ad2: with ability above 80 and a rise above random(20), the human manager may get the praising
     training report (`news_post_training_report`, 1) from the assistant, by the selection score.
   - 0x5d4ad2..0x5d50a0: the ability held to the potential; the physical attributes through `clamp_char_sum` (26 calls,
     arguments shuffled in the pseudo-C), by random(growth) / factor.
   - 0x5d50a0..0x5d5771: **not read** (the rest of the attributes; the random(2) draws in bytes [esp+0x74] and
     [esp+0x78], whose dword carries 3 bytes of stack residue, harmless because `clamp_char_sum` reads only the low
     byte).
   - Traps: nearly every `fstp`/`fld` with `[esp+X]` is shifted by pending `push`es (e.g. 0x5d4a94 writes [esp+0x4c],
     not [esp+0x70]); [esp+0x30] holds the club job (0x5d3952); [esp+0x27] becomes the coach loop counter; [esp+0x28]
     is the growth (read at 0x5d5780 for the "moved" mark, [esp+0x67]).
2. **The loop and the joins**: the prologue and the per-person gate (0x5ce2d0..0x5ce4b6, described in gra_2), 0x5d352d
   (the unattached decline only without a club: `staff_day_unattached_decline(p, sheet)`), 0x5d377a (no sheet: next
   person), the training condition, the 3-days gate (0x5d38f9), the tables, `game_average`, the turning age, the drift,
   the development, 0x5d5771 (`staff_normalise_player_attributes`, `staff_fade_staff_relationships`, the mark), the
   shortlist check and the end (0x5d5864).
3. Then: `rotinas.py` (the prologue at 0x5ce2d0 starts `push ebp; mov ebp, esp; and esp, -8`, no relative), `codigo.rs`
   from `pedacos.rs`, the pass-through comparer, `mutacoes.py` and the emulation proof of the whole routine.

## Defects of the original found in this reading

1. **[esp+0x70]**, the favoured team mate's bonus (0x5ce830): a short never initialised, added from one person to the
   next; the first one reads stack residue. The copy starts at 0 (`StaffDayRun::slot70`).
2. **Gates that almost never open**: `(id % 7) + day == 0` (0x5ce48d and 0x5cfbe8), `(day % 14) + id == 0` (0x5cf23c),
   `(day % 10) + id == 0` (0x5cf890), `(day % 7) + id == 0` (0x5d2485): only for id 0 (or day 0); the intent would be
   `(id + day) % k`. Effect: the ordinary player's daily pass, the unsettled player's training report, grievance
   0x80000000 fading, the first team football story and the star's wage demand almost never happen.
3. **0x5d2e6f**: `(grievances & 1) == 0x10000` never holds: the temperament draw that would drop grievance 0x10000 is
   dead code (only the transfer list drops it).
4. **0x5d23a6 and 0x5d23e6**: the ambition and temperament tests before dropping grievances 0x10 and 4 are useless: on
   every branch "left out more than once" decides.
5. **The coach loop (0x5d4454)**: an empty coach slot repeats the previous coach (edi kept), who counts twice in the
   training factor.
6. The name `staff_state::INJURY_COUNTER` (+0xa) does not fit its use here: it is the day's unrest counter (compared
   with 50, 15 and ability × 0.15, raised by the grievance draws, lowered with morale). A request for the field readers.

The gra_2 defects still hold (the table's last person never gets the day; the turning age with "defender ≠ 0").

## To go on

Read 0x5d50a0..0x5d5771 (`compact.py` and `desmonta.py jogo/cm0102.exe 5d50a0 5d5771`), port the development from the
map above, join the loop, and extend `check_player.py` to the whole routine from 0x5ce2d0 (the world already has
nearly everything; it lacks the staff table, the sheet table for the ability tables and stubs for `daily_wear`,
`game_average` and `get_shortlist_manager`).
