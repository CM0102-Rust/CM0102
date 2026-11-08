*[English version](LEGIBILIDADE.en.md).*

# Placar de legibilidade

Gerado por `python engenharia-reversa/legibilidade.py`. O que ainda atrapalha quem lê o motor de fora (`rust/src/motor.rs` e `rust/src/motor/`); cada rodada da etapa 4 da reorganização deve baixar esses números.

## Total

| Medida | Quantidade |
|---|---:|
| deslocamentos crus | 9483 |
| chaves fixas cruas | 1681 |
| chamadas por endereço | 0 |
| cópias de acesso à memória | 0 |
| rotinas sem cabeçalho | 0 |
| locais/parâmetros em português | 1418 |
| linhas | 205686 |

## Por arquivo (os 40 piores, pelos deslocamentos crus)

| Arquivo | deslocamentos crus | chaves fixas cruas | chamadas por endereço | cópias de acesso à memória | rotinas sem cabeçalho | locais/parâmetros em português | linhas |
|---|---:|---:|---:|---:|---:|---:|---:|
| `motor.rs` | 614 | 0 | 0 | 0 | 0 | 106 | 10722 |
| `motor/match_screens.rs` | 603 | 4 | 0 | 0 | 0 | 79 | 6233 |
| `motor/manager_manager.rs` | 385 | 2 | 0 | 0 | 0 | 26 | 5468 |
| `motor/transfer_manager.rs` | 355 | 29 | 0 | 0 | 0 | 108 | 11312 |
| `motor/comp/cup.rs` | 334 | 5 | 0 | 0 | 0 | 16 | 3614 |
| `motor/match_eng.rs` | 283 | 7 | 0 | 0 | 0 | 106 | 11700 |
| `motor/comp/league.rs` | 279 | 20 | 0 | 0 | 0 | 32 | 4276 |
| `motor/national_teams.rs` | 261 | 32 | 0 | 0 | 0 | 27 | 3717 |
| `motor/staff_records.rs` | 259 | 2 | 0 | 0 | 0 | 22 | 2435 |
| `motor/comp/fix_man.rs` | 245 | 31 | 0 | 0 | 0 | 27 | 2766 |
| `motor/comp/friendly.rs` | 232 | 26 | 0 | 0 | 0 | 37 | 3495 |
| `motor/injury.rs` | 227 | 0 | 0 | 0 | 0 | 23 | 3183 |
| `motor/contract_manager.rs` | 221 | 15 | 0 | 0 | 0 | 34 | 4510 |
| `motor/match_pl.rs` | 209 | 5 | 0 | 0 | 0 | 39 | 10101 |
| `motor/shortlist_manager.rs` | 178 | 29 | 0 | 0 | 0 | 30 | 4476 |
| `motor/media.rs` | 173 | 2 | 0 | 0 | 0 | 33 | 3570 |
| `motor/player_stats.rs` | 168 | 8 | 0 | 0 | 0 | 10 | 1566 |
| `motor/comp/leagues/fin_prm.rs` | 159 | 24 | 0 | 0 | 0 | 9 | 3205 |
| `motor/club_records.rs` | 157 | 3 | 0 | 0 | 0 | 4 | 1325 |
| `motor/match_man.rs` | 142 | 1 | 0 | 0 | 0 | 28 | 4587 |
| `motor/match_day.rs` | 140 | 9 | 0 | 0 | 0 | 17 | 2029 |
| `motor/comp/stadium.rs` | 138 | 19 | 0 | 0 | 0 | 15 | 1410 |
| `motor/player_regen.rs` | 117 | 6 | 0 | 0 | 0 | 18 | 3530 |
| `motor/scout_manager.rs` | 117 | 9 | 0 | 0 | 0 | 12 | 1317 |
| `motor/database.rs` | 113 | 82 | 0 | 0 | 0 | 52 | 9503 |
| `motor/officials_manager.rs` | 104 | 0 | 0 | 0 | 0 | 6 | 918 |
| `motor/tactics.rs` | 94 | 0 | 0 | 0 | 0 | 7 | 1486 |
| `motor/discipline.rs` | 88 | 2 | 0 | 0 | 0 | 15 | 1157 |
| `motor/player_search.rs` | 87 | 0 | 0 | 0 | 0 | 6 | 966 |
| `motor/formation.rs` | 72 | 0 | 0 | 0 | 0 | 3 | 1139 |
| `motor/comp/eurocomp/european_cup.rs` | 72 | 8 | 0 | 0 | 0 | 9 | 1587 |
| `motor/training_manager.rs` | 65 | 0 | 0 | 0 | 0 | 1 | 1382 |
| `motor/comp/worldcup/world_cup.rs` | 62 | 14 | 0 | 0 | 0 | 3 | 708 |
| `motor/area.rs` | 59 | 0 | 0 | 0 | 0 | 4 | 354 |
| `motor/comp/uefa_seeding.rs` | 59 | 7 | 0 | 0 | 0 | 7 | 1056 |
| `motor/comp/comp.rs` | 57 | 9 | 0 | 0 | 0 | 2 | 788 |
| `motor/match_events.rs` | 53 | 0 | 0 | 0 | 0 | 8 | 1966 |
| `motor/compare/club_records.rs` | 53 | 0 | 0 | 0 | 0 | 0 | 450 |
| `motor/human_manager.rs` | 52 | 3 | 0 | 0 | 0 | 5 | 815 |
| `motor/news.rs` | 49 | 0 | 0 | 0 | 0 | 4 | 1133 |
