# Mapa das competições

Gerado por `python engenharia-reversa/competicoes.py` (o mapa de funções de maxevans60, CC BY 4.0, e a cobertura de uma carreira com as 8 ligas `ENG ESP ITA FRA BRA JPN AUS USA` até 2002-06-01, `levas/competicoes/`; as rotinas já em Rust não levam contador). "falta" = chamada ao menos uma vez nessa carreira e ainda não está em Rust (a lista para `candidatos.py --competicoes` fica em `docs/cobertura_competicoes.txt`). Como cada competição funciona por dentro (objeto, vtable, estágios, vagas): `docs/.md`.

**Total:** 2695 funções, 1475.7 KB; em Rust 489 (15.5% dos bytes); faltam 463 (452.9 KB).

| Grupo | Funções | KB | Rust | Falta (KB) | Progresso |
|---|---|---|---|---|---|
| Base: Liga, Copa, comp, calendário, sorteios | 396 | 272.6 | 250 | 29 (85.0) | 🟩🟩🟩🟩🟨⬜⬜⬜⬜⬜ **45.3%** |
| Ligas nacionais (formato de cada divisão) | 1048 | 647.8 | 126 | 177 (206.3) | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **7.4%** |
| Copas nacionais | 526 | 228.3 | 30 | 79 (48.8) | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **9.5%** |
| Competições europeias | 221 | 98.2 | 29 | 87 (47.9) | 🟩🟨⬜⬜⬜⬜⬜⬜⬜⬜ **16.3%** |
| Continentais fora da Europa | 203 | 105.2 | 7 | 57 (37.3) | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **1.5%** |
| Copa do Mundo e eliminatórias | 158 | 92.4 | 14 | 34 (27.5) | 🟨⬜⬜⬜⬜⬜⬜⬜⬜⬜ **9.6%** |
| Torneios amistosos | 17 | 4.5 | 0 | 0 (0.0) | ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜ **0.0%** |
| Regras de transferência por nação (`transfer/*_rules.cpp`) | 126 | 26.7 | 33 | 0 (0.0) | 🟩🟩🟩🟨⬜⬜⬜⬜⬜⬜ **34.0%** |

## Por arquivo

Só os arquivos com alguma rotina que falta; ordem: grupo, depois o que falta.

