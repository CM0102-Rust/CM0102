# Os grupos semifinais e o grupo final das eliminatórias da CONCACAF (wc_concacaf_cup.cpp; leitor rx11, rodada 6,
# 25/11/2026). Simulação na campanha de 2006 (2004 e 2005 na carreira de teste): LIGAS = 'ENG' e ATE = '2005-12-31',
# como os cx16_7..12. Chamadas pela advance_stage do cx16_10 (fases 3 e 6).
# - 0x917320 WcConcacafCup::create_semi_final_groups thiscall ret 0 (quadro SEH só para os new): malloc(0x10) (caixa
#   0x65a); os três primeiros com situação 1 da tabela de times da copa (+0xa7) e da repescagem (+0xc[2]); para os
#   grupos g = +0x30 (byte), g+1, g+2: a vaga 0 a próxima cabeça das vagas 25..34 da lista +0xb2 (zerada), as vagas 1 e
#   2 os sobreviventes, a vaga 3 o primeiro do grupo 0 (A), a vaga da lista no índice +0x3e da copa (B, defeito
#   [inferência]) e o primeiro do grupo 1 (C); a tabela (virtual +0x3c), new 0xee e League_Stage 0x68b100 (4, os times,
#   2, o registro, a tabela, rodadas, 3, 1, 0xa, {1,2,3,0}, {2,0,0,0}, o ano, o índice, o rótulo, 0xf, 4, 0, 0x28, -1,
#   0, 2); nulo: caixas 0x698/0x6c4/0x6f3 e fim; A e B: free(tabela) e a notícia do sorteio; C: free dos dois, +0x30 =
#   g+2 e a notícia. Prólogo push -1; push imm32 (7 bytes).
# - 0x917830 WcConcacafCup::create_final_group thiscall ret 0: malloc(0x18) (caixa 0x723); as linhas 0 e 1
#   (League::get_table_row 0x6826d0) dos grupos +0x30-3..+0x30-1; League_Stage de 6 em +0xc[+0x30] com {3 - +0xba, 0,
#   0, 0}, 0x14 e 0x24; free dos dois; nulo: caixa 0x742. Sem notícia. Prólogo push -1; push imm32 (7 bytes).
# Iguais no cm0102-1080.exe. Comparadores: só a original (montam fases e tiram as cabeças da lista), com o plano: cada
# grupo com os seus quatro (ou seis) como conjunto e a fase atual.
SAVE = False
LIGAS = 'ENG'
ATE = '2005-12-31'
ROTINAS = [('wc_concacaf_cup_create_semi_final_groups', 0x917320, [0x6a, 0xff, 0x68, 0xfa, 0x58, 0x96, 0x00]),
           ('wc_concacaf_cup_create_final_group', 0x917830, [0x6a, 0xff, 0x68, 0x1e, 0x59, 0x96, 0x00])]
DOCS = ('os grupos semifinais e o grupo final das eliminatórias da CONCACAF',
        'the CONCACAF qualifying semi-final groups and final group',
        [('wc_concacaf_cup_create_semi_final_groups', 'WcConcacafCup::create_semi_final_groups', '0x917320', 'wc_concacaf_cup.cpp', 'thiscall, ret 0',
          'regra: as semifinais da CONCACAF são três grupos de quatro em turno e returno, com dois classificados por grupo: cada um tem a próxima cabeça de chave, um sobrevivente do mata-mata caribenho, um da repescagem e uma seleção centro-americana; cada grupo tem a notícia do sorteio',
          'rule: the CONCACAF semi finals are three round robin groups of four, two qualifying from each: each takes the next seeded nation, one Caribbean knockout survivor, one play-off survivor and a Central American nation; each group gets its draw news'),
         ('wc_concacaf_cup_create_final_group', 'WcConcacafCup::create_final_group', '0x917830', 'wc_concacaf_cup.cpp', 'thiscall, ret 0',
          'regra: o grupo final da CONCACAF é um turno e returno de seis, os dois primeiros de cada grupo semifinal, com três vagas para a Copa menos as seleções isentas',
          'rule: the CONCACAF final group is a round robin of six, the top two of each semi-final group, with three World Cup places less the exempt nations')])
CAMPOS = []
INTERESSA = []
