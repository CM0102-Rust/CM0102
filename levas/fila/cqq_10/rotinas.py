# A Copa da UEFA e a Intertoto: as rodadas da Copa da UEFA, os inscritos da Intertoto e a lista dos seus clubes
# (uefa_cup.cpp, intertoto_cup.cpp, em eurocomp\). Frente das competições (Agente D, 06/11/2026), carreira de 8 ligas
# (começa em janeiro de 2001).
# - 0x9025c0 (UefaCup::create_round_table, 1483 bytes; 1 chamada até 2002-06-01 com as 8 ligas): as 8 ou 9 rodadas
#   (a preliminar só com +0xb2 > 0). Duas mutações.
# - 0x6336c0 (IntertotoCup::collect_entrants, 597 bytes; 1 chamada): os 60 inscritos. Uma mutação.
# - 0x633f70 (IntertotoCup::collect_club_list, 299 bytes; 3 chamadas): a lista dos clubes (passageira).
# Reaproveita do motor.rs os ajudantes da 276a (cql10_rodada, cql10_gravados, cql10_comparar, Cql10R): a tabela da UEFA
# tem o corpo das copas inglesas, com a preliminar e quatro campos calculados de +0xb2 (extraídos da desmontagem).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): 0x9025c0 é a virtual +0x3c (push saida; push &extra; push &rodadas; push
# fase; mov ecx, copa -> thiscall, ret 0x10); 0x6336c0 é thiscall, ret 0 (mov ecx, esi; call 0x6336c0, no construtor e
# na virada); 0x633f70 é uma virtual thiscall, ret 8 (saída e o byte ativos). Dentro: 0x944e46 (push tamanho; add esp,
# 4 -> cdecl), 0x521e60/0x521eb0 pela cql10_rodada, 0x945333 (o qsort com 0x522190: 4 pushes; add esp junto ->
# cdecl), 0x9452ca (push; add esp -> cdecl) e a caixa (aviso_em).
# Comparadores: lado a lado. A tabela como as copas inglesas (cql10_comparar, com as rodadas que a original deixa em
# *rodadas). Os inscritos: o +0x3e e o [0xb67a34]; a lista da cópia comparada nos 60 inscritos e liberada. A lista dos
# clubes: a da cópia numa saída própria, comparada na conta e nos ponteiros e liberada.
# Determinismo: nada lê pilha, heap ou endereço como valor (o vetor local dos inscritos só é lido nas vagas gravadas).
# Provas (05-06/11/2026, cópia fora do projeto: motor.rs da principal com a 277a publicada e a 278a integrada + os 12
# itens cqo_10..13, cqp_10..13, cqq_10..13, 39 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2001-10-01 (parou em 2001-273): 0 linhas DIVERGENCIA fora
#   da cqo11_copa_c_inscritos (8 dezenas embaralhadas, não 9: corrigida e conferida de novo até 2001-01-05, 0
#   divergências), 0 EXCECAO, nenhuma caixa;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão b690295afd222f6a; os contadores das mutações rodaram;
# - mutações (carreira Rust de 8 ligas até 2002-06-01 contra a base f29c85195c2a534f): as três ACUSADAS (a 1a rodada
#   de qualificação da UEFA em 2001-218, a rodada de novembro em 2001-314, a classe dos inscritos da Intertoto em
#   2001-174). A lista dos clubes da Intertoto é passageira (roda: contador 2); a preliminar da UEFA não roda (+0xb2 = 0).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqq10_uefa_tabela', 0x9025c0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqq10_intertoto_inscritos', 0x6336c0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('cqq10_intertoto_lista', 0x633f70, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Copa da UEFA e a Intertoto: as rodadas da Copa da UEFA, os inscritos da Intertoto e a lista dos seus clubes',
        'the UEFA Cup and the Intertoto Cup: the UEFA Cup rounds, the Intertoto entrants and its club list',
        [('cqq10_uefa_tabela', 'UefaCup::create_round_table', '0x9025c0', 'uefa_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa da UEFA tem, só se há clubes da preliminar (+0xb2, os de coeficiente 10 ou mais [inferência]), uma preliminar sorteada em 28/6 e jogada em 6/7 (b times); depois: 1ª fase de qualificação sorteada em 28/7 e jogada em 10/8 (82 times, 0x52 − b/2 novos), 2ª em 14/9 (96 times, 55 novos), 1ª rodada em 19/10 (48 times), 2ª em 23/11 (32 times, 8 novos), 3ª em 1/3 do ano seguinte, oitavas em 15/3, quartas em 5/4 e a final em jogo único em 16/5 (mando 4, prêmio de 1,5 milhão ao campeão); ida e volta com 14 dias entre os jogos (7 na 3ª rodada e nas oitavas); categoria 7 na qualificação e 8 depois',
          'rule: the UEFA Cup has, only when there are preliminary clubs (+0xb2, those with coefficient 10 or more [inference]), a preliminary round drawn on 28/6 and played on 6/7 (b teams); then: 1st qualifying round drawn on 28/7 and played on 10/8 (82 teams, 0x52 − b/2 new), 2nd on 14/9 (96 teams, 55 new), 1st round on 19/10 (48 teams), 2nd on 23/11 (32 teams, 8 new), 3rd on 1/3 of the following year, round of 16 on 15/3, quarter-finals on 5/4 and the single-match final on 16/5 (venue 4, prize of 1.5 million to the winner); two legs 14 days apart (7 in the 3rd round and the round of 16); category 7 in qualifying and 8 after'),
         ('cqq10_intertoto_inscritos', 'IntertotoCup::collect_entrants', '0x6336c0', 'intertoto_cup.cpp', 'thiscall, ret 0',
          'regra: entram na Intertoto os 60 clubes que as regras europeias puseram nela (copa europeia +0x1db = a Intertoto), pela ordem da tabela de clubes; cada um com a classe = o seu coeficiente (+0x1df), e a lista é ordenada pela classe (as sementes); com menos de 60, a caixa de erro',
          'rule: the Intertoto Cup takes the 60 clubs the European rules put in it (European cup +0x1db = the Intertoto), in club table order; each with class = its coefficient (+0x1df), and the list is sorted by class (the seeds); with fewer than 60, the error box'),
         ('cqq10_intertoto_lista', 'IntertotoCup::collect_club_list', '0x633f70', 'intertoto_cup.cpp', 'thiscall, ret 8',
          'regra: a lista dos clubes da Intertoto (para quem chama): todos os inscritos, ou só os ainda vivos (situação 0 ou 1) quando pedido',
          'rule: the Intertoto club list (for the caller): all entrants, or only those still in (status 0 or 1) when asked')])
CAMPOS = [('copa +0x40 ano, +0xb2 clubes da preliminar (byte com sinal); valor; lê (tabela da UEFA)',
           'cup +0x40 year, +0xb2 preliminary clubs (signed byte); value; reads (UEFA table)'),
          ('clube +0x1db copa europeia (id), +0x1df coeficiente europeu (byte) [inferência]; id/valor; lê (inscritos)',
           'club +0x1db European cup (id), +0x1df European coefficient (byte) [inference]; id/value; reads (entrants)'),
          ('copa +0x3e inscritos, +0xa7 lista (6 bytes: clube, +4 classe, +5 situação); valor/ponteiro; grava (inscritos) e lê (lista)',
           'cup +0x3e entrants, +0xa7 list (6 bytes: club, +4 class, +5 status); value/pointer; writes (entrants) and reads (list)'),
          ('[0x9cf6f8] a Intertoto; id; lê', '[0x9cf6f8] the Intertoto; id; reads')]
INTERESSA = ['CMRTE']
