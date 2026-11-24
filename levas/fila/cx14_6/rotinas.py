# As fases da segunda divisao turca categoria B: os quatro outros grupos da primeira fase, o grupo de acesso e os cinco
# grupos de classificacao (tur_second_b.cpp; leitor cx14, rodada 5, 24/11/2026). Simulacao com a Turquia (as ligas e
# a data dos itens mwd): os grupos na criacao do jogo e na virada (8 chamadas na cobertura do mundo), o grupo de
# acesso e os de classificacao no fim da primeira fase (primavera de 2002, 1 chamada cada, pelo advance_stage do
# mwd_31).
# - 0x900780 TurSecondB::create_group_stage thiscall(grupo) ret 4 (le so o byte): a tabela de rodadas pela virtual
#   +0x3c (grupo, &rodadas, &extra, 0); malloc 0x28 (caixa 0x299); os clubes com divisao reserva da chave do grupo
#   (0 -> 0x9cf8e8, 1 -> 0x9cf8ec, 2 -> 0x9cf8f0, outro -> 0x9cf8f4) e divisao = a competicao; o 11o em diante: caixa
#   0x2aa e a divisao zerada (defeito do original, como no cx09_13); menos de 10: caixa 0x2b7. League_Stage
#   0x68b100 (thiscall, 22 argumentos, ret 0x58) com os lugares {0, 2, 0, 0} num local, a importancia (+0x51) e os
#   bits (+0xd9) da liga; sem a fase, caixa 0x2c2; senao free da lista e da tabela e fase atual = grupo.
# - 0x900ab0 TurSecondB::create_promotion_group thiscall() ret 0: as linhas de status 1 dos grupos -1..3 num local
#   de 10 (o 11o: caixa 0x2f2); a fase 4 com os lugares {2, 3, 0, 0}, importancia 0x14 e bits 3; sem a fase, caixa
#   0x300; senao free da tabela e fase atual = 4.
# - 0x900d00 TurSecondB::create_classification_groups thiscall() ret 0: para g = -1..3 as linhas de status != 1 num
#   local de 8 (caixa 0x333), a fase g + 6 com 8 clubes, lugares {0, 1, 0, 2}, importancia 0x14 e bits 2; sem a
#   fase, caixa 0x340 e fim; senao free da tabela, fase atual e, para as 8 linhas novas, os 42 bytes de +0xd da linha
#   do clube no grupo da primeira fase (rep movsd de 10 + movsw) com +0x11, +0x1f e +0x2d zerados.
# Os pushes dos construtores conferidos um a um (o ctor recebe as rodadas, o extra, a importancia e os bits como dwords
# com restos da pilha acima: le a palavra / o byte, a mesma simplificacao do cx09_13). Prologos: push -1; push imm32
# (7 bytes). Iguais no cm0102-1080.exe (inteiras). Comparadores: so a original (alocam), os clubes de cada fase contra
# o plano (como conjunto), a fase atual e, nos grupos de classificacao, o registro levado de cada linha.
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('tur_second_b_create_group_stage', 0x900780, [0x6a, 0xff, 0x68, 0xae, 0x50, 0x96, 0x00]),
           ('tur_second_b_create_promotion_group', 0x900ab0, [0x6a, 0xff, 0x68, 0xce, 0x50, 0x96, 0x00]),
           ('tur_second_b_create_classification_groups', 0x900d00, [0x6a, 0xff, 0x68, 0xee, 0x50, 0x96, 0x00])]
DOCS = ('as fases da segunda divisão turca categoria B: os grupos da primeira fase, o grupo de acesso e os grupos de classificação',
        'the stages of the Turkish second division category B: the first phase groups, the promotion group and the classification groups',
        [('tur_second_b_create_group_stage', 'TurSecondB::create_group_stage', '0x900780', 'tur_second_b.cpp', 'thiscall, ret 4',
          'regra: os grupos 2 a 5 da primeira fase (o grupo 1 é a tabela da própria liga): os dez clubes da categoria B de cada grupo, pela ordem do banco de dados, em turno e returno; dois de cada grupo vão adiante; um décimo primeiro perde a divisão (caixa de erro)',
          'rule: first phase groups 2 to 5 (group 1 is the league\'s own table): the ten category B clubs of each group, in database order, home and away; two of each group go through; an eleventh loses its division (error box)'),
         ('tur_second_b_create_promotion_group', 'TurSecondB::create_promotion_group', '0x900ab0', 'tur_second_b.cpp', 'thiscall, ret 0',
          'regra: o grupo de acesso: os dez que passaram da primeira fase (dois de cada grupo), em turno e returno; dois sobem e três vão ao mata-mata',
          'rule: the promotion group: the ten through from the first phase (two from each group), home and away; two go up and three go to the play-off'),
         ('tur_second_b_create_classification_groups', 'TurSecondB::create_classification_groups', '0x900d00', 'tur_second_b.cpp', 'thiscall, ret 0',
          'regra: os cinco grupos de classificação, um por grupo da primeira fase, com os oito que não passaram, em turno e returno, levando os pontos e gols da primeira fase (os jogos disputados zerados); o primeiro vai ao mata-mata e dois caem',
          'rule: the five classification groups, one for each first phase group, with the eight that did not go through, home and away, carrying the first phase points and goals (the games played cleared); the first goes to the play-off and two go down')])
CAMPOS = [('linha da tabela +0xd..+0x36 (os totais), levados da primeira fase', 'table row +0xd..+0x36 (the totals), carried from the first phase')]
INTERESSA = []
