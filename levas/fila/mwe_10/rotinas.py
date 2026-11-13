# The stage advance of the Swedish and Norwegian First Divisions and the Superettan promotion play-off (reader mwe,
# whole-world round, 12/11/2026). They run at the end of the 2001 Swedish and Norwegian seasons (October/November
# 2001): the First Division's advance is asked when its own league ends and waits for the Premier Division:
# - 0x88f310 SweFirst::advance_stage (208 bytes) and 0x790d50 NorFirst::advance_stage (208 bytes): the same body
#   (only the source, the line, the Premier key and the play-off routine change);
# - 0x88f3e0 SweFirst::create_play_off_stage (544 bytes): the Superettan 3rd against the Allsvenskan 12th.
# Equal in cm0102-1080.exe (whole).
# Conventions checked: advance_stage through the vtable (+0x28, mov ecx) -> thiscall, ret 0 (plain `ret`); the
# Premier Division's virtuals +0x10 (push 1; push 0 -> thiscall, ret 8, al) and +0x94 (no push -> thiscall, ret 0);
# the play-off routine direct (mov ecx, esi; call) -> thiscall, ret 0. Inside the play-off: malloc (cdecl), the
# virtual +0x3c (push 0; push &extra; push &rounds; push 0 -> thiscall, ret 0x10), operator new (cdecl),
# Cup_Stage::Cup_Stage 0x522e00 (15 pushes -> thiscall, ret 0x3c), free (cdecl), League::get_table_row 0x6826d0
# (push i -> thiscall, ret 4), Cup::set_up_round (push 0 -> thiscall, ret 4).
# Defect of the original kept: SweFirst::create_play_off_stage calls Cup::set_up_round on the first slot before
# checking it (with no stage, on a null object); a list slot with no club of that status keeps the malloc contents.
# Comparers: only the original. The advances: the current stage stays or goes up by one, and at 0 the stage exists;
# the play-off: the two planned clubs (as a set) in the stage, the count 2, the competition and +0x30 = 0.
# Determinism: the rounds, extra and year are pushed as dwords with garbage above the word; the Cup_Stage
# constructor reads the words (the copy passes them clean).
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('swe_first_advance_stage', 0x88f310, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('nor_first_advance_stage', 0x790d50, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('swe_first_create_play_off_stage', 0x88f3e0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a passagem de fase das segundas divisões sueca e norueguesa e o play-off de acesso da Superettan',
        'the stage advance of the Swedish and Norwegian First Divisions and the Superettan promotion play-off',
        [('swe_first_advance_stage', 'SweFirst::advance_stage', '0x88f310', 'swe_first.cpp', 'thiscall, ret 0',
          'depois da liga, a Superettan espera a Allsvenskan acabar: com ela completa, atualiza as situações da Allsvenskan, sobe a fase e, ao chegar à fase 0, cria o play-off de acesso; na última fase, a caixa de erro',
          'after the league, the Superettan waits for the Allsvenskan to finish: once it is complete, it updates the Allsvenskan statuses, moves up a stage and, on reaching stage 0, creates the promotion play-off; at the last stage, the error box'),
         ('nor_first_advance_stage', 'NorFirst::advance_stage', '0x790d50', 'nor_first.cpp', 'thiscall, ret 0',
          'a mesma regra na primeira divisão norueguesa, esperando a Tippeligaen',
          'the same rule in the Norwegian First Division, waiting for the Tippeligaen'),
         ('swe_first_create_play_off_stage', 'SweFirst::create_play_off_stage', '0x88f3e0', 'swe_first.cpp', 'thiscall, ret 0',
          'o play-off de acesso sueco: o 3º da Superettan (a situação de play-off de acesso) contra o 12º da Allsvenskan (a de play-off de rebaixamento), num confronto de copa de ida e volta com o calendário da própria liga',
          'the Swedish promotion play-off: the Superettan 3rd (the promotion play-off status) against the Allsvenskan 12th (the relegation play-off status), a two-legged cup tie on the league\'s own schedule')])
CAMPOS = []
INTERESSA = []
