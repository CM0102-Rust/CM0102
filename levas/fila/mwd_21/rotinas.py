# The Polish Super Cup (pol_super.cpp). Reader mwd (12/11/2026), round 2 (the constructor 1 call; the round table 2; the
# turnover 1).
# - 0x7cc920 PolSuper::PolSuper: the national cup constructor template without prize fund nor rank_teams (mwd_8).
# - 0x7ccbd0 PolSuper::create_round_table: the Greek Super Cup template (checked instruction by instruction against
#   0x5ec2f0: only the match date, 18/7 weekday 1, and the file change).
# - 0x7ccf90 PolSuper::start_new_season: the template of usa_mls_all_stars_cup_start_new_season without the predicted
#   positions (checked against JapSuper's 0x6664e0).
# Conventions: as in mwd_8. Comparers: the constructor only the original; the round side by side; the turnover only the
# original (the year +1). Equal in cm0102-1080.exe.
# Depends on: construct_east_nation_cup / compare_east_nation_cup_ctor (mwd_8), super_cup_round_table (mwd_20).
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_pol_super', 0x7cc920, [0x6a, 0xff, 0x68, 0xa9, 0xf8, 0x95, 0x00]),
           ('pol_super_create_round_table', 0x7ccbd0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('pol_super_start_new_season', 0x7ccf90, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Supercopa da Polônia', 'the Polish Super Cup',
        [('construct_pol_super', 'PolSuper::PolSuper', '0x7cc920', 'pol_super.cpp', 'thiscall, ret 8',
          'regra: a Supercopa da Polônia ao criar a carreira: a regra de disciplina 0x1f, 7 reservas e 3 substituições e, sem estado gravado, os dois clubes (o campeão da liga e o da copa), a rodada, a árvore de jogos e as estatísticas; sem fundo de prêmios nem posições previstas',
          'rule: the Polish Super Cup at career creation: discipline rule 0x1f, 7 substitutes and 3 substitutions and, with no saved state, its two clubs (the league champion and the cup winner), the round, the fixture tree and the statistics; no prize fund nor predicted positions'),
         ('pol_super_create_round_table', 'PolSuper::create_round_table', '0x7ccbd0', 'pol_super.cpp', 'thiscall, ret 0x10',
          'regra: a Supercopa da Polônia é um jogo único entre 2 clubes, sorteado em 5/7 e jogado em 18/7 no local 4 [inferência: campo neutro], desempate 3, categoria 6, sem prêmios',
          'rule: the Polish Super Cup is a single match between 2 clubs, drawn on 5/7 and played on 18/7 at venue 4 [inference: a neutral ground], tie rule 3, category 6, no prizes'),
         ('pol_super_start_new_season', 'PolSuper::start_new_season', '0x7ccf90', 'pol_super.cpp', 'thiscall, ret 0',
          'regra: a virada da Supercopa da Polônia: as tabelas da edição anterior liberadas, as estatísticas arquivadas, o ano seguinte, os dois clubes, a rodada e a árvore de jogos; sem posições previstas',
          "rule: the Polish Super Cup season turnover: last edition's tables freed, the statistics archived, the next year, the two clubs, the round and the fixture tree; no predicted positions")])
CAMPOS = [('copa +4, +0x40, +0x50, +0xa3, +0xa7, +8, +0x4c, +0xab, +0xb1; a rodada de copa (0x68 bytes)', 'cup +4, +0x40, +0x50, +0xa3, +0xa7, +8, +0x4c, +0xab, +0xb1; the cup round (0x68 bytes)')]
