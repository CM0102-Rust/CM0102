*[English version](LEGIBILIDADE.en.md).*

# Placar de legibilidade

Gerado por `python engenharia-reversa/legibilidade.py`. O que ainda atrapalha quem lê o motor de fora (`rust/src/motor.rs` e `rust/src/motor/`); cada rodada da etapa 4 da reorganização deve baixar esses números.

## Total

| Medida | Quantidade |
|---|---:|
| deslocamentos crus | 3746 |
| chaves fixas cruas | 1120 |
| chamadas por endereço | 0 |
| cópias de acesso à memória | 0 |
| rotinas sem cabeçalho | 0 |
| locais/parâmetros em português | 222 |
| linhas | 386088 |

## Por arquivo (os 40 piores, pelos deslocamentos crus)

| Arquivo | deslocamentos crus | chaves fixas cruas | chamadas por endereço | cópias de acesso à memória | rotinas sem cabeçalho | locais/parâmetros em português | linhas |
|---|---:|---:|---:|---:|---:|---:|---:|
| `motor.rs` | 583 | 0 | 0 | 0 | 0 | 72 | 12080 |
| `motor/comp_screens.rs` | 339 | 11 | 0 | 0 | 0 | 0 | 9534 |
| `motor/club_screens.rs` | 321 | 2 | 0 | 0 | 0 | 0 | 13774 |
| `motor/news.rs` | 164 | 0 | 0 | 0 | 0 | 0 | 3647 |
| `motor/match_pl.rs` | 129 | 0 | 0 | 0 | 0 | 8 | 9641 |
| `motor/staff_screens.rs` | 123 | 2 | 0 | 0 | 0 | 0 | 7899 |
| `motor/match_man.rs` | 122 | 1 | 0 | 0 | 0 | 1 | 4267 |
| `motor/match_screens.rs` | 92 | 0 | 0 | 0 | 0 | 6 | 6374 |
| `motor/match_eng.rs` | 90 | 0 | 0 | 0 | 0 | 22 | 11071 |
| `motor/manager_screens.rs` | 65 | 0 | 0 | 0 | 0 | 0 | 1811 |
| `motor/search_screens.rs` | 61 | 1 | 0 | 0 | 0 | 0 | 4877 |
| `motor/national_teams.rs` | 54 | 1 | 0 | 0 | 0 | 2 | 3230 |
| `motor/fields.rs` | 51 | 0 | 0 | 0 | 0 | 0 | 3392 |
| `motor/transfer_manager.rs` | 50 | 0 | 0 | 0 | 0 | 3 | 12650 |
| `motor/club_records.rs` | 48 | 3 | 0 | 0 | 0 | 0 | 2761 |
| `motor/media.rs` | 48 | 0 | 0 | 0 | 0 | 0 | 10477 |
| `motor/award_screens.rs` | 41 | 0 | 0 | 0 | 0 | 0 | 1110 |
| `motor/menubar.rs` | 40 | 0 | 0 | 0 | 0 | 0 | 2904 |
| `motor/player_search.rs` | 40 | 0 | 0 | 0 | 0 | 0 | 1599 |
| `motor/comp/cup.rs` | 37 | 3 | 0 | 0 | 0 | 2 | 4261 |
| `motor/tactics.rs` | 36 | 0 | 0 | 0 | 0 | 4 | 1605 |
| `motor/player_stats.rs` | 35 | 0 | 0 | 0 | 0 | 1 | 1911 |
| `motor/manager_manager.rs` | 34 | 0 | 0 | 0 | 0 | 0 | 7289 |
| `motor/tactics_screens.rs` | 32 | 0 | 0 | 0 | 0 | 0 | 5668 |
| `motor/printouts.rs` | 31 | 0 | 0 | 0 | 0 | 0 | 2137 |
| `motor/transfer_screens.rs` | 31 | 0 | 0 | 0 | 0 | 0 | 5641 |
| `motor/display.rs` | 30 | 0 | 0 | 0 | 0 | 0 | 2096 |
| `motor/contract_manager.rs` | 29 | 3 | 0 | 0 | 0 | 4 | 6841 |
| `motor/training_screens.rs` | 29 | 0 | 0 | 0 | 0 | 0 | 1864 |
| `motor/comp/eurocomp/european_cup.rs` | 29 | 1 | 0 | 0 | 0 | 1 | 1625 |
| `motor/setup.rs` | 27 | 0 | 0 | 0 | 0 | 0 | 8258 |
| `motor/comp/intercomp/oceania_nations.rs` | 27 | 0 | 0 | 0 | 0 | 0 | 506 |
| `motor/comp/league.rs` | 26 | 0 | 0 | 0 | 0 | 5 | 4345 |
| `motor/database.rs` | 25 | 4 | 0 | 0 | 0 | 8 | 8530 |
| `motor/file_screens.rs` | 23 | 0 | 0 | 0 | 0 | 0 | 1142 |
| `motor/history.rs` | 23 | 0 | 0 | 0 | 0 | 0 | 1085 |
| `motor/comp/uefa_seeding.rs` | 22 | 0 | 0 | 0 | 0 | 0 | 1033 |
| `motor/contract_screens.rs` | 21 | 0 | 0 | 0 | 0 | 0 | 5129 |
| `motor/comp/stadium.rs` | 21 | 0 | 0 | 0 | 0 | 2 | 1322 |
| `motor/comp/eurocomp/asia_club_champ.rs` | 21 | 0 | 0 | 0 | 0 | 2 | 1005 |