| Arquivo | Grupo | Funções | Rust | Falta (KB) |
|---|---|---|---|---|
| `comp_util.cpp` | (raiz) | 38 | 23 | 5 (40.7) |
| `Cup.cpp` | (raiz) | 47 | 36 | 2 (12.9) |
| `friendly.cpp` | (raiz) | 67 | 48 | 3 (6.7) |
| `stadium.cpp` | (raiz) | 17 | 8 | 4 (6.7) |
| `league.cpp` | (raiz) | 61 | 50 | 2 (5.8) |
| `host_country.cpp` | (raiz) | 12 | 4 | 4 (5.0) |
| `fix_man.cpp` | (raiz) | 37 | 31 | 3 (4.0) |
| `fifa_rankings.cpp` | (raiz) | 20 | 9 | 1 (1.2) |
| `uefa_seeding.cpp` | (raiz) | 26 | 10 | 2 (1.0) |
| `comp.cpp` | (raiz) | 46 | 14 | 2 (0.6) |
| `conmebol_seeding.cpp` | (raiz) | 9 | 7 | 1 (0.5) |
| `squad_manager.cpp` | (raiz) | 16 | 10 | 0 (0.0) |
| `ita_ser_b.cpp` | leagues | 14 | 2 | 4 (18.1) |
| `ita_ser_a.cpp` | leagues | 18 | 2 | 5 (15.8) |
| `ita_ser_c1a.cpp` | leagues | 11 | 3 | 5 (15.1) |
| `ita_ser_c1b.cpp` | leagues | 12 | 3 | 5 (15.1) |
| `usa_mls.cpp` | leagues | 22 | 5 | 12 (10.3) |
| `spa_lower.cpp` | leagues | 7 | 1 | 3 (9.7) |
| `spa_second.cpp` | leagues | 7 | 1 | 3 (8.6) |
| `spa_first.cpp` | leagues | 15 | 3 | 2 (6.6) |
| `bra_nat_first.cpp` | leagues | 16 | 3 | 8 (6.4) |
| `bra_nat_third.cpp` | leagues | 16 | 4 | 8 (5.5) |
| `eng_prm.cpp` | leagues | 11 | 2 | 2 (5.5) |
| `eng_first.cpp` | leagues | 10 | 5 | 2 (5.1) |
| `bra_reg_minas_gerais.cpp` | leagues | 14 | 3 | 9 (4.6) |
| `bra_reg_parana.cpp` | leagues | 13 | 2 | 9 (4.6) |
| `bra_reg_rio.cpp` | leagues | 14 | 2 | 9 (4.6) |
| `eng_third.cpp` | leagues | 11 | 4 | 3 (4.3) |
| `eng_second.cpp` | leagues | 10 | 5 | 2 (4.2) |
| `fra_first.cpp` | leagues | 14 | 2 | 4 (4.0) |
| `jap_j1.cpp` | leagues | 15 | 2 | 7 (3.9) |
| `bra_nat_second.cpp` | leagues | 12 | 2 | 5 (3.9) |
| `bra_reg_central.cpp` | leagues | 11 | 3 | 6 (3.8) |
| `bra_reg_goias.cpp` | leagues | 11 | 2 | 6 (3.8) |
| `bra_reg_gaucho.cpp` | leagues | 13 | 3 | 6 (3.7) |
| `aus_nsl.cpp` | leagues | 16 | 3 | 5 (3.5) |
| `fra_cfa.cpp` | leagues | 8 | 1 | 3 (3.4) |
| `bra_reg_bahia.cpp` | leagues | 10 | 3 | 5 (3.3) |
| `bra_reg_north.cpp` | leagues | 9 | 2 | 5 (3.3) |
| `bra_reg_pern.cpp` | leagues | 10 | 3 | 5 (3.3) |
| `bra_reg_santa.cpp` | leagues | 10 | 3 | 5 (3.3) |
| `bra_reg_sp.cpp` | leagues | 10 | 2 | 6 (3.1) |
| `bra_reg_northeast.cpp` | leagues | 9 | 2 | 5 (3.1) |
| `jap_j2.cpp` | leagues | 7 | 2 | 3 (3.1) |
| `fra_lower.cpp` | leagues | 5 | 1 | 3 (3.0) |
| `fra_second.cpp` | leagues | 7 | 2 | 2 (2.8) |
| `fra_third.cpp` | leagues | 7 | 2 | 2 (2.4) |
| `league_stage.cpp` | leagues | 6 | 0 | 2 (1.1) |
| `sub_league.cpp` | leagues | 4 | 0 | 1 (0.7) |
| `arg_second.cpp` | leagues | 18 | 1 | 0 (0.0) |
| `ita_ser_c2a.cpp` | leagues | 13 | 1 | 0 (0.0) |
| `bra_cup.cpp` | cups | 8 | 0 | 6 (8.8) |
| `ita_cup.cpp` | cups | 17 | 1 | 10 (5.6) |
| `ita_c_cup.cpp` | cups | 17 | 1 | 10 (5.4) |
| `eng_auto_cup.cpp` | cups | 15 | 1 | 8 (4.6) |
| `bra_champ_cup.cpp` | cups | 16 | 1 | 11 (4.5) |
| `cup_stage.cpp` | cups | 7 | 0 | 3 (3.5) |
| `eng_fa_cup.cpp` | cups | 12 | 2 | 2 (2.7) |
| `usa_mls_all_stars.cpp` | cups | 8 | 2 | 4 (2.7) |
| `spa_cup.cpp` | cups | 7 | 1 | 3 (2.5) |
| `ita_super.cpp` | cups | 6 | 0 | 3 (1.3) |
| `ita_c1_super.cpp` | cups | 6 | 0 | 3 (1.0) |
| `jap_emp_cup.cpp` | cups | 7 | 2 | 2 (0.9) |
| `jap_j_cup.cpp` | cups | 6 | 2 | 2 (0.9) |
| `usa_open_cup.cpp` | cups | 6 | 2 | 2 (0.9) |
| `jap_super.cpp` | cups | 6 | 2 | 2 (0.9) |
| `eng_cc_cup.cpp` | cups | 8 | 3 | 1 (0.5) |
| `eng_charity.cpp` | cups | 6 | 2 | 1 (0.5) |
| `fra_cup.cpp` | cups | 9 | 2 | 1 (0.5) |
| `fra_lge_cup.cpp` | cups | 7 | 2 | 1 (0.5) |
| `fra_super.cpp` | cups | 6 | 2 | 1 (0.4) |
| `spa_super.cpp` | cups | 7 | 2 | 1 (0.4) |
| `gre_cup.cpp` | cups | 19 | 0 | 1 (0.0) |
| `ire_lge_cup.cpp` | cups | 17 | 0 | 1 (0.0) |
| `asia_club_champ.cpp` | eurocomp | 15 | 0 | 10 (6.5) |
| `asia_cup_winner.cpp` | eurocomp | 17 | 0 | 9 (5.7) |
| `con_merc_cup.cpp` | eurocomp | 16 | 1 | 11 (5.1) |
| `conmebol_merc.cpp` | eurocomp | 16 | 0 | 10 (4.9) |
| `oceania_club_champ.cpp` | eurocomp | 16 | 1 | 9 (4.5) |
| `conmebol_liber.cpp` | eurocomp | 16 | 4 | 7 (3.7) |
| `uefa_cup.cpp` | eurocomp | 11 | 1 | 4 (3.2) |
| `intertoto_cup.cpp` | eurocomp | 13 | 2 | 5 (2.4) |
| `world_club_cup.cpp` | eurocomp | 6 | 0 | 3 (2.0) |
| `world_club_champ.cpp` | eurocomp | 18 | 0 | 3 (2.0) |
| `inter_amer_cup.cpp` | eurocomp | 6 | 0 | 3 (1.9) |
| `asia_super_cup.cpp` | eurocomp | 7 | 0 | 4 (1.7) |
| `european_cup.cpp` | eurocomp | 50 | 17 | 3 (1.6) |
| `con_champ.cpp` | eurocomp | 8 | 3 | 3 (1.5) |
| `eur_super_cup.cpp` | eurocomp | 6 | 0 | 3 (1.2) |
| `euro_champ_qual.cpp` | intercomp | 32 | 0 | 13 (9.3) |
| `goldcup.cpp` | intercomp | 57 | 2 | 11 (6.8) |
| `african_nations.cpp` | intercomp | 20 | 5 | 7 (4.9) |
| `oceania_nations.cpp` | intercomp | 16 | 0 | 8 (4.9) |
| `euro_champ.cpp` | intercomp | 25 | 0 | 5 (3.5) |
| `olympics.cpp` | intercomp | 16 | 0 | 4 (2.9) |
| `asia_nations.cpp` | intercomp | 17 | 0 | 5 (2.8) |
| `fifa_confed.cpp` | intercomp | 20 | 0 | 4 (2.2) |
| `wc_concacaf_cup.cpp` | worldcup | 21 | 0 | 5 (5.8) |
| `wc_oceania_league.cpp` | worldcup | 21 | 2 | 9 (5.0) |
| `world_cup.cpp` | worldcup | 26 | 9 | 3 (4.1) |
| `wc_african_cup.cpp` | worldcup | 20 | 0 | 5 (4.0) |
| `wc_asia_league.cpp` | worldcup | 23 | 1 | 4 (3.2) |
| `wc_south_american_league.cpp` | worldcup | 17 | 1 | 5 (3.1) |
| `wc_europe_league.cpp` | worldcup | 30 | 1 | 3 (2.4) |
| `argentina_rules.cpp` | rules | 12 | 1 | 0 (0.0) |
| `australia_rules.cpp` | rules | 3 | 1 | 0 (0.0) |
| `belgium_rules.cpp` | rules | 10 | 1 | 0 (0.0) |
| `brazil_rules.cpp` | rules | 3 | 1 | 0 (0.0) |
| `croatia_rules.cpp` | rules | 2 | 1 | 0 (0.0) |
| `england_rules.cpp` | rules | 7 | 5 | 0 (0.0) |
| `finland_rules.cpp` | rules | 4 | 1 | 0 (0.0) |
| `france_rules.cpp` | rules | 10 | 1 | 0 (0.0) |
| `germany_rules.cpp` | rules | 7 | 1 | 0 (0.0) |
| `greece_rules.cpp` | rules | 4 | 1 | 0 (0.0) |
| `holland_rules.cpp` | rules | 1 | 1 | 0 (0.0) |
| `ireland_rules.cpp` | rules | 6 | 2 | 0 (0.0) |
| `italy_rules.cpp` | rules | 9 | 2 | 0 (0.0) |
| `korea_rules.cpp` | rules | 3 | 1 | 0 (0.0) |
| `northern_ireland_rules.cpp` | rules | 4 | 1 | 0 (0.0) |
| `norway_rules.cpp` | rules | 6 | 1 | 0 (0.0) |
| `poland_rules.cpp` | rules | 2 | 1 | 0 (0.0) |
| `portugal_rules.cpp` | rules | 5 | 1 | 0 (0.0) |
| `russia_rules.cpp` | rules | 4 | 1 | 0 (0.0) |
| `scotland_rules.cpp` | rules | 5 | 3 | 0 (0.0) |
| `spain_rules.cpp` | rules | 7 | 2 | 0 (0.0) |
| `turkey_rules.cpp` | rules | 3 | 1 | 0 (0.0) |
| `usa_rules.cpp` | rules | 2 | 1 | 0 (0.0) |
| `wales_rules.cpp` | rules | 2 | 1 | 0 (0.0) |

