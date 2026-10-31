# Game.cpp, três rotinas do laço do dia do jogo (0x5cccd0) fora das da 255ª: os coeficientes das nações 0x5d5f20 (todo
# dia, 0x5cda10, 516 chamadas na carreira até 2002-06-01; age no dia do mês da tabela 0x9cd200 e, no de dezembro
# [0x9cd216], dá o bônus e desloca a história de cinco anos de cada nação), o dia das aposentadorias 0x5d6250 (todo dia,
# 0x5cda97) e a soma presa de dois bytes 0x5d61d0 (26 chamadas no diário dos jogadores, depois de 0x5ce2d0). Fora de
# fechar_partida_do_dia e de eventos_do_pos_jogo. Iguais no cm0102-1080.exe. Sem SEH e sem local lida antes de gravada
# (a 0x5d5f20 só usa locais nos buffers da caixa de erro). x87 na 0x5d5f20 (somas e produtos na ordem da original, iguais
# em f64). Comparadores: a 0x5d5f20 aloca, ordena e solta o vetor: só a original, conferida contra o plano feito antes
# (a história e a marca de cada nação); as outras duas pelo valor (al; ax).
SAVE = False
ROTINAS = [('coeficientes_das_nacoes', 0x5d5f20, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8]),
           ('dia_das_aposentadorias', 0x5d6250, [0x66, 0x8b, 0x44, 0x24, 0x04]),
           ('soma_presa', 0x5d61d0, [0x66, 0x0f, 0xbe, 0x4c, 0x24, 0x04])]
DOCS = ('os coeficientes das nações (o bônus de dezembro e a história de cinco anos), o dia das aposentadorias e a soma presa de dois bytes',
        'the nation coefficients (the December bonus and the five-year history), the retirement day and the clamped two-byte sum',
        [('coeficientes_das_nacoes', 'game_update_nation_coefficients', '0x5d5f20', 'game.cpp', 'cdecl',
          'na primeira vez, a história de cada nação recebe o coeficiente de +0xa8; no dia do mês da tabela, a soma ponderada da história num vetor ordenado e solto; no dia de dezembro, o ano corrente ganha reputação × 0,05 × (12 − marca) × 3 (marca < 12), a história anda um ano e o ano corrente e a marca zeram',
          'the first time, each nation history gets the coefficient at +0xa8; on the table day of the month, the weighted history sum in an array that is sorted and freed; on the December day, the current year gets reputation × 0.05 × (12 − mark) × 3 (mark < 12), the history moves one year and the current year and the mark are cleared'),
         ('dia_das_aposentadorias', 'is_player_regen_day', '0x5d6250', 'game.cpp', 'cdecl',
          '1 nos dias 0, 80, 173 e 250 do ano (os da busca das aposentadorias), senão 0',
          '1 on days 0, 80, 173 and 250 of the year (the retirement scan days), otherwise 0'),
         ('soma_presa', 'clamp_char_sum', '0x5d61d0', 'game.cpp', 'cdecl',
          'a soma de dois bytes com sinal presa entre o mínimo e o máximo dados',
          'the sum of two signed bytes clamped between the given minimum and maximum')])
CAMPOS = [('nação (0x122 bytes em [0xae23a8], conta [0xae2350]) +0xa8 coeficiente; double; lê (coeficientes_das_nacoes)',
           'nation (0x122 bytes at [0xae23a8], count [0xae2350]) +0xa8 coefficient; double; reads (coeficientes_das_nacoes)'),
          ('nação +0xb0..+0xd0 história de cinco anos, +0xd8 ano corrente; doubles; lê e grava (coeficientes_das_nacoes)',
           'nation +0xb0..+0xd0 five-year history, +0xd8 current year; doubles; reads and writes (coeficientes_das_nacoes)'),
          ('nação +0x8e reputação; valor; lê (coeficientes_das_nacoes)',
           'nation +0x8e reputation; value; reads (coeficientes_das_nacoes)'),
          ('nação +0x121 marca do bônus de dezembro (< 12 ganha); valor; lê e zera (coeficientes_das_nacoes)',
           'nation +0x121 December bonus mark (< 12 earns it); value; reads and clears (coeficientes_das_nacoes)'),
          ('[0x9cd200] dia do mês da atualização (12 palavras), [0x9cd216] o de dezembro, [0x9cd218] história por iniciar; valores; lê, zera [0x9cd218] (coeficientes_das_nacoes)',
           '[0x9cd200] update day of each month (12 words), [0x9cd216] the December one, [0x9cd218] history to initialise; values; reads, clears [0x9cd218] (coeficientes_das_nacoes)')]
INTERESSA = ['CMRTE']
