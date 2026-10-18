MOTOR = True
ROTINAS = [('combinar_com_o_anterior', 0x6d60b0, [0x83, 0xec, 0x08, 0x53, 0x55]),
           ('comentario_de_contexto', 0x6d4b70, [0x6a, 0xff, 0x68, 0x00, 0xd4, 0x95, 0x00]),
           ('construir_jogador_da_partida', 0x6eada0, [0x8b, 0xc1, 0x83, 0xca, 0xff])]
DOCS = ('o fechamento do motor (5): a narração combinada, o comentário de contexto e o jogador da partida',
        'closing the engine (5): the combined commentary, the context commentary and the match player',
        [('combinar_com_o_anterior', 'combine_with_previous', '0x6d60b0', 'match_events.cpp', 'thiscall',
          'combina o evento novo da narração com o último da fila (devolve o evento combinado e copia jogador, lado e valor do anterior)',
          'combines the new commentary event with the last queued one (returns the combined event and copies the previous player, side and value)'),
         ('comentario_de_contexto', 'context_commentary', '0x6d4b70', 'match_eng.cpp', 'thiscall',
          'decide se um comentário de contexto (corrida da liga, notícias do clube) pode tocar agora',
          'decides whether a context commentary (league race, club news) may play now'),
         ('construir_jogador_da_partida', 'construct_match_player', '0x6eada0', 'match_pl.cpp', 'thiscall',
          'o construtor do jogador da partida (zera os campos, com os valores iniciais de alguns)',
          'the match player constructor (clears the fields, with initial values for some)')])
