MOTOR = True
ROTINAS = [('abrir_ultimos_placares', 0x71b1b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('montar_ultimos_placares', 0x71b300, [0x83, 0xec, 0x08, 0x6a, 0x0e]),
           ('responder_ultimos_placares', 0x71b3d0, [0xa1, 0x84, 0x7c, 0xb7, 0x00])]
DOCS = ('as telas da partida (2): a tela dos últimos placares',
        'the match screens (2): the latest scores screen',
        [('abrir_ultimos_placares', 'open_latest_scores', '0x71b1b0', 'match_screens.cpp', 'cdecl',
          'abre a tela dos últimos placares de uma competição e grava os parâmetros dela',
          'opens the latest scores screen of a competition and writes its parameters'),
         ('montar_ultimos_placares', 'build_latest_scores', '0x71b300', 'match_screens.cpp', 'cdecl',
          'monta a tela dos últimos placares: fundo, título, menu, painel e botões de voltar e avançar',
          'builds the latest scores screen: background, title, menu, panel and back and next buttons'),
         ('responder_ultimos_placares', 'latest_scores_input', '0x71b3d0', 'match_screens.cpp', 'cdecl',
          'a resposta da tela dos últimos placares a um clique (troca de competição, redesenho, menu)',
          'the latest scores screen\'s response to a click (competition switch, redraw, menu)')])
