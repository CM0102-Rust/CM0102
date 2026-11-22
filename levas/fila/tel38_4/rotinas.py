# A base da interface (4): os pacotes de widget do gerente de telas (scrman.cpp; leitor tel38, rodada das telas,
# 21/11/2026): as marcas, o estilo, as cores, as marcas de texto, a fonte, o texto, o id e o dado de um widget da tela
# atual, aplicados ao Display local ou mandados pela rede. Usa os auxiliares do tel29_1 (screen_record,
# screen_history, SCRMAN_CPP, NETWORK_H, DISPLAY_REDRAW_BLOCKED, c_string_len) e o show_error_at do motor.
# Convencoes conferidas na desmontagem (0x800580..0x800885, 0x800d30..0x8025d6): todas thiscall(widget short, valor)
# ret 8, ecx = o gerente (0xb74450). O pacote: o comando 8 (byte), o widget (word), o subcomando (byte: flags 0,
# style 2, colour 3, text_flags 5, font 6, text_colour 7, text 8, id 9, data 0xa) e o valor (dword; word nas cores;
# byte na fonte; no texto o tamanho com o terminador em word e o texto copiado SEM conferir o espaco, defeito do
# original). Cada escrita inline do network.h confere o espaco e mostra a caixa "network.h <linha>" (133, 134, 136,
# 137, 139, 141 conforme a escrita). A entrega: registro local -> Display::process_command_packet 0x564080 thiscall
# (Display local +0x3844, sessao, 0, buffer); remoto com o Display bloqueado (+0x12f53d) -> o mesmo no Display de eco
# (+0x132ea5); senao Network::send_packet 0x77ac30 thiscall (sessao, dados, tamanho, jogador), falha -> caixa
# "scrman.cpp <linha>". set_widget_font nao pula o widget -1 (as outras pulam).
# Fora: set_widget_rect (0x800890), alterada no cm0102-1080.exe (0x800898..0x80089f, os limites da tela).
# Iguais no cm0102-1080.exe (as rotinas inteiras).
# Comparadores: so a original (o pacote e aplicado ao Display, que pode redesenhar, ou mandado pela rede),
# conferindo o plano: o pacote no buffer (comando, widget, subcomando, valor), quando havia espaco.
TELAS = True
ROTINAS = [('screen_manager_set_widget_flags', 0x800580, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('screen_manager_set_widget_style', 0x800d30, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('screen_manager_set_widget_colour', 0x801040, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('screen_manager_set_widget_text_flags', 0x801350, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('screen_manager_set_widget_font', 0x801660, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('screen_manager_set_widget_text_colour', 0x801960, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('screen_manager_set_widget_text', 0x801c70, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('screen_manager_set_widget_id', 0x801fc0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('screen_manager_set_widget_data', 0x8022d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a base da interface (4): os pacotes de widget: as marcas, o estilo, as cores, as marcas de texto, a fonte, o texto, o id e o dado de um widget da tela atual',
        "the interface base (4): the widget packets: a widget's flags, style, colours, text flags, font, text, id and data on the current screen",
        [('screen_manager_set_widget_flags', 'ScreenManager::set_widget_flags', '0x800580', 'scrman.cpp', 'thiscall, ret 8',
          'as marcas de um widget da tela atual: o pacote (comando 8, o widget, o subcomando 0, um dword) aplicado ao Display local, ao de eco ou mandado ao jogador remoto; nada para o widget -1',
          "a widget's flags on the current screen: the packet (command 8, the widget, sub-command 0, a dword) applied to the local Display, to the echo one or sent to the remote player; nothing for widget -1"),
         ('screen_manager_set_widget_style', 'ScreenManager::set_widget_style', '0x800d30', 'scrman.cpp', 'thiscall, ret 8',
          'o estilo de um widget (subcomando 2, um dword)',
          "a widget's style (sub-command 2, a dword)"),
         ('screen_manager_set_widget_colour', 'ScreenManager::set_widget_colour', '0x801040', 'scrman.cpp', 'thiscall, ret 8',
          'a cor de um widget (subcomando 3, um word)',
          "a widget's colour (sub-command 3, a word)"),
         ('screen_manager_set_widget_text_flags', 'ScreenManager::set_widget_text_flags', '0x801350', 'scrman.cpp', 'thiscall, ret 8',
          'as marcas de texto de um widget (subcomando 5, um dword)',
          "a widget's text flags (sub-command 5, a dword)"),
         ('screen_manager_set_widget_font', 'ScreenManager::set_widget_font', '0x801660', 'scrman.cpp', 'thiscall, ret 8',
          'a fonte de um widget (subcomando 6, um byte); esta não pula o widget -1',
          "a widget's font (sub-command 6, a byte); this one does not skip widget -1"),
         ('screen_manager_set_widget_text_colour', 'ScreenManager::set_widget_text_colour', '0x801960', 'scrman.cpp', 'thiscall, ret 8',
          'a cor do texto de um widget (subcomando 7, um word)',
          "a widget's text colour (sub-command 7, a word)"),
         ('screen_manager_set_widget_text', 'ScreenManager::set_widget_text', '0x801c70', 'scrman.cpp', 'thiscall, ret 8',
          'o texto de um widget (subcomando 8: o tamanho com o terminador e o texto); defeito: o texto é copiado sem conferir o espaço do buffer',
          "a widget's text (sub-command 8: the length with the terminator and the text); defect: the text is copied without checking the buffer's room"),
         ('screen_manager_set_widget_id', 'ScreenManager::set_widget_id', '0x801fc0', 'scrman.cpp', 'thiscall, ret 8',
          'o id de um widget (subcomando 9, um dword)',
          "a widget's id (sub-command 9, a dword)"),
         ('screen_manager_set_widget_data', 'ScreenManager::set_widget_data', '0x8022d0', 'scrman.cpp', 'thiscall, ret 8',
          'o dado de um widget (subcomando 0xa, um dword)',
          "a widget's data (sub-command 0xa, a dword)")])
