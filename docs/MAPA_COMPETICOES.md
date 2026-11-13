# Mapa das competições

Gerado por `python engenharia-reversa/competicoes.py` (o mapa de funções de maxevans60, CC BY 4.0, e a cobertura de uma carreira com as 8 ligas `ENG ESP ITA FRA BRA JPN AUS USA` até 2002-06-01, `levas/competicoes/`; as rotinas já em Rust não levam contador). "falta" = chamada ao menos uma vez nessa carreira e ainda não está em Rust (a lista para `candidatos.py --competicoes` fica em `docs/cobertura_competicoes.txt`).

**Total:** 2695 funções, 1475.7 KB; em Rust 875 (37.3% dos bytes); faltam 93 (136.3 KB).

| Grupo | Funções | KB | Rust | Falta (KB) | Progresso |
|---|---|---|---|---|---|
| Base: Liga, Copa, comp, calendário, sorteios | 396 | 272.6 | 260 | 19 (73.9) | 🟩🟩🟩🟩🟨⬜⬜⬜⬜⬜ **49.3%** |
| Ligas nacionais (formato de cada divisão) | 1048 | 647.8 | 303 | 3 (7.2) | 🟩🟩🟩🟨⬜⬜⬜⬜⬜⬜ **38.8%** |
| Copas nacionais | 526 | 228.3 | 97 | 12 (7.6) | 🟩🟩🟨⬜⬜⬜⬜⬜⬜⬜ **27.5%** |
| Competições europeias | 221 | 98.2 | 114 | 0 (0.0) | 🟩🟩🟩🟩🟩🟩🟨⬜⬜⬜ **64.3%** |
| Continentais fora da Europa | 203 | 105.2 | 33 | 31 (22.9) | 🟩🟨⬜⬜⬜⬜⬜⬜⬜⬜ **15.2%** |
| Copa do Mundo e eliminatórias | 158 | 92.4 | 20 | 28 (24.6) | 🟩🟨⬜⬜⬜⬜⬜⬜⬜⬜ **12.7%** |
| Torneios amistosos | 17 | 4.5 | 0 | 0 (0.0) | ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜ **0.0%** |
| Regras de transferência por nação (`transfer/*_rules.cpp`) | 126 | 26.7 | 48 | 0 (0.0) | 🟩🟩🟩🟩🟨⬜⬜⬜⬜⬜ **42.2%** |

## Por arquivo

Só os arquivos com alguma rotina que falta; ordem: grupo, depois o que falta.

