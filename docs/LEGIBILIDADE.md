*[English version](LEGIBILIDADE.en.md).*

# Placar de legibilidade

Gerado por `python engenharia-reversa/legibilidade.py`. O que ainda atrapalha quem lê o motor de fora (`rust/src/motor.rs` e `rust/src/motor/`); cada rodada da etapa 4 da reorganização deve baixar esses números.

## Total

| Medida | Quantidade |
|---|---:|
| deslocamentos crus | 2790 |
| chaves fixas cruas | 1821 |
| chamadas por endereço | 0 |
| cópias de acesso à memória | 0 |
| rotinas sem cabeçalho | 0 |
| locais/parâmetros em português | 241 |
| linhas | 232561 |

## Por arquivo (os 40 piores, pelos deslocamentos crus)

| Arquivo | deslocamentos crus | chaves fixas cruas | chamadas por endereço | cópias de acesso à memória | rotinas sem cabeçalho | locais/parâmetros em português | linhas |
|---|---:|---:|---:|---:|---:|---:|---:|
| `motor.rs` | 587 | 0 | 0 | 0 | 0 | 104 | 6427 |
| `motor/match_screens.rs` | 128 | 4 | 0 | 0 | 0 | 6 | 5878 |
| `motor/match_man.rs` | 124 | 1 | 0 | 0 | 0 | 1 | 4498 |
| `motor/match_eng.rs` | 95 | 7 | 0 | 0 | 0 | 13 | 11686 |
| `motor/national_teams.rs` | 63 | 33 | 0 | 0 | 0 | 2 | 3510 |
| `motor/match_pl.rs` | 58 | 5 | 0 | 0 | 0 | 7 | 10183 |
| `motor/fields.rs` | 39 | 0 | 0 | 0 | 0 | 0 | 2999 |
| `motor/db_files.rs` | 37 | 0 | 0 | 0 | 0 | 0 | 1282 |
| `motor/comp/cups/eng_cc_cup.rs` | 34 | 12 | 0 | 0 | 0 | 1 | 620 |
| `motor/comp/cups/ita_cup.rs` | 34 | 19 | 0 | 0 | 0 | 1 | 1155 |
| `motor/zipdir.rs` | 33 | 0 | 0 | 0 | 0 | 0 | 604 |
| `motor/match_events.rs` | 32 | 0 | 0 | 0 | 0 | 1 | 1892 |
| `motor/comp/fifa_rankings.rs` | 32 | 0 | 0 | 0 | 0 | 0 | 382 |
| `motor/manager_manager.rs` | 31 | 2 | 0 | 0 | 0 | 0 | 5143 |
| `motor/record_utils.rs` | 31 | 0 | 0 | 0 | 0 | 0 | 248 |
| `motor/scout_manager.rs` | 31 | 18 | 0 | 0 | 0 | 0 | 1225 |
| `motor/tactics.rs` | 31 | 0 | 0 | 0 | 0 | 2 | 1444 |
| `motor/match_official.rs` | 30 | 0 | 0 | 0 | 0 | 0 | 695 |
| `motor/comp/cup.rs` | 30 | 8 | 0 | 0 | 0 | 3 | 3519 |
| `motor/comp/intercomp/african_nations.rs` | 29 | 4 | 0 | 0 | 0 | 0 | 482 |
| `motor/staff_records.rs` | 27 | 2 | 0 | 0 | 0 | 0 | 2379 |
| `motor/transfer_manager.rs` | 27 | 29 | 0 | 0 | 0 | 3 | 10492 |
| `motor/setup.rs` | 26 | 0 | 0 | 0 | 0 | 0 | 1613 |
| `motor/comp/intercomp/goldcup.rs` | 26 | 14 | 0 | 0 | 0 | 2 | 890 |
| `motor/comp/leagues/ita_ser_a.rs` | 24 | 3 | 0 | 0 | 0 | 6 | 1746 |
| `motor/injury.rs` | 23 | 0 | 0 | 0 | 0 | 4 | 3041 |
| `motor/training_manager.rs` | 23 | 0 | 0 | 0 | 0 | 1 | 1319 |
| `motor/comp/comp_util.rs` | 23 | 3 | 0 | 0 | 0 | 0 | 926 |
| `motor/comp/friendly.rs` | 23 | 26 | 0 | 0 | 0 | 1 | 3308 |
| `motor/comp/eurocomp/european_cup.rs` | 23 | 9 | 0 | 0 | 0 | 1 | 1524 |
| `motor/comp/eurocomp/oceania_club_champ.rs` | 23 | 12 | 0 | 0 | 0 | 4 | 776 |
| `motor/compare/match_eng.rs` | 23 | 0 | 0 | 0 | 0 | 1 | 1067 |
| `motor/comp/host_country.rs` | 22 | 27 | 0 | 0 | 0 | 0 | 484 |
| `motor/comp/eurocomp/conmebol_liber.rs` | 22 | 4 | 0 | 0 | 0 | 1 | 503 |
| `motor/database.rs` | 21 | 86 | 0 | 0 | 0 | 8 | 9344 |
| `motor/human_manager.rs` | 21 | 3 | 0 | 0 | 0 | 0 | 773 |
| `motor/comp/league.rs` | 21 | 20 | 0 | 0 | 0 | 6 | 4132 |
| `motor/compare/club_records.rs` | 21 | 0 | 0 | 0 | 0 | 0 | 460 |
| `motor/game.rs` | 20 | 0 | 0 | 0 | 0 | 0 | 727 |
| `motor/comp/eurocomp/con_merc_cup.rs` | 19 | 14 | 0 | 0 | 0 | 0 | 567 |
