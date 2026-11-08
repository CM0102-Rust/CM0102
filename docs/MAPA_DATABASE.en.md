# Map of the database block

*[Versão em português](MAPA_DATABASE.md).*

Survey for the database batches (agreed order: match_screens/match_day, save, **Database**, competitions). Read only: disassembly of `cm0102.exe` 3.9.68, the community map (`referencias/cm0102-function-map`, `map/Database.cpp.md`, `map/Db_files.cpp.md`, `map/index.cpp.md` and `data-map/Database.cpp.md`), the Bosman Edition inventories (`referencias/cm0102-bosman-edition/docs/research/2026-10-13-database-inventory.md`, `2026-10-14-start-new-game-inventory.md`, `2026-10-14-game-creation-inventory.md`: pseudocode checked against the disassembly, an emulation of the original in Unicorn), `docs/mapa_funcoes.csv` and the sibling projects (§3). The save block is in [MAPA_SAVE.en.md](MAPA_SAVE.en.md).

Sizes in bytes, measured from the entry to the last byte before the padding (the community map's "size" is not in bytes). "Rust" = there is a `Prologue { address: ... }` in `rust/src/motor.rs`. Effect columns: **SEH** (frame with `fs:[0]`), **allocates** (`malloc` 0x944e46, `new` 0x944cf1, `realloc` 0x945667, `GlobalAlloc`), **file** (CRT `fopen`/`fread`/`fwrite` or `zopen`/`zread`/`zwrite`), **random** (0x90cfc0 or `rand` 0x946a04), **box** (0x5e8290: asserts and errors open a box and wait for a click).

**Numbers:** `Database.cpp` has 181 real routines (184 entries, 3 false fragments), `Db_files.cpp` 40, `index.cpp` 11 (21 entries, 10 fragments). In Rust: **19** (11 from `Database.cpp` plus 8 of the preferences, 0x547790..0x547d90). None from `Db_files.cpp` or `index.cpp`.

**Automatic career:** the 17/10 coverage (`docs/cobertura_simulacao.txt`) did not instrument `Database.cpp` or `Db_files.cpp`; from `index.cpp` it recorded the key matchers (13,640 calls, one per club). Creation through `auto_setup.cfg` goes `create_game_from_setup_file` 0x827110 → `load_database_from_files` (0x827430/0x8275d3, when the nation table is empty) → `create_game` 0x828110 (0x82790b): the whole load and creation run **once**. The "Career" column below comes from that path; the day-to-day queries are "to measure" (`python engenharia-reversa\cobertura.py Database.cpp Db_files.cpp` is the first step of the batches).

## 1. The functions

### cm3/code/Db_files.cpp (table readers and writers)

All have the shape `int f(void *stream, int use_zip, void *records, index_entry *e)` (readers) or `(stream, use_zip, records, count)` (writers): `use_zip` picks `zread`/`zwrite` (save) or `fread`/`fwrite` (`Data` folder). The reader checks the entry version (+0x3f), which is fixed per table (batch 228, from the binary): 2 for nation, clubs, competitions, staff_comp and player/non-player; 1 for continents, stadiums, staff v1, histories, names, colours and referees; 1 or 2 for cities. Any other version opens the "Error" box and the game exits. Then the reader reads `e->count` records in one go. None has SEH.

| Address | Name (community) | Bytes | Table (index id) | Record | Rust | Career |
|---|---|---|---|---|---|---|
| 0x54d580 | `db_files_open_for_read` | 43 | builds the path in 0xde1f64 and `fopen` "rb" | — | no | yes (24×) |
| 0x54d5b0 | `db_files_open_for_write` | 43 | the same, "wb" | — | no | no |
| 0x54d5e0 | `db_files_read_continents` | 241 | continent (3) | 198 | no | yes |
| 0x54d740 | `db_files_read_nations` | 243 | nation (4) | 290 | no | yes |
| 0x54d8a0 | `db_files_read_cities` | 241 | city (21) | 56 | no | yes |
| 0x54da00 | `db_files_read_stadiums` | 235 | stadium (5) | 78 | no | yes |
| 0x54db50 | `db_files_read_clubs` | 314 | club (0), nat_club (1); then turns +0x80 = 0 into 1 | 581 | no | yes (2×) |
| 0x54dc90 | `db_files_read_clubs_v1` | 2365 | club version 1, field by field | 581 | no | **no** (the shipped database is version 2) |
| 0x54e630 | `db_files_read_staff` | 237 | staff (6) version 2 | 110 | no | no (save and databases like CLZ) |
| 0x54e780 | `db_files_read_staff_v1` | 241 | staff (6) version 1, into the temporary table 0xae2404 | 157 | no | yes |
| 0x54e880 | `db_files_read_players` ¹ | 283 | id 9 → 0xae23c0 | 68 | no | yes |
| 0x54e9a0 | `db_files_read_players_v1` ¹ | 1583 | id 9 version 1 | 68 | no | **no** |
| 0x54f030 | `db_files_read_non_players` ¹ | 283 | id 10 → 0xae23c4 | 70 | no | yes |
| 0x54f150 | `db_files_read_non_players_v1` ¹ | 2652 | id 10 version 1 | 70 | no | **no** |
| 0x54fc10 | `db_files_read_staff_extra` | 235 | preferences (22) | 52 | no | no (only version 2 databases and saves) |
| 0x54fd60 | `db_files_read_staff_history` | 235 | staff_history (17) | 17 | no | yes |
| 0x54feb0 | `db_files_read_comps` | 283 | club_comp (12), nation_comp (16) | 107 | no | yes (2×) |
| 0x54ffd0 | `db_files_read_comps_v1` | 510 | comps version 1 | 107 | no | **no** |
| 0x550230 | `db_files_read_comp_history` | 235 | club_comp_history (19), nation_comp_history (20) | 26 | no | yes (2×) |
| 0x550380 | `db_files_read_staff_comps` | 237 | staff_comp (11) | 101 | no | yes |
| 0x5504d0 | `db_files_read_staff_comp_history` | 235 | staff_comp_history (18) | 58 | no | yes |
| 0x5505c0 | `db_files_read_officials` | 235 | officials (7) | 43 | no | yes |
| 0x550710 | `db_files_read_colours` | 235 | colour (2) | 58 | no | yes |
| 0x550860 | `db_files_read_names` | 235 | first/second/common_names (13, 14, 15) | 60 | no | yes (3×) |
| 0x54d6e0, 0x54d840, 0x54d9a0, 0x54daf0 | `db_files_write_continents/nations/cities/stadiums` | 87, 87, 81, 81 | writers | 198, 290, 56, 78 | no | no |
| 0x54e5d0, 0x54e720, 0x54efd0, 0x54fbb0 | `db_files_write_clubs/staff/players/non_players` | 87, 81, 81, 81 | writers | 581, 110, 68 ¹, 70 ¹ | no | no |
| 0x54fd00, 0x54fe50, 0x5501d0, 0x550320 | `db_files_write_staff_extra/staff_history/comps/comp_history` | 81 each | writers | 52, 17, 107, 26 | no | no |
| 0x550470, 0x5506b0, 0x550800, 0x550950 | `db_files_write_staff_comps/officials/records_0x3a/or_read_records_0x3c` | 81 each | writers (0x3a: colours and staff comp history; 0x3c: names, writes or reads) | 101, 43, 58, 60 | no | no |

¹ **The community map swaps player and non-player.** By `resolve_database_indexes` (staff +0x61 → 0xae23c4 of 70 bytes; +0x69 → 0xae23c0 of 68), by Bosman's creation inventory and by CMScout (`player.dat` 70, `nonplayer.dat` 68): **id 9 = non-player (68 bytes), id 10 = player (70)**. It matches `docs/CAMPOS.en.md` (player current ability at +0x05, a short after a byte at +4). The routine names stay as in the map; the batches use the right meaning.

### cm3/code/Database.cpp: load, save and release

| Address | Name | Bytes | What it does | Callers | Effects | Rust | Career |
|---|---|---|---|---|---|---|---|
| 0x524d50 | `load_database_from_files(dir, shared)` | 21 | `load_database(dir, shared, 0)` | `setup_start_new_game` 0x81cea5, `create_game_from_setup_file` (2×), `create_game` 0x8282a7 | — | no | **yes** |
| 0x524d10 | `load_database_from_stream` | 21 | `load_database(…, 1)` | `load_game_data` 0x82abf2 | — | no | no |
| 0x524d30 | `save_database_for_game` | 26 | `save_database(…, 1, …)` | `save_game_data` 0x82e6d6 | — | no | no |
| 0x528540 | `load_database(dir, shared, from_save)` | 21903 | reads `index.dat` and the tables, converts staff from 157 to 110 bytes, turns indexes into pointers, fixes staff, finds the keys (§2) | the two wrappers | allocates (one `GlobalAlloc` block in shared mode, one `malloc` per table otherwise); file; random (only on bad data); box; progress bar | no | **yes** (1×) |
| 0x52dad0 | `save_database(dir, to_save, a3, a4)` | 15053 | turns pointers back into indexes (the id at +0 of the target, null → −1), writes each table and `index.dat` (with the tail), packs the `*.lng` (`zadd`) and restores the pointers (`resolve_database_indexes`, 5 calls) | `save_database_for_game` | file; box ("%s table valid, but no entry in %s") | no | no |
| 0x5315a0 | `resolve_database_indexes(lenient)` | 6226 | replaces each id field with the pointer to the record; negative or out of range → null (logged, or a box for staff → player/preferences/non-player). Full field table: Bosman's creation inventory §2d | `load_database`, `save_database` | box | no | **yes** |
| 0x528080 | `database_free_all_tables` | 1201 | frees everything (`free` or `GlobalFree` by mode), zeroes the counts, deletes the `.lng` files from `CM3_TEMP` | `load_database`, `load_game_data`, `free_game_data` | frees; deletes files | no | **yes** |
| 0x527ee0 | `database_free_staff_history_table` | 57 | frees the staff history | `PlayerStats::PlayerStats` 0x7c0ccd (it then lives in `player stats history.tmp`) | frees | no | **yes** |
| 0x527f20 | `database_free_staff_comp_history_table` | 57 | frees the award history | `AWARD_MANAGER::AWARD_MANAGER` 0x413360 | frees | no | **yes** |
| 0x527f60 | `database_grow_comp_history_table(n)` | 287 | grows the competition history (26 bytes) and moves the pointer of the national part | `comp_util_add_history_record` 0x4afd2c | allocates | no | yes (end of a competition) |

### cm3/code/Database.cpp: game creation (once)

Order in `create_game` 0x828110 (Bosman's creation inventory §3a, checked on the `call`s):

| Address | Name | Bytes | What it does | Effects | Rust | Career |
|---|---|---|---|---|---|---|
| 0x541e60 | `database_trim_senior_and_b_team_squads` | 299 | club +0x244 = has a B pair; joint A+B squad cut to 50 (qsort 0x5402f0), keeping 3 goalkeepers | `qsort` | no | **yes** |
| 0x539750 | `database_build_nation_name_index` | 908 | per-nation index of the name tables (first row, count, weight total), at 0xae2338, 0x24 bytes per nation | allocates; box | no | **yes** |
| 0x539ae0 | `database_prune_comp_history_table` | 162 | keeps only rows of selected nations; `realloc` and `qsort` (0x540780) | allocates | no | **yes** |
| 0x524d70 | `database_trim_staff_tables(size)` | 6730 ² | trims staff, players, non-players and preferences by the database size option (`[0xae3138]+4 & 7`), keeping the best per nation; in-place `realloc` with a same-address assert; copies 10 random non-players over the last 10 rows | SEH; allocates; random (0x90cfc0 15× and direct `rand`); box | no | **yes** |
| 0x527240 | `database_generate_extra_non_playing_staff` | 3232 | creates extra staff for clubs that lack them (names from the nation, count by reputation) | random; box | no | **yes** (called by 0x524d70) |
| 0x5267c0 | `database_trim_club_table(size)` | 2676 | trims clubs by the same option, keeping those referenced by staff and histories | allocates; box | no | **yes** |
| 0x544970 | `database_merge_b_team_staff` | 756 | B squad cut to a joint 50; B backroom staff moves to A | — | no | **yes** |
| 0x53db40 | `database_sort_clubs_by_reputation(refill)` | 89 | sorts the club id vector 0xae2d1c by reputation (`qsort` 0x540020) | — | no | **yes** (creation, `load_game_data` and `game_main_day_loop`) |
| 0x535a80 | `database_player_initialisation(date)` | 12602 | sets up every player and staff member of the new game: attributes, reputations, contracts, values; "Player Initialisation" bar | random; box; bar | no | **yes** |
| 0x532e00 | `database_apply_fake_players` | 332 | "Use Real Players: No": shuffles players, random names from the nation, caps | random | no | no (option off) |
| 0x533080 | `database_shuffle_player_records` | 1378 | swaps records of random pairs (keeps id, club, ability) | SEH; random | no | no |
| 0x533710 | `database_shuffle_squad_player_attributes` | 647 | swaps ability and physicals between similar team mates | random | no | no |
| 0x53a610, 0x53aa10 | `database_player_to_attribute_vector`, `…vector_to_player` | 661, 378 | player record ↔ vector of 63 cells (value, weight) | — | no | **yes** |
| 0x53a8b0, 0x53a980 | `database_non_player_to_attribute_vector`, `…vector_to_non_player` | 208, 134 | the same for the non-player bytes 0x0e..0x22 | — | no | **yes** |
| 0x538bc0, 0x538ec0, 0x539490 | `staff_normalise_player_attributes`, `…_physical_attributes`, `…_non_player_attributes` | 758, 1488, 701 | rebalances the attribute vector (also on the training update) | — | no | **yes** |

² 0x524d70 runs to 0x5267c0; the community map cuts it at 0x524f50 (a "false fragment", a country code read as a pointer).

### cm3/code/index.cpp (the keys)

The **keys** are 1,284 globals (0x9cf178..0x9d0524, plus 26 German amateur teams at 0xb63c9c..0xb63d00; list and uses in `CHAVES_DO_EXE.en.md`) holding the index of records the code uses by name: colours ("Black", "Red 1"…), nations, club and national competitions, continents, staff competitions (awards) and clubs. Each matcher is a chain `if (!_stricmp(name, "Argentina")) key = row; else if …` (0x951c50).

| Address | Name | Bytes | What it does | Effects | Rust | Career |
|---|---|---|---|---|---|---|
| 0x601a40 | `set_key_ids` | 41022 ³ | sets every key to −1; matches colours (inline), nations, club competitions (up to `count − 127`), continents, awards and clubs; with `[0x9b5984] ≠ 0` it first reads `CM3_TEMP\eng.lng`; every key left at −1: "Unable to find the %s index" and a box | file (only with translated names); box | no | **yes** (1×) |
| 0x60d810 | `match_nation_key_id(name, row)` | 6071 | the nation chain | — | no | **yes** (213×) |
| 0x60efd0 | `match_club_comp_key_id` | 9737 | the competition chain | — | no | **yes** (452×) |
| 0x6115e0 | `match_staff_comp_key_id` | 4494 | the award chain | — | no | **yes** (543×) |
| 0x6146b0 | `match_club_key_id` | 10341 ³ | the club chain | — | no | **yes** (13,640×) |
| 0x60ba80 | `remap_club_key_id(old, new)` | 7416 | stores the new row in the club key that held the old id | — | no | **yes** (13,640×) |
| 0x60d780 | `report_missing_key_id` | 129 | logs and opens the missing-key box (line 0x16b1) | box | no | no (shipped database is complete) |
| 0x616f20 | `get_region_name(region, form)` | 2132 ³ | localised name or adjective of one of the 14 regions | — | no | to measure (screens) |
| 0x617780, 0x6177f0 | `get_region_article_form`, `get_continent_adjective` | 104, 629 | Italian article; continent adjective | — | no | to measure (screens) |
| 0x6181d0 | `nation_get_nationality_adjective` | 26014 | nationality adjective by language, gender and number | — | no | to measure (screens, news) |

³ Measured up to the next real routine: the community map splits 0x601a40 at 0x606060, 0x6146b0 into five pieces and 0x616f20 into four.

### cm3/code/Database.cpp: language of the names (`.lng`)

| Address | Name | Bytes | What it does | Effects | Rust | Career |
|---|---|---|---|---|---|---|
| 0x535920 | `database_get_names_language` | 6 | `[0x9b5984]` | — | **yes** | yes |
| 0x535930 | `database_language_file_exists(language)` | 329 | does `CM3_TEMP\<language>.lng` exist? (0, 1, 11, 12 → eng) | file | no | **yes** |
| 0x5339a0 | `database_load_language_names(language, force)` | 8052 ⁴ | reads the translated names of clubs, nations, competitions, cities and stadiums from the `.lng` into the tables | SEH; allocates; file; box; bar | no | no (language 0) |
| 0x544fc0 | `eng_lng_load_tables` | 1066 ⁴ | reads `CM3_TEMP\eng.lng` and checks the 8 counts against the tables | allocates; file; box | no | no |
| 0x544f30 | `eng_lng_free_tables` | 133 | frees the 8 English tables | frees | no | no |
| 0x5453f0, 0x5454c0, 0x5455a0, 0x5456a0, 0x545770, 0x545880, 0x545990 | `nation_get_english_name`, `…three_letter_name`, `comp_…`, `staff_comp_…`, `club_get_english_short_name`, `club_get_english_name`, `stadium_…` | 207, 209, 246, 206, 259, 259, 205 | the English name (loads `eng.lng` on first use) | SEH; allocates | no | no (language 0) |
| 0x53cee0 | `database_get_language_name` | 2136 | language name in the buffer 0xae21d4 | — | no | to measure |

⁴ With the pieces 0x535552 (0x5339a0) and 0x545350 (0x544fc0).

### cm3/code/Database.cpp: queries and texts (the rest)

They are not the "database" in the loading sense: they are queries over the tables, called by the simulation and the screens, and go through the usual queues (`candidatos.py --simulacao`, screens). Bytes in brackets.

- **National teams and A/B pairs** (Rust where marked ✓): `database_get_national_team` 0x53b3d0 (282) ✓, `…_u21_team` 0x53b4f0 (280) ✓, `database_is_national_team` 0x53b900 (159) ✓, `club_is_national_team` 0x5409b0 (156) ✓, `club_get_linked_b_team` 0x540a50 (5132) ✓, `club_is_b_team` 0x5448c0 (162), `club_get_national_team_name` 0x542100 (1588), `database_club_has_reserve_squad` 0x542740 (345), `club_can_be_managed` 0x544820 (151), `club_is_in_season` 0x547770 (23).
- **Club squad and jobs:** `database_club_has_room_for_job` 0x53b9a0 (868), `database_add_staff_to_club` 0x53bd10 (1148), `database_remove_staff_from_club` 0x53c190 (1020; also in the load), `database_count_club_squad` 0x53c590 (820), `…_coaches` 0x53c8d0 (224), `…_physios` 0x53c9b0 (224), `database_get_club_manager` 0x53ca90 (237), `…_assistant` 0x53cb80 (167), `…_coaches` 0x53cc30 (293), `…_scouts` 0x53cd60 (293), `database_collect_club_squad` 0x541f90 (360), `club_collect_non_playing_staff` 0x544c70 (696), `staff_set_squad_flags` 0x540980 (40), `database_is_not_plain_player_job` 0x53b610 (13) ✓, `database_is_player_job` 0x53b620 (29) ✓, `database_get_contract_rating_modifier` 0x53ce90 (79) ✓, `club_count_squad_with_position_mask` 0x548150 (66), `staff_get_natural_position_mask` 0x5481a0 (332).
- **Attributes and reputation:** `staff_get_player_attribute` 0x5431e0 (160), `staff_is_player_attribute_visible` 0x543280 (440), `…_for_viewer` 0x543440 (256), `staff_compute_player_attribute` 0x543540 (1488), `staff_get_non_player_attribute` 0x543f30 (160), `staff_compute_non_player_attribute` 0x543fd0 (788), `player_attribute_rating_with_scaled_ability` 0x5442f0 (85), `player_attribute_rating` 0x544380 (135), `non_player_attribute_rating` 0x544410 (135) ✓, `database_get_staff_reputation_for_viewer` 0x5407e0 (224), `database_get_reputation_scope` 0x5408c0 (184), `club_reputation_for_staff` 0x545e90 (797), `staff_adjust_morale` 0x544710 (265, random), `database_get_staff_age` 0x53ab90 (171), `staff_get_player_strength_flags` 0x546ab0 (1567, random), `…_weakness_flags` 0x5470d0 (1684, random).
- **Preferences and opinions:** 0x547790, 0x5477d0, 0x547810, 0x547840, 0x547af0, 0x547d30, 0x547d60, 0x547d90 ✓; still to do `club_add_favoured_staff` 0x547870 (208), `club_add_disliked_staff` 0x547940 (208), `staff_add_favoured_staff` 0x547a10 (211), `club_remove_favoured_staff` 0x547bd0 (63), `staff_remove_favoured_staff` 0x547c10 (52), `staff_remove_disliked_staff` 0x547c50 (52), `staff_count_disliked_staff_at_club` 0x547c90 (70), `…favoured…` 0x547ce0 (70), `staff_add_disliked_club` 0x547e70 (213), `staff_remove_favoured_club` 0x547f50 (52), `staff_fade_staff_relationships` 0x547f90 (243, random), `staff_get_favoured_staff` 0x548090 (94), `…disliked…` 0x5480f0 (94), `staff_is_best_favoured_candidate` 0x548810 (1422), `staff_pick_favoured_team_mate` 0x548da0 (856), `staff_clear_opinion_links` 0x549c90 (82), `staff_add_opinion_link` 0x549cf0 (112), `staff_remove_opinion_link` 0x549d60 (101), `staff_get_opinion_link_staff` 0x549dd0 (113).
- **Squad strength (AI):** 0x549100 (253), 0x549200 (187), 0x5492c0 (253), 0x5493c0 (166), 0x549470 (87), 0x5494d0, 0x5495b0, 0x549690, 0x549770 (216 each), 0x549850, 0x549940 (238 each), `club_squad_strength_band` 0x549a30 (506).
- **Contract and lists:** `staff_is_transfer_listed` 0x5444a0 (128), `…_or_requested` 0x544520 (98), `staff_is_loan_listed` 0x544590 (98), `staff_is_listed_by_request` 0x544600 (98).
- **Rules by nation or club (fixed names):** `database_is_staff_eu_national` 0x53d740 (144), `database_club_nation_is_eec` 0x53d7d0 (26), `database_is_basque_club` 0x53d7f0 (264), `database_is_melanesian_nation` 0x53d900 (57), `…polynesian…` 0x53d940 (57), `database_is_staff_foreign` 0x53d980 (436), `nation_allows_free_transfers` 0x544670 (85), `nation_is_uk_or_ireland` 0x5446d0 (57), `club_nation_is_not_argentina` 0x5487b0 (32), `club_nation_not_argentina_france_croatia` 0x5487d0 (49), `club_is_in_major_nation` 0x549e50 (73), `nation_is_major_nation` 0x549ea0 (66).
- **Names:** `database_pick_first_name_by_weight` 0x539b90 (207), `…_for_nation` 0x539c60 (1152), `database_pick_second_name_by_weight` 0x53a0e0 (208), `…_for_nation` 0x53a1b0 (1016), `database_get_empty_common_name` 0x53a5b0 (93) ✓, `Staff::operator=` 0x5335f0 (286), `database_staff_matches_names` 0x53e990 (282), `database_format_staff_name` 0x53dba0 (1428; also in the load, only for the log), `database_format_name_from_ids` 0x53e140 (408), `…_from_strings` 0x53e2e0 (1708), `database_format_official_name` 0x53eab0 (916).
- **Screen texts:** `database_get_coaching_job_name` 0x53ac40 (180), `database_get_job_name` 0x53ad00 (1740), `database_format_player_positions` 0x53ee50 (4653), `database_format_years_between_dates` 0x540340 (325), `database_get_player_attribute_name` 0x5428a0 (2356 with 0x542f48), `database_get_non_player_attribute_name` 0x543b10 (1048), `write_position_name` 0x5461b0 (1007), `write_player_description` 0x5465a0 (1291), `position_mask_get_text` 0x5482f0 (1204), `get_adjective_form_index` 0x545a60 (852), `get_regions_sorted_by_name` 0x545dc0 (200), `compare_region_names` 0x545e00 (136).
- **Colours:** `database_get_club_kit_colours` 0x53b640 (373), `database_get_comp_colours` 0x53b7c0 (309), `database_colours_clash` 0x540490 (139), `database_get_colour_family` 0x540520 (604).
- **`qsort` comparators:** 0x53fd40 and 0x53fe50 (265 each: latitude and longitude of the stadium city), 0x53ff60 (124), 0x53ffe0 (61), 0x540020 (93), 0x540080 (489, SEH), 0x540130 (313, SEH), 0x540270 (197), 0x5402f0 (69), 0x540780 (82), 0x549c30 (82).
- **Static initialiser:** `init_transfer_rule_change_date` 0x524cd0 (33: 31/08/2001) and the empty destructor 0x524d00. They run before the game; left out.

## 2. The flows

### New game (`load_database(Data folder, 1, 0)`)

Path: Start New Game (`setup_start_new_game` 0x81ce00) or `create_game_from_setup_file` 0x827110 → 0x524d50 → 0x528540. Returns 1, or 0 on a short read.

1. `CM3_TEMP` folder; `database_free_all_tables`; "Loading database" bar (46 steps: 0, 1, 22 for sizing, 22 for reading, 1 final).
2. Copies the 10 `*.lng` from `Data` to `CM3_TEMP` (0x4c0aa0/0x4c0b50 and 0x90cc50); without the `.lng` of the preferences language, language 0.
3. `index.dat`: two ints (0xae2344, 0xae2348; 0 and 0 in the database), 22 entries of 67 bytes at 0xae1a80 (name 51, id +0x33, count +0x37, offset +0x3b, version +0x3f) and **one more byte**, which the database lacks (it stays 0). With that byte ≠ 0 (the `index.dat` inside a save), it reads 0x1cc bytes, a list into 0xae23fc (club → row of the English name) and the names language 0x9b5984.
4. Sizing: sums the counts by id (`switch`) and reserves rows: staff + 16 human managers + 23,000 reserve rows (with the save byte, without the 23,000), non-players + 16 + 23,000 (likewise), plus 1,325 rows allocated and not counted at the end of staff and players, club competitions + 127 (the friendlies), each name table + 16. In shared mode (the new game's), **one single `GlobalAlloc` block**; otherwise one `malloc` per table. Temporary 157-byte staff table at 0xae2404.
5. The 127 reserve competitions, the 23,000 reserve staff (kind +0x5f = 6, default date 10/02/1972) and the 16 human manager rows (empty names at the end of the three name tables).
6. Read in index order, reopening the file and seeking (`fseek`) for each entry (`staff.dat` opens four times).
7. Staff conversion 157 → 110 and building the preferences (52 bytes: id + 12 ints from the file at +0x61..+0x8d); frees the temporary table.
8. `resolve_database_indexes(0)`; deletes `duff.out`.
9. Staff pass (all but the last 16 and kind 6): removes from extinct clubs, fixes job 7 → 6, missing nation (from the club, the national team or **random**), missing player/non-player data, null name → first empty common name (for all three names: a bug of the original, to copy), national team without a job.
10. Under-21 national teams (the second half of `nat_club`) get half the senior team's reputation (+0x80); a division named "Reserve" → null.
11. `set_key_ids`.

| # | File (shipped database) | id | Records | Record (file / memory) | Pointer [count] |
|---|---|---|---|---|---|
| 0 | club.dat | 0 | 13,214 | 581 | 0xae23b4 [0xae235c = 13,640 with nat_club] |
| 1 | nat_club.dat | 1 | 426 | 581 | 0xae23b8 (in the club block) [0xae2360] |
| 2 | colour.dat | 2 | 34 | 58 | 0xae23ec [0xae2394] |
| 3 | continent.dat | 3 | 6 | 198 | 0xae23a4 [0xae234c] |
| 4 | nation.dat | 4 | 213 | 290 | 0xae23a8 [0xae2350] |
| 5 | officials.dat | 7 | 4,567 | 43 | 0xae23e8 [0xae2390] |
| 6 | stadium.dat | 5 | 8,917 | 78 | 0xae23b0 [0xae2358] |
| 7 | staff.dat (off. 0) | 6 | 169,342 | 157 / 110 | 0xae23bc [0xae2364 = 192,358] |
| 8 | staff.dat | 8 | 0 | — | (no `switch` case) |
| 9 | staff.dat (off. 26,586,694) | 9 | 30,865 | 68 (non-player) | 0xae23c0 [0xae2368 = 53,881] |
| 10 | staff.dat (off. 28,685,514) | 10 | 139,769 | 70 (player) | 0xae23c4 [0xae236c] |
| 11 | staff_comp.dat | 11 | 543 | 101 | 0xae23e0 [0xae2388] |
| 12 | city.dat | 21 | 6,682 | 56 | 0xae23ac [0xae2354] |
| 13 | club_comp.dat | 12 | 433 | 107 | 0xae23d0 [0xae2378 = 579] |
| 14 | nation_comp.dat | 16 | 19 | 107 | 0xae23d8 (in the competition block) [0xae2380] |
| 15–17 | first/second/common_names.dat | 13–15 | 44,281 / 104,395 / 13,256 | 60 | 0xae23f0 / 0xae23f4 / 0xae23f8 [0xae2398..0xae23a0] |
| 18 | staff_history.dat | 17 | 385,846 | 17 | 0xae23cc [0xae2374] |
| 19 | staff_comp_history.dat | 18 | 2,609 | 58 | 0xae23e4 [0xae238c] |
| 20–21 | club/nation_comp_history.dat | 19, 20 | 10,766 / 273 | 26 | 0xae23d4 / 0xae23dc [0xae237c, 0xae2384] |
| — | (built) preferences | 22 | 169,358 | 52 | 0xae23c8 [0xae2370] |

At load time this is ~70 MB (staff 21 MB, players 10, names 10, preferences 9, clubs 8, staff history 7); after the creation trims, the ~15 MB that the career's `banco` fingerprint (batch 130) covers.

After the load: `Config::apply_and_save` 0x5d8480 (with a language different from the names' one, `database_load_language_names`), the league choice and `create_game` 0x828110, in the order of the creation table of §1 (trims by `[0xae3138]+4 & 7`, club vector by reputation, `database_player_initialisation`, the 44 steps and the days up to the start date). During creation the staff history and the award history move into the subsystems (`PlayerStats`, `AWARD_MANAGER`) and are freed from the database.

### Link with the save

- **Save:** `save_game_data` → `save_database_for_game` → `save_database(…, 1, …)`. Each table becomes a container item (`staff.dat` 110 bytes version 2, `Preferences.dat`, `nonplayer.dat`, `player.dat` as their own items, offset 0 in the index). The save's `index.dat` has the header `7f 00 00 00 10 00 00 00` (127 reserve competitions, 16 managers), the 22 entries and the **tail** (the byte ≠ 0, the 0x1cc bytes, the 0xae23fc list and the language: 30,045 tail bytes in CMExplorer's Valencia.sav; a whole index of 47,235 bytes in CM3SaveEditor's save). `staff_history` and `staff_comp_history` stay in the index but not in the container (they have already become subsystem `.tmp` files). The `*.lng` go along (`zadd`). Order checked: [MAPA_SAVE.en.md](MAPA_SAVE.en.md) §2.
- **Load:** `load_game_data` → `database_free_all_tables` → `load_database_from_stream` → `load_database(…, 1)`: the same path with `zopen`/`zread`/`zseek` instead of `fopen`/`fread`/`fseek` and the non-shared branch (0x5289c7..0x52b378). The `index.dat` byte being set changes the reserves (without the 23,000), reads the English names and the language; `zextract("*.lng")` returns the `.lng` to `CM3_TEMP`. A missing history table is freed and left with count 0 (0x52bcea).
- The save's tables are those of the already trimmed, living database (clubs, staff and players change along the career); the save **is** the database as of that day, plus the subsystems.

## 3. What the siblings offer (read only, owner rule)

| What | Where | Use |
|---|---|---|
| Database inventory, loader pseudocode, full `resolve_database_indexes` table, keys, creation steps | Bosman Edition: `referencias/cm0102-bosman-edition/docs/research/2026-10-13-database-inventory.md` (beware: swaps player and non-player, corrected in `2026-10-14-game-creation-inventory.md` §2b), `2026-10-14-start-new-game-inventory.md` §2, `2026-10-14-game-creation-inventory.md` §2–§3 | the main reference for the batches: order, counts, fields that become pointers, null conditions and the original's bugs to copy |
| Reader of `index.dat` and the tables, database and save; two staff versions (157/110); writing the database and the save | `D:\Sistemas\CMExplorer\docs\ABRIR_GRAVAR.md` §3–§6, `engenharia-reversa\index_dat.py` (`le_index`, `le_sav`) | format oracle for the readers; shows that community databases (CLZ 25-26) already come as **version 2** (110 bytes and preferences in id 22, `staff.dat` with four tables), a path the shipped database does not exercise |
| Record fields: staff, player, non-player, preferences, club, nation, city, stadium, names, official, contract | `D:\Sistemas\CMExplorer\docs\CAMPOS.md` (CMExplorer × SaveGame Editor), `docs\LIGACOES.md` (screen ↔ field of each form) | field names for the batches and the editor |
| Structures and editing screens in Rust | `D:\Sistemas\CM3SaveEditor\rust\cm3sav\src\`: `pessoa.rs` (`TelaPessoa`, `Registros`), `clube.rs` (`TelaClube`), `nacao.rs` (`TelaNacao`, `limpar_elenco`, `allow_management`), `competicao.rs` (`Tipo`: 107/101 bytes), `arbitro.rs`, `preferencias.rs` (`itens`, `gostar`, `incluir`), `nomes.rs` (`Nomes::completo`); `docs\CAMPOS.md` (TStaff, TClub, TNation, TComp, TPlayer, TStaffComp, TNonPlayer, TStadiums, TCities), `docs\REGISTROS.md`, `docs\FORMATOS.md` §4–§5 (the 20 database items in a real save and each record size), `docs\VALIDADOR.md` | the future editor (item 8) and the validator; sizes checked on a save |
| Reading the database from inside the save and the id → pointer swap | `D:\Sistemas\CMScout\docs\ABRIR.md` §4–§5 (the 8 loaders and the link table), `rust\olheiro\src\banco.rs` (`Banco`: `staff`, `jogador`, `clube`, `nacao`, `registro_nome`, `idade`; size constants) | an independent check of the `resolve_database_indexes` links (staff +0x61 → player, +0x69 → non-player) and of the contract at +0x6e/+0x72 of CMScout's memory |
| Editing the live game memory | `D:\Sistemas\CMRTE\docs\CAMPOS.md`, `rust\cmrte-nucleo` | the "CMRTE inside the game" of item 8: the same tables, already with pointers |
| Not useful | CMExplorer's writing (re-serialises whole tables, skips `colour`, `continent`, `staff_comp`, the competitions and `nation_comp_history`); the siblings' reading with an out-of-range id → null without the game's `>`/`>=` bounds | they belong to the editors; the game has its own rules (Bosman §2d) |

## 4. Proposed batches

New item type (**database**), next to screens, simulation and save. 33 items (≈ 95 routines) in 11 batches; the other queries and texts of §1 (except the names of N1/N2 and the comparators of T2) go through the simulation and screen queues.

| Batch | Item | Routines | Expected comparator |
|---|---|---|---|
| B1 | L1 readers | 0x54d5e0 continents, 0x54d740 nations, 0x54d8a0 cities | **file**: the same `.dat` (copy in `jogo_testes`) read by the original and the Rust into separate buffers; compares buffer, return and file position. Self-test with versions 1 and 2 and a short read (never an invalid version: box). Career: passes 1× |
| B1 | L2 readers | 0x54da00 stadiums, 0x54db50 clubs (+0x80), 0x54e780 staff 157 | same |
| B1 | L3 readers | 0x54e880 non-players, 0x54f030 players, 0x54fc10 preferences | same; 0x54fc10 only by self-test (a version 2 database built in `jogo_testes`) |
| B2 | L4 readers | 0x54fd60, 0x54feb0, 0x550230 | same |
| B2 | L5 readers | 0x550380, 0x5504d0, 0x5505c0 | same |
| B2 | L6 readers | 0x550710, 0x550860, 0x54e630 (staff 110) | same; 0x54e630 by self-test and later through the `.qst` |
| B3 | V1 version 1 converters | 0x54dc90 clubs, 0x54e9a0, 0x54f150 | `side_by_side` over synthetic version 1 bytes (fixed seeds, 1 to N records, a read that fails midway); the career does not pass (the shipped database is version 2) |
| B3 | V2 | 0x54ffd0 competitions v1, 0x54d580 open, 0x54d5b0 create | converter: same; open/create: return and path in 0xde1f64 |
| B3 | G1 writers | 0x54d6e0, 0x54d840, 0x54d9a0 | **file**: writes to two temporary files, compares bytes and return; also through `zwrite` into a test container |
| B4 | G2, G3, G4 | 0x54daf0, 0x54e5d0, 0x54e720; 0x54efd0, 0x54fbb0, 0x54fd00; 0x54fe50, 0x5501d0, 0x550320 | same |
| B5 | G5 | 0x550470, 0x5506b0, 0x550800 | same |
| B5 | G6 release | 0x550950 (60 bytes, reads or writes), 0x527ee0, 0x527f20 | 0x550950: file; releases: one-shot effect, through the career (database fingerprint) and a self-test on a test table |
| B5 | K1 keys | 0x60d810 nations, 0x60efd0 competitions, 0x6115e0 awards | `side_by_side`: region = the keys 0x9cf178..0x9d0528 (~5 KB) + return. Chains generated by a script from the exe (each pair is `push text; push edi; call 0x951c50; …; mov [key], esi`). Self-test: every name of the chain, upper and lower case, absent names. Career: 1,208× |
| B6 | K2 keys | 0x6146b0 clubs, 0x60ba80 remap, 0x60d780 report | same (13,640× each in the career); 0x60d780 only by self-test with the box detoured |
| B6 | T1 tables | 0x527f60 grow history, 0x539ae0 prune, 0x53db40 sort | `side_by_side` with the tables and the 0xae2d1c vector; `qsort` 0x945333 and `realloc` 0x945667 called from the game (same order of equal items, same address) |
| B6 | T2 orders | 0x540020, 0x540780, 0x5402f0 | pure `side_by_side` (two pointers → −1/0/1) with an exhaustive self-test |
| B7 | C1 creation | 0x541e60, 0x544970, 0x539750 | one-shot `side_by_side`: region = the whole database (normalised pointers, below) + random tape; career |
| B7 | C2 trims | 0x524d70 (6.7 KB, SEH), 0x5267c0, 0x527240 | same; `realloc` and direct `rand` risk (§5) |
| B7 | C3 vectors | 0x53a610, 0x53aa10, 0x53a8b0 | `side_by_side`: record + output vector (189 bytes); self-test with real records |
| B8 | C4 normalise | 0x53a980, 0x538bc0, 0x539490 | same |
| B8 | C5 initialisation | 0x538ec0, 0x535a80 (12.6 KB) | 0x538ec0: side by side; 0x535a80: one-shot orchestrator, whole database + tape (thousands of draws); career |
| B8 | N1 names | 0x539b90, 0x539c60, 0x53a0e0 | `side_by_side` + tape (they draw); self-test per nation |
| B9 | N2 names | 0x53a1b0, 0x5335f0, 0x53ab90 | same |
| B9 | C6 fake players | 0x532e00, 0x533080 (SEH), 0x533710 | whole database + tape; only with "Use Real Players: No" (to check whether `auto_setup.cfg` has the option; otherwise a `quick_start.cfg` built by `save_grande.py`) |
| B9 | I1 language | 0x535930, 0x544fc0, 0x544f30 | self-test with each `.lng` in a copy of `CM3_TEMP`; compares English tables and returns |
| B10 | I2, I3 English names | 0x5453f0, 0x5454c0, 0x5455a0; 0x5456a0, 0x545770, 0x545880 | `side_by_side` with the translated names loaded (language ≠ 0 in an isolated preferences folder, `CM3_PREFS`) |
| B10 | I4 | 0x545990, 0x5339a0 (8 KB, SEH) | same; 0x5339a0 = whole tables before and after |
| B11 | O1 orchestrators | 0x5315a0 resolve, 0x601a40 keys, 0x528080 free all | resolve: whole database once (ids before, normalised pointers after); keys: the globals; free: career |
| B11 | O2 load | 0x528540, 0x524d50, 0x524d10 | **load** (new): at creation, the original loads and takes the normalised snapshot (tables, counts 0xae2340..0xae2408, keys, `index` at 0xae1a80, draws); the Rust loads over it (the load starts by freeing everything) and takes the same snapshot; career with the database fingerprint on day 1 |
| after S4 | O3 save | 0x52dad0, 0x524d30 | through the `.qst` of the save test bench ([MAPA_SAVE.en.md](MAPA_SAVE.en.md) §4): database items and `index.dat` byte for byte |

**Normalised pointers:** after `resolve`, the id fields hold pointers. Between two loads the block may land at another address; the snapshot replaces each pointer by (table, offset), as `save_database` does (the id at +0 of the target). In Rust mode without comparison, the Rust must make the same allocations, same size and same order (the game's `GlobalAlloc`), or the database fingerprint changes with no logic error.

**Test bench for the block:** (1) the automatic career goes through the load and creation once, and the day 1 `banco` fingerprint covers the trimmed tables (a mutation in the readers, in `resolve` or in the keys shows there); (2) the **load** comparator (once per career, seconds) locates the divergence; (3) file self-tests for readers, converters and writers, on copies in `jogo_testes`; (4) the save block's `.qst` for saving and the save path (`load_database(…, 1)`); (5) language and fake players with isolated preferences (`CM3_PREFS`, or copying and restoring `game.cfg`). Before the batches: `cobertura.py Database.cpp Db_files.cpp` to separate the daily queries from the creation ones.

## 5. Risks

| Risk | Where | Care |
|---|---|---|
| One-shot effect | `load_database` (frees everything, copies `.lng`, deletes `duff.out`), trims (in-place `realloc`), `database_free_*`, `save_database`, `database_player_initialisation` | compare with a one-shot snapshot; the rest through the career |
| Allocation and address | one `GlobalAlloc` block in shared mode; `realloc` with a same-address assert (Database.cpp lines 0x18e..0x191; to check how this lives with the shared block) | call the game's allocation routines, same order and size |
| Random draws | staff pass (random nation, random common name: they do not fire on the shipped database), trims (0x90cfc0 and direct `rand` 0x946a04), staff creation, initialisation, names, fake players | random tape; `rng.rs` already calls the CRT `rand` |
| CRT `qsort` | 0x539ae0, 0x53db40, 0x541e60, prune (0x945333) | unstable: the Rust calls the same `qsort` with the ported comparator |
| Error boxes | invalid version in the readers, `resolve` (staff → player/preferences/non-player out of range), missing key, null-pointer asserts in almost every query | self-tests never on the error paths |
| Stack garbage | large frames: `load_database` (0x7d0), `save_database` (0xa10), `resolve` (0x604), `create_game` (0xb58), `database_player_initialisation` | the routine-by-routine comparison by outcome (no zeroing since 2026-10-31: stack garbage is not compared); the extra `index.dat` byte is zeroed on entry (0x528556) |
| Bugs of the original to copy | `resolve`: with a high club +0x5b it zeroes +0x60 and leaves +0x5b raw; a high stadium +0x48 zeroes +0x38; the three null names go to the **common** names table; club +0x80 `(char)(random(4000)+1)` | port as is; note in `ROTINAS.md` |
| Paths the shipped database does not exercise | version 1 converters, staff 110 and preferences in id 22 (version 2 databases such as CLZ), the extra `index.dat` byte (save only), missing keys | self-tests with synthetic databases; Lucas's CLZ only as a copy and without changing anything (`clz_teste/`) |
| Swapped names in the community map | `db_files_read_players`/`non_players` and writers | use id 9 = non-player (68), id 10 = player (70) |
| On-screen progress | bar 0x802f80 inside the load, the save and the initialisation | the Rust calls the same routine at the same steps (the screen is part of what Lucas sees) |
| Future independence (64 bits) | pointers in 4-byte fields | when the core leaves the exe, keep ids and resolve on access (Bosman's suggestion); not now |

## 6. For the editor (item 8 of [PROJETO.en.md](PROJETO.en.md))

Modelled on the Elifoot II Bombado editor (`D:\Sistemas\Elifoot II\bombado\editor`: a headless core with database, entities, validation and patch; the window on top).

**Data (editable in an editor, lives in `Data/`):**

- The 19 tables and `index.dat` (count, offset and version of each): staff, players, non-players, preferences (only in version 2 databases), clubs and national teams, nations, continents, cities, stadiums, colours, club and national competitions, awards, officials, names, histories.
- The translated names (`*.lng`) and the texts of `language.ldb`; `player_setup.cfg`, `pundit.cfg`, `weather.cfg`, `euro.cfg`, `events_<language>.cfg`.

**Fixed in the code (the editor needs to know, to validate or to turn into data later):**

- **The 1,284 keys by name** (`index.cpp`; where the code uses each one: `CHAVES_DO_EXE.en.md` and `chaves_uso.csv`): colours, nations, competitions, continents, awards and clubs. Renaming "England", "Premier Division" or a club on the list breaks the key and the game stops with "Unable to find the %s index". The editor's validation must check the list (generated from the exe), and "rules as data" (PROJETO §6.9) starts with it.
- A/B pairs (82 pairs of club keys from Spain, Germany and Portugal, `club_get_linked_b_team`), the 24 Basque clubs, the 5 Melanesian and 5 Polynesian nations, UK and Ireland, the 6 major nations, the free-transfer exceptions, Argentina/France/Croatia, the "Reserve" division, the "Bright…"/"Wimble…" clubs (+0x6d), Greece's 14 places (0x839920).
- National team table layout: the first 213 rows of `nat_club` are the senior teams, the next 213 the under-21 teams (half the reputation at load); the last 16 staff ids are the human managers; the last 127 club competitions are the friendlies; reserves of 23,000 staff and 1,325 players.
- The trim by the database size option (who stays, by nation and reputation) and the player initialisation (tables 0x9b5988, 22 shorts from 1 to 175, and 0x9b59c0, 20 `double`s from 2.1 to 0.9 by league position).
- The silent fixes of the load (§2, step 9, and the `resolve` nulls): the editor can warn about what the game will "repair" (job 7, missing nation, out-of-range ids) instead of letting the game decide.
- The texts of jobs, attributes and positions are in the code in English and go through `langlib_sprintf` (0x66f4e0), which swaps the text for the active language's template: the English text is the translation key (`database_get_job_name` 0x53ad00, `database_get_player_attribute_name` 0x5428a0, `database_get_non_player_attribute_name` 0x543b10, `position_mask_get_text` 0x5482f0) [observed]. Regions (`get_region_name` 0x616f20) use the same path and, in some languages, their own tables in the code. The **nationality adjectives** (`nation_get_nationality_adjective` 0x6181d0, the 24 KB) are in the code: nearly a thousand distinct texts for languages 9 (Portuguese), 4 (Italian), 5 (Spanish), 10 and 6, by key nation and form (gender and number). In form 0 and in other languages, the game uses the text of the nation record [observed; the routine is already in Rust, batch 240]. For the editor: changing an adjective in those languages requires changing the exe, not the database.
- Two staff versions (157 bytes in the shipped database, 110 + preferences in community databases): the editor reads both and writes the one of the source database.

## Request P7 (2026-10-22/23, approved by Lucas)

In items B9/B10 (language), bring in the `idiomas` crate from the langtool session (D:\Sistemas\CM0102-LangTool\rust, no dependencies; `cargo test --release`) and the formats of `docs/MAPA_LANGTOOL.md` §2, §4, §5 and §6.1 (language.ldb, .lng, the rule for applying translated names). §5.4 answers the question about job titles and adjectives. Use the crate's MSVC 6 qsort/bsearch: 202 repeated keys change the on-screen text. Proof: the Rust load compared with the original.

## Request P8 (2026-10-23, approved by Lucas): the findings of the original editor

Repository D:\Sistemas\CM0102-Editor (commit 35c219b; MAPA_EDITOR.md §3.1, §3.2, §6 and §10, LIGACOES.md, IDA_E_VOLTA.md):
- staff.dat v1 (157 bytes) decoded and checked on all 169,342 records (closes the FORMATOS.md gap); colour.dat (58) and officials.dat (43) complete;
- 369 fields of the official editor match the offsets and names in FORMATOS.md;
- database rules: club and nation reputations ×500; nat_club.dat rebuilt from the nations on every save; fixed club slots 3/50/5/7/3; reputation threshold 1000 with division; cm_date leap flag at 0 on births in leap years;
- the official editor's mandatory list does not apply (57 clubs and 6 nations do not match): the index.cpp keys are what count;
- IDA_E_VOLTA: what the official editor changes in a database (NationalJob −2 → −1, PERMANENT_ injuries vanish, seat −1 → 0, files not shortened);
- rust/nucleo: database reader and writer with byte-for-byte round trip, a candidate oracle for the Rust database load.

When the database batches start: format questions go to the session "Descompilar o editor original do CM 01/02"; once the Rust load and the index.cpp key list are ready, send CM0102-Editor a request (ORQUESTRACAO.md §4) with the keys and the real names, so it resumes the Pre-Editor.

## Coverage measured (2026-10-23)

Automatic career to 2002-01-01 with counters (`cobertura.py Database.cpp Db_files.cpp index.cpp`, `simular.py --cobertura`), 160 functions with a counter:

| File | Functions | Not called | Once | More than once, no Rust |
|---|---|---|---|---|
| Database.cpp | 106 | 25 | 19 | 62 (the everyday queries: reputation seen by another manager, 81 M calls; age, 18 M; foreign, 15 M; free transfer in the nation, 13 M; transfer list, 12 M...) |
| Db_files.cpp | 37 | 21 | 12 | 4 (reading the .dat files, at the start) |
| index.cpp | 17 | 7 | 1 | 9 (the club keys, ~13 thousand times) |

Order decided: first the Database.cpp queries (ordinary simulation batches, proven by the career); then reading the .dat files and the creation (which the save proof's .qst write already goes through: the quick start creates the game by reading the database); finally the language (P7) and the load.