| Arquivo | Grupo | Funções | Rust | Falta (KB) |
|---|---|---|---|---|
| `comp_util.cpp` | (raiz) | 38 | 23 | 5 (40.7) |
| `Cup.cpp` | (raiz) | 47 | 36 | 2 (12.9) |
| `friendly.cpp` | (raiz) | 67 | 48 | 3 (6.7) |
| `league.cpp` | (raiz) | 61 | 51 | 1 (5.7) |
| `fix_man.cpp` | (raiz) | 37 | 31 | 3 (4.0) |
| `fifa_rankings.cpp` | (raiz) | 20 | 9 | 1 (1.2) |
| `stadium.cpp` | (raiz) | 17 | 11 | 1 (1.1) |
| `uefa_seeding.cpp` | (raiz) | 26 | 11 | 1 (0.7) |
| `conmebol_seeding.cpp` | (raiz) | 9 | 7 | 1 (0.5) |
| `host_country.cpp` | (raiz) | 12 | 7 | 1 (0.4) |
| `comp.cpp` | (raiz) | 46 | 16 | 0 (0.0) |
| `squad_manager.cpp` | (raiz) | 16 | 10 | 0 (0.0) |
| `usa_mls.cpp` | leagues | 22 | 15 | 2 (5.6) |
| `bra_nat_third.cpp` | leagues | 16 | 11 | 1 (1.6) |
| `arg_second.cpp` | leagues | 18 | 1 | 0 (0.0) |
| `aus_nsl.cpp` | leagues | 16 | 8 | 0 (0.0) |
| `bra_nat_first.cpp` | leagues | 16 | 11 | 0 (0.0) |
| `bra_nat_second.cpp` | leagues | 12 | 7 | 0 (0.0) |
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
| `eng_first.cpp` | leagues | 10 | 7 | 0 (0.0) |
| `eng_prm.cpp` | leagues | 11 | 4 | 0 (0.0) |
| `eng_second.cpp` | leagues | 10 | 7 | 0 (0.0) |
| `eng_third.cpp` | leagues | 11 | 7 | 0 (0.0) |
| `fra_cfa.cpp` | leagues | 8 | 5 | 0 (0.0) |
| `fra_first.cpp` | leagues | 14 | 6 | 0 (0.0) |
| `fra_lower.cpp` | leagues | 5 | 4 | 0 (0.0) |
| `fra_second.cpp` | leagues | 7 | 4 | 0 (0.0) |
| `fra_third.cpp` | leagues | 7 | 4 | 0 (0.0) |
| `ita_ser_a.cpp` | leagues | 18 | 7 | 0 (0.0) |
| `ita_ser_b.cpp` | leagues | 14 | 7 | 0 (0.0) |
| `ita_ser_c1a.cpp` | leagues | 11 | 8 | 0 (0.0) |
| `ita_ser_c1b.cpp` | leagues | 12 | 8 | 0 (0.0) |
| `ita_ser_c2a.cpp` | leagues | 13 | 1 | 0 (0.0) |
| `jap_j1.cpp` | leagues | 15 | 9 | 0 (0.0) |
| `jap_j2.cpp` | leagues | 7 | 5 | 0 (0.0) |
| `league_stage.cpp` | leagues | 6 | 2 | 0 (0.0) |
| `spa_first.cpp` | leagues | 15 | 5 | 0 (0.0) |
| `spa_lower.cpp` | leagues | 7 | 4 | 0 (0.0) |
| `spa_second.cpp` | leagues | 7 | 4 | 0 (0.0) |
| `sub_league.cpp` | leagues | 4 | 1 | 0 (0.0) |
| `bra_champ_cup.cpp` | cups | 16 | 2 | 10 (3.6) |
| `cup_stage.cpp` | cups | 7 | 2 | 1 (2.7) |
| `usa_mls_all_stars.cpp` | cups | 8 | 5 | 1 (1.3) |
| `bra_cup.cpp` | cups | 8 | 6 | 0 (0.0) |
| `eng_auto_cup.cpp` | cups | 15 | 9 | 0 (0.0) |
| `eng_cc_cup.cpp` | cups | 8 | 4 | 0 (0.0) |
| `eng_charity.cpp` | cups | 6 | 3 | 0 (0.0) |
| `eng_fa_cup.cpp` | cups | 12 | 4 | 0 (0.0) |
| `fra_cup.cpp` | cups | 9 | 3 | 0 (0.0) |
| `fra_lge_cup.cpp` | cups | 7 | 3 | 0 (0.0) |
| `fra_super.cpp` | cups | 6 | 3 | 0 (0.0) |
| `gre_cup.cpp` | cups | 19 | 1 | 0 (0.0) |
| `ire_lge_cup.cpp` | cups | 17 | 1 | 0 (0.0) |
| `ita_c1_super.cpp` | cups | 6 | 3 | 0 (0.0) |
| `ita_c_cup.cpp` | cups | 17 | 11 | 0 (0.0) |
| `ita_cup.cpp` | cups | 17 | 11 | 0 (0.0) |
| `ita_super.cpp` | cups | 6 | 3 | 0 (0.0) |
| `jap_emp_cup.cpp` | cups | 7 | 4 | 0 (0.0) |
| `jap_j_cup.cpp` | cups | 6 | 4 | 0 (0.0) |
| `jap_super.cpp` | cups | 6 | 4 | 0 (0.0) |
| `spa_cup.cpp` | cups | 7 | 4 | 0 (0.0) |
| `spa_super.cpp` | cups | 7 | 3 | 0 (0.0) |
| `usa_open_cup.cpp` | cups | 6 | 4 | 0 (0.0) |
| `asia_club_champ.cpp` | eurocomp | 15 | 10 | 0 (0.0) |
| `asia_cup_winner.cpp` | eurocomp | 17 | 9 | 0 (0.0) |
| `asia_super_cup.cpp` | eurocomp | 7 | 4 | 0 (0.0) |
| `con_champ.cpp` | eurocomp | 8 | 6 | 0 (0.0) |
| `con_merc_cup.cpp` | eurocomp | 16 | 12 | 0 (0.0) |
| `conmebol_liber.cpp` | eurocomp | 16 | 11 | 0 (0.0) |
| `conmebol_merc.cpp` | eurocomp | 16 | 10 | 0 (0.0) |
| `eur_super_cup.cpp` | eurocomp | 6 | 3 | 0 (0.0) |
| `european_cup.cpp` | eurocomp | 50 | 18 | 0 (0.0) |
| `inter_amer_cup.cpp` | eurocomp | 6 | 3 | 0 (0.0) |
| `intertoto_cup.cpp` | eurocomp | 13 | 7 | 0 (0.0) |
| `oceania_club_champ.cpp` | eurocomp | 16 | 10 | 0 (0.0) |
| `uefa_cup.cpp` | eurocomp | 11 | 5 | 0 (0.0) |
| `world_club_champ.cpp` | eurocomp | 18 | 3 | 0 (0.0) |
| `world_club_cup.cpp` | eurocomp | 6 | 3 | 0 (0.0) |
| `euro_champ_qual.cpp` | intercomp | 32 | 4 | 9 (8.6) |
| `euro_champ.cpp` | intercomp | 25 | 0 | 5 (3.5) |
| `olympics.cpp` | intercomp | 16 | 0 | 4 (2.9) |
| `asia_nations.cpp` | intercomp | 17 | 0 | 5 (2.8) |
| `oceania_nations.cpp` | intercomp | 16 | 5 | 3 (2.5) |
| `goldcup.cpp` | intercomp | 57 | 11 | 2 (1.7) |
| `fifa_confed.cpp` | intercomp | 20 | 1 | 3 (1.0) |
| `african_nations.cpp` | intercomp | 20 | 12 | 0 (0.0) |
| `wc_concacaf_cup.cpp` | worldcup | 21 | 0 | 5 (5.8) |
| `world_cup.cpp` | worldcup | 26 | 9 | 3 (4.1) |
| `wc_african_cup.cpp` | worldcup | 20 | 0 | 5 (4.0) |
| `wc_asia_league.cpp` | worldcup | 23 | 1 | 4 (3.2) |
| `wc_south_american_league.cpp` | worldcup | 17 | 1 | 5 (3.1) |
| `wc_europe_league.cpp` | worldcup | 30 | 1 | 3 (2.4) |
| `wc_oceania_league.cpp` | worldcup | 21 | 8 | 3 (2.1) |
| `argentina_rules.cpp` | rules | 12 | 1 | 0 (0.0) |
| `australia_rules.cpp` | rules | 3 | 2 | 0 (0.0) |
| `belgium_rules.cpp` | rules | 10 | 1 | 0 (0.0) |
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

