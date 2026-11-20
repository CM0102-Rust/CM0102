# Os calendários da Liga da Irlanda (ire_prm.cpp e ire_first.cpp, em leagues\): as rodadas das duas divisões e o
# play-off de acesso. Fecham os dois arquivos com o mwa_11, mwa_12, mwa_14 e mwa_15 (a lista não tem mais nada deles).
# Leitor r4_mwab (20/11/2026), lista do mundo (Irlanda e Irlanda do Norte), continuação.
# - 0x639e90 (IrePrm::create_schedule) e 0x634f40 (IreFirst::create_schedule): o mesmo corpo da liga (lido com o emu
#   dos caminhos de 36 e 34 rodadas e dos dois casos de Natal: as rodadas das duas são iguais, só as linhas das caixas
#   mudam, 0xad/0x156 e 0xb5/0x15e); 36 rodadas salvo LEGS (+0x3c) = 3 (34: sem 17/8 e 8/2); o Natal por Date::Date
#   (0x549f70, thiscall, ret 0x10): 25/12 sem dia da semana contra 26/12 no dia 6, depois 24/12: igual, 22/12 no meio da
#   semana, senão 26/12. Só a IreFirst tem a fase 0 (o play-off: uma rodada de 0x68 bytes, linha 0x171).
# Convenções conferidas: thiscall, ret 0x10 (0x68a160 e 0x68a1c0 cdecl; 0x521e60 e 0x521eb0 cdecl).
# Comparadores: lado a lado (compare_c1_schedule: as saídas e os bytes escritos da tabela).
# Quando roda: na criação da carreira (temporada 2001/02, de 15/8/2001 a 23/4/2002) e na virada de 2002; o play-off na
# fase 0 quando a Premier acaba (abril de 2002).
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('ire_prm_create_schedule', 0x639e90, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('ire_first_create_schedule', 0x634f40, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os calendários da Liga da Irlanda: as duas divisões e o play-off de acesso',
        "the League of Ireland's schedules: both divisions and the promotion play-off",
        [('ire_prm_create_schedule', 'IrePrm::create_schedule', '0x639e90', 'ire_prm.cpp', 'thiscall, ret 0x10',
          'regra: a Premier da Irlanda joga 36 rodadas (34 quando os clubes se enfrentam 3 vezes), de 15 de agosto a 23 de abril do ano seguinte, quase todas aos sábados [inferência: dia 6], com duas rodadas no meio da semana numa temporada de 36 (17 de agosto e 8 de fevereiro) e uma em 7 de março; a rodada de Natal é em 26 de dezembro, ou no meio da semana em 22 de dezembro quando o Natal ou a véspera caem no sábado; só a fase de liga',
          "rule: the League of Ireland Premier Division plays 36 matchdays (34 when the clubs meet 3 times), from 15 August to 23 April of the next year, almost all on Saturdays [inference: weekday 6], with two midweek rounds in a 36-round season (17 August and 8 February) and one on 7 March; the Christmas round is on 26 December, or midweek on 22 December when Christmas or Christmas Eve is the Saturday; only the league stage"),
         ('ire_first_create_schedule', 'IreFirst::create_schedule', '0x634f40', 'ire_first.cpp', 'thiscall, ret 0x10',
          'regra: a Primeira Divisão da Irlanda joga nas datas da Premier (36 rodadas, ou 34); depois o play-off de acesso contra o clube de play-off da Premier: ida e volta a partir de 27 de abril (sorteio em 24 de abril), categoria 5, sem prêmio',
          "rule: the League of Ireland First Division plays on the Premier Division's dates (36 matchdays, or 34); then the promotion play-off against the Premier Division's play-off club: two legs from 27 April (drawn on 24 April), category 5, no prize")])
CAMPOS = []
INTERESSA = []
