# The season turnover of the Eerste Divisie and the Belgian Second and Third Divisions (hol_first.cpp, bel_second.cpp,
# bel_third.cpp). Reader r3_mwcd (19/11/2026), round 2 (1 call each, the turnover of summer 2002, called by the top
# division's).
# - 0x5f63f0 HolFirst::start_new_season and 0x41ff50 BelSecond::start_new_season: instruction by instruction the
#   template of FraSecond::start_new_season (0x5bf160, mwf_7): rebuild_season(.., 1) + set_expectations_and_renew_officials.
# - 0x4227b0 BelThird::start_new_season: the same opening (stages destroyed), but sort_table before create_fixtures,
#   then BelThird::create_group_b_stage (0x422970, thiscall, ret 0, not ported), the predicted positions and the
#   officials, without League::set_club_season_expectations; written out (no shared helper: TurSecondB, mwd_29, has
#   the same shape in another batch).
# Conventions: thiscall, ret 0. Comparers: only the original (the year +1), as the sisters.
# Mutations: none of their own (the guide: what the turnover builds is used by the 2002/03 season, after the chain's
# date; a mutation in the group B stage would only be seen in 2002/03). The template is proven by mwf_7.
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('hol_first_start_new_season', 0x5f63f0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('bel_second_start_new_season', 0x41ff50, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('bel_third_start_new_season', 0x4227b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a virada das divisões de baixo da Holanda e da Bélgica', "the Dutch and Belgian lower divisions' season turnover",
        [('hol_first_start_new_season', 'HolFirst::start_new_season', '0x5f63f0', 'hol_first.cpp', 'thiscall, ret 0',
          'regra: a virada da Eerste Divisie (chamada pela da Eredivisie): a temporada reaberta, as fases da temporada passada destruídas, o ano seguinte, as regras, a tabela, os jogos e a ordem refeitos; as posições previstas, as expectativas da diretoria e os árbitros',
          "rule: the Eerste Divisie turnover (called by the Eredivisie's): the season reopened, last season's stages destroyed, the next year, the rules, the table, the fixtures and the order rebuilt; the predicted positions, the board expectations and the officials"),
         ('bel_second_start_new_season', 'BelSecond::start_new_season', '0x41ff50', 'bel_second.cpp', 'thiscall, ret 0',
          'regra: a virada da Segunda Divisão belga (chamada pela da Primeira): o mesmo molde da Eerste Divisie',
          "rule: the Belgian Second Division turnover (called by the First Division's): the same template as the Eerste Divisie"),
         ('bel_third_start_new_season', 'BelThird::start_new_season', '0x4227b0', 'bel_third.cpp', 'thiscall, ret 0',
          'regra: a virada da Terceira Divisão belga (chamada pela da Primeira): a temporada reaberta, as fases destruídas, o ano seguinte; as regras, a tabela, a ordem e os jogos (nesta ordem); a fase do grupo B refeita; as posições previstas e os árbitros (sem as expectativas da diretoria)',
          "rule: the Belgian Third Division turnover (called by the First Division's): the season reopened, the stages destroyed, the next year; the rules, the table, the order and the fixtures (in this order); the group B stage built again; the predicted positions and the officials (no board expectations)")])
CAMPOS = [('liga +0x4c, +0xba, +8, +0xc, +0x30, +0x40', 'league +0x4c, +0xba, +8, +0xc, +0x30, +0x40')]
