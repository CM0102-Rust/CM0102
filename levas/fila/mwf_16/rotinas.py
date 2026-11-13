# O acesso e o rebaixamento na Itália, a troca com a Série C2 sem ela carregada e o reagrupamento da Série C1
# (ita_ser_a.cpp). Leitor mwf (12/11/2026), rodada 2 (o mundo até 2002-08-01; docs/cobertura_mundo.txt: cada uma 1
# vez, na virada italiana de meados de 2002; a troca sem a C2 rodou, então a carreira do mundo não tem a C2).
# - 0x642410 (ItaSerA::process_promotion_relegation_chain, a virtual +0xb0): as posições finais de A, B, C1 A e C1 B;
#   4 sobem e 4 descem entre A e B; as trocas de B com cada grupo da C1; com a C2, as dela e as trocas de cada grupo
#   da C1 com cada grupo da C2 (C1 B com a = 0).
# - 0x641fc0 (ItaSerA::exchange_serie_c2_clubs_unplayed): os seis melhores da C2 (pela reputação, embaralhados de 6
#   em 6) sobem para a C1 A; os rebaixados dos dois grupos da C1 descem para a C2 A (a C1 B percorrida com o tamanho da
#   C1 A).
# - 0x642630 (ita_ser_a_regroup_serie_c1_clubs): os 36 da C1 ordenados de norte a sul (0x53fd40) e divididos: os 18
#   primeiros na C1 B, os outros na C1 A; o técnico humano de quem muda de grupo recebe a notícia 0x13. Percorre
#   sempre 36 entradas (com menos clubes lê lixo da pilha: defeito [inferência]; a cópia pula a entrada vazia).
# Convenções: a +0xb0 com push 1 -> thiscall, ret 4; 0x641fc0 e 0x642630 chamadas com mov ecx, esi e sem argumentos
# (ret 0); dentro: 0x944e46 (cdecl), 0x4afbd0 (3 pushes -> cdecl), 0x945333 (qsort, 4 pushes -> cdecl), 0x6826d0
# (thiscall, ret 4), 0x6830b0 e 0x6831a0 (thiscall, ret 0xc), 0x689c80 (thiscall, ret 0x18), 0x7875e0 (push club; mov
# ecx, [0xb7431c] -> thiscall, ret 4), 0x77bbe0 (push 0; push 0x13; lea ecx -> thiscall, ret 8), 0x785790 (thiscall,
# ret 8), 0x7861e0 (thiscall, ret 8).
# Comparadores: a cadeia só a original; a troca só a original, conferindo depois que os rebaixados estão na C2 A; o
# reagrupamento só a original (manda notícias), conferindo o grupo de cada um dos 36 pelo plano (a mesma ordenação
# sobre a lista de antes).
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('ita_ser_a_process_promotion_relegation_chain', 0x642410, [0xa1, 0xfc, 0xad, 0xad, 0x00]),
           ('ita_ser_a_exchange_serie_c2_clubs_unplayed', 0x641fc0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('ita_ser_a_regroup_serie_c1_clubs', 0x642630, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o acesso e o rebaixamento na Itália, a troca com a Série C2 e o reagrupamento da Série C1',
        'the Italian promotion and relegation, the Serie C2 exchange and the Serie C1 regrouping',
        [('ita_ser_a_process_promotion_relegation_chain', 'ItaSerA::process_promotion_relegation_chain', '0x642410', 'ita_ser_a.cpp', 'thiscall, ret 4',
          'regra: o acesso e o rebaixamento na Itália: grava as posições finais da Série A, B, C1 A e C1 B; quatro sobem e quatro descem entre A e B; trocas entre a B e cada grupo da C1; com a C2 carregada, as posições dos seus três grupos e as trocas entre cada grupo da C1 e cada grupo da C2',
          'rule: promotion and relegation in Italy: records the final positions of Serie A, B, C1 A and C1 B; four up and four down between A and B; swaps between B and each C1 group; with Serie C2 loaded, its three groups\' positions and the swaps between each C1 group and each C2 group'),
         ('ita_ser_a_exchange_serie_c2_clubs_unplayed', 'ItaSerA::exchange_serie_c2_clubs_unplayed', '0x641fc0', 'ita_ser_a.cpp', 'thiscall, ret 0',
          'regra: sem a Série C2 carregada, os seis melhores clubes da C2 (pela reputação, embaralhados de seis em seis) sobem para a C1 A e os rebaixados dos dois grupos da C1 descem para a C2 A',
          'rule: without Serie C2 loaded, the six best C2 clubs (by reputation, shuffled in sixes) go up to C1 A and the clubs relegated from both C1 groups go down to C2 A'),
         ('ita_ser_a_regroup_serie_c1_clubs', 'ita_ser_a_regroup_serie_c1_clubs', '0x642630', 'ita_ser_a.cpp', 'thiscall, ret 0',
          'regra: os 36 clubes da Série C1 são reagrupados de norte a sul (pela cidade do estádio): os 18 primeiros na C1 B e os outros na C1 A; o técnico humano de um clube que muda de grupo recebe a notícia',
          'rule: the 36 Serie C1 clubs are regrouped from north to south (by their stadium\'s city): the first 18 into C1 B and the others into C1 A; the human manager of a club that changes group gets the news')])
CAMPOS = [('clube +0x57 (divisão: o grupo da C1), +0x5b; linha da tabela +0x37 (status 3 = rebaixado); valor/ponteiro; lê/grava',
           'club +0x57 (division: the C1 group), +0x5b; table row +0x37 (status 3 = relegated); value/pointer; reads/writes')]
