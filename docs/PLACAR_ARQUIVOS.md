# Placar por arquivo (exe inteiro)

Gerado por `python engenharia-reversa/placar_arquivos.py` com as fronteiras e os arquivos-fonte do mapa de [maxevans60](https://github.com/maxevans60/cm0102-function-map) (CC BY 4.0). Uma função conta quando o início dela é desviado para Rust (`Prologo` em `rust/src/motor.rs`). O placar do motor de partidas, pelas nossas fronteiras, fica em `PLACAR.md`.

**Exe inteiro:** 433 de 6709 funções, 448.1 de 5528.0 KB (**8.11%**) · 37 de 393 arquivos com rotinas em Rust

| Arquivo | Funções | KB | Progresso |
|---|---|---|---|
| `match_eng.cpp` | 129/129 | 119.2/119.2 | 🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩 **100.0%** |
| `match_events.cpp` | 15/15 | 22.4/22.4 | 🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩 **100.0%** |
| `match_man.cpp` | 21/21 | 55.8/55.8 | 🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩 **100.0%** |
| `match_official.cpp` | 8/8 | 6.3/6.3 | 🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩 **100.0%** |
| `match_pl.cpp` | 96/96 | 179.8/179.8 | 🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩 **100.0%** |
| `match_day.cpp` | 19/20 | 15.7/16.6 | 🟩🟩🟩🟩🟩🟩🟩🟩🟩🟨 **95.0%** |
| `formation.cpp` | 18/45 | 8.3/26.5 | 🟩🟩🟩🟨⬜⬜⬜⬜⬜⬜ **31.3%** |
| `zipdir.cpp` | 3/10 | 1.3/4.8 | 🟩🟩🟨⬜⬜⬜⬜⬜⬜⬜ **28.1%** |
| `tactics.cpp` | 14/29 | 4.7/23.1 | 🟩🟩🟨⬜⬜⬜⬜⬜⬜⬜ **20.4%** |
| `human_manager.cpp` | 8/61 | 2.7/25.5 | 🟩🟨⬜⬜⬜⬜⬜⬜⬜⬜ **10.5%** |
| `main.cpp` | 2/16 | 0.2/3.0 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **8.2%** |
| `player_stats.cpp` | 4/34 | 2.0/28.6 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **6.9%** |
| `fog_of_war.cpp` | 4/34 | 0.6/12.4 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **4.9%** |
| `media.cpp` | 4/69 | 9.8/202.1 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **4.8%** |
| `Database.cpp` | 19/181 | 7.0/148.5 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **4.7%** |
| `staff_contracts.cpp` | 6/45 | 1.8/39.9 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **4.6%** |
| `injury.cpp` | 7/47 | 1.1/30.5 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **3.7%** |
| `league.cpp` | 5/61 | 1.3/35.8 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **3.6%** |
| `european_cup.cpp` | 2/50 | 0.8/29.3 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **2.8%** |
| `comp.cpp` | 1/46 | 0.3/11.8 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **2.8%** |
| `national_teams.cpp` | 16/69 | 1.3/58.6 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **2.2%** |
| `shortlist_manager.cpp` | 3/54 | 1.2/71.7 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **1.7%** |
| `officials_manager.cpp` | 1/34 | 0.2/12.0 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **1.7%** |
| `fix_man.cpp` | 1/37 | 0.4/22.4 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **1.6%** |
| `printouts.cpp` | 1/20 | 0.4/27.2 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **1.5%** |
| `scout_manager.cpp` | 1/39 | 0.3/23.0 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **1.5%** |
| `contract_manager.cpp` | 7/121 | 1.2/87.1 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **1.3%** |
| `comp_util.cpp` | 8/38 | 0.5/68.4 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **0.7%** |
| `record_utils.cpp` | 1/32 | 0.1/12.0 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **0.7%** |
| `club_records.cpp` | 1/56 | 0.3/61.6 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **0.5%** |
| `news.cpp` | 1/33 | 0.2/49.0 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **0.3%** |
| `game.cpp` | 1/34 | 0.1/43.1 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **0.2%** |
| `manager_manager.cpp` | 1/98 | 0.2/129.4 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **0.2%** |
| `discipline.cpp` | 1/38 | 0.1/38.6 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **0.2%** |
| `friendly.cpp` | 1/67 | 0.0/41.7 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **0.1%** |
| `transfer_manager.cpp` | 2/199 | 0.1/167.1 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **0.1%** |
| `player_regen.cpp` | 1/51 | 0.0/44.7 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **0.1%** |

Os outros 356 arquivos ainda não têm rotina em Rust.
