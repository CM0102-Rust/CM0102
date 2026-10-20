ROTINAS = [('tirar_reserva_de_tecnico', 0x7a82b0, [0x8b, 0x51, 0x20, 0x53, 0x33, 0xc0]),
           ('tirar_jogador_do_banco', 0x7a8470, [0x8b, 0x51, 0x40, 0x53, 0x33, 0xc0]),
           ('premio_da_liga', 0x682f70, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as listas de reserva do gerador de jogadores e o prêmio da liga',
        'the player generator reserve lists and the league prize fund',
        [('tirar_reserva_de_tecnico', 'PLAYER_REGEN::remove_spare_non_player_record', '0x7a82b0', 'player_regen.cpp', 'thiscall',
          'tira da lista de reserva o registro de não jogador de um id',
          'removes a non-player record from the spare list by id'),
         ('tirar_jogador_do_banco', 'PLAYER_REGEN::remove_player_data_from_pool', '0x7a8470', 'player_regen.cpp', 'thiscall',
          'tira do banco de jogadores livres o registro dos dados de jogador',
          'removes the player data record from the free player pool'),
         ('premio_da_liga', 'League::calc_prize_fund', '0x682f70', 'league.cpp', 'thiscall',
          'o prêmio total da liga (uma cota a mais por posição acima do último)',
          'the league total prize fund (one more share per place above the last)')])
