# Placar por arquivo (exe inteiro)

Gerado por `python engenharia-reversa/placar_arquivos.py` com as fronteiras e os arquivos-fonte do mapa de [maxevans60](https://github.com/maxevans60/cm0102-function-map) (CC BY 4.0). Uma função conta quando o início dela é desviado para Rust (`Prologo` em `rust/src/motor.rs`). O placar do motor de partidas, pelas nossas fronteiras, fica em `PLACAR.md`.

**Exe inteiro:** 391 de 6709 funções, 408.9 de 5528.0 KB (**7.40%**) · 30 de 393 arquivos com rotinas em Rust

| Arquivo | Funções | KB | Progresso |
|---|---|---|---|
| `match_pl.cpp` | 94/96 | 179.5/179.8 | 🟩🟩🟩🟩🟩🟩🟩🟩🟩🟨 **99.9%** |
| `match_official.cpp` | 7/8 | 6.3/6.3 | 🟩🟩🟩🟩🟩🟩🟩🟩🟩🟨 **99.8%** |
| `match_eng.cpp` | 123/129 | 115.5/119.2 | 🟩🟩🟩🟩🟩🟩🟩🟩🟩🟨 **96.8%** |
| `match_day.cpp` | 18/20 | 15.6/16.6 | 🟩🟩🟩🟩🟩🟩🟩🟩🟩🟨 **94.4%** |
| `match_events.cpp` | 8/15 | 16.3/22.4 | 🟩🟩🟩🟩🟩🟩🟩🟨⬜⬜ **72.8%** |
| `match_man.cpp` | 17/21 | 30.5/55.8 | 🟩🟩🟩🟩🟩🟨⬜⬜⬜⬜ **54.7%** |
| `formation.cpp` | 18/45 | 8.3/26.5 | 🟩🟩🟩🟨⬜⬜⬜⬜⬜⬜ **31.3%** |
| `zipdir.cpp` | 3/10 | 1.3/4.8 | 🟩🟩🟨⬜⬜⬜⬜⬜⬜⬜ **28.1%** |
| `tactics.cpp` | 13/29 | 4.5/23.1 | 🟩🟨⬜⬜⬜⬜⬜⬜⬜⬜ **19.4%** |
| `human_manager.cpp` | 7/61 | 2.2/25.5 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **8.5%** |
| `main.cpp` | 2/16 | 0.2/3.0 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **8.2%** |
| `player_stats.cpp` | 3/34 | 1.7/28.6 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **6.0%** |
| `fog_of_war.cpp` | 4/34 | 0.6/12.4 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **4.9%** |
| `media.cpp` | 4/69 | 9.8/202.1 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **4.8%** |
| `Database.cpp` | 19/181 | 7.0/148.5 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **4.7%** |
| `staff_contracts.cpp` | 6/45 | 1.8/39.9 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **4.6%** |
| `league.cpp` | 3/61 | 1.2/35.8 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **3.5%** |
| `european_cup.cpp` | 2/50 | 0.8/29.3 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **2.8%** |
| `comp.cpp` | 1/46 | 0.3/11.8 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **2.8%** |
| `injury.cpp` | 5/47 | 0.7/30.5 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **2.3%** |
| `national_teams.cpp` | 15/69 | 1.2/58.6 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **2.0%** |
| `shortlist_manager.cpp` | 3/54 | 1.2/71.7 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **1.7%** |
| `officials_manager.cpp` | 1/34 | 0.2/12.0 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **1.7%** |
| `fix_man.cpp` | 1/37 | 0.4/22.4 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **1.6%** |
| `printouts.cpp` | 1/20 | 0.4/27.2 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **1.5%** |
| `comp_util.cpp` | 8/38 | 0.5/68.4 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **0.7%** |
| `record_utils.cpp` | 1/32 | 0.1/12.0 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **0.7%** |
| `club_records.cpp` | 1/56 | 0.3/61.6 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **0.5%** |
| `contract_manager.cpp` | 2/121 | 0.3/87.1 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **0.3%** |
| `friendly.cpp` | 1/67 | 0.0/41.7 | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **0.1%** |

Os outros 363 arquivos ainda não têm rotina em Rust.
