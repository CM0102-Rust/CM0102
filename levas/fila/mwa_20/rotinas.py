# The schedules of the Northern Irish Premier and First Divisions (nir_prm.cpp, nir_first.cpp, in leagues\).
# Whole-world front, round 2 (reader mwa, 12/11/2026), career with ENG IRL NIR to 2002-08-01.
# - 0x78e330 (NirPrm::create_schedule, 2 calls: the rules at the creation and at the turnover) and 0x78b310
#   (NirFirst::create_schedule, 3 calls: the same and the play-off round). The 36 rounds read by a path-following
#   symbolic reader of the pushes (emu.py: one path per case of the Christmas jump table 0x78ec3c / 0x78bc34), checked
#   case by case: both share the first 22 rounds; after Christmas the First Division plays on weekday 6. The structure
#   is EngFirst::create_schedule's (eng_first_set_christmas_rounds: 1 December, the day of a reference round, k − 13),
#   with six cases and a default without the error box.
# Conventions checked at the call sites: the virtual +0x3c (push out; push &extra; push &rounds; push stage -> thiscall,
# ret 0x10). Inside: malloc 0x944e46 (cdecl), League round dates 0x68a160 and slots 0x68a1c0 (cdecl, 9 and 7 pushes),
# Date::Date 0x549f70 (4 pushes; lea ecx -> thiscall, ret 0x10), the cup round dates 0x521e60/0x521eb0 (cdecl), the
# error box.
# Comparers: side by side (compare_c1_schedule: the outputs and the written bytes of the league rounds, or of the cup
# round at stage 0).
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('nir_prm_create_schedule', 0x78e330, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('nir_first_create_schedule', 0x78b310, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('as tabelas de jogos da Premier e da First Division da Irlanda do Norte',
        'the schedules of the Northern Irish Premier and First Divisions',
        [('nir_prm_create_schedule', 'NirPrm::create_schedule', '0x78e330', 'nir_prm.cpp', 'thiscall, ret 0x10',
          'regra: a Premier da Irlanda do Norte joga 36 rodadas (dez clubes que se enfrentam quatro vezes), de 12 de agosto a 28 de abril do ano seguinte, quase sempre aos sábados, com algumas rodadas no meio da semana; as duas rodadas de Natal dependem do dia da semana da rodada de 16 de dezembro; só a fase de liga',
          'rule: the Northern Irish Premier Division plays 36 matchdays (ten clubs meeting four times), from 12 August to 28 April of the next year, mostly on Saturdays with a few midweek rounds; the two Christmas rounds depend on the weekday of the 16 December round; only the league stage'),
         ('nir_first_create_schedule', 'NirFirst::create_schedule', '0x78b310', 'nir_first.cpp', 'thiscall, ret 0x10',
          'regra: a First Division da Irlanda do Norte joga 36 rodadas, nas datas da Premier até o Natal e aos domingos depois [inferência: dia 6]; depois a repescagem contra o penúltimo da Premier, em ida e volta com sete dias de intervalo a partir de 5 de maio',
          "rule: the Northern Irish First Division plays 36 matchdays, on the Premier's dates up to Christmas and on Sundays after it [inference: weekday 6]; then the promotion play-off against the Premier's second last, two legs seven days apart from 5 May")])
CAMPOS = []
INTERESSA = []
