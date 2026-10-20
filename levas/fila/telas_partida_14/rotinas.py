MOTOR = True
ROTINAS = [('montar_tela_da_partida', 0x718e90, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('responder_tela_da_partida', 0x71a240, [0xa1, 0x84, 0x7c, 0xb7, 0x00]),
           ('montar_detalhes_da_partida', 0x71b630, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as telas da partida (14): a montagem e a resposta da tela da partida ao vivo e a montagem da tela de detalhes',
        'the match screens (14): building and handling the live match screen and building the match details screen',
        [('montar_tela_da_partida', 'build_match_screen', '0x718e90', 'match_screens.cpp', 'cdecl',
          'monta a tela da partida ao vivo: cabeçalhos dos times e placar, narração, relógio, controle, botões táticos, velocidades da narração, abas, o painel da aba, o estádio e o painel de narração',
          'builds the live match screen: team headers and score, commentary, clock, control, tactics buttons, commentary speeds, tabs, the selected tab panel, the stadium and the commentary panel'),
         ('responder_tela_da_partida', 'handle_match_screen_input', '0x71a240', 'match_screens.cpp', 'cdecl',
          'responde na tela da partida ao vivo: volume e velocidade da narração, o lance novo (relógio, narração, modos), assistir ou não, a tática de um time, a troca de aba e as respostas de cada aba',
          'handles the live match screen: commentary volume and speed, the new event (clock, commentary, modes), watching or not, a team\'s tactics, switching tabs and each tab\'s input'),
         ('montar_detalhes_da_partida', 'build_match_details_screen', '0x71b630', 'match_screens.cpp', 'cdecl',
          'monta a tela de detalhes de uma partida guardada: lê o registro e a foto da partida nas estatísticas, os cabeçalhos dos times, as 7 abas e o painel da aba',
          'builds the details screen of a stored match: loads the match record and snapshot from the match stats, the team headers, the 7 tabs and the selected tab panel')])
