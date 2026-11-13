# The Irish First Division (ire_first.cpp, in leagues\): the season turnover, the next stage and the promotion
# play-off. Whole-world front, round 2 (reader mwa, 12/11/2026), career with ENG IRL NIR to 2002-08-01. Uses mwa_13's
# helpers (advance_irish_first_stage, create_irish_promotion_play_off and their comparers): it goes in the same batch
# or after it.
# - 0x635b30 (IreFirst::start_new_season, 1 call: called by the Premier's turnover, mwa_15): BraNatSecond's template
#   (same.py against NirFirst's and 0x42cc10: only the table routine, the file and the lines).
# - 0x635cf0 (IreFirst::advance_stage, 1 call) and 0x635dc0 (IreFirst::create_promotion_playoff_stage, 1 call):
#   same.py against NirFirst's 0x78bfd0 and 0x78c0a0: only the Premier's key global, the file and the lines.
# Conventions: see mwa_13 (the same calls).
# Comparers: the turnover only the original (the year up by one); the next stage only the original; the play-off only
# the original, with the plan.
# Defect of the original: when one of the two play-off clubs is missing, the list stays allocated (a leak).
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('ire_first_start_new_season', 0x635b30, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ire_first_advance_stage', 0x635cf0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ire_first_create_promotion_playoff_stage', 0x635dc0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a First Division da Irlanda: a virada da temporada, a fase seguinte e a repescagem do acesso',
        'the Irish First Division: the season turnover, the next stage and the promotion play-off',
        [('ire_first_start_new_season', 'IreFirst::start_new_season', '0x635b30', 'ire_first.cpp', 'thiscall, ret 0',
          'regra: a virada da First Division da Irlanda: as estatísticas arquivadas, a tabela e a tabela de rodadas limpas, o ano avança e as regras, a tabela, a tabela de jogos e a ordem são refeitas; depois as posições previstas, as expectativas das diretorias e a renovação dos árbitros',
          'rule: the Irish First Division turnover: the statistics archived, the table and the round table cleared, the year moved on and the rules, the table, the fixtures and the order rebuilt; then the predicted positions, the board expectations and the referee renewal'),
         ('ire_first_advance_stage', 'IreFirst::advance_stage', '0x635cf0', 'ire_first.cpp', 'thiscall, ret 0',
          'regra: a fase de liga da First Division da Irlanda só termina quando a Premier acima terminou: então as posições da Premier são atualizadas e a repescagem do acesso é montada; depois da última fase, a caixa de erro',
          "rule: the Irish First Division's league stage ends only when the Premier Division above has finished: then the Premier's positions are updated and the promotion play-off is built; past the last stage, the error box"),
         ('ire_first_create_promotion_playoff_stage', 'IreFirst::create_promotion_playoff_stage', '0x635dc0', 'ire_first.cpp', 'thiscall, ret 0',
          'regra: a repescagem da Irlanda: o vice da First Division contra o penúltimo da Premier (os clubes com a marca de repescagem em cada tabela), numa fase de dois clubes; sem um deles, não há repescagem',
          "rule: the Irish promotion play-off: the First Division runner-up against the Premier Division's second last (the clubs with the play-off status in each table), in a two-club stage; without one of them, no play-off")])
CAMPOS = []
INTERESSA = []
