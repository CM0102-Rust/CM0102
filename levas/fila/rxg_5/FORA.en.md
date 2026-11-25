# rxg: what was left out (reader rxg, round 6, 25/11/2026)

No `rotinas.py`: this folder only keeps the notes. The rxg list was the 14 routines of `rxf_7/FORA.md`. Items `rxg_1`
to `rxg_4` cover 5: `TRANSFER_MANAGER::load_from_disk`, `Tactics::read_from_save`, `WorldClubChamp::pick_entrant`,
`WorldClubChamp::create_knockout_stage` and `TRANSFER_MANAGER::submit_loan_contract_offer`. The 9 below were left for
lack of time; none is a false entry, a thunk or an empty body, and the notes in the original `FORA.md` files (`rx10_15`,
`rx11b_5`, `rxs1_12`) still hold.

## SAVE

- **0x52dad0 `save_database`** (15 KB, Database.cpp): Ghidra's pseudocode has 2,600 lines. The first part swaps each
  table's pointers for the id (`p ? *p : -1`) in every pointer field of competitions (+0x59, +0x5d, +0x61, +0x65), clubs
  (about 70 slots, among them the arrays at +0xd7 (50), +0x1b3 (7), +0x19f (5), +0x1cf (3) and +0xc3 (3)), nations,
  cities, stadiums, people etc., only when `[0xae2340]` is not 0. Then, for each of the 24 tables: the progress bar
  (0x802f80 with the step), the lookup of its entry in the index (`0xae1ab3` to `0xae2074`), the "X table valid, but no
  entry in index.dat" box when missing, and the write to a file (0x54d5b0) or to the zip (0x9326e0 with
  `param_3`/`param_4`) by each table's writer (0x54d6e0, 0x5506b0...). It needs a session of its own, in named pieces
  (the `gra_1` model); the proof is the save's (I do not know whether the proof's `.qst` goes through it: check who
  calls 0x524d30).

## Simulation

- **0x7a30c0 `player_setup_cfg_load`**, **0x7a3a10 `player_setup_apply_line`** (5.7 KB) and **0x8bf060
  `TRANSFER_MANAGER::add_config_future_transfer`**: they only run with a `player_setup.cfg` in `Data/` (count 0 in the
  test career and in the world). Writing the three in one item only makes sense together with the test file in the
  bench data (one line of each kind, the future transfer included); without it no mutation is caught. It needs the main
  session's decision on the test file first.

## SCREENS

- **0x843800 `Shortlist_Manager::club_wants_player`**: see `rx11b_5/FORA.md` (the body is nearly its sibling's, already
  in Rust).
- **0x8ccb80 `TRANSFER_MANAGER::get_club_future_transfer_list`** (1.5 KB): read this round through the pseudocode;
  Ghidra hides the `ecx` of the `Date::operator=` calls (0x417c10) that write each row's date, so it still needs the
  line-by-line disassembly. Notes: the local list (0x68d020) of the club's live offer slots (statuses 2, 3, 0x14, 0x10,
  4 and 0xf with +0x2e ≥ 3 stay out), today's date (0x8c6100), the category by status (1→1; 5..8→2; 0xa→3; 0xb..0xd→4;
  0xe and 0x13→6; 0x11 and 0x12→5; the rest 0), the future transfer's date (the +0x920 array by slot +0x20) or the
  offer's (+0x12), the contract goes to `G_CONTRACT_LOOKUP_RESULT`, and one more row per exchange player; sorted with
  0x945333 (qsort, 0x1b, 0x8e55f0).
- **0x8ce0e0 `TRANSFER_MANAGER::set_player_squad_status`**: see `rxs1_12/FORA.md`.
- **0x806630 `SearchFilters::set_option_flag`** and **0x807450 `SearchFilters::player_matches_filter`**: see
  `rx10_15/FORA.md`.