**Não rodam nesta carreira (87 arquivos):** competições de nações fora das 8 ligas, ou que só começam depois de 2002-06-01. Para elas é preciso outra carreira (outras ligas ou mais longa).

`arg_prm.cpp`, `bel_fa_cup.cpp`, `bel_first.cpp`, `bel_second.cpp`, `bel_super.cpp`, `bel_third.cpp`, `cro_a1.cpp`, `cro_a2a.cpp`, `cro_a2b.cpp`, `cro_cup.cpp`, `den_cup.cpp`, `den_first.cpp`, `den_prm.cpp`, `den_second.cpp`, `eng_conf.cpp`, `eng_fa_trophy.cpp`, `fin_cup.cpp`, `fin_first.cpp`, `ger_cup.cpp`, `ger_first.cpp`, `ger_lge_cup.cpp`, `ger_regional.cpp`, `ger_second.cpp`, `gre_prm.cpp`, `gre_second.cpp`, `gre_super.cpp`, `hol_cup.cpp`, `hol_first.cpp`, `hol_prm.cpp`, `hol_super.cpp`, `ire_chal_cup.cpp`, `ire_first.cpp`, `ire_leinster_cup.cpp`, `ire_munster_cup.cpp`, `ire_pres_cup.cpp`, `ire_prm.cpp`, `ire_super_cup.cpp`, `ita_ser_c2b.cpp`, `ita_ser_c2c.cpp`, `japan_rules.cpp`, `kor_fa_cup.cpp`, `kor_league.cpp`, `kor_league_cup.cpp`, `kor_super_cup.cpp`, `mini_cup.cpp`, `mini_league.cpp`, `nir_charity.cpp`, `nir_cup.cpp`, `nir_first.cpp`, `nir_lge_cup.cpp`, `nir_prm.cpp`, `nor_cup.cpp`, `nor_first.cpp`, `nor_prm.cpp`, `pol_cup.cpp`, `pol_first.cpp`, `pol_lge_cup.cpp`, `pol_second.cpp`, `pol_super.cpp`, `por_cup.cpp`, `por_prm.cpp`, `por_second.cpp`, `por_second_b.cpp`, `por_super.cpp`, `rus_cup.cpp`, `rus_first.cpp`, `rus_prm.cpp`, `sco_chal_cup.cpp`, `sco_fa_cup.cpp`, `sco_first.cpp`, `sco_lge_cup.cpp`, `sco_prm.cpp`, `sco_second.cpp`, `sco_third.cpp`, `spa_second_b.cpp`, `swe_cup.cpp`, `swe_first.cpp`, `swe_prm.cpp`, `swe_second.cpp`, `tur_cup.cpp`, `tur_first.cpp`, `tur_second.cpp`, `tur_second_b.cpp`, `wel_cup.cpp`, `wel_first.cpp`, `wel_lge_cup.cpp`, `wel_prm_cup.cpp`
