*[English version](LEGIBILIDADE.en.md).*

# Placar de legibilidade

Gerado por `python engenharia-reversa/legibilidade.py`. O que ainda atrapalha quem lê o motor de fora (`rust/src/motor.rs` e `rust/src/motor/`); cada rodada da etapa 4 da reorganização deve baixar esses números.

## Total

| Medida | Quantidade |
|---|---:|
| deslocamentos crus | 5251 |
| chaves fixas cruas | 1821 |
| chamadas por endereço | 0 |
| cópias de acesso à memória | 0 |
| rotinas sem cabeçalho | 0 |
| locais/parâmetros em português | 855 |
| linhas | 225494 |

## Por arquivo (os 40 piores, pelos deslocamentos crus)

| Arquivo | deslocamentos crus | chaves fixas cruas | chamadas por endereço | cópias de acesso à memória | rotinas sem cabeçalho | locais/parâmetros em português | linhas |
|---|---:|---:|---:|---:|---:|---:|---:|
| `motor.rs` | 621 | 0 | 0 | 0 | 0 | 104 | 6395 |
| `motor/match_pl.rs` | 287 | 5 | 0 | 0 | 0 | 24 | 10179 |
| `motor/match_eng.rs` | 244 | 7 | 0 | 0 | 0 | 75 | 11650 |
| `motor/match_screens.rs` | 240 | 4 | 0 | 0 | 0 | 43 | 5875 |
| `motor/manager_manager.rs` | 226 | 2 | 0 | 0 | 0 | 12 | 5142 |
| `motor/national_teams.rs` | 143 | 33 | 0 | 0 | 0 | 15 | 3512 |
| `motor/transfer_manager.rs` | 143 | 29 | 0 | 0 | 0 | 40 | 10395 |
| `motor/injury.rs` | 132 | 0 | 0 | 0 | 0 | 15 | 3045 |
| `motor/match_man.rs` | 115 | 1 | 0 | 0 | 0 | 10 | 4483 |
| `motor/media.rs` | 113 | 2 | 0 | 0 | 0 | 6 | 3444 |
| `motor/contract_manager.rs` | 109 | 16 | 0 | 0 | 0 | 16 | 4246 |
| `motor/database.rs` | 103 | 86 | 0 | 0 | 0 | 30 | 9353 |
| `motor/shortlist_manager.rs` | 103 | 31 | 0 | 0 | 0 | 11 | 4244 |
| `motor/comp/cup.rs` | 96 | 8 | 0 | 0 | 0 | 9 | 3505 |
| `motor/comp/fix_man.rs` | 88 | 31 | 0 | 0 | 0 | 12 | 2591 |
| `motor/formation.rs` | 85 | 0 | 0 | 0 | 0 | 2 | 1089 |
| `motor/comp/league.rs` | 84 | 20 | 0 | 0 | 0 | 22 | 4134 |
| `motor/comp/leagues/fin_prm.rs` | 75 | 24 | 0 | 0 | 0 | 10 | 3104 |
| `motor/player_search.rs` | 73 | 0 | 0 | 0 | 0 | 2 | 901 |
| `motor/training_manager.rs` | 65 | 0 | 0 | 0 | 0 | 1 | 1321 |
| `motor/player_regen.rs` | 63 | 6 | 0 | 0 | 0 | 6 | 3367 |
| `motor/area.rs` | 59 | 0 | 0 | 0 | 0 | 4 | 353 |
| `motor/comp/uefa_seeding.rs` | 59 | 7 | 0 | 0 | 0 | 2 | 997 |
| `motor/match_day.rs` | 58 | 9 | 0 | 0 | 0 | 10 | 1901 |
| `motor/human_manager.rs` | 56 | 3 | 0 | 0 | 0 | 6 | 772 |
| `motor/scout_manager.rs` | 56 | 18 | 0 | 0 | 0 | 4 | 1224 |
| `motor/comp/eurocomp/european_cup.rs` | 55 | 9 | 0 | 0 | 0 | 2 | 1514 |
| `motor/comp/friendly.rs` | 52 | 26 | 0 | 0 | 0 | 20 | 3296 |
| `motor/shared.rs` | 50 | 2 | 0 | 0 | 0 | 3 | 591 |
| `motor/comp/stadium.rs` | 50 | 19 | 0 | 0 | 0 | 15 | 1368 |
| `motor/tactics.rs` | 49 | 0 | 0 | 0 | 0 | 3 | 1450 |
| `motor/player_stats.rs` | 46 | 8 | 0 | 0 | 0 | 8 | 1502 |
| `motor/comp/worldcup/world_cup.rs` | 42 | 14 | 0 | 0 | 0 | 0 | 665 |
| `motor/comp/comp.rs` | 40 | 9 | 0 | 0 | 0 | 4 | 756 |
| `motor/awol.rs` | 37 | 0 | 0 | 0 | 0 | 0 | 896 |
| `motor/db_files.rs` | 37 | 0 | 0 | 0 | 0 | 1 | 1346 |
| `motor/staff_records.rs` | 36 | 2 | 0 | 0 | 0 | 15 | 2380 |
| `motor/comp/cups/eng_cc_cup.rs` | 34 | 12 | 0 | 0 | 0 | 1 | 620 |
| `motor/comp/cups/ita_cup.rs` | 34 | 19 | 0 | 0 | 0 | 2 | 1155 |
| `motor/match_events.rs` | 33 | 0 | 0 | 0 | 0 | 4 | 1891 |
