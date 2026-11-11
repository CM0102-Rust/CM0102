*[English version](LEGIBILIDADE.en.md).*

# Placar de legibilidade

Gerado por `python engenharia-reversa/legibilidade.py`. O que ainda atrapalha quem lê o motor de fora (`rust/src/motor.rs` e `rust/src/motor/`); cada rodada da etapa 4 da reorganização deve baixar esses números.

## Total

| Medida | Quantidade |
|---|---:|
| deslocamentos crus | 3346 |
| chaves fixas cruas | 1821 |
| chamadas por endereço | 0 |
| cópias de acesso à memória | 0 |
| rotinas sem cabeçalho | 0 |
| locais/parâmetros em português | 241 |
| linhas | 231136 |

## Por arquivo (os 40 piores, pelos deslocamentos crus)

| Arquivo | deslocamentos crus | chaves fixas cruas | chamadas por endereço | cópias de acesso à memória | rotinas sem cabeçalho | locais/parâmetros em português | linhas |
|---|---:|---:|---:|---:|---:|---:|---:|
| `motor.rs` | 587 | 0 | 0 | 0 | 0 | 104 | 6427 |
| `motor/match_screens.rs` | 128 | 4 | 0 | 0 | 0 | 6 | 5878 |
| `motor/match_man.rs` | 124 | 1 | 0 | 0 | 0 | 1 | 4498 |
| `motor/match_eng.rs` | 95 | 7 | 0 | 0 | 0 | 13 | 11686 |
| `motor/national_teams.rs` | 63 | 33 | 0 | 0 | 0 | 2 | 3510 |
| `motor/training_manager.rs` | 63 | 0 | 0 | 0 | 0 | 1 | 1319 |
| `motor/comp/uefa_seeding.rs` | 59 | 7 | 0 | 0 | 0 | 0 | 997 |
| `motor/match_pl.rs` | 58 | 5 | 0 | 0 | 0 | 7 | 10183 |
| `motor/match_day.rs` | 56 | 9 | 0 | 0 | 0 | 1 | 1899 |
| `motor/scout_manager.rs` | 55 | 18 | 0 | 0 | 0 | 0 | 1225 |
| `motor/human_manager.rs` | 54 | 3 | 0 | 0 | 0 | 0 | 772 |
| `motor/comp/eurocomp/european_cup.rs` | 54 | 9 | 0 | 0 | 0 | 1 | 1524 |
| `motor/player_regen.rs` | 47 | 6 | 0 | 0 | 0 | 2 | 3365 |
| `motor/comp/friendly.rs` | 47 | 26 | 0 | 0 | 0 | 1 | 3308 |
| `motor/comp/leagues/fin_prm.rs` | 45 | 24 | 0 | 0 | 0 | 1 | 3103 |
| `motor/player_stats.rs` | 44 | 8 | 0 | 0 | 0 | 1 | 1501 |
| `motor/shared.rs` | 43 | 2 | 0 | 0 | 0 | 1 | 591 |
| `motor/comp/stadium.rs` | 42 | 19 | 0 | 0 | 0 | 2 | 1362 |
| `motor/comp/worldcup/world_cup.rs` | 42 | 14 | 0 | 0 | 0 | 0 | 663 |
| `motor/officials_manager.rs` | 40 | 0 | 0 | 0 | 0 | 2 | 872 |
| `motor/comp/comp.rs` | 39 | 9 | 0 | 0 | 0 | 0 | 755 |
| `motor/db_files.rs` | 37 | 0 | 0 | 0 | 0 | 0 | 1282 |
| `motor/fields.rs` | 35 | 0 | 0 | 0 | 0 | 0 | 2838 |
| `motor/comp/cups/eng_cc_cup.rs` | 34 | 12 | 0 | 0 | 0 | 1 | 620 |
| `motor/comp/cups/ita_cup.rs` | 34 | 19 | 0 | 0 | 0 | 1 | 1155 |
| `motor/zipdir.rs` | 33 | 0 | 0 | 0 | 0 | 0 | 604 |
| `motor/awol.rs` | 32 | 0 | 0 | 0 | 0 | 0 | 896 |
| `motor/match_events.rs` | 32 | 0 | 0 | 0 | 0 | 1 | 1892 |
| `motor/comp/fifa_rankings.rs` | 32 | 0 | 0 | 0 | 0 | 0 | 382 |
| `motor/manager_manager.rs` | 31 | 2 | 0 | 0 | 0 | 0 | 5143 |
| `motor/record_utils.rs` | 31 | 0 | 0 | 0 | 0 | 0 | 248 |
| `motor/tactics.rs` | 31 | 0 | 0 | 0 | 0 | 2 | 1444 |
| `motor/match_official.rs` | 30 | 0 | 0 | 0 | 0 | 0 | 695 |
| `motor/comp/cup.rs` | 30 | 8 | 0 | 0 | 0 | 3 | 3519 |
| `motor/comp/intercomp/african_nations.rs` | 29 | 4 | 0 | 0 | 0 | 0 | 482 |
| `motor/staff_records.rs` | 27 | 2 | 0 | 0 | 0 | 0 | 2379 |
| `motor/transfer_manager.rs` | 27 | 29 | 0 | 0 | 0 | 3 | 10492 |
| `motor/setup.rs` | 26 | 0 | 0 | 0 | 0 | 0 | 1613 |
| `motor/comp/intercomp/goldcup.rs` | 26 | 14 | 0 | 0 | 0 | 2 | 890 |
| `motor/comp/leagues/ita_ser_a.rs` | 24 | 3 | 0 | 0 | 0 | 6 | 1746 |
