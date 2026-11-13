# A Premier Division escocesa (sco_prm.cpp): a virada de toda a liga escocesa, a atualização do dia e a divisão em dois
# grupos de seis (o split). Só usa o que já está no motor: independente dos outros itens mwb. Leitor mwb (12/11/2026),
# lista do mundo (Escócia e País de Gales).
# - 0x7f2a70 (ScoPrm::start_new_season): a virada da liga escocesa (sobe e desce, o teto de divisão dos clubes da Second e
#   da Third, a virada da First, da Second e da Third); em 1999 Elgin City e Peterhead entram na Third. Roda uma vez, no
#   meio de 2002.
# - 0x7f2d20 (ScoPrm::update_for_date): roda todo dia (2308 vezes); o split quando todos jogaram 33 (abril de 2002).
# - 0x7f2d90 (ScoPrm::create_split_stages): os seis de cima e os seis de baixo ganham cada um uma fase (League_Stage
#   0x68b100) que é destruída logo depois de criada (os jogos dela ficam); depois a liga passa a 4 turnos com a
#   comparação do split (0x7f3220). Roda uma vez.
# Convenções conferidas: a virada thiscall, ret 0 (a virtual +0xb0 push 0 -> ret 4; 0x68a980 thiscall, ret 0x10; as
# virtuais +8, +0x8c, +0x5c thiscall, ret 0; 0x7f2820 thiscall, ret 0); a do dia thiscall, ret 8 (0x6847c0 com os
# mesmos dois argumentos, ret 8; a original devolve o eax dela inteiro); o split thiscall, ret 0 (a virtual +0x3c push
# 0; push &extra; push &rodadas; push fase -> ret 0x10; 0x944cf1 cdecl; 0x68b100 thiscall com 21 argumentos, ret 0x58;
# a virtual 0 push 1 -> ret 4; 0x9452ca cdecl).
# Comparadores: a virada só a original (o ano subiu um); a do dia só a original, com o plano (o split devido deixa 4
# turnos e a comparação 0x7f3220); o split só a original, conferindo o mesmo.
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('sco_prm_start_new_season', 0x7f2a70, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('sco_prm_update_for_date', 0x7f2d20, [0x8b, 0x44, 0x24, 0x08, 0x55]),
           ('sco_prm_create_split_stages', 0x7f2d90, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a Premier Division escocesa: a virada, o dia e o split',
        'the Scottish Premier Division: the turnover, the day update and the split',
        [('sco_prm_start_new_season', 'ScoPrm::start_new_season', '0x7f2a70', 'sco_prm.cpp', 'thiscall, ret 0',
          'regra: a virada escocesa começa pela Premier: sobe e desce entre as divisões; em 1999 Elgin City e Peterhead entram na Third (a ampliação de 2000); a Premier passa ao ano seguinte e refaz regras, tabela, jogos e ordem; o teto de divisão dos clubes da Second e da Third; a virada da First, da Second e da Third; as posições previstas, as expectativas da diretoria e a renovação dos árbitros',
          'rule: the Scottish turnover is led by the Premier: promotion and relegation between the divisions; in 1999 Elgin City and Peterhead join the Third (the 2000 expansion); the Premier moves to the next year and rebuilds the rules, the table, the fixtures and the order; the division tier of the Second and Third Division clubs is capped; the First, the Second and the Third turn over; the predicted positions, the board expectations and the renewal of the referees'),
         ('sco_prm_update_for_date', 'ScoPrm::update_for_date', '0x7f2d20', 'sco_prm.cpp', 'thiscall, ret 8',
          'regra: na temporada de 3 turnos, num dia com resultados, quando todos os clubes jogaram 33 vezes a Premier se divide (split); depois a atualização do dia da liga',
          'rule: in the season of 3 rounds, on a day with results, when every club has played 33 games the Premier splits; then the league day update'),
         ('sco_prm_create_split_stages', 'ScoPrm::create_split_stages', '0x7f2d90', 'sco_prm.cpp', 'thiscall, ret 0',
          'regra: o split: os seis primeiros e os seis últimos (do último para cima) jogam mais cinco rodadas entre si; depois a liga segue em 4 turnos com a comparação que mantém as duas metades separadas na tabela',
          'rule: the split: the top six and the bottom six (from the last up) play five more rounds among themselves; then the league goes on in 4 rounds with the compare that keeps the two halves apart in the table')])
