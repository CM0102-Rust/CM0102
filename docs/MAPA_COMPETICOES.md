# Mapa das competições

Gerado por `python engenharia-reversa/competicoes.py` (o mapa de funções de maxevans60, CC BY 4.0, e a cobertura de uma carreira com as 8 ligas `ENG ESP ITA FRA BRA JPN AUS USA` até 2002-06-01, `levas/competicoes/`; as rotinas já em Rust não levam contador). "falta" = chamada ao menos uma vez nessa carreira e ainda não está em Rust (a lista para `candidatos.py --competicoes` fica em `docs/cobertura_competicoes.txt`).

**Total:** 2695 funções, 1475.7 KB; em Rust 1374 (60.8% dos bytes); faltam 41 (46.1 KB).

| Grupo | Funções | KB | Rust | Falta (KB) | Progresso |
|---|---|---|---|---|---|
| Base: Liga, Copa, comp, calendário, sorteios | 396 | 272.6 | 271 | 9 (20.3) | 🟩🟩🟩🟩🟩🟩🟨⬜⬜⬜ **69.0%** |
| Ligas nacionais (formato de cada divisão) | 1048 | 647.8 | 539 | 1 (3.8) | 🟩🟩🟩🟩🟩🟩🟨⬜⬜⬜ **62.1%** |
| Copas nacionais | 526 | 228.3 | 286 | 11 (4.9) | 🟩🟩🟩🟩🟩🟩🟩🟨⬜⬜ **71.7%** |
| Competições europeias | 221 | 98.2 | 122 | 0 (0.0) | 🟩🟩🟩🟩🟩🟩🟨⬜⬜⬜ **67.3%** |
| Continentais fora da Europa | 203 | 105.2 | 58 | 9 (6.3) | 🟩🟩🟩🟨⬜⬜⬜⬜⬜⬜ **32.4%** |
| Copa do Mundo e eliminatórias | 158 | 92.4 | 47 | 11 (10.7) | 🟩🟩🟩🟨⬜⬜⬜⬜⬜⬜ **32.3%** |
| Torneios amistosos | 17 | 4.5 | 0 | 0 (0.0) | ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜ **0.0%** |
| Regras de transferência por nação (`transfer/*_rules.cpp`) | 126 | 26.7 | 51 | 0 (0.0) | 🟩🟩🟩🟩🟨⬜⬜⬜⬜⬜ **47.9%** |

## Por arquivo

Só os arquivos com alguma rotina que falta; ordem: grupo, depois o que falta.

