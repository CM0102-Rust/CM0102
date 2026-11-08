*[English version](LEGIBILIDADE.en.md).*

# Placar de legibilidade

Gerado por `python engenharia-reversa/legibilidade.py`. O que ainda atrapalha quem lê o motor de fora (`rust/src/motor.rs` e `rust/src/motor/`); cada rodada da etapa 4 da reorganização deve baixar esses números.

## Total

| Medida | Quantidade |
|---|---:|
| deslocamentos crus | 8567 |
| chaves fixas cruas | 1821 |
| chamadas por endereço | 0 |
| cópias de acesso à memória | 0 |
| rotinas sem cabeçalho | 0 |
| locais/parâmetros em português | 1337 |
| linhas | 206530 |

## Por arquivo (os 40 piores, pelos deslocamentos crus)

| Arquivo | deslocamentos crus | chaves fixas cruas | chamadas por endereço | cópias de acesso à memória | rotinas sem cabeçalho | locais/parâmetros em português | linhas |
|---|---:|---:|---:|---:|---:|---:|---:|
| `motor.rs` | 614 | 0 | 0 | 0 | 0 | 105 | 10669 |
| `motor/match_screens.rs` | 576 | 4 | 0 | 0 | 0 | 76 | 6202 |
| `motor/manager_manager.rs` | 348 | 2 | 0 | 0 | 0 | 23 | 5367 |
| `motor/transfer_manager.rs` | 338 | 29 | 0 | 0 | 0 | 103 | 11073 |
| `motor/match_eng.rs` | 238 | 7 | 0 | 0 | 0 | 99 | 11902 |
| `motor/staff_records.rs` | 237 | 2 | 0 | 0 | 0 | 17 | 2429 |
| `motor/comp/league.rs` | 231 | 20 | 0 | 0 | 0 | 32 | 4234 |
| `motor/injury.rs` | 222 | 0 | 0 | 0 | 0 | 22 | 3143 |
| `motor/national_teams.rs` | 221 | 33 | 0 | 0 | 0 | 27 | 3690 |
| `motor/comp/friendly.rs` | 210 | 26 | 0 | 0 | 0 | 34 | 3461 |
| `motor/contract_manager.rs` | 209 | 16 | 0 | 0 | 0 | 32 | 4427 |
| `motor/comp/cup.rs` | 204 | 8 | 0 | 0 | 0 | 16 | 3590 |
| `motor/shortlist_manager.rs` | 179 | 31 | 0 | 0 | 0 | 25 | 4413 |
| `motor/match_pl.rs` | 178 | 5 | 0 | 0 | 0 | 35 | 10343 |
| `motor/media.rs` | 174 | 2 | 0 | 0 | 0 | 22 | 3564 |
| `motor/player_stats.rs` | 168 | 8 | 0 | 0 | 0 | 10 | 1534 |
| `motor/club_records.rs` | 157 | 6 | 0 | 0 | 0 | 4 | 1277 |
| `motor/match_day.rs` | 132 | 9 | 0 | 0 | 0 | 17 | 2030 |
| `motor/player_regen.rs` | 117 | 6 | 0 | 0 | 0 | 18 | 3477 |
| `motor/match_man.rs` | 114 | 1 | 0 | 0 | 0 | 25 | 4628 |
| `motor/database.rs` | 113 | 86 | 0 | 0 | 0 | 47 | 9456 |
| `motor/scout_manager.rs` | 108 | 18 | 0 | 0 | 0 | 13 | 1295 |
| `motor/officials_manager.rs` | 101 | 0 | 0 | 0 | 0 | 5 | 890 |
| `motor/comp/fix_man.rs` | 91 | 31 | 0 | 0 | 0 | 22 | 2730 |
| `motor/discipline.rs` | 88 | 2 | 0 | 0 | 0 | 15 | 1140 |
| `motor/comp/leagues/fin_prm.rs` | 85 | 24 | 0 | 0 | 0 | 9 | 3140 |
| `motor/tactics.rs` | 84 | 0 | 0 | 0 | 0 | 6 | 1467 |
| `motor/player_search.rs` | 76 | 0 | 0 | 0 | 0 | 5 | 935 |
| `motor/formation.rs` | 72 | 0 | 0 | 0 | 0 | 2 | 1115 |
| `motor/training_manager.rs` | 65 | 0 | 0 | 0 | 0 | 1 | 1346 |
| `motor/comp/worldcup/world_cup.rs` | 62 | 14 | 0 | 0 | 0 | 3 | 689 |
| `motor/area.rs` | 59 | 0 | 0 | 0 | 0 | 4 | 361 |
| `motor/comp/uefa_seeding.rs` | 59 | 7 | 0 | 0 | 0 | 6 | 1038 |
| `motor/comp/eurocomp/european_cup.rs` | 55 | 9 | 0 | 0 | 0 | 8 | 1570 |
| `motor/match_events.rs` | 53 | 0 | 0 | 0 | 0 | 7 | 1963 |
| `motor/compare/club_records.rs` | 53 | 0 | 0 | 0 | 0 | 0 | 467 |
| `motor/comp/stadium.rs` | 50 | 19 | 0 | 0 | 0 | 16 | 1386 |
| `motor/human_manager.rs` | 49 | 3 | 0 | 0 | 0 | 7 | 792 |
| `motor/news.rs` | 49 | 0 | 0 | 0 | 0 | 5 | 1084 |
| `motor/compare/comp/friendly.rs` | 48 | 0 | 0 | 0 | 0 | 14 | 743 |
