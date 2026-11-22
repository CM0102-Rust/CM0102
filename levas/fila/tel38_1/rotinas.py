# A base da interface (1): os auxiliares da lista de exibição do gerente de telas (scrman.cpp; leitor tel38, rodada das
# telas, 21/11/2026): o título e o subtítulo da tela, os itens de texto e de botão, os itens de área, a linha de
# rolagem de uma área, as teclas da barra de rolagem e as contagens de registros e de telas marcadas.
# Usa os auxiliares do tel01_1 (screens_current_display, screens_forwarder_regions, DisplayAddItemFn), do tel29_1
# (screen_record, screen_history) e o display_list/display_area_at do tel26_1.
# Convencoes conferidas na desmontagem (0x803320..0x8033db, 0x803d50..0x8041bd), todas thiscall, ecx = o gerente
# (0xb74450): add_screen_title ret 0xc (cor, cor do texto, texto); add_screen_subtitle ret 8 (texto, um argumento nao
# lido); add_text_item ret 0x2c (11; o ultimo nao lido); add_button_item ret 0x30 (12); add_area_button ret 0x2c (11);
# add_area_text ret 0x24 (9); add_area_text_by_ref ret 0x2c (11; a altura por ponteiro, os dois ultimos nao lidos);
# todos repassam 18 dwords ao Display::add_item (0x55f990, ret 0x48) do Display do canal atual e devolvem o short dele.
# get_area_scroll_row ret 4 (area como short); press_scrollbar_key ret 8 (repassa ao 0x5621f0 thiscall ret 8);
# count_all_records e count_flagged_screens ret 0, devolvem short (os chamadores leem ax; o original deixa lixo do
# esi/edi do chamador na metade alta).
# Iguais no cm0102-1080.exe (as rotinas inteiras).
# Comparadores: os auxiliares de item lado a lado no que escrevem (o pacote de rede, as contagens, a proxima vaga de
# item, a ordem dos itens livres e, para item de area, a area inteira) e no indice; get_area_scroll_row e as contagens
# pelo valor; press_scrollbar_key lado a lado nos globais do mouse.
TELAS = True
ROTINAS = [('screen_manager_add_screen_title', 0x803f30, [0x8b, 0x44, 0x24, 0x0c, 0x8b, 0x54, 0x24, 0x08]),
           ('screen_manager_add_screen_subtitle', 0x803f80, [0x8b, 0x44, 0x24, 0x04, 0x66, 0x8b, 0x15, 0xec, 0xbd, 0xae, 0x00]),
           ('screen_manager_add_text_item', 0x803fd0, [0x8b, 0x44, 0x24, 0x28, 0x8b, 0x54, 0x24, 0x24]),
           ('screen_manager_add_button_item', 0x804030, [0x8b, 0x44, 0x24, 0x30, 0x8b, 0x54, 0x24, 0x2c]),
           ('screen_manager_add_area_button', 0x8040a0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x2c]),
           ('screen_manager_add_area_text', 0x804100, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x24]),
           ('screen_manager_add_area_text_by_ref', 0x804160, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x24]),
           ('screen_manager_get_area_scroll_row', 0x803320, [0x0f, 0xbf, 0x81, 0x2a, 0x38, 0x00, 0x00]),
           ('screen_manager_press_scrollbar_key', 0x803390, [0x8b, 0x44, 0x24, 0x08, 0x8b, 0x54, 0x24, 0x04]),
           ('screen_manager_count_all_records', 0x8033c0, [0x33, 0xc0, 0x56, 0x81, 0xc1, 0x0a, 0x38, 0x00, 0x00]),
           ('screen_manager_count_flagged_screens', 0x803d50, [0x56, 0x33, 0xc0, 0x57, 0x81, 0xc1, 0xe6, 0x07, 0x00, 0x00])]
