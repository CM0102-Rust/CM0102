# A Premier Division escocesa (sco_prm.cpp): o calendário. Leitor mwb (12/11/2026), lista do mundo (Escócia e País de
# Gales).
# - 0x7f0e80 (ScoPrm::create_schedule): 38 rodadas (33 nos três turnos de 29/7 a 7/4, o bloco do fim do ano pelo dia da
#   21a rodada: 7 casos e o padrão depois da caixa) e as duas fases da divisão (0 e 1: 5 rodadas de 14/4 a 12/5, rótulo
#   0x44b/0x44c); em 1999, 36 rodadas e nenhuma divisão. Defeitos do original: a rodada 34 grava a faixa de TV (5, 2, 2)
#   e por cima a marca do fim (fica sem faixa); em 1999 os casos do fim do ano gravam só a rodada 21 e a 22 fica com o
#   que o malloc deixou.
# Convenções conferidas: thiscall, ret 0x10 (0x68a160/0x68a1c0 cdecl; 0x549f70 push -1; push ano; push 0xb; push 1; lea
# ecx -> thiscall, ret 0x10).
# Comparador: lado a lado (compare_league_round_table, as rodadas de 0x41 bytes também nas fases da divisão).
# As fases da divisão são pedidas pela 0x7f2d90 (mwb_15) em abril de 2002, quando todos jogaram 33.
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('sco_prm_create_schedule', 0x7f0e80, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('o calendário da Premier Division escocesa',
        'the Scottish Premier Division schedule',
        [('sco_prm_create_schedule', 'ScoPrm::create_schedule', '0x7f0e80', 'sco_prm.cpp', 'thiscall, ret 0x10',
          'regra: 38 rodadas (12 clubes): 33 em três turnos de 29/7 a 7/4 do ano seguinte, aos sábados com a faixa de TV (5, 2, 2) na maioria delas e três no meio da semana (17/10, 29/11, 2/1), o bloco do fim do ano de duas rodadas escolhido pelo dia em que cai a 21a; depois a divisão em dois grupos de seis (fases 0 e 1: cinco sábados de 14/4 a 12/5); em 1999 (10 clubes, 4 turnos) 36 rodadas e nenhuma divisão',
          'rule: 38 rounds (12 clubs): 33 in three rounds from 29/7 to 7/4 of the next year, Saturdays with the TV slot (5, 2, 2) in most of them and three midweek (17/10, 29/11, 2/1), the year-end block of two rounds chosen by the day the 21st falls on; then the split into two groups of six (stages 0 and 1: five Saturdays from 14/4 to 12/5); in 1999 (10 clubs, 4 rounds) 36 rounds and no split')])
