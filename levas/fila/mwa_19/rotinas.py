# The Northern Irish League Cup (nir_lge_cup.cpp, in cups\): the next stage and the season turnover.
# Whole-world front, round 2 (reader mwa, 12/11/2026), career with ENG IRL NIR to 2002-08-01. Uses mwa_18's start_irish_league_cup_season: it goes in
# the same batch or after it.
# - 0x78d330 (NirLgeCup::advance_stage, 1 call, when the groups end [inference: autumn 2001]): advance_cup_stage (ItaCup::advance_stage 0x63f5b0:
#   only the file, the line, the end of the groups and the knockout routine).
# - 0x78d160 (NirLgeCup::start_new_season, 1 call, at the season turn [inference: summer 2002]): start_irish_league_cup_season
#   (close to rebuild_season, with the entrant vector freed and the entrants and predicted positions before the rules).
# Conventions checked at the call sites: advance_stage the comp virtual +0x28 (thiscall, ret 0); start_new_season the
# virtual +8 (thiscall, ret 0). Inside: League::free_table 0x687970 (push 0 -> thiscall, ret 4), free 0x9452ca
# (cdecl), Comp_Stats::archive_season_and_reset 0x4a1c50 (push 1 -> thiscall, ret 4), the stages' virtual 0 (push 1 ->
# thiscall, ret 4), the cup's own routines (thiscall, ret 0; create_group_stage push g -> thiscall, ret 4), the
# virtuals +0x5c and +0x8c, League::create_fixtures 0x6835c0, League::sort_table 0x6827d0 (push 0), the error box.
# Comparers: both only the original: the stage goes up by one except at the last; the year goes up by one.
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('nir_lge_cup_advance_stage', 0x78d330, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('nir_lge_cup_start_new_season', 0x78d160, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Copa da Liga da Irlanda do Norte: a fase seguinte e a virada da temporada',
        'the Northern Irish League Cup: the next stage and the season turnover',
        [('nir_lge_cup_advance_stage', 'NirLgeCup::advance_stage', '0x78d330', 'nir_lge_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Liga da Irlanda do Norte avança fase a fase pelos seus 3 grupos; acabados os grupos, monta o mata-mata; depois da última fase, a caixa de erro',
          'rule: the Northern Irish League Cup moves stage by stage through its 3 groups; when the groups are over, the knockout is built; past the last stage, the error box'),
         ('nir_lge_cup_start_new_season', 'NirLgeCup::start_new_season', '0x78d160', 'nir_lge_cup.cpp', 'thiscall, ret 0',
          'regra: na virada a Copa da Liga da Irlanda do Norte é refeita: a tabela, a tabela de rodadas, os inscritos e as fases da edição anterior são liberados, as estatísticas arquivadas, o ano avança; depois os 20 inscritos, as posições previstas, as regras, a tabela do primeiro grupo, os seus jogos e ordem e as fases dos 3 grupos',
          "rule: at the season turn the Northern Irish League Cup is rebuilt: the table, the round table, the entrants and the stages of the last edition are freed, the statistics archived, the year moves on; then the 20 entrants, the predicted positions, the rules, the first group's table, its fixtures and order and the stages of the 3 groups")])
CAMPOS = []
INTERESSA = []
