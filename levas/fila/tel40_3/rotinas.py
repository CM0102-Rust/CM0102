# O lado de rede do Display: o texto de um campo de entrada devolvido a quem chamou e o clique de um jogador remoto
# mandado ao servidor (display.cpp; leitor tel40, rodada 2 das telas, 22/11/2026). Usa display_item, guio_fields,
# display_item_at, tel26_copy_text, display_life::DWORD_551, NET_SEND_BUFFER, network_buffer_* (tel37_1/3),
# SCREEN_CHANNEL (tel37_5), c_string_len, FN_LANGLIB_SPRINTF, DISPLAY_CPP.
# Convencoes conferidas na desmontagem:
# - guio_copy_text_to_caller 0x567b00 thiscall(item) ret 0: chamada so pelo comando 6 do process_command_packet
#   (0x566ffc, item reconstruido por um clique remoto). O prologo de 6 bytes termina num jcc rel8 (0x567b04 je):
#   entra em RELATIVAS (rust/src/detours.rs) como (0x567b00, 4); sem isso tirar essa entrada.
# - send_widget_click 0x568560 thiscall(sessao, widget short) ret 8 -> 1/0; Guio::get_fields 0x5ee4a0 thiscall com
#   19 ponteiros de saida, ret 0x4c (o mapa das vagas de pilha conferido push a push nas duas chamadas, 0x568681 e
#   0x5691a8); put_long 0x5694a0 e put_sized_block 0x569540 thiscall(buf, ptr, n) ret 8; Network::send_packet com o
#   canal SCREEN_CHANNEL; a caixa "Network Error" de display.cpp linha 3322.
# Rodam SO em rede (send_widget_click: so chamada pelo run_network_client 0x7fe1df; guio_copy_text_to_caller: so
# pelo comando 6, que so chega de um cliente).
# Defeitos do original: o pacote manda como ultimo campo o dword +0x58 do widget, nao o alvo de entrada (+0x54) que
# get_fields devolve (o leitor do comando 6 grava esse valor como alvo); o texto entra no buffer sem conferir o espaco.
# Comparadores: guio_copy_text_to_caller lado a lado (o buffer de quem chamou); send_widget_click so a original, com
# o plano conferido (o pacote comeca com o comando 6 e o widget; enviado, DWORD_551 = 0).
# Iguais no cm0102-1080.exe (conferir com assinaturas.py --conferir).
TELAS = True
ROTINAS = [('guio_copy_text_to_caller', 0x567b00, [0xf6, 0x41, 0x0c, 0x08, 0x74, 0x2e]),
           ('display_send_widget_click', 0x568560, [0x81, 0xec, 0x84, 0x03, 0x00, 0x00])]
DOCS = ('o lado de rede do Display: o texto de um campo de entrada devolvido a quem chamou e o clique de um jogador remoto mandado ao servidor',
        "the Display's network side: a text entry's text back to its caller and a remote player's click sent to the server",
        [('guio_copy_text_to_caller', 'guio_copy_text_to_caller', '0x567b00', 'display.cpp', 'thiscall',
          'um campo de entrada de texto com buffer de quem chamou recebe o texto de volta, com o terminador',
          "a text entry widget with a caller's buffer gets its text copied back, with the terminator"),
         ('display_send_widget_click', 'Display::send_widget_click', '0x568560', 'display.cpp', 'thiscall, ret 8',
          'o clique de um jogador remoto vai ao servidor: comando 6 com o widget e todos os campos atuais dele e, quando foi solto sobre outro widget, o retângulo e os campos desse outro; falhando, a caixa "Network Error" e 0',
          "a remote player's click goes to the server: command 6 with the widget and all its current fields and, when it was dropped on another widget, the drop rectangle and that widget's fields; failing, the \"Network Error\" box and 0")])
