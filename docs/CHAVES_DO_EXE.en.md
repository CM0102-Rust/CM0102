# The exe's fixed keys: what the code handles by name

Survey of 2026-10-30, read only (disassembly of `cm0102.exe` 3.9.68). Tool: `engenharia-reversa/chaves.py`, which writes `docs/chaves_uso.csv` (one row per key). The global names come from maxevans60's map (`referencias/cm0102-function-map`, CC BY 4.0; see `ENDERECOS_COMUNIDADE.en.md`). Marking: [observed] = seen in the binary; [inference] = conclusion without direct proof.

## Short answer

- The exe holds no nation, competition or club ids written as numbers. It holds **names**. On load, `set_key_ids` (0x601a40) sets each `g_key_*` global to −1 and looks up each fixed name in the database tables. It then stores the row it finds in the global. The rules only compare a record's id with that global [observed]. The load and what happens when a name is missing are in `TESTE_CHAVES.en.md`.
- There are **1,284 keys**, used at **8,809 places** in the code outside the load. The load checks 1,263 of them: when one stays −1, the "Unable to find the %s index" box appears [observed].
- Decoupling the competitions means, in practice, replacing those places with data. The full list, with the source files of each use, is `chaves_uso.csv`.

## By type

| Type | Keys | Checked on load | Uses outside the load | Source files |
|---|---|---|---|---|
| nation | 210 | 209 | 3,679 | 115 |
| club competition (includes the national-team ones, which are in the same table) | 304 | 296 | 2,989 | 238 |
| club | 313 | 308 | 1,332 | 25 |
| award (`staff_comp`) | 417 | 415 | 458 | 35 |
| continent | 6 | 6 | 315 | 43 |
| colour | 34 | 29 | 36 | 2 |

The source files come from `docs/mapa_funcoes.csv`. About a quarter of the uses fall in routines with no known file ("?" in the CSV).

## Where it weighs

- **Nations in the national-team competitions:** the Euro qualifiers (`euro_champ_qual.cpp`, 49 nations, 546 uses) and the European World Cup qualifiers (`wc_europe_league.cpp`, 50 nations, 510 uses) build groups and pots nation by nation. The South American ones (`wc_south_american_league.cpp`, 180 uses) do the same. Each tournament's host (`host_country.cpp`, 45 nations and 15 competitions) is fixed too [observed: the references; that they are lists of groups, pots and hosts is inference from the file names and the counts].
- **Nations spread through the simulation:** contracts (`staff_contracts.cpp`), transfers and the shortlist (`transfer_manager.cpp`, `shortlist_manager.cpp`) and the World Cup (`world_cup.cpp`) ask about specific nations. This is where the Northern Ireland for Austria swap mentioned in `CAMPOS.en.md` lives: Northern Ireland (0x9cf3e4) has 61 uses in 21 files.
- **Club competitions:** each league rewrites its own rules by key id (`bra_nat_first.cpp`, `spa_first.cpp`, `swe_prm.cpp`, `ita_ser_a.cpp`, `por_prm.cpp`, with 57 to 71 uses each). `Setup.cpp` (189) ties the competitions to the nations while the game is set up. The most cited are the World Cup (91 uses) and "a lower division" (52 uses), the generic lower division.
- **Clubs:** almost all in the database load (`Database.cpp`, `index.cpp`). Outside it, the rules for named clubs in Argentina (`arg_prm.cpp`, `arg_second.cpp`), in the MLS (`usa_mls.cpp`) and in the CONMEBOL draw (`conmebol_seeding.cpp`). Of the 313 clubs, 49 are B or II teams (Spanish and German reserves).
- **Awards:** each `*_awards.cpp` hands out its own by key id (Brazil has 58 uses, Italy 40, England 32).
- **Continents:** mostly the friendlies (`friendly.cpp`, 112 uses) and the Club World Cup.

## Keys the load does not check (21)

- 5 colours;
- Burma (Myanmar);
- groups A to D of the Northern Irish league cup;
- the Irish and Turkish "unmatched" cups;
- the Turkish Football Writers' Cup;
- 2 Belgian awards;
- 5 clubs: Godoy Cruz, Elgin City, Peterhead, C.P. Mérida and C.P. Mérida B.

If one of them is missing, no warning appears: the global stays −1 and the comparisons just stop matching [observed: no `cmp [key], edi` in the load; the in-game effect is inference]. The 6 counters named `g_key_nation_*_loop_end`/`active_count` in the map are not keys, and the tool leaves them out.

## Keys with no use outside the load (40)

Examples:

- the South American Super Cup and Recopa;
- the Argentine Metropolitan and Interior zones;
- the Croatian divisions;
- the Turkish 2nd division groups;
- the Brazilian regional cups;
- Belgian, Danish and Brazilian 3rd division awards.

These keys are looked up and checked, but no code reads them afterwards. If the name is not in the database, the load shows the box for nothing [observed].

## For decoupling and the editor

- Renaming a competition, nation or club on the list breaks the key on load (`TESTE_CHAVES.en.md`). Swapping one country for another requires changing the places of use, not just the name. That is why this kind of swap is done by patch today.
- `chaves_uso.csv` says, for each key, how many places and which files need to move to data. The qualifiers (Euro and World Cup) and `host_country.cpp` concentrate the nations. The league rules and the awards concentrate the competitions.
- In the port, the Rust version can read the same name table and return the rows. That way the name table stops being code and becomes data [inference; proposal].
