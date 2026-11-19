# The Turkish second division category B turnover and stages, and the Polish First Division stages (tur_second_b.cpp,
# pol_first.cpp). Reader r3_mwcd (19/11/2026), round 2.
# - 0x900500 TurSecondB::start_new_season (1 call, the turnover of summer 2002): the opening of rebuild_season (stages
#   destroyed), but sort_table before create_fixtures, then create_group_stage(i) (0x900780, thiscall, ret 4) for i =
#   0..3, the predicted positions and the officials, without League::set_club_season_expectations; the same shape as
#   BelThird's (mwc_19), written out (another batch).
# - 0x9006c0 TurSecondB::advance_stage: stage 4 builds the promotion group (0x900ab0) and the classification groups
#   (0x900d00), stage 10 the play-off (0x901040); line 0x265.
# - 0x7c9ad0 PolFirst::advance_stage: inc/store/dec/jne: the final stage (0x7c9b60) when the OLD stage was 0 (the new
#   one 1), unlike the state leagues' template (new stage 0); line 0x24f.
# Conventions: thiscall, ret 0. Comparers: the turnover only the original (the year +1); the advance_stage ones side by
# side on +0x30 when nothing is built (compare_advance_stage_building), otherwise only the original. The advance_stage
# calls run in spring 2002, at the end of each phase.
# Mutations: none of their own (the turnover builds the 2002/03 season, after the chain's date; an advance_stage that
# builds another stage or none would hang the league rather than change who plays). The comparers carry the proof.
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('tur_second_b_start_new_season', 0x900500, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tur_second_b_advance_stage', 0x9006c0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('pol_first_advance_stage', 0x7c9ad0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30])]
DOCS = ('a virada e as fases da categoria B turca e as fases da Primeira Divisão polonesa', 'the Turkish category B turnover and stages and the Polish First Division stages',
        [('tur_second_b_start_new_season', 'TurSecondB::start_new_season', '0x900500', 'tur_second_b.cpp', 'thiscall, ret 0',
          'regra: a virada da segunda divisão turca categoria B (chamada pela da Primeira): a temporada reaberta, as fases destruídas, o ano seguinte; as regras, a tabela, a ordem e os jogos (nesta ordem); as quatro fases de grupos refeitas; as posições previstas e os árbitros (sem as expectativas da diretoria)',
          "rule: the Turkish second division category B turnover (called by the First Division's): the season reopened, the stages destroyed, the next year; the rules, the table, the order and the fixtures (in this order); the four group stages built again; the predicted positions and the officials (no board expectations)"),
         ('tur_second_b_advance_stage', 'TurSecondB::advance_stage', '0x9006c0', 'tur_second_b.cpp', 'thiscall, ret 0',
          'regra: a fase seguinte da categoria B turca: depois da última, a caixa de erro; senão a fase atual sobe uma; ao chegar à fase 4 são montados o grupo de acesso e os grupos de classificação, ao chegar à 10, o mata-mata',
          'rule: the next stage of the Turkish category B: past the last stage, the error box; otherwise the current stage goes up by one; on reaching stage 4 the promotion group and the classification groups are built, on reaching stage 10 the play-off'),
         ('pol_first_advance_stage', 'PolFirst::advance_stage', '0x7c9ad0', 'pol_first.cpp', 'thiscall, ret 0',
          'regra: a fase seguinte da Primeira Divisão polonesa: depois da última, a caixa de erro; senão a fase atual sobe uma; ao sair da fase 0 (chegando à 1) é montada a fase final',
          'rule: the next stage of the Polish First Division: past the last stage, the error box; otherwise the current stage goes up by one; leaving stage 0 (reaching 1) the final stage is built')])
CAMPOS = [('liga +0x2c número de fases, +0x30 fase atual, +0x4c, +0xba, +8, +0xc, +0x40', 'league +0x2c stage count, +0x30 current stage, +0x4c, +0xba, +8, +0xc, +0x40')]
