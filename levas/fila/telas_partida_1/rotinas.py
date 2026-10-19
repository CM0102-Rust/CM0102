MOTOR = True
ROTINAS = [('rotulo_do_controle', 0x725a70, [0x8b, 0x4c, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('fechar_tela_da_partida', 0x71b120, [0x56, 0x6a, 0x2d, 0xb9, 0x50, 0x44, 0xb7, 0x00]),
           ('abrir_tela_de_espera', 0x727f00, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as telas da partida (1): o rótulo do controle, fechar a tela e a espera',
        'the match screens (1): the control label, closing the screen and the waiting screen',
        [('rotulo_do_controle', 'match_control_label', '0x725a70', 'match_screens.cpp', 'cdecl',
          'o rótulo do botão de controle da partida (esperando, começar, recomeçar, pausar ou retomar, continuar) e se ele fica desligado',
          'the match control button label (waiting, kick off, restart, pause or resume, continue) and whether it is disabled'),
         ('fechar_tela_da_partida', 'close_match_screen', '0x71b120', 'match_screens.cpp', 'cdecl',
          'fecha a tela da partida: destrói e libera os parâmetros 0x2d e 0x1d do gerente de telas',
          'closes the match screen: destroys and frees screen manager parameters 0x2d and 0x1d'),
         ('abrir_tela_de_espera', 'open_waiting_screen', '0x727f00', 'match_screens.cpp', 'cdecl',
          'abre a tela de espera do técnico humano no gerente de telas',
          'opens the human manager\'s waiting screen in the screen manager')])