**Não rodam nesta carreira (87 arquivos):** competições de nações fora das 8 ligas, ou que só começam depois de 2002-06-01. Para elas é preciso outra carreira (outras ligas ou mais longa).

`arg_prm.cpp`, `bel_fa_cup.cpp`, `bel_first.cpp`, `bel_second.cpp`, `bel_super.cpp`, `bel_third.cpp`, `cro_a1.cpp`, `cro_a2a.cpp`, `cro_a2b.cpp`, `cro_cup.cpp`, `den_cup.cpp`, `den_first.cpp`, `den_prm.cpp`, `den_second.cpp`, `eng_conf.cpp`, `eng_fa_trophy.cpp`, `fin_cup.cpp`, `fin_first.cpp`, `ger_cup.cpp`, `ger_first.cpp`, `ger_lge_cup.cpp`, `ger_regional.cpp`, `ger_second.cpp`, `gre_prm.cpp`, `gre_second.cpp`, `gre_super.cpp`, `hol_cup.cpp`, `hol_first.cpp`, `hol_prm.cpp`, `hol_super.cpp`, `ire_chal_cup.cpp`, `ire_first.cpp`, `ire_leinster_cup.cpp`, `ire_munster_cup.cpp`, `ire_pres_cup.cpp`, `ire_prm.cpp`, `ire_super_cup.cpp`, `ita_ser_c2b.cpp`, `ita_ser_c2c.cpp`, `japan_rules.cpp`, `kor_fa_cup.cpp`, `kor_league.cpp`, `kor_league_cup.cpp`, `kor_super_cup.cpp`, `mini_cup.cpp`, `mini_league.cpp`, `nir_charity.cpp`, `nir_cup.cpp`, `nir_first.cpp`, `nir_lge_cup.cpp`, `nir_prm.cpp`, `nor_cup.cpp`, `nor_first.cpp`, `nor_prm.cpp`, `pol_cup.cpp`, `pol_first.cpp`, `pol_lge_cup.cpp`, `pol_second.cpp`, `pol_super.cpp`, `por_cup.cpp`, `por_prm.cpp`, `por_second.cpp`, `por_second_b.cpp`, `por_super.cpp`, `rus_cup.cpp`, `rus_first.cpp`, `rus_prm.cpp`, `sco_chal_cup.cpp`, `sco_fa_cup.cpp`, `sco_first.cpp`, `sco_lge_cup.cpp`, `sco_prm.cpp`, `sco_second.cpp`, `sco_third.cpp`, `spa_second_b.cpp`, `swe_cup.cpp`, `swe_first.cpp`, `swe_prm.cpp`, `swe_second.cpp`, `tur_cup.cpp`, `tur_first.cpp`, `tur_second.cpp`, `tur_second_b.cpp`, `wel_cup.cpp`, `wel_first.cpp`, `wel_lge_cup.cpp`, `wel_prm_cup.cpp`
