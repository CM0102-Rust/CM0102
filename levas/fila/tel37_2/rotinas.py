# The popup areas, scrolling an area, the area colour, the widget rectangles, the scroll keys, the media paths and the
# screen-sent log (Area.cpp, display.cpp; reader tel37, the screens round, 21/11/2026).
# Conventions checked in the disassembly:
# - Area::open_popup 0x402d10 and Area::close_popup 0x402d80 thiscall, ret 0; Area::scroll_rows 0x403ee0 thiscall
#   (rows) ret 4; Area::convert_colour_to_pixel_format 0x4041d0 thiscall (to_565) ret 4.
# - guio_set_fixed_rect 0x567ac0 / guio_set_rect 0x567ae0 thiscall (x1, y1, x2, y2) ret 0x10;
#   ScreenManager::set_area_selection 0x5680b0 thiscall (short key, data) ret 8; display_press_scrollbar_key 0x5621f0
#   stdcall (part, held) ret 8 (jump table 0x562290: parts 0..3); display_localise_media_path 0x5680f0 cdecl (path,
#   is_picture); Display::log_screen_sent 0x5622a0 thiscall (session, player) ret 8.
# - Called: Area::get_popup_position 0x403760 thiscall(out_x, out_y) ret 8; Area::move_to 0x403a70 thiscall(x, y)
#   ret 8; Area::draw 0x402e70 thiscall(redraw) ret 4; gui_restore_screen_image 0x5e3fd0 cdecl(x, y, image);
#   gui_free_image; graphics_flip_rect 0x5e2eb0 cdecl(x1, y1, x2, y2); Guio::draw 0x5ee590 thiscall(item, 0);
#   ScreenManager 0x800330 thiscall(area index, row) (the scroll position to the network buffer); 0x5e49f0 cdecl(colour,
#   *r, *g, *b, format) and graphics_rgb_to_pixel cdecl(r, g, b, 0) (both mask the bytes); _strlwr 0x9532a6, strstr,
#   utils_file_exists, utils_build_file_path(key, name); Display::send_screen_section 0x562370 thiscall(session,
#   player, section) ret 0xc.
# Where they run: open/close_popup on the popup menus (the top menu bar, the action menu of a player); scroll_rows on
# any list scrolled by the wheel or the bar; convert_colour, guio_set_*, set_area_selection and localise_media_path
# only from Display::process_command_packet (every screen of a local game too); press_scrollbar_key from the
# keyboard (arrow / page keys on a list); log_screen_sent only for a network player.
# The 1080 exe does not change these routines.
# Depends on tel26_1 (display_list, display_item, display_rect_call, display_method1, display_restore_image,
# tel26_copy_text, JOB_TEXT, DISPLAY_MOUSE_X/Y, first_differences) and tel29_1 (screen_record, screen_history).
# Left out of this group: guio_copy_text_to_caller 0x567b00 (its first 5 bytes end in a jcc rel8: the hook needs the
# entry (0x567b00, 4) in detours.rs RELATIVAS, outside the items).
# Defects of the original: none found.
TELAS = True
ROTINAS = [('area_open_popup', 0x402d10, [0x83, 0xec, 0x08, 0x8d, 0x44, 0x24, 0x04]),
           ('area_close_popup', 0x402d80, [0x53, 0x56, 0x8b, 0xf1, 0x33, 0xdb]),
           ('area_scroll_rows', 0x403ee0, [0x51, 0x56, 0x8b, 0xf1, 0x8a, 0x86, 0xac, 0x0b, 0x00, 0x00]),
           ('area_convert_colour_to_pixel_format', 0x4041d0, [0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x28]),
           ('guio_set_fixed_rect', 0x567ac0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x0c]),
           ('guio_set_rect', 0x567ae0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x0c]),
           ('screen_manager_set_area_selection', 0x5680b0, [0x0f, 0xbf, 0x81, 0x2a, 0x38, 0x00, 0x00]),
           ('display_press_scrollbar_key', 0x5621f0, [0xa1, 0x58, 0x9c, 0xad, 0x00]),
           ('display_localise_media_path', 0x5680f0, [0x81, 0xec, 0x04, 0x01, 0x00, 0x00]),
           ('display_log_screen_sent', 0x5622a0, [0x83, 0xec, 0x0c, 0x56, 0x57])]
