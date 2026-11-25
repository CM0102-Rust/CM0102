# rxf: what was left out (reader rxf, round 6, 25/11/2026)

No `rotinas.py`: this folder only keeps the notes. The rxf list gathered the leftovers of `rx11b_5`, `rxs1_12` and
`rx10_15` (24 routines). Items `rxf_1` to `rxf_6` cover 10: the whole award manager (constructor, destructor, load,
`create_nation_awards`, save, the shortlist news text and the two destructors of the `AWARD` base), the
`Shortlist_Manager::load` and `WorldClubChamp::pick_entrants`. The 14 below were left for lack of time; none is a false
entry, a thunk or an empty body, and none is in `rust/src` or in the queue (checked on 25/11 through `Prologue`/`ROTINAS`).
The notes on each are still those of the source `FORA.md` files.

## Simulation

- **0x92ae60 `WorldClubChamp::pick_entrant`** and **0x92a140 `WorldClubChamp::create_knockout_stage`**: see
  `rx10_15/FORA.md`. The signature of `pick_entrant`, checked on the pushes of `pick_entrants` (rxf_6): thiscall(continent,
  nation, list, slot, club competition) ret 0x14; slot 5 passes the UEFA Cup's id (`[0x9cf6f0]`), the others -1; slot 11
  passes (-1, host).
- **0x7a30c0 `player_setup_cfg_load`** and **0x7a3a10 `player_setup_apply_line`**: see `rx10_15/FORA.md` (they need a
  test `player_setup.cfg` in the bench's data).
- **0x8bf060 `TRANSFER_MANAGER::add_config_future_transfer`**: see `rxs1_12/FORA.md` (it only runs with the same player
  configuration file: with 0x7a3a10 it would be the same proof).

## SAVE

- **0x8985c0 `Tactics::read_from_save`** (4 KB, CARGA): see `rx11b_5/FORA.md`.
- **0x8bc020 `TRANSFER_MANAGER::load_from_disk`** (5.2 KB, CARGA, after rxs1_11): see `rxs1_12/FORA.md`. It calls the
  `Shortlist_Manager` constructor (0x839a80, sx20_5), which calls `rxf_5`.
- **0x52dad0 `save_database`** (15 KB): see `rxs1_12/FORA.md`.

## SCREENS

- **0x843800 `Shortlist_Manager::club_wants_player`**: see `rx11b_5/FORA.md`.
- **0x8c0f40 `submit_loan_contract_offer`**, **0x8ccb80 `get_club_future_transfer_list`** and **0x8ce0e0
  `set_player_squad_status`**: see `rxs1_12/FORA.md`.
- **0x806630 `SearchFilters::set_option_flag`** and **0x807450 `SearchFilters::player_matches_filter`**: see
  `rx10_15/FORA.md`.