DOCS = ('a base da interface (1): o título e o subtítulo da tela, os itens de texto e de botão, os itens de área, a linha de rolagem de uma área, as teclas da barra de rolagem e as contagens de registros e de telas marcadas',
        "the interface base (1): the screen title and subtitle, the text and button items, the area items, an area's scroll row, the scrollbar keys and the record and flagged screen counts",
        [('screen_manager_add_screen_title', 'ScreenManager::add_screen_title', '0x803f30', 'scrman.cpp', 'thiscall, ret 0xc',
          'o título da tela: um item de texto no alto, retângulo (100, 10)–(790, 70), fonte 0xc, estilo 7, alinhamento 0x30, com a cor de fundo, a cor do texto e o texto dados',
          'the screen title: a text item across the top, rectangle (100, 10)–(790, 70), font 0xc, style 7, alignment 0x30, with the given background colour, text colour and text'),
         ('screen_manager_add_screen_subtitle', 'ScreenManager::add_screen_subtitle', '0x803f80', 'scrman.cpp', 'thiscall, ret 8',
          'o subtítulo: um item de texto sob o título, retângulo (100, 80)–(790, 125), fonte 0xc, estilo 6, na cor amarela',
          'the subtitle: a text item under the title, rectangle (100, 80)–(790, 125), font 0xc, style 6, in the yellow colour'),
         ('screen_manager_add_text_item', 'ScreenManager::add_text_item', '0x803fd0', 'scrman.cpp', 'thiscall, ret 0x2c',
          'um item de texto (tipo 1) fora de qualquer área, sem id e sem dado',
          'a text item (type 1) outside any area, with no id and no data'),
         ('screen_manager_add_button_item', 'ScreenManager::add_button_item', '0x804030', 'scrman.cpp', 'thiscall, ret 0x30',
          'um botão (tipo 2) fora de qualquer área, com a própria cor como destaque, o id e o dado',
          'a button (type 2) outside any area, with its own colour as the highlight, its id and data'),
         ('screen_manager_add_area_button', 'ScreenManager::add_area_button', '0x8040a0', 'scrman.cpp', 'thiscall, ret 0x2c',
          'um botão numa área (a área o posiciona: só largura e altura), com o id e o dado',
          'a button in an area (the area lays it out: only width and height), with its id and data'),
         ('screen_manager_add_area_text', 'ScreenManager::add_area_text', '0x804100', 'scrman.cpp', 'thiscall, ret 0x24',
          'um texto numa área (só largura e altura), sem id e sem dado',
          'a text in an area (only width and height), with no id and no data'),
         ('screen_manager_add_area_text_by_ref', 'ScreenManager::add_area_text_by_ref', '0x804160', 'scrman.cpp', 'thiscall, ret 0x2c',
          'o mesmo texto numa área, com a altura lida por ponteiro',
          'the same area text, with the height read through a pointer'),
         ('screen_manager_get_area_scroll_row', 'ScreenManager::get_area_scroll_row', '0x803320', 'scrman.cpp', 'thiscall, ret 4',
          'a linha que uma área da tela atual mostra (0 para a área -1); para um técnico remoto, a última linha que ele informou daquela área, ou 0',
          'the row an area of the current screen shows (0 for area -1); for a remote manager, the row he last reported for that area, or 0'),
         ('screen_manager_press_scrollbar_key', 'ScreenManager::press_scrollbar_key', '0x803390', 'scrman.cpp', 'thiscall, ret 8',
          'uma tecla da barra de rolagem vira um clique simulado na seta ou no trilho da barra aberta',
          'a scrollbar key becomes a faked click on the arrow or the track of the open scrollbar'),
         ('screen_manager_count_all_records', 'ScreenManager::count_all_records', '0x8033c0', 'scrman.cpp', 'thiscall',
          'quantos registros de técnico há nos 16 canais',
          'how many manager records there are in the 16 channels'),
         ('screen_manager_count_flagged_screens', 'ScreenManager::count_flagged_screens', '0x803d50', 'scrman.cpp', 'thiscall',
          'quantas telas de instância única estão abertas em todos os registros de todos os canais',
          'how many single-instance screens are open in all records of all channels')])
