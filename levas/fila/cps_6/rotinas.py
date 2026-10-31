# O dia do jogo (game.cpp), três rotinas do laço do dia (a função de 0x5cd5xx que chama a 0x5d5ae0, 516 chamadas na
# carreira até 2002-06-01): as faltas ao treino 0x5d6aa0 (0x5cdb0f, todo dia), as competições do dia 0x5ce060
# (0x5cd9e9, todo dia, com 1; a 0x5aaf00 vai em salto de cauda e não tem argumento) e a confiança da diretoria
# 0x5cdde0 (0x5cd959, com −5, num ramo do laço dos elencos). Fora de fechar_partida_do_dia e de eventos_do_pos_jogo.
# Iguais no cm0102-1080.exe. Sem SEH, sem x87 e sem local lida antes de gravada. Comparadores: a 0x5d6aa0 e a 0x5ce060
# só a original (sorteios, falta 0x41a360, virtuais das competições); a 0x5cdde0 lado a lado pela confiança.
SAVE = False
ROTINAS = [('faltas_ao_treino', 0x5d6aa0, [0x51, 0x8b, 0x0d, 0x64, 0x23, 0xae, 0x00]),
           ('competicoes_do_dia', 0x5ce060, [0xa1, 0xf8, 0xad, 0xad, 0x00]),
           ('mudar_confianca_da_diretoria', 0x5cdde0, [0x8b, 0x54, 0x24, 0x04, 0x85, 0xd2])]
DOCS = ('o dia do jogo: as faltas ao treino, as competições do dia e a confiança da diretoria no clube',
        'the game day: missed training, the competitions of the day and the board confidence in the club',
        [('faltas_ao_treino', 'game_check_players_missing_training', '0x5d6aa0', 'game.cpp', 'cdecl',
          'n/250 pessoas seguidas a partir de uma sorteada: o jogador presente, com treino e sem lesão grave falta ao treino (0x41a360) por um sorteio da soma de atributos, moral e preferências pelo técnico (mais provável quando chegou há pouco) ou pelas marcas do vínculo',
          'n/250 consecutive people from a random one: a present player, with training and without a serious injury misses training (0x41a360) by a draw on the sum of attributes, morale and likes of the manager (more likely when newly arrived) or by the contract flags'),
         ('competicoes_do_dia', 'game_update_comps_for_date', '0x5ce060', 'game.cpp', 'cdecl',
          'cada competição da lista recebe o dia (virtual +0x14 com a data do jogo e a marca); com a marca, o passo seguinte do [0xae2a58] (0x5aaf00)',
          'each competition in the list gets the day (virtual +0x14 with the game date and the flag); with the flag, the next step of [0xae2a58] (0x5aaf00)'),
         ('mudar_confianca_da_diretoria', 'manager_manager_adjust_board_confidence', '0x5cdde0', 'game.cpp', 'thiscall, ret 8',
          'soma o delta à confiança da diretoria no clube (+6 da linha de 73 bytes do clube), presa em 1..10000',
          'adds the delta to the board confidence in the club (+6 of the 73-byte club row), held to 1..10000')])
CAMPOS = [('pessoa +0x3d função de origem (11 = jogador), +0x3e data de chegada, +0x18, +0x5b atributos; valores; lê (faltas_ao_treino)',
           'staff +0x3d original job (11 = player), +0x3e arrival date, +0x18, +0x5b attributes; values; reads (faltas_ao_treino)'),
          ('jogador (pessoa +0x61) +0x2f, +0x44 moral; valores; lê (faltas_ao_treino)',
           'player (staff +0x61) +0x2f, +0x44 morale; values; reads (faltas_ao_treino)'),
          ('vínculo (0x4cf5f0) +0x45 marcas 0x8000, 0x30000, 0x7800; valores; lê (faltas_ao_treino)',
           'contract (0x4cf5f0) +0x45 flags 0x8000, 0x30000, 0x7800; values; reads (faltas_ao_treino)'),
          ('clube +0xcf técnico; ponteiro; lê (faltas_ao_treino)',
           'club +0xcf manager; pointer; reads (faltas_ao_treino)'),
          ('[0xae1080] o último contrato achado (0x4cf5a0); ponteiro; grava (faltas_ao_treino)',
           '[0xae1080] the last contract found (0x4cf5a0); pointer; writes (faltas_ao_treino)'),
          ('[0xadadfc]/[0xadadf8] a lista das competições e a conta; ponteiro/valor; lê (competicoes_do_dia)',
           '[0xadadfc]/[0xadadf8] the competition list and its count; pointer/value; reads (competicoes_do_dia)'),
          ('MANAGER_MANAGER [0xb67a44] +0: linhas de 73 bytes por clube, +6 confiança da diretoria; valor; lê e grava (mudar_confianca_da_diretoria)',
           'MANAGER_MANAGER [0xb67a44] +0: 73-byte rows per club, +6 board confidence; value; reads and writes (mudar_confianca_da_diretoria)')]
INTERESSA = ['CMRTE']
