MOTOR = True
ROTINAS = [('abrir_detalhes_da_partida', 0x71b4a0, [0x6a, 0x00, 0x68, 0x70, 0xc1, 0x71, 0x00]),
           ('responder_detalhes_da_partida', 0x71bdc0, [0x51, 0xa1, 0x84, 0x7c, 0xb7, 0x00]),
           ('fechar_detalhes_da_partida', 0x71c170, [0x56, 0x6a, 0x2d, 0xb9, 0x50, 0x44, 0xb7, 0x00])]
DOCS = ('as telas da partida (3): a tela de detalhes da partida',
        'the match screens (3): the match details screen',
        [('abrir_detalhes_da_partida', 'open_match_details', '0x71b4a0', 'match_screens.cpp', 'cdecl',
          'abre a tela de detalhes de uma partida e grava os parâmetros dela',
          'opens the details screen of a match and writes its parameters'),
         ('responder_detalhes_da_partida', 'match_details_input', '0x71bdc0', 'match_screens.cpp', 'cdecl',
          'a resposta da tela de detalhes a um clique (troca de aba, tática do clube, redesenho das abas, menu)',
          'the match details screen\'s response to a click (tab switch, club tactics, tab redraw, menu)'),
         ('fechar_detalhes_da_partida', 'close_match_details', '0x71c170', 'match_screens.cpp', 'cdecl',
          'fecha a tela de detalhes: destrói e libera os objetos guardados nos parâmetros',
          'closes the details screen: destroys and frees the objects kept in the parameters')])
