# rx11: what was left out (reader rx11, round 6, 25/11/2026)

No `rotinas.py`: this folder only holds the notes (the Portuguese `FORA.md` is the original, with the full details).
Of the 46 addresses in `rx11.txt`, 30 became items rx11_1 to rx11_12; the other 16 follow.

## Out for good

- **0x55f1a0 and 0x55f1b0:** 5-byte `jmp rel32` thunks passed as element destructors (see `tel40_4/FORA.md`).
- **0x567000 and 0x567260:** not entries: the middle of an instruction and an epilogue of `Display::process_command_packet`
  0x564080 (see `tel46_3`, `tel26_2`).
- **0x9216d0 and 0x922350:** dead code with the game starting in 2001 (see `cx17_7/FORA.md`).

## Not written this round (time; none waits for a decision)

- **Physio, screens (the physio report click 0x7a2f10):** 0x7a1a90 (files the 0x1392 news item, 16 slots), 0x7a2650
  (the assessment; floating point and side effects on `[0xae1080]` and bit 0 of +0xc of the player's 0x4f-byte record
  at `[0xae2d0c]`) and 0x7a1cd0 (the 2.3 KB report text, `compare_text_writer`).
- **Staff_Records::format_staff_history_entry 0x86e0b0, screens:** 4.6 KB, a switch on the kind byte; one item in
  named pieces, as `gra_1`.
- **ReserveComp, simulation (runs daily in the ENG career):** 0x7e6380 `schedule_reserve_fixtures`, 0x7e6720
  `add_b_or_under21_fixture`, 0x7e6f70 `add_result`, 0x7e7280 `reserve_result_news_text`. The constructor is in rx11_12.
- **Shortlist_Manager:** 0x843800 `club_wants_player` (0 calls in the world coverage to 2002-08-01; called by the AI's
  swap offer 0x8c843b and a screen 0x47ed88) and 0x83a9d0 `load` (a SAVE item with CARGA).
- **Tactics::read_from_save 0x8985c0:** a SAVE item with CARGA (4 KB, with the old-version upgrade path).

## Defects of the original found

- 0x9109e0: with the format byte +0xba outside 0..3 the group count and size are stack leftovers; on a failed `new` the
  team array and the round table are not freed.
- 0x917320: group B's fourth nation comes from the entrant list at index `team_count` (+0x3e) instead of group 1's
  winner [inference: a slip]; with fewer than three knockout or play-off survivors, slots 1 and 2 read stack leftovers;
  with no seed left, slot 0 keeps the malloc garbage.
- 0x911ba0: returns −1 whenever the current code is 1, even when the previous one was 1 too [inference: a slip].
- 0x5788c0 and the other `init_awards`: the award index only moves on when `new` succeeds.
- 0x7a2650 (not written): writes `[0xae1080]` as a side effect of a query.
