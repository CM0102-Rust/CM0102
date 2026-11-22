# The display list input (display.cpp; reader tel26, the screens round, 21/11/2026). Uses the helpers of tel26_1.
# Conventions checked in the disassembly: the four input handlers 0x560490/0x560c30/0x561050/0x561210 thiscall(this)
# -> ax; 0x5617c0 thiscall(this, short item) ret 4 -> eax 0/1; 0x561f10 thiscall(this, short area) ret 4 -> eax 0/1;
# 0x568470 thiscall(this, session) ret 4. Guio::highlight_on/off 0x5eec20/0x5eecb0 thiscall; Area::open_popup /
# close_popup 0x402d10/0x402d80 thiscall; Area::scroll_rows 0x403ee0 thiscall(area, rows) ret 4;
# Guio::set_text_and_refresh 0x5eed50 thiscall(item, text, refresh) ret 8; gui_draw_circle 0x5e7840 and
# graphics_draw_rect 0x5e3b50 cdecl(x1, y1, x2, y2, 1, colour); display_send_status_packet 0x568290 stdcall(session,
# slot, 0); Network::send_packet 0x77ac30 thiscall(session, data, length, slot) ret 0xc.
# The 1080 exe patches the screen clamps of drag_widget (0x31f -> 0x77f, 0x257 -> 0x437): read from the running exe.
# Left out: 0x567260 of the list is not a routine (the community map: a tail of Display::process_command_packet
# 0x564080, the network command handler, which is not in this group).
# Defects of the original kept: when a key changes a free text entry, the key is passed again to that same entry
# (the second walk starts at it); after a key changes an area text entry the first-pass walk starts at the area
# count and does nothing.
MOTOR = True
TELAS = True
ROTINAS = [('display_hit_test_widgets', 0x560490, [0x83, 0xec, 0x10, 0x53, 0x55]),
           ('display_track_popup_hover', 0x560c30, [0x83, 0xec, 0x10, 0xa1, 0x34, 0x7a, 0xb6, 0x00]),
           ('display_drag_scroll_thumb', 0x561050, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('display_drag_widget', 0x561210, [0x83, 0xec, 0x10, 0x53, 0x8b, 0x1d, 0x20, 0x7a, 0xb6, 0x00]),
           ('display_handle_text_entry_key', 0x5617c0, [0x81, 0xec, 0x04, 0x03, 0x00, 0x00]),
           ('display_hit_test_area_scrollbar', 0x561f10, [0x81, 0xec, 0x38, 0x02, 0x00, 0x00]),
           ('display_send_keepalive', 0x568470, [0x8b, 0x81, 0x05, 0xf5, 0x12, 0x00])]
DOCS = ('a lista de exibição (2): o teste do mouse, o menu que abre por cima, arrastar a barra de rolagem e um item, a tecla na caixa de texto, o clique na barra de rolagem e o sinal de vida da rede',
        'the display list (2): the mouse hit test, the popup hover, dragging the scroll thumb and a widget, the text entry key, the scroll bar click and the network keepalive',
        [('display_hit_test_widgets', 'Display::hit_test_widgets', '0x560490', 'display.cpp', 'thiscall',
          'modo 0 da entrada: testa o mouse contra os itens soltos e os das áreas abertas (acende, apaga, cursor de mão), passa a tecla às caixas de texto e o clique às barras de rolagem; o item clicado abre o seu menu ou começa a ser arrastado',
          'input mode 0: tests the mouse against the free widgets and those of the open areas (highlight, unhighlight, hand cursor), passes the key to the text entries and the click to the scroll bars; the clicked widget opens its popup or starts a drag'),
         ('display_track_popup_hover', 'Display::track_popup_hover', '0x560c30', 'display.cpp', 'thiscall',
          'modo 1: o menu aberto segue o mouse (acende o item e o dono do menu), abre e fecha submenus depois de 300 tiques e fecha o menu num clique fora ou num item sem submenu',
          'mode 1: the open popup follows the mouse (highlights the item and the popup owner), opens and closes submenus after 300 ticks and closes the popup on a click outside or on an item without a submenu'),
         ('display_drag_scroll_thumb', 'Display::drag_scroll_thumb', '0x561050', 'display.cpp', 'thiscall',
          'modo 2: o cursor da barra de rolagem segue o mouse dentro do trilho e a área rola até a linha correspondente',
          'mode 2: the scroll thumb follows the mouse within the track and the area scrolls to the matching row'),
         ('display_drag_widget', 'Display::drag_widget', '0x561210', 'display.cpp', 'thiscall',
          'modo 3: o contorno do item arrastado segue o mouse dentro da tela; ao soltar, o primeiro item clicável sob o mouse vira o destino, com o contorno como retângulo',
          'mode 3: the dragged widget outline follows the mouse within the screen; on release the first clickable widget under the mouse becomes the drop target, with the outline as its rectangle'),
         ('display_handle_text_entry_key', 'Display::handle_text_entry_key', '0x5617c0', 'display.cpp', 'thiscall',
          'aplica a tecla à caixa de texto: começar e parar a edição pelo clique, Tab e Enter (Tab passa o foco), apagar na primeira tecla, setas, apagar e inserir caracteres permitidos que caibam na largura',
          'applies the key to a text entry: starting and stopping the edit by click, Tab and Enter (Tab passes the focus), clearing on the first key, arrows, deleting and inserting allowed chars that fit the width'),
         ('display_hit_test_area_scrollbar', 'Display::hit_test_area_scrollbar', '0x561f10', 'display.cpp', 'thiscall',
          'o clique na barra de rolagem: as setas rolam uma linha, o trilho uma página, o cursor começa o arrasto; redesenha a barra e rola a área',
          'a click on the scroll bar: the arrows scroll a row, the track a page, the thumb starts the drag; redraws the bar and scrolls the area'),
         ('display_send_keepalive', 'Display::send_keepalive', '0x568470', 'display.cpp', 'thiscall',
          'no jogo em rede, manda o pacote de situação 0 ou reenvia o pacote pendente (erro de rede na falha)',
          'in a network game, sends the status 0 packet or resends the pending buffer (network error box on failure)')])
