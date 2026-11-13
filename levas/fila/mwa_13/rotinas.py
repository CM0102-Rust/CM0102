# The Northern Irish First Division (nir_first.cpp, in leagues\): the season turnover, the next stage and the promotion
# play-off. Whole-world front, round 2 (reader mwa, 12/11/2026), career with ENG IRL NIR to 2002-08-01.
# - 0x78be10 (NirFirst::start_new_season, 1 call: called by the Premier's turnover, mwa_15): BraNatSecond's template
#   (0x42cc10, same.py: only the table routine, the file and the lines), over rebuild_season and
#   set_expectations_and_renew_officials.
# - 0x78bfd0 (NirFirst::advance_stage, 1 call, when the league stage ends [inference: April 2002]): waits for the
#   Premier Division (its CHECK_COMPLETE virtual) before the play-off (advance_irish_first_stage, also used by mwa_14).
# - 0x78c0a0 (NirFirst::create_playoff_stage, 1 call): the two-club play-off (create_irish_promotion_play_off, also used
#   by mwa_14): EngFirst::create_playoff_stage's Cup_Stage call (the same 15 arguments with 2 clubs).
# Conventions checked at the call sites: start_new_season the virtual +8 (thiscall, ret 0); advance_stage the comp
# virtual (thiscall, ret 0); create_playoff_stage (mov ecx, esi; call -> thiscall, ret 0). Inside: the Premier's
# virtuals +0x10 (push 1; push 0 -> thiscall, ret 8, al) and +0x94 (thiscall, ret 0), League::get_table_row 0x6826d0
# (push i -> thiscall, ret 4), the virtual +0x3c (push 0; push &extra; push &rounds; push 0 -> thiscall, ret 0x10),
# operator new 0x944cf1 (cdecl), Cup_Stage::Cup_Stage 0x522e00 (15 pushes -> thiscall, ret 0x3c), free 0x9452ca
# (cdecl), Cup::set_up_round 0x51c800 (push 0 -> thiscall, ret 4), the error box.
# Comparers: the turnover only the original (the year up by one); the next stage only the original (the stage stays or
# goes up by one); the play-off only the original, with the plan (the two clubs as a set).
# Defect of the original: when one of the two play-off clubs is missing, the list stays allocated (a leak).
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('nir_first_start_new_season', 0x78be10, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('nir_first_advance_stage', 0x78bfd0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('nir_first_create_playoff_stage', 0x78c0a0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a First Division da Irlanda do Norte: a virada da temporada, a fase seguinte e a repescagem do acesso',
        'the Northern Irish First Division: the season turnover, the next stage and the promotion play-off',
        [('nir_first_start_new_season', 'NirFirst::start_new_season', '0x78be10', 'nir_first.cpp', 'thiscall, ret 0',
          'regra: a virada da First Division da Irlanda do Norte: as estatísticas arquivadas, a tabela e a tabela de rodadas limpas, o ano avança e as regras, a tabela, a tabela de jogos e a ordem são refeitas; depois as posições previstas, as expectativas das diretorias e a renovação dos árbitros',
          'rule: the Northern Irish First Division turnover: the statistics archived, the table and the round table cleared, the year moved on and the rules, the table, the fixtures and the order rebuilt; then the predicted positions, the board expectations and the referee renewal'),
         ('nir_first_advance_stage', 'NirFirst::advance_stage', '0x78bfd0', 'nir_first.cpp', 'thiscall, ret 0',
          'regra: a fase de liga da First Division da Irlanda do Norte só termina quando a Premier acima terminou: então as posições da Premier são atualizadas e a repescagem do acesso é montada; depois da última fase, a caixa de erro',
          'rule: the Northern Irish First Division\'s league stage ends only when the Premier Division above has finished: then the Premier\'s positions are updated and the promotion play-off is built; past the last stage, the error box'),
         ('nir_first_create_playoff_stage', 'NirFirst::create_playoff_stage', '0x78c0a0', 'nir_first.cpp', 'thiscall, ret 0',
          'regra: a repescagem da Irlanda do Norte: o vice da First Division contra o penúltimo da Premier (os clubes com a marca de repescagem em cada tabela), numa fase de dois clubes; sem um deles, não há repescagem',
          "rule: the Northern Irish promotion play-off: the First Division runner-up against the Premier Division's second last (the clubs with the play-off status in each table), in a two-club stage; without one of them, no play-off")])
CAMPOS = [('linha da tabela da liga +0x37 a marca da faixa (1 repescagem na divisão de baixo, 2 repescagem na de cima) [inferência pelo uso]; valor; lê',
           'league table row +0x37 the band status (1 play-off in the lower division, 2 play-off in the upper one) [inference from use]; value; reads')]
INTERESSA = []
