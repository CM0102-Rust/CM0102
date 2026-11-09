*[English version](LEGIBILIDADE.en.md).*

# Placar de legibilidade

Gerado por `python engenharia-reversa/legibilidade.py`. O que ainda atrapalha quem lê o motor de fora (`rust/src/motor.rs` e `rust/src/motor/`); cada rodada da etapa 4 da reorganização deve baixar esses números.

## Total

| Medida | Quantidade |
|---|---:|
| deslocamentos crus | 5273 |
| chaves fixas cruas | 1821 |
| chamadas por endereço | 0 |
| cópias de acesso à memória | 0 |
| rotinas sem cabeçalho | 0 |
| locais/parâmetros em português | 838 |
| linhas | 205440 |

## Por arquivo (os 40 piores, pelos deslocamentos crus)

| Arquivo | deslocamentos crus | chaves fixas cruas | chamadas por endereço | cópias de acesso à memória | rotinas sem cabeçalho | locais/parâmetros em português | linhas |
|---|---:|---:|---:|---:|---:|---:|---:|
| `motor.rs` | 621 | 0 | 0 | 0 | 0 | 105 | 6388 |
| `motor/match_screens.rs` | 240 | 4 | 0 | 0 | 0 | 43 | 5872 |
| `motor/match_eng.rs` | 237 | 7 | 0 | 0 | 0 | 74 | 11634 |
| `motor/manager_manager.rs` | 231 | 2 | 0 | 0 | 0 | 10 | 5113 |
| `motor/match_pl.rs` | 224 | 5 | 0 | 0 | 0 | 25 | 10155 |
| `motor/transfer_manager.rs` | 144 | 29 | 0 | 0 | 0 | 37 | 10363 |
| `motor/national_teams.rs` | 143 | 33 | 0 | 0 | 0 | 15 | 3485 |
| `motor/injury.rs` | 133 | 0 | 0 | 0 | 0 | 15 | 3036 |
| `motor/match_man.rs` | 118 | 1 | 0 | 0 | 0 | 10 | 4466 |
| `motor/media.rs` | 118 | 2 | 0 | 0 | 0 | 6 | 3411 |
| `motor/contract_manager.rs` | 113 | 16 | 0 | 0 | 0 | 16 | 4211 |
| `motor/database.rs` | 110 | 86 | 0 | 0 | 0 | 30 | 9295 |
| `motor/shortlist_manager.rs` | 107 | 31 | 0 | 0 | 0 | 11 | 4218 |
| `motor/comp/cup.rs` | 96 | 8 | 0 | 0 | 0 | 8 | 3498 |
| `motor/comp/fix_man.rs` | 88 | 31 | 0 | 0 | 0 | 12 | 2582 |
| `motor/comp/league.rs` | 88 | 20 | 0 | 0 | 0 | 20 | 4087 |
| `motor/formation.rs` | 85 | 0 | 0 | 0 | 0 | 2 | 1089 |
| `motor/comp/leagues/fin_prm.rs` | 85 | 24 | 0 | 0 | 0 | 9 | 3075 |
| `motor/player_search.rs` | 73 | 0 | 0 | 0 | 0 | 2 | 898 |
| `motor/player_regen.rs` | 67 | 6 | 0 | 0 | 0 | 6 | 3350 |
| `motor/training_manager.rs` | 65 | 0 | 0 | 0 | 0 | 1 | 1312 |
| `motor/comp/worldcup/world_cup.rs` | 62 | 14 | 0 | 0 | 0 | 0 | 659 |
| `motor/area.rs` | 59 | 0 | 0 | 0 | 0 | 4 | 353 |
| `motor/comp/uefa_seeding.rs` | 59 | 7 | 0 | 0 | 0 | 2 | 997 |
| `motor/match_day.rs` | 58 | 9 | 0 | 0 | 0 | 10 | 1899 |
| `motor/human_manager.rs` | 56 | 3 | 0 | 0 | 0 | 4 | 769 |
| `motor/scout_manager.rs` | 56 | 18 | 0 | 0 | 0 | 3 | 1220 |
| `motor/comp/eurocomp/european_cup.rs` | 55 | 9 | 0 | 0 | 0 | 2 | 1513 |
| `motor/comp/friendly.rs` | 52 | 26 | 0 | 0 | 0 | 20 | 3288 |
| `motor/comp/stadium.rs` | 50 | 19 | 0 | 0 | 0 | 15 | 1360 |
| `motor/player_stats.rs` | 47 | 8 | 0 | 0 | 0 | 8 | 1493 |
| `motor/shared.rs` | 44 | 2 | 0 | 0 | 0 | 3 | 593 |
| `motor/comp/comp.rs` | 40 | 9 | 0 | 0 | 0 | 2 | 742 |
| `motor/staff_records.rs` | 38 | 2 | 0 | 0 | 0 | 15 | 2367 |
| `motor/awol.rs` | 37 | 0 | 0 | 0 | 0 | 0 | 893 |
| `motor/db_files.rs` | 37 | 0 | 0 | 0 | 0 | 1 | 1346 |
| `motor/comp/cups/ita_cup.rs` | 37 | 19 | 0 | 0 | 0 | 2 | 1154 |
| `motor/tactics.rs` | 34 | 0 | 0 | 0 | 0 | 3 | 1447 |
| `motor/comp/cups/eng_cc_cup.rs` | 34 | 12 | 0 | 0 | 0 | 1 | 620 |
| `motor/match_events.rs` | 33 | 0 | 0 | 0 | 0 | 4 | 1891 |
