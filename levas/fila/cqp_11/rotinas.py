# O calendário das copas de clubes da Ásia e do Troféu da Liga inglesa: a Liga dos Campeões e a Recopa asiáticas e o
# LDV Vans Trophy (o slot +0x3c create_round_table; asia_club_champ.cpp, asia_cup_winner.cpp em eurocomp\,
# eng_auto_cup.cpp em cups\). Frente das competições (Agente D, 06/11/2026), carreira de 8 ligas (Japão e Inglaterra
# jogáveis; começa em janeiro de 2001).
# - 0x40ae90 (AsiaClubChamp::create_round_table, 1481 bytes; 5 chamadas até 2002-06-01 com as 8 ligas): as
#   eliminatórias, os grupos finais e a fase final. Uma mutação.
# - 0x40cf90 (AsiaCupWinner::create_round_table, 1229 bytes; 3 chamadas): as eliminatórias e a fase final da Recopa
#   asiática. Uma mutação.
# - 0x56ade0 (EngAutoCup::create_round_table, 1221 bytes; 3 chamadas): as chaves do Norte e do Sul e a final. Uma
#   mutação.
# As três montam a tabela por fase com as mesmas operações (as datas de grupo pela 0x68a160 e os jogos pela 0x68a1c0;
# o sorteio e o jogo de copa pela 0x521e60/0x521eb0; os confrontos fixos pela 0x521f10; e as gravações dos campos):
# o cqp11_tabela executa a lista de operações de cada fase, extraída da desmontagem (na ordem da original), e o
# comparador confere os bytes que cada operação grava.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três são a virtual +0x3c (push saida; push &extra; push &rodadas; push fase;
# mov ecx, copa -> thiscall, ret 0x10; a fase lida como byte com sinal). Dentro: 0x944e46 (push tamanho; add esp, 4 ->
# cdecl), 0x68a160 (9 pushes -> cdecl; o ano num registrador com lixo nos 16 bits altos, a 0x68a160 usa só a palavra:
# a cópia passa limpo), 0x68a1c0 (7 pushes -> cdecl), 0x521e60 (7 pushes -> cdecl), 0x521eb0 (9 pushes -> cdecl),
# 0x521f10 (7 pushes; add esp junto -> cdecl: tabela, rodada, vaga, dia da semana, horário, código, mando) e a caixa
# (aviso_em). O extra é gravado sem conferir o ponteiro, menos na fase final asiática (conferido, como na original).
# Comparadores: lado a lado (a original com as saídas de quem chama, a cópia com saídas próprias que começam iguais; o
# [0xb67a34]); as saídas e os bytes gravados pelas operações nas duas tabelas têm de ser iguais; a tabela da cópia é
# liberada. Fora das fases só a original (nenhuma grava nada).
# Determinismo: nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqp11_asia_clubes_tabela', 0x40ae90, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqp11_asia_recopa_tabela', 0x40cf90, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqp11_trofeu_tabela', 0x56ade0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o calendário das copas de clubes da Ásia e do Troféu da Liga inglesa: a Liga dos Campeões e a Recopa asiáticas e o LDV Vans Trophy',
        'the calendar of the Asian club cups and the English League Trophy: the Asian Club Championship and Cup Winners\' Cup and the LDV Vans Trophy',
        [('cqp11_asia_clubes_tabela', 'AsiaClubChamp::create_round_table', '0x40ae90', 'asia_club_champ.cpp', 'thiscall, ret 0x10',
          'regra: a Liga dos Campeões asiática tem, em cada zona (fases −1 e 0, ids 0x480 e 0x481), duas eliminatórias em ida e volta (1ª sorteada em 1/8 e jogada em 23/8, 2ª em 14/9 e 20/9; 21 dias entre os jogos na 1ª, 7 na 2ª); os grupos finais (fases 1 e 2, ids 0x482 e 0x483) de três rodadas em 21, 23 e 25 de março do ano seguinte com os confrontos fixos (os jogos 0 e 1 de cada rodada); a fase final (fase 3) em maio: semifinal em 24/5, terceiro lugar e final em 26/5, em jogo único (desempate 0x383 e 0x83); categoria 4 nas eliminatórias e 6 depois',
          'rule: the Asian Club Championship has, in each zone (stages −1 and 0, ids 0x480 and 0x481), two two-legged qualifying rounds (1st drawn on 1/8 and played on 23/8, 2nd on 14/9 and 20/9; 21 days between legs in the 1st, 7 in the 2nd); the final groups (stages 1 and 2, ids 0x482 and 0x483) of three rounds on 21, 23 and 25 March of the following year with fixed pairings (matches 0 and 1 of each round); the final stage (stage 3) in May: semi-final on 24/5, third place and final on 26/5, single matches (tie-break 0x383 and 0x83); category 4 in the qualifiers and 6 after'),
         ('cqp11_asia_recopa_tabela', 'AsiaCupWinner::create_round_table', '0x40cf90', 'asia_cup_winner.cpp', 'thiscall, ret 0x10',
          'regra: a Recopa asiática tem, em cada zona (fases −1 e 0, ids 0x480 e 0x481), três eliminatórias em ida e volta: 1ª em 23/8, 2ª em 20/9, quartas em 24 de março do ano seguinte (21 dias entre os jogos na 1ª, 7 depois); a fase final (fase 1) em maio: semifinal em 24/5, terceiro lugar e final em 26/5, em jogo único; categoria 4 nas eliminatórias e 6 depois',
          'rule: the Asian Cup Winners\' Cup has, in each zone (stages −1 and 0, ids 0x480 and 0x481), three two-legged qualifying rounds: 1st on 23/8, 2nd on 20/9, quarter-finals on 24 March of the following year (21 days between legs in the 1st, 7 after); the final stage (stage 1) in May: semi-final on 24/5, third place and final on 26/5, single matches; category 4 in the qualifiers and 6 after'),
         ('cqp11_trofeu_tabela', 'EngAutoCup::create_round_table', '0x56ade0', 'eng_auto_cup.cpp', 'thiscall, ret 0x10',
          'regra: o LDV Vans Trophy tem duas chaves (fases −1 e 0, Norte e Sul [inferência], ids 0x41d e 0x41e) de cinco rodadas em jogo único: 1ª em 10/12 (24 times), 2ª em 14/1 (16 times, 4 novos), quartas em 4/2, semifinal em 18/2 e a final da chave em ida e volta a partir de 4/3 (7 dias); a final (fase 1) sorteada em 12/3 e jogada em 20/4 do ano seguinte com mando 4 (estádio neutro [inferência]); categoria 4 nas chaves e 6 nas finais; sem prêmios por rodada',
          'rule: the LDV Vans Trophy has two sections (stages −1 and 0, North and South [inference], ids 0x41d and 0x41e) of five single-match rounds: 1st on 10/12 (24 teams), 2nd on 14/1 (16 teams, 4 new), quarter-finals on 4/2, semi-final on 18/2 and the section final over two legs from 4/3 (7 days); the final (stage 1) drawn on 12/3 and played on 20/4 of the following year with venue 4 (neutral ground [inference]); category 4 in the sections and 6 in the finals; no per-round prizes')])
CAMPOS = [('copa +0x40 ano da copa; valor (palavra com sinal); lê (as três)',
           'cup +0x40 cup year; value (signed word); reads (all three)'),
          ('rodada de copa (0x68 bytes) +0..+0x22 (com os confrontos fixos em +0x23 + 7 × vaga) e +0x5c..+0x67; valor; grava (mata-mata)',
           'cup round (0x68 bytes) +0..+0x22 (with the fixed pairings at +0x23 + 7 × slot) and +0x5c..+0x67; value; writes (knockout)'),
          ('rodada de grupo (0x41 bytes) +0 dia do ano, +2 ano relativo, +4 código, +5 + 7 × jogo os jogos, +0x3d valor; valor; grava (grupos asiáticos)',
           'group round (0x41 bytes) +0 day of year, +2 relative year, +4 code, +5 + 7 × match the matches, +0x3d value; value; writes (Asian groups)')]
INTERESSA = ['CMRTE']
