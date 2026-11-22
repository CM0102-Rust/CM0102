# A base da interface (7): o objeto de widget (Guio.cpp; leitor tel38b, rodada das telas, 21/11/2026): preencher um
# widget novo e desenhá-lo.
# Usa os auxiliares do tel26_1 (display_list, display_item, display_area, exe_immediate, DISPLAY_TICKS), do tel29_1
# (copy_c_string, c_string_len, STR_EMPTY) e do tel38_3 (guio_fields, guio_same_c_string, gui_restore_background,
# GUIO_SHARED_IMAGE_NAME/COUNT): vem depois deles.
# Convencoes conferidas na desmontagem (0x5ee150..0x5ee494, 0x5ee590..0x5eec1a): Guio::init thiscall(widget, areas,
# itens, indice, os 18 campos do Display::add_item) ret 0x54; Guio::draw thiscall(widget, refresh) ret 4, sem retorno.
# Chamadas: font_text_width 0x5e5ad0 e font_get_height 0x5e5cd0 cdecl; sprintf 0x944c9f; gui_scale_colour 0x5e4890
# cdecl(cor, 110, 0) -> ax; gui_free_image 0x5e42b0; gui_save_screen_image 0x5e3dd0 cdecl(x1, y1, x2, y2, 0);
# gui_load_rgn_picture 0x5e40d0 cdecl(caminho, 0); gui_restore_screen_image 0x5e3fd0 cdecl(x, y, imagem);
# gui_draw_panel 0x5e5df0 cdecl(x1, y1, x2, y2, estilo, cor, cor do texto); Area::get_popup_position 0x403760
# thiscall(area, &x, &y) ret 8; gui_draw_glyph 0x5e3ce0 cdecl(x, y, cor, glifo); gui_draw_text_box 0x5e6d80 cdecl(x1,
# y1, x2, y2, marcas, fonte, cor, texto, cursor). As cores e a fonte vao como words (as metades altas do original sao
# sobras).
# O cm0102-1080.exe reescreve os limites da tela do Guio::init (0x5ee1f6: 0x31f -> 0x77f; 0x5ee20b: 0x257 -> 0x437): a
# copia le cada imediato do exe em execucao (exe_immediate). O Guio::draw e igual no 1080.
# Defeitos do original: o texto do widget e copiado sem limite (0x100 bytes no widget); com uma area de 0 colunas o
# calculo da coluna divide por zero.
# Comparadores: Guio::init lado a lado no widget e na area (a largura, a altura, o passeio, as linhas); Guio::draw so a
# original (desenha e guarda ou libera imagens), conferindo o plano: a marca de linha impar apagada e o texto como
# estava (o byte 1 de volta).
TELAS = True
ROTINAS = [('guio_init', 0x5ee150, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x14]),
           ('guio_draw', 0x5ee590, [0x83, 0xec, 0x0c, 0x53, 0x55])]
DOCS = ('a base da interface (7): o widget: preencher um widget novo e desenhá-lo',
        'the interface base (7): the widget: filling a new widget and drawing it',
        [('guio_init', 'Guio::init', '0x5ee150', 'Guio.cpp', 'thiscall, ret 0x54',
          'preenche um widget novo: o retângulo posto em ordem e dentro da tela (799 × 599; 1919 × 1079 no exe 1080); numa área que recua, o texto com cinco espaços na frente, a coluna seguinte à do último item, a linha (as ímpares escurecem no desenho) e a área crescida para caber; o resto dos campos como vêm',
          'fills a new widget: its rectangle put in order and inside the screen (799 × 599; 1919 × 1079 in the 1080 exe); in an indenting area, the text with five spaces in front, the column after the last item\'s, the row (odd ones are darkened when drawn) and the area grown to fit; the other fields as they come'),
         ('guio_draw', 'Guio::draw', '0x5ee590', 'Guio.cpp', 'thiscall, ret 4',
          'desenha um widget: o fundo guardado, a figura (compartilhada entre widgets do mesmo nome), o painel (afundado 2 pixels quando destacado), a seta da área suspensa que ele abre, as marcas e o texto',
          'draws a widget: the background saved, its picture (shared by widgets of the same name), the panel (pressed 2 pixels when highlighted), the arrow of the popup area it opens, its marks and its text')])
