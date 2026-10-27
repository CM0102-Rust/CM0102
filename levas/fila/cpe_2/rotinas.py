# Bloco das competições, a tabela da liga (league.cpp): somar o resultado na tabela (League::apply_result_to_table
# 0x685f60, 8.322 chamadas na carreira: a 0x685d30 a cada resultado, as cópias da grupo_decidido 0x6854a0 e a tabela ao
# vivo 0x7280e0) e os pontos do resultado (League::points_for_result 0x68aad0, a virtual +0x90 das ligas, também pelo
# desvio 0x43c4a0: os pontos de cada vitória). Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no
# jogo real comparando. Sem SEH, sem sorteio, sem caixa, locais todas gravadas antes de lidas. A 0x685f60 chama os
# pontos pelo mesmo ponteiro (a virtual +0x90 da competição de tipo 2 ou da liga, lida a cada vez; as três que existem,
# 0x68aad0, 0x43c4a0 e 0x66d770, só leem) e o confronto direto 0x6869a0 (Rust): comparador lado a lado pelas linhas
# da tabela, os confrontos de cada linha, +0x3d/+0x3e da partida e os dois pontos devolvidos; a 0x68aad0 pelo valor
# (o eax inteiro: os 3 bytes altos são os da partida). Em laço quente: a 0x685f60 (8 mil) e a 0x68aad0 (uma por
# vitória); correm o risco da 0x689ad0 (241ª), de o desvio mudar o resíduo de pilha que uma original lê depois.
SAVE = False
ROTINAS = [('aplicar_resultado_na_tabela', 0x685f60, [0x83, 0xec, 0x0c, 0x8b, 0xd1]),
           ('pontos_do_resultado', 0x68aad0, [0x8b, 0x44, 0x24, 0x08, 0x53])]
DOCS = ('a tabela da liga: somar o resultado e os pontos do resultado',
        'the league table: adding a result and the points for a result',
        [('aplicar_resultado_na_tabela', 'League::apply_result_to_table', '0x685f60', 'league.cpp', 'thiscall, ret 0x18',
          'acha as linhas do mandante e do visitante e soma o jogo em casa e fora (jogos, gols, vitória/empate/derrota, pela prorrogação e pelos pênaltis, marcando +0x3d/+0x3e), os pontos de vitória pela virtual +0x90 e os de empate +0xc3; refaz os totais, devolve os pontos das duas linhas e soma o confronto direto (0x6869a0)',
          'finds the home and away rows and adds the game at home and away (played, goals, win/draw/loss, by extra time and penalties, marking +0x3d/+0x3e), the win points by the +0x90 virtual and the draw points +0xc3; redoes the totals, returns both rows points and adds the head-to-head (0x6869a0)'),
         ('pontos_do_resultado', 'League::points_for_result', '0x68aad0', 'league.cpp', 'thiscall, ret 8',
          'os pontos de vitória (+0xc2) a quem venceu, 0 a quem perdeu; empatado, pela prorrogação (+0xab bit 2), pelos pênaltis (bits 0x21: os de empate +0xc3 a quem venceu) ou os de empate',
          'the win points (+0xc2) to the winner, 0 to the loser; when level, by extra time (+0xab bit 2), by penalties (bits 0x21: the draw points +0xc3 to the winner) or the draw points')])
CAMPOS = [('linha da tabela (0x3b bytes): +0 clube, +5 confrontos, +0x11..+0x1a totais (jogos, V, E, D, gols pró, contra, pontos), +0x1f..+0x28 em casa, +0x2d..+0x36 fora; lê e grava (aplicar_resultado_na_tabela)',
           'table row (0x3b bytes): +0 club, +5 head-to-head, +0x11..+0x1a totals (played, W, D, L, goals for, against, points), +0x1f..+0x28 home, +0x2d..+0x36 away; reads and writes (aplicar_resultado_na_tabela)'),
          ('partida: +0x1c/+0x20 clubes, +0x38 regras (2 prorrogação, 0x20/1 pênaltis), +0x43/+0x44 placar, +0x47/+0x48 prorrogação (0xff = sem), +0x49/+0x4a pênaltis; grava +0x3d/+0x3e (aplicar_resultado_na_tabela); League +0xab, +0xc2, +0xc3; lê (as duas)',
           'fixture: +0x1c/+0x20 clubs, +0x38 rules (2 extra time, 0x20/1 penalties), +0x43/+0x44 score, +0x47/+0x48 extra time (0xff = none), +0x49/+0x4a penalties; writes +0x3d/+0x3e (aplicar_resultado_na_tabela); League +0xab, +0xc2, +0xc3; reads (both)')]
INTERESSA = ['SGE', 'CMRTE']
