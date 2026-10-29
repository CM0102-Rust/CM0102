# A tabela da liga (league.cpp): iniciar a linha do clube (League::init_table_row 0x687430, ~120 chamadas diretas das
# ligas e copas de cada país; 322 na carreira até 2002-06-01), o vetor dos clubes (League::get_club_array 0x687a20,
# pela vtable; 8) e as posições finais (League::record_final_positions 0x68aa30, pela vtable no fim da temporada). A
# 0x687430 aloca o histórico das posições (+9) e, na regra 4, os confrontos diretos (+5), que a última linha preenche
# com os adversários; a 0x687a20 aloca o vetor. Sem SEH, sem sorteio, sem leitura de lixo; caixa só com mais de 50
# clubes ou sem memória (a cópia faz a mesma, por aviso_em). A 0x68a850 (set_predicted_positions) fica de fora: passa
# ao 0x4a2540 o registrador com os bits altos de lixo (só o byte é lido, mas o resíduo de pilha muda). Iguais no
# cm0102-1080.exe. Simulação: as mutações pela carreira e a confirmação no jogo real comparando. Comparadores: as duas
# que alocam rodam só a original e conferem o plano; as posições finais lado a lado.
SAVE = False
ROTINAS = [('iniciar_linha_da_tabela', 0x687430, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('vetor_dos_clubes_da_liga', 0x687a20, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('gravar_posicoes_finais_da_liga', 0x68aa30, [0x53, 0x33, 0xdb, 0x66, 0x39, 0x59, 0x3e])]
DOCS = ('a tabela da liga: iniciar a linha do clube, o vetor dos clubes e as posições finais',
        'the league table: setting up the club row, the club array and the final positions',
        [('iniciar_linha_da_tabela', 'League::init_table_row', '0x687430', 'league.cpp', 'thiscall, ret 0x10',
          'põe o clube na linha i da tabela: totais zerados, situação 0xff, marca, total de rodadas (+0xa7) se ainda −1, histórico das posições e, na regra 4, os confrontos diretos (a última linha preenche os adversários de todas)',
          'puts the club in table row i: totals cleared, status 0xff, flag, total rounds (+0xa7) if still −1, position history and, under rule 4, the head-to-head entries (the last row fills every row\'s opponents)'),
         ('vetor_dos_clubes_da_liga', 'League::get_club_array', '0x687a20', 'league.cpp', 'thiscall, ret 8',
          'um vetor novo com os clubes da tabela e a conta; sem memória, caixa e conta 0',
          'a new array with the table clubs and the count; without memory, error box and count 0'),
         ('gravar_posicoes_finais_da_liga', 'League::record_final_positions', '0x68aa30', 'league.cpp', 'thiscall',
          'cada clube ainda na divisão da liga guarda a divisão da última temporada (+0x5b) e a posição final (+0x5f)',
          'each club still in the league division stores last season\'s division (+0x5b) and the final position (+0x5f)')])
CAMPOS = [('liga +0xa7: o total de rodadas (−1 até a primeira linha); valor; lê e grava (iniciar_linha_da_tabela)',
           'league +0xa7: total rounds (−1 until the first row); value; reads and writes (iniciar_linha_da_tabela)'),
          ('liga +0xc5: as 4 regras de desempate (4 = confrontos diretos); valor; lê (iniciar_linha_da_tabela)',
           'league +0xc5: the 4 tie-break rules (4 = head-to-head); value; reads (iniciar_linha_da_tabela)'),
          ('linha da tabela (0x3b bytes) +0, +4, +5, +9, +0xd..+0x36, +0x37..+0x3a: clube, posição, confrontos diretos, histórico, totais, situação, marca, 1, clubes; ponteiro e valor; grava (iniciar_linha_da_tabela)',
           'table row (0x3b bytes) +0, +4, +5, +9, +0xd..+0x36, +0x37..+0x3a: club, position, head-to-head, history, totals, status, flag, 1, clubs; pointer and value; writes (iniciar_linha_da_tabela)'),
          ('clube +0x64: situação (3 passa a 2 ao entrar na tabela); valor; lê e grava (iniciar_linha_da_tabela)',
           'club +0x64: status (3 becomes 2 when entering the table); value; reads and writes (iniciar_linha_da_tabela)'),
          ('clube +0x57/+0x5b/+0x5f: divisão, divisão da última temporada e posição final; ponteiro e valor; lê e grava (gravar_posicoes_finais_da_liga)',
           'club +0x57/+0x5b/+0x5f: division, last season\'s division and final position; pointer and value; reads and writes (gravar_posicoes_finais_da_liga)')]
INTERESSA = ['CMRTE', 'SGE']
