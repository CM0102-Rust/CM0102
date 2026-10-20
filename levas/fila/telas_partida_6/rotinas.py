MOTOR = True
ROTINAS = [('abrir_jogos_do_dia', 0x717cc0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('responder_jogos_do_dia', 0x718930, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('vaga_do_mais_bem_avaliado', 0x727e20, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('as telas da partida (6): a tela dos jogos do dia e o capitão padrão',
        'the match screens (6): the day fixtures screen and the default captain',
        [('abrir_jogos_do_dia', 'open_day_fixtures_screen', '0x717cc0', 'match_screens.cpp', 'cdecl',
          'abre a tela dos jogos do dia numa vaga com o grupo de jogos (caixa de erro sem grupo)',
          'opens the day fixtures screen in a slot with the fixture group (error box without a group)'),
         ('responder_jogos_do_dia', 'handle_day_fixtures_input', '0x718930', 'match_screens.cpp', 'cdecl',
          'a resposta da tela dos jogos do dia: outro grupo de jogos, o andamento da competição ou a barra de menu',
          'the day fixtures screen input: another fixture group, the competition progress or the menu bar'),
         ('vaga_do_mais_bem_avaliado', 'match_screens_find_best_rated_starter_slot', '0x727e20', 'match_screens.cpp', 'cdecl',
          'a vaga dos onze com a maior nota da comissão (o capitão padrão), −1 sem ninguém',
          'the first eleven slot with the highest staff rating (the default captain), -1 for none')])
