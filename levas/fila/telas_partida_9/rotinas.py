MOTOR = True
ROTINAS = [('atribuir_bloco_do_time', 0x71dd70, [0x8b, 0x44, 0x24, 0x04, 0x53]),
           ('atribuir_linha_do_jogador', 0x71df00, [0x83, 0xec, 0x08, 0x56, 0x57]),
           ('contar_placar_ao_vivo', 0x728e90, [0x8b, 0x44, 0x24, 0x0c, 0x53])]
DOCS = ('as telas da partida (9): a cópia do bloco do time e da linha do jogador e o placar ao vivo',
        'the match screens (9): the team block and player row copies and the live score',
        [('atribuir_bloco_do_time', 'match_team_block_assign', '0x71dd70', 'match_screens.cpp', 'thiscall',
          'copia o bloco do time da partida (0x625 bytes) campo a campo, com as 20 linhas de jogador',
          'copies the match team block (0x625 bytes) field by field, with its 20 player rows'),
         ('atribuir_linha_do_jogador', 'match_player_row_assign', '0x71df00', 'match_screens.cpp', 'thiscall',
          'copia uma linha de jogador do bloco do time (0x49 bytes), a data pela Date::operator=',
          'copies one player row of the team block (0x49 bytes), the date through Date::operator='),
         ('contar_placar_ao_vivo', 'match_screens_tally_live_score', '0x728e90', 'match_screens.cpp', 'cdecl',
          'conta os gols e os gols nos pênaltis de uma partida até o minuto atual da partida ao vivo',
          'counts the goals and shoot-out goals of a match up to the current minute of the live match')])