DOCS = ('as áreas suspensas, rolar uma área, a cor da área, os retângulos dos itens, as teclas da barra de rolagem, os caminhos de mídia e o registro da tela enviada',
        'the popup areas, scrolling an area, the area colour, the widget rectangles, the scroll bar keys, the media paths and the screen-sent log',
        [('area_open_popup', 'Area::open_popup', '0x402d10', 'Area.cpp', 'thiscall',
          'abre uma área suspensa: leva-a à sua posição (get_popup_position; move_to se mudou), marca aberta, desenha (draw com 0) e vira o retângulo',
          'opens a popup area: moves it to its popup position (get_popup_position; move_to when it changed), marks it open, draws it (draw with 0) and flips its rectangle'),
         ('area_close_popup', 'Area::close_popup', '0x402d80', 'Area.cpp', 'thiscall',
          'fecha uma área suspensa aberta: apaga o destaque dos itens do primeiro visível ao fim (fechando as suspensas que eles abrem), marca fechada e, com imagem guardada, devolve a tela de baixo, libera a imagem e vira o retângulo',
          'closes an open popup area: unhighlights the widgets from the first visible one to the end (closing the popups they own), marks it closed and, with a saved image, puts the screen under it back, frees the image and flips the rectangle'),
         ('area_scroll_rows', 'Area::scroll_rows', '0x403ee0', 'Area.cpp', 'thiscall, ret 4',
          'rola a área N linhas: para baixo, o primeiro item pelo menos N linhas depois do atual (ou o último); para cima, o último até N linhas antes (ou o 0), recuando ao primeiro da mesma linha; reposiciona e desenha os itens que cabem nas linhas, vira o retângulo menos a barra (22 pixels) e avisa o gerenciador de telas a nova linha',
          'scrolls the area N rows: down, the first widget at least N rows past the current one (or the last); up, the last one within N rows back (or 0), back to the first of that row; places and draws the widgets that fit the rows, flips the rectangle minus the bar (22 pixels) and tells the screen manager the new row'),
         ('area_convert_colour_to_pixel_format', 'Area::convert_colour_to_pixel_format', '0x4041d0', 'Area.cpp', 'thiscall, ret 4',
          'converte a cor da área do outro formato de pixel (5-5-5 ou 5-6-5) para o da tela: separa vermelho, verde e azul pelas máscaras e junta com graphics_rgb_to_pixel',
          "converts the area colour from the other pixel format (5-5-5 or 5-6-5) to the screen's: splits red, green and blue by the masks and joins them with graphics_rgb_to_pixel"),
         ('guio_set_fixed_rect', 'guio_set_fixed_rect', '0x567ac0', 'display.cpp', 'thiscall, ret 0x10',
          'grava o retângulo fixo do item (x1, y1, x2, y2; 0 = posto pela grade da área)',
          "writes the widget's fixed rectangle (x1, y1, x2, y2; 0 = placed by the area's grid)"),
         ('guio_set_rect', 'guio_set_rect', '0x567ae0', 'display.cpp', 'thiscall, ret 0x10',
          'grava o retângulo do item (x1, y1, x2, y2)',
          "writes the widget's rectangle (x1, y1, x2, y2)"),
         ('screen_manager_set_area_selection', 'ScreenManager::set_area_selection', '0x5680b0', 'display.cpp', 'thiscall, ret 8',
          'grava a chave da tela e o dado dela no canal atual do gerenciador de telas',
          "writes the screen key and its data in the screen manager's current channel"),
         ('display_press_scrollbar_key', 'display_press_scrollbar_key', '0x5621f0', 'display.cpp', 'stdcall, ret 8',
          'aperta uma tecla da barra de rolagem ativa: o mouse vai a x1 + 1 da barra e à altura da parte pedida (0..3, campos +0x14, +4, +0x28 e +0x10 da barra) com o botão esquerdo apertado; outra parte solta o botão',
          "presses a key of the active scroll bar: the mouse goes to the bar's x1 + 1 and the height of the given part (0..3, the bar's fields +0x14, +4, +0x28 and +0x10) with the left button down; another part lets the button go"),
         ('display_localise_media_path', 'display_localise_media_path', '0x5680f0', 'display.cpp', 'cdecl',
          'acerta um caminho de foto ou som vindo de outro PC: se o arquivo não existe, o resto depois da última pasta "pictures" (ou "sounds"; senão "data"), com ":" virando "\\", vai para a pasta local daquele tipo',
          'fixes a picture or sound path from another PC: when the file does not exist, the rest after the last "pictures" folder (or "sounds"; else "data"), with ":" turned into "\\", goes to the local folder of that kind'),
         ('display_log_screen_sent', 'Display::log_screen_sent', '0x5622a0', 'display.cpp', 'thiscall, ret 8',
          'envia as seções da tela a um jogador de rede (a exibição; as áreas, os itens e o desenho quando não é reenvio) e escreve a linha "Screen sent" com os tamanhos',
          'sends the screen sections to a network player (the display; the areas, the widgets and the draw when it is not a resend) and writes the "Screen sent" line with the sizes')])
