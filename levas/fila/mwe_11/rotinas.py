# The Norwegian promotion play-off and the Ykkonen stage advance and club progress (reader mwe, whole-world round,
# 12/11/2026). They run at the end of the 2001 Norwegian and Finnish seasons (September to November 2001):
# - 0x790e20 NorFirst::create_play_off_stage (608 bytes): the First Division 3rd against the Tippeligaen 12th (unlike
#   the Superettan's: the last row of each status, a box when the First Division has none, silent when the
#   Tippeligaen has none, Cup::set_up_round only with a stage, +0x30 not written);
# - 0x593f90 FinFirst::advance_stage (240 bytes): stage 1 = the play-off stages (0x594080), stage 3 = the Premier
#   Division play-off (0x594610) after the Veikkausliiga is complete;
# - 0x593e10 FinFirst::set_club_progress (384 bytes): the Comp_Stats progress codes (2v - 1 in the groups; 11/12 for
#   the Veikkausliiga club at stage 3; 15/17 at stage 2).
# Equal in cm0102-1080.exe (whole).
# Conventions checked: NorFirst::create_play_off_stage direct from NorFirst::advance_stage (mov ecx, esi; call) ->
# thiscall, ret 0; FinFirst::advance_stage through vtable +0x28 -> thiscall, ret 0; set_club_progress through vtable
# +0x68 with 5 pushes -> thiscall, ret 0x14 (its `ret 0x14` checked). Inside: malloc, operator new, free (cdecl),
# the virtuals +0x3c (ret 0x10), +0x10 (push 1; push 0, ret 8, al), +0x94 (ret 0), Cup_Stage::Cup_Stage 0x522e00 (ret
# 0x3c), League::get_table_row 0x6826d0 (ret 4), Cup::set_up_round (ret 4), Comp_Stats::find_club_entry 0x4a4850
# (push club -> thiscall, ret 4).
# Defects of the original kept: the list leaks when NorFirst::create_play_off_stage gives up; set_club_progress does
# not check the entry nor the club's division for null.
# Comparers: the play-off only the original (it allocates and draws), the planned clubs as a set; the advance only the
# original (the stage stays or goes up by one); the progress side by side on the 3 bytes of the entry written.
# Determinism: the Cup_Stage words are pushed as dwords with garbage above (the constructor reads the words).
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('nor_first_create_play_off_stage', 0x790e20, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('fin_first_advance_stage', 0x593f90, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('fin_first_set_club_progress', 0x593e10, [0x8b, 0x49, 0x08, 0x53, 0x56])]
DOCS = ('o play-off de acesso norueguês e a passagem de fase e o progresso dos clubes da Ykkönen',
        'the Norwegian promotion play-off and the Ykkönen stage advance and club progress',
        [('nor_first_create_play_off_stage', 'NorFirst::create_play_off_stage', '0x790e20', 'nor_first.cpp', 'thiscall, ret 0',
          'o play-off de acesso norueguês: o 3º da primeira divisão (a situação de play-off de acesso) contra o 12º da Tippeligaen (a de play-off de rebaixamento), num confronto de copa; sem o clube da primeira divisão, a caixa de erro; sem o da Tippeligaen, nada',
          'the Norwegian promotion play-off: the First Division 3rd (the promotion play-off status) against the Tippeligaen 12th (the relegation play-off status), a cup tie; without the First Division club, the error box; without the Tippeligaen one, nothing'),
         ('fin_first_advance_stage', 'FinFirst::advance_stage', '0x593f90', 'fin_first.cpp', 'thiscall, ret 0',
          'a Ykkönen avança de fase: depois dos grupos Norte e Sul, as fases de play-off (fase 1); a fase 2 espera a Veikkausliiga acabar e então cria o play-off contra o clube da primeira divisão (fase 3); na última fase, a caixa de erro',
          'the Ykkönen moves to its next stage: after the North and South groups, the play-off stages (stage 1); stage 2 waits for the Veikkausliiga to finish and then builds the play-off against the Premier Division club (stage 3); at the last stage, the error box'),
         ('fin_first_set_club_progress', 'FinFirst::set_club_progress', '0x593e10', 'fin_first.cpp', 'thiscall, ret 0x14',
          'o progresso do clube nas estatísticas: nos grupos a posição v vira 2v − 1 (os dois grupos intercalados); no play-off contra a Veikkausliiga o clube dela ganha 11 ou 12 (nas estatísticas da Veikkausliiga); nos play-offs da Ykkönen 1 vira 15 e 3 vira 17',
          'the club progress in the statistics: in the groups place v becomes 2v − 1 (the two groups interleaved); in the play-off against the Veikkausliiga its club gets 11 or 12 (in the Veikkausliiga statistics); in the Ykkönen play-offs 1 becomes 15 and 3 becomes 17')])
CAMPOS = []
INTERESSA = []
