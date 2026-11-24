# O laco das telas e o laco do cliente de rede (scrman.cpp; leitor tel45, rodada 5, 23/11/2026).
# Usa o que ja esta no motor: screen_history, screen_node_fields, display_list, display_life, display::ITEM_ID,
# screen_record, current_screen_node, free_screen_node, screens_current_display, widget_field, scrman_channel_record,
# scrman_channel_count, channel_record_at, scrman_command_packet, scrman_net_put, status_message_packet,
# network_buffer_*, NET_SEND_BUFFER, SCREEN_CHANNEL, SOUND_PLAYER, SCRMAN_ECHO_DISPLAY, SCRMAN_NEXT_CHANNEL,
# SCRMAN_RECORD_STAMP, DISPLAY_REDRAW_BLOCKED, tel26_error_box, Tel28FormatFn, GUI_SCRATCH_TEXT, MENUBAR_HANDLER,
# NETWORK_JOIN_BY_ADDRESS, G_MATCH_WAITING_MANAGER, G_CURRENT_COMMAND_ID, JOB_COLOUR_BDEA/BDEC, DISPLAY_RIGHT_BUTTON.
# Convencoes conferidas na desmontagem (cada push contra esp, cada ret):
# - monitor 0x7fc7e0 thiscall, ret 0 -> al (eax zerado antes do sete): fim no ret 0x7fdbc8, tabela de salto de 13
#   entradas em 0x7fdbcc (codigos -13..-1). Chamadores: 0x5cd2a3 (o turno humano do laco do dia 0x5cccd0), 0x5cde6e e
#   0x5ce051 (0x5cde40), 0x6b433e (pick_day_match), 0x6b481a/0x6b49c7 (0x6b4743, match_day.cpp), 0x81ba47
#   (setup_run_front_end), sempre com ecx = 0xb74450.
# - run_network_client 0x7fdc00 thiscall, ret 0 (fim no ret 0x7fe498); chamador 0x81fe16 (setup_start_new_game, entrar
#   num jogo em rede).
# - chamados: time 0x946ebb cdecl(*t); Display::reset 0x55f7c0 thiscall(1) ret 4; draw_screen 0x55f300 thiscall(-1 x4)
#   ret 0x10; log_screen_sent 0x5622a0 thiscall(net, jogador) ret 8; process_command_packet 0x564080 thiscall(net,
#   jogador, buffer) ret 0xc; poll_input 0x55f620 thiscall; send_widget_click 0x568560 thiscall(net, item) ret 8;
#   send_keepalive 0x568470 thiscall(net) ret 4; Network::receive_packet 0x77af20 thiscall(dados, *jogador) ret 8;
#   send_packet 0x77ac30 thiscall(dados, tamanho, jogador) ret 0xc; release_slot_socket 0x77ac00 thiscall(jogador) ret 4;
#   SoundPlayer 0x84cf00 thiscall -> al e stop_slot 0x84cb70 thiscall(byte) ret 4; pump_messages 0x68d520 cdecl;
#   switch_to_manager 0x7ffcd0 thiscall(chave) ret 4; redraw_screen 0x802d00 thiscall(0, 0) ret 8; free_all_screens
#   0x7fef10 thiscall; utils_swap_bytes 0x90ba90 cdecl(a, b, 0x30); as funcoes do no da tela cdecl (build/leave/exit sem
#   argumentos, input(clique) -> codigo); o gancho da barra de menu cdecl(0 ou 1); o retorno por passada 0x262506 cdecl.
# Convivencia: nada do laco e desviado pelo simulation.rs (ele desvia time 0x946ebb, clock 0x947299 e 0x8387e0; o
# monitor chama o time pelo endereco, como o original, e pega o desvio da simulacao no modo ferias). A carreira
# automatica nao passa pelo monitor (ausente da cobertura de 17/10). A camada de video propria da DLL fica abaixo do
# Display (draw_screen/flip), sem mudanca. No modo comparar so a original roda (o laco desenha, le a entrada e roda
# as telas: nada roda duas vezes); as rotinas que ele chama passam pelos proprios desvios e comparadores.
# Iguais no cm0102-1080.exe (bytes das duas rotinas inteiras).
TELAS = True
ROTINAS = [('screen_manager_monitor', 0x7fc7e0, [0x81, 0xec, 0x38, 0x04, 0x00, 0x00]),
           ('screen_manager_run_network_client', 0x7fdc00, [0x81, 0xec, 0x14, 0x06, 0x00, 0x00])]
DOCS = ('o laço das telas e o laço do cliente de rede',
        'the screen loop and the network client loop',
        [('screen_manager_monitor', 'ScreenManager::monitor', '0x7fc7e0', 'scrman.cpp', 'thiscall',
          'o laço das telas: monta e desenha a tela de cada canal que precisa, lê uma entrada (o canal de quem entrou, uma mensagem de rede ou o próximo canal local), roda a função de entrada da tela e age pelo código que ela devolve (voltar, avançar, terminar a tela, montar de novo), até todos os técnicos terminarem (Continuar) ou uma tela devolver -7; então manda "Processing, Please Wait ..." a todas as telas e fecha todos os históricos; devolve se o último código foi -5 (a tela terminou sozinha)',
          'the screen loop: builds and draws the screen of every channel that needs it, reads one input (a joined player\'s channel, a network message or the next local channel), runs the screen\'s input function and acts on the code it returns (back, forward, end the screen, build again), until every manager has finished (Continue) or a screen returns -7; then it sends "Processing, Please Wait ..." to every screen and closes every history; returns whether the last code was -5 (the screen ended by itself)'),
         ('screen_manager_run_network_client', 'ScreenManager::run_network_client', '0x7fdc00', 'scrman.cpp', 'thiscall',
          'o laço do cliente de rede: o canal 0 mostra a tela-eco, cada pacote do servidor é aplicado a ela e cada clique local vai para o servidor (com o diálogo da barra de menu montado localmente e "Contacting Server..." na entrada por endereço), mandando sinais de vida enquanto espera, até o servidor encerrar o jogo ou um clique não ir; então as telas sabem que o jogo saiu e o canal 0 volta à tela local',
          'the network client loop: channel 0 shows the echo screen, each server packet is applied to it and each local click goes to the server (with the menubar dialog built locally and "Contacting Server..." when joining by address), sending keepalives while it waits, until the server ends the game or a click does not go through; then the screens are told the game left and channel 0 shows the local screen again')])