| Arquivo | Grupo | Funções | Rust | Falta (KB) |
|---|---|---|---|---|
| `league.cpp` | (raiz) | 61 | 51 | 1 (5.7) |
| `Cup.cpp` | (raiz) | 47 | 37 | 1 (5.2) |
| `fix_man.cpp` | (raiz) | 37 | 31 | 3 (4.0) |
| `friendly.cpp` | (raiz) | 67 | 49 | 2 (3.8) |
| `fifa_rankings.cpp` | (raiz) | 20 | 9 | 1 (1.2) |
| `host_country.cpp` | (raiz) | 12 | 8 | 1 (0.4) |
| `comp.cpp` | (raiz) | 46 | 16 | 0 (0.0) |
| `comp_util.cpp` | (raiz) | 38 | 28 | 0 (0.0) |
| `conmebol_seeding.cpp` | (raiz) | 9 | 8 | 0 (0.0) |
| `squad_manager.cpp` | (raiz) | 16 | 10 | 0 (0.0) |
| `stadium.cpp` | (raiz) | 17 | 12 | 0 (0.0) |
| `uefa_seeding.cpp` | (raiz) | 26 | 12 | 0 (0.0) |
| `usa_mls.cpp` | leagues | 22 | 16 | 1 (3.8) |
| `arg_second.cpp` | leagues | 18 | 2 | 0 (0.0) |
| `aus_nsl.cpp` | leagues | 16 | 10 | 0 (0.0) |
| `bra_nat_first.cpp` | leagues | 16 | 11 | 0 (0.0) |
| `bra_nat_second.cpp` | leagues | 12 | 7 | 0 (0.0) |
| `bra_nat_third.cpp` | leagues | 16 | 12 | 0 (0.0) |
| `bra_reg_bahia.cpp` | leagues | 10 | 8 | 0 (0.0) |
| `bra_reg_central.cpp` | leagues | 11 | 9 | 0 (0.0) |
| `bra_reg_gaucho.cpp` | leagues | 13 | 9 | 0 (0.0) |
| `bra_reg_goias.cpp` | leagues | 11 | 8 | 0 (0.0) |
| `bra_reg_minas_gerais.cpp` | leagues | 14 | 12 | 0 (0.0) |
| `bra_reg_north.cpp` | leagues | 9 | 7 | 0 (0.0) |
| `bra_reg_northeast.cpp` | leagues | 9 | 7 | 0 (0.0) |
| `bra_reg_parana.cpp` | leagues | 13 | 11 | 0 (0.0) |
| `bra_reg_pern.cpp` | leagues | 10 | 8 | 0 (0.0) |
| `bra_reg_rio.cpp` | leagues | 14 | 11 | 0 (0.0) |
| `bra_reg_santa.cpp` | leagues | 10 | 8 | 0 (0.0) |
| `bra_reg_sp.cpp` | leagues | 10 | 8 | 0 (0.0) |
| `eng_first.cpp` | leagues | 10 | 8 | 0 (0.0) |
| `eng_prm.cpp` | leagues | 11 | 5 | 0 (0.0) |
| `eng_second.cpp` | leagues | 10 | 8 | 0 (0.0) |
| `eng_third.cpp` | leagues | 11 | 8 | 0 (0.0) |
| `fra_cfa.cpp` | leagues | 8 | 5 | 0 (0.0) |
| `fra_first.cpp` | leagues | 14 | 8 | 0 (0.0) |
| `fra_lower.cpp` | leagues | 5 | 4 | 0 (0.0) |
| `fra_second.cpp` | leagues | 7 | 5 | 0 (0.0) |
| `fra_third.cpp` | leagues | 7 | 5 | 0 (0.0) |
| `ita_ser_a.cpp` | leagues | 18 | 11 | 0 (0.0) |
| `ita_ser_b.cpp` | leagues | 14 | 8 | 0 (0.0) |
| `ita_ser_c1a.cpp` | leagues | 11 | 9 | 0 (0.0) |
| `ita_ser_c1b.cpp` | leagues | 12 | 10 | 0 (0.0) |
| `ita_ser_c2a.cpp` | leagues | 13 | 1 | 0 (0.0) |
| `jap_j1.cpp` | leagues | 15 | 9 | 0 (0.0) |
| `jap_j2.cpp` | leagues | 7 | 5 | 0 (0.0) |
| `league_stage.cpp` | leagues | 6 | 2 | 0 (0.0) |
| `spa_first.cpp` | leagues | 15 | 8 | 0 (0.0) |
| `spa_lower.cpp` | leagues | 7 | 5 | 0 (0.0) |
| `spa_second.cpp` | leagues | 7 | 5 | 0 (0.0) |
| `sub_league.cpp` | leagues | 4 | 1 | 0 (0.0) |
| `bra_champ_cup.cpp` | cups | 16 | 2 | 10 (3.6) |
| `usa_mls_all_stars.cpp` | cups | 8 | 5 | 1 (1.3) |
| `bra_cup.cpp` | cups | 8 | 6 | 0 (0.0) |
| `cup_stage.cpp` | cups | 7 | 3 | 0 (0.0) |
| `eng_auto_cup.cpp` | cups | 15 | 11 | 0 (0.0) |
| `eng_cc_cup.cpp` | cups | 8 | 5 | 0 (0.0) |
| `eng_charity.cpp` | cups | 6 | 4 | 0 (0.0) |
| `eng_fa_cup.cpp` | cups | 12 | 8 | 0 (0.0) |
| `fra_cup.cpp` | cups | 9 | 4 | 0 (0.0) |
| `fra_lge_cup.cpp` | cups | 7 | 4 | 0 (0.0) |
| `fra_super.cpp` | cups | 6 | 4 | 0 (0.0) |
| `gre_cup.cpp` | cups | 19 | 2 | 0 (0.0) |
| `ire_lge_cup.cpp` | cups | 17 | 12 | 0 (0.0) |
| `ita_c1_super.cpp` | cups | 6 | 4 | 0 (0.0) |
| `ita_c_cup.cpp` | cups | 17 | 12 | 0 (0.0) |
| `ita_cup.cpp` | cups | 17 | 12 | 0 (0.0) |
| `ita_super.cpp` | cups | 6 | 4 | 0 (0.0) |
| `jap_emp_cup.cpp` | cups | 7 | 4 | 0 (0.0) |
| `jap_j_cup.cpp` | cups | 6 | 4 | 0 (0.0) |
| `jap_super.cpp` | cups | 6 | 4 | 0 (0.0) |
| `spa_cup.cpp` | cups | 7 | 5 | 0 (0.0) |
| `spa_super.cpp` | cups | 7 | 4 | 0 (0.0) |
| `usa_open_cup.cpp` | cups | 6 | 4 | 0 (0.0) |
| `asia_club_champ.cpp` | eurocomp | 15 | 10 | 0 (0.0) |
| `asia_cup_winner.cpp` | eurocomp | 17 | 9 | 0 (0.0) |
| `asia_super_cup.cpp` | eurocomp | 7 | 4 | 0 (0.0) |
| `con_champ.cpp` | eurocomp | 8 | 6 | 0 (0.0) |
| `con_merc_cup.cpp` | eurocomp | 16 | 12 | 0 (0.0) |
| `conmebol_liber.cpp` | eurocomp | 16 | 11 | 0 (0.0) |
| `conmebol_merc.cpp` | eurocomp | 16 | 10 | 0 (0.0) |
| `eur_super_cup.cpp` | eurocomp | 6 | 4 | 0 (0.0) |
| `european_cup.cpp` | eurocomp | 50 | 19 | 0 (0.0) |
| `inter_amer_cup.cpp` | eurocomp | 6 | 4 | 0 (0.0) |
| `intertoto_cup.cpp` | eurocomp | 13 | 8 | 0 (0.0) |
| `oceania_club_champ.cpp` | eurocomp | 16 | 11 | 0 (0.0) |
| `uefa_cup.cpp` | eurocomp | 11 | 7 | 0 (0.0) |
| `world_club_champ.cpp` | eurocomp | 18 | 3 | 0 (0.0) |
| `world_club_cup.cpp` | eurocomp | 6 | 4 | 0 (0.0) |
| `euro_champ_qual.cpp` | intercomp | 32 | 5 | 8 (5.3) |
| `goldcup.cpp` | intercomp | 57 | 12 | 1 (1.0) |
| `african_nations.cpp` | intercomp | 20 | 12 | 0 (0.0) |
| `asia_nations.cpp` | intercomp | 17 | 5 | 0 (0.0) |
| `euro_champ.cpp` | intercomp | 25 | 5 | 0 (0.0) |
| `fifa_confed.cpp` | intercomp | 20 | 7 | 0 (0.0) |
| `oceania_nations.cpp` | intercomp | 16 | 8 | 0 (0.0) |
| `olympics.cpp` | intercomp | 16 | 4 | 0 (0.0) |
| `wc_concacaf_cup.cpp` | worldcup | 21 | 1 | 5 (5.8) |
| `wc_african_cup.cpp` | worldcup | 20 | 1 | 5 (4.0) |
| `wc_europe_league.cpp` | worldcup | 30 | 4 | 1 (0.9) |
| `wc_asia_league.cpp` | worldcup | 23 | 6 | 0 (0.0) |
| `wc_oceania_league.cpp` | worldcup | 21 | 12 | 0 (0.0) |
| `wc_south_american_league.cpp` | worldcup | 17 | 7 | 0 (0.0) |
| `world_cup.cpp` | worldcup | 26 | 16 | 0 (0.0) |
| `argentina_rules.cpp` | rules | 12 | 1 | 0 (0.0) |
| `australia_rules.cpp` | rules | 3 | 2 | 0 (0.0) |
| `belgium_rules.cpp` | rules | 10 | 4 | 0 (0.0) |
| `brazil_rules.cpp` | rules | 3 | 1 | 0 (0.0) |
| `croatia_rules.cpp` | rules | 2 | 1 | 0 (0.0) |
| `england_rules.cpp` | rules | 7 | 5 | 0 (0.0) |
| `finland_rules.cpp` | rules | 4 | 4 | 0 (0.0) |
| `france_rules.cpp` | rules | 10 | 2 | 0 (0.0) |
| `germany_rules.cpp` | rules | 7 | 2 | 0 (0.0) |
| `greece_rules.cpp` | rules | 4 | 1 | 0 (0.0) |
| `holland_rules.cpp` | rules | 1 | 1 | 0 (0.0) |
| `ireland_rules.cpp` | rules | 6 | 4 | 0 (0.0) |
| `italy_rules.cpp` | rules | 9 | 3 | 0 (0.0) |
| `korea_rules.cpp` | rules | 3 | 1 | 0 (0.0) |
| `northern_ireland_rules.cpp` | rules | 4 | 3 | 0 (0.0) |
| `norway_rules.cpp` | rules | 6 | 2 | 0 (0.0) |
| `poland_rules.cpp` | rules | 2 | 1 | 0 (0.0) |
| `portugal_rules.cpp` | rules | 5 | 1 | 0 (0.0) |
| `russia_rules.cpp` | rules | 4 | 2 | 0 (0.0) |
| `scotland_rules.cpp` | rules | 5 | 3 | 0 (0.0) |
| `spain_rules.cpp` | rules | 7 | 2 | 0 (0.0) |
| `turkey_rules.cpp` | rules | 3 | 2 | 0 (0.0) |
| `usa_rules.cpp` | rules | 2 | 1 | 0 (0.0) |
| `wales_rules.cpp` | rules | 2 | 2 | 0 (0.0) |

**Não rodam nesta carreira (12 arquivos):** competições de nações fora das 8 ligas, ou que só começam depois de 2002-06-01. Para elas é preciso outra carreira (outras ligas ou mais longa).

`eng_conf.cpp`, `eng_fa_trophy.cpp`, `ger_regional.cpp`, `ita_ser_c2b.cpp`, `ita_ser_c2c.cpp`, `japan_rules.cpp`, `mini_cup.cpp`, `mini_league.cpp`, `por_second_b.cpp`, `spa_second_b.cpp`, `swe_cup.cpp`, `swe_second.cpp`
