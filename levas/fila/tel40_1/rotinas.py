# O gerente de telas: os registros dos jogadores e tecnicos, a sessao de rede, os pacotes de clique e de rolagem e a
# mensagem de estado (scrman.cpp; leitor tel40, rodada 2 das telas, 22/11/2026). Usa os auxiliares ja publicados:
# screen_history, screen_record, scrman_command_packet, scrman_net_put, scrman_net_put_text, scrman_send_packet,
# SCRMAN_ECHO_DISPLAY, SCRMAN_CPP (tel29_1/tel38_*), NET_SEND_BUFFER, network_buffer_* (tel37_1), SCREEN_CHANNEL
# (tel37_5), tcpip_fn (tel36_14), tel26_error_box, G_SCREEN_MANAGER_NETWORK(_HOST_FLAG) (tel11_2).
# Convencoes conferidas na desmontagem (cada push contra esp, cada ret):
# - get_local_addresses 0x7fc7c0 thiscall(out) ret 4: so repassa a Network::get_local_addresses 0x77a980 da sessao
#   (+0x3840, sem conferir nulo).
# - start_network_session 0x7ff500 thiscall(on_join, on_leave) ret 8 -> ok; o "ok" mora no lugar do argumento on_join
#   (defeito: operator new falhando devolve o gancho, "abriu", sem sessao).
# - scrman_add_player_record 0x7ff630 e scrman_remove_player_records 0x7ff7b0 cdecl(jogador, word; o dword empurrado
#   leva a metade alta do registrador), usam o gerente global 0xb74450 pelo endereco absoluto; sao os ganchos que
#   start_network_session passa ao Network::Network. utils_swap_bytes 0x90ba90 (TROCA, nao copia) cdecl(a, b, n).
# - add_manager_record 0x7ff8f0 thiscall(on_join, on_leave, key) ret 0xc: chama 0x7ff630 direto (call rel32).
# - send_clear_click 0x800000 thiscall ret 0; set_pending_click_widget 0x800160 thiscall(widget) ret 4;
#   send_area_scroll_row 0x800330 thiscall(area, row) ret 8 (usa o buffer global 0xae28f8 e o registro do canal 0,
#   nao o do canal atual); send_network_status_message 0x802f80 thiscall(text, value, total, channel) ret 0x10.
# O que roda sem rede (conferido na desmontagem): add_manager_record (o recurso de -load/carga e a criacao de tecnico
# humano: 0x76404a, 0x8233dc, 0x823852, 0x838886, sempre com uma chave) e por ele scrman_add_player_record (sem
# sessao a guarda nao corta; sem gancho de entrada vai para o lugar 1); send_clear_click e set_pending_click_widget
# (registro local: o pacote e aplicado ao Display local); send_area_scroll_row (toda rolagem de area: so enche o
# buffer, nao manda); send_network_status_message (barra de progresso: aplicado ao Display local). So em rede:
# get_local_addresses, start_network_session, scrman_remove_player_records (so no anfitriao) e os ramos remotos.
# Comparadores: os registros lado a lado (os 16 canais, contagens, canal atual e seguinte, ganchos); com gancho de
# entrada (abre telas) so a original. Os pacotes: a montagem lado a lado (a copia monta, guarda os bytes, restaura;
# a original monta e entrega uma vez; os bytes comparados), a entrega so pela original (o Display desenha). A rolagem
# local lado a lado inteira (so mexe no buffer). get_local_addresses, start_network_session e
# scrman_remove_player_records so a original (Winsock; libera historico), com o plano conferido onde da.
# Iguais no cm0102-1080.exe (conferir com assinaturas.py --conferir).
TELAS = True
ROTINAS = [('screen_manager_get_local_addresses', 0x7fc7c0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x89, 0x40, 0x38, 0x00, 0x00]),
           ('screen_manager_start_network_session', 0x7ff500, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('scrman_add_player_record', 0x7ff630, [0x51, 0xa1, 0x90, 0x7c, 0xb7, 0x00]),
           ('scrman_remove_player_records', 0x7ff7b0, [0xa1, 0x80, 0x7c, 0xb7, 0x00]),
           ('screen_manager_add_manager_record', 0x7ff8f0, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14]),
           ('screen_manager_send_clear_click', 0x800000, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('screen_manager_set_pending_click_widget', 0x800160, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('screen_manager_send_area_scroll_row', 0x800330, [0xa1, 0x00, 0x29, 0xae, 0x00]),
           ('screen_manager_send_network_status_message', 0x802f80, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('o gerente de telas: os registros dos jogadores e técnicos, a sessão de rede, os pacotes de clique e de rolagem e a mensagem de estado',
        "the screen manager's players and network packets: the player records, the network session, the click and scroll packets and the status message",
        [('screen_manager_get_local_addresses', 'ScreenManager::get_local_addresses', '0x7fc7c0', 'scrman.cpp', 'thiscall, ret 4',
          'os endereços da máquina, pela sessão de rede (repassa à Network::get_local_addresses)',
          "the machine's addresses, through the network session (forwards to Network::get_local_addresses)"),
         ('screen_manager_start_network_session', 'ScreenManager::start_network_session', '0x7ff500', 'scrman.cpp', 'thiscall, ret 8',
          'guarda os ganchos de entrada e saída, fecha a sessão antiga e abre uma nova cujas conexões chamam os registros de jogador; devolve se abriu (defeito: sem memória devolve "abriu" sem sessão)',
          'keeps the join and leave hooks, closes the old session and opens a new one whose connections call the player records; returns whether it opened (defect: out of memory it returns "opened" with no session)'),
         ('scrman_add_player_record', 'scrman_add_player_record', '0x7ff630', 'scrman.cpp', 'cdecl',
          'quem entra ganha um registro novo no canal dele (o canal cujo primeiro registro é dele, ou o primeiro vazio): até 16 por canal, os outros sobem um lugar; com gancho de entrada o gancho roda e o canal fica marcado para o monitor; sem gancho o registro novo vai para trás do atual; num cliente de rede nada',
          'a player who joins gets a fresh record in his channel (the one whose first record is his, or the first empty one): up to 16 per channel, the others move up; with a join hook the hook runs and the channel is marked for the monitor; without one the new record goes behind the current one; nothing on a network client'),
         ('scrman_remove_player_records', 'scrman_remove_player_records', '0x7ff7b0', 'scrman.cpp', 'cdecl',
          'quem sai perde todos os registros dos canais que são dele; só no anfitrião',
          'a player who leaves loses every record of the channels that are his; only on the host'),
         ('screen_manager_add_manager_record', 'ScreenManager::add_manager_record', '0x7ff8f0', 'scrman.cpp', 'thiscall, ret 0xc',
          'um técnico ganha um registro novo no canal atual; com a chave do técnico o registro vai para trás do atual e leva a chave, e os ganchos da sessão ficam como estavam',
          'a manager gets a new record in the current channel; with his key the record goes behind the current one and gets the key, and the session hooks stay as they were'),
         ('screen_manager_send_clear_click', 'ScreenManager::send_clear_click', '0x800000', 'scrman.cpp', 'thiscall',
          'apaga o clique na fila da tela atual (comando 0x11), no Display local ou no do jogador remoto',
          "clears the current screen's queued click (command 0x11), on the local Display or the remote player's"),
         ('screen_manager_set_pending_click_widget', 'ScreenManager::set_pending_click_widget', '0x800160', 'scrman.cpp', 'thiscall, ret 4',
          'o próximo clique esquerdo na tela atual devolve o widget dado (comando 0x13)',
          'the next left click on the current screen returns the given widget (command 0x13)'),
         ('screen_manager_send_area_scroll_row', 'ScreenManager::send_area_scroll_row', '0x800330', 'scrman.cpp', 'thiscall, ret 8',
          'quando uma área rola, a primeira linha visível vira um pacote (comando 0xf), mandado à tela remota quando o canal 0 mostra o Display remoto; no jogo local só enche o buffer',
          'when an area scrolls its first visible row becomes a packet (command 0xf), sent to the remote screen when channel 0 shows the remote Display; a local game only fills the buffer'),
         ('screen_manager_send_network_status_message', 'ScreenManager::send_network_status_message', '0x802f80', 'scrman.cpp', 'thiscall, ret 0x10',
          'uma mensagem de estado (texto e dois números, a barra de progresso) vai para a tela do canal dado, ou de todos com -1: aplicada ao Display local para um registro local, mandada ao jogador para um remoto',
          "a status message (a text and two numbers, the progress bar) goes to the given channel's screen, or to every channel's with -1: applied to the local Display for a local record, sent to the player for a remote one")])
