MOTOR = True
ROTINAS = [('responder_continuar_da_partida', 0x725ca0, [0x83, 0xec, 0x44, 0x53, 0x55]),
           ('abrir_escalacao_da_partida', 0x725f70, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('responder_escalacao_da_partida', 0x727c30, [0x53, 0x55, 0x56, 0x57, 0x6a, 0x00])]
DOCS = ('as telas da partida (7): o Continuar da partida, abrir a escalação e a resposta da escalação',
        'the match screens (7): the match Continue, opening team selection and the team selection response',
        [('responder_continuar_da_partida', 'match_screens_handle_continue_command', '0x725ca0', 'match_screens.cpp', 'cdecl',
          'o Continuar da partida: o comando pendente da escalação (em rede, a vaga livre do clube ou a espera pelos outros), as partidas da rede e a escalação',
          'the match Continue: the pending team selection command (online, the club\'s free slot or waiting for the others), the network matches and team selection'),
         ('abrir_escalacao_da_partida', 'match_screens_open_team_selection', '0x725f70', 'match_screens.cpp', 'cdecl',
          'abre a tela da escalação da próxima partida do humano e grava o clube, a partida e se joga em casa',
          'opens the Team Selection screen for the human\'s next match and stores the club, the match and the home flag'),
         ('responder_escalacao_da_partida', 'match_screens_handle_team_selection_input', '0x727c30', 'match_screens.cpp', 'cdecl',
          'os botões da escalação: nomeia o capitão (o melhor titular) se faltar e deixa o comando pendente; os outros vão à resposta comum',
          'the Team Selection buttons: appoints the captain (the best starter) when missing and stores the pending command; the rest go to the common handler')])
