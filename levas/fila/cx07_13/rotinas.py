# A base grafica: as cores da interface, os retangulos, a caixa cinzenta e a carga das fontes (goldcup.cpp pelo mapa; a
# biblioteca grafica; leitor cx07, rodada 5, 24/11/2026). Item de TELAS: as cores e as fontes na abertura do video
# (graphics_init_display, ainda no original), os retangulos e o cinza nas telas (o cinza com a marca [0xaebe28], ao
# passar para a tela). Caminho no jogo: abrir o jogo comparando (as cores sao comparadas lado a lado na abertura) e
# navegar pelas telas.
# Convencoes conferidas: todas cdecl.
# - 0x5e4560 gui_init_palette_colours(): 9 palavras fixas por formato (565 com o verde 0x7e0 em 0xae316c, senao 555) e 16
#   por graphics_rgb_to_pixel 0x5e4800 (cdecl, 4 pushes, formato 0). ALTERADA no cm0102-1080.exe (3 bytes: 0x5e4588,
#   0x5e4712): EXE original. Prologo mov eax, [0xaebe0c] (5 bytes).
# - 0x5e3b50 graphics_draw_rect(x0, y0, x1, y1, estilo, cor): com o bit 0 ou 1 do estilo quatro graphics_draw_line
#   0x5e3730 (cdecl, 6 pushes); senao clip_rect_to_surface e Blt (slot 0x14) com 0x1000400 e o DDBLTFX (+0 = 0x64, +0x50
#   a cor). Prologo mov eax, [0xaebe0c].
# - 0x5e7f60 gui_greyscale_rect(x0, y0, x1, y1): Lock (0x64, 0x21) e Unlock (0x80); a media (r + g + b) / 3 pelo
#   0x55555556. Prologo mov eax, [0xaebe0c].
# - 0x5e4a60 gui_load_fonts(modo): font_set_mode 0x5b1f30 (cdecl), FONT::load_font_file 0x5b0b60 (push tamanho; mov ecx,
#   fonte -> thiscall, ret 4; ja em fila no sx10_13) nas quatro T2K, utils_build_file_path 0x90bde0 (cdecl, "SI_DATA") e
#   gui_load_bitmap_font 0x5e4ba0 (cdecl, 2 pushes; add esp, 0x10 limpa as quatro) nas sete. Prologo mov eax, [0xaebe0c].
# Defeito do original: o DDBLTFX do preenchimento so tem o tamanho e a cor; o resto e lixo da pilha (o Blt de cor so le
#   esses dois; a copia zera o resto).
# Iguais no cm0102-1080.exe, menos o gui_init_palette_colours.
# Depende do cx07_11 (DISPLAY_OFF, BACK_BUFFER, clip_rect_to_surface) e do cx07_12 (SCREEN_PIXEL_FORMAT, pack_pixel,
# unpack_pixel, lock_back_buffer, unlock_back_buffer, graphics_rgb_to_pixel): integrar depois deles (a mesma leva).
# Comparadores: as cores lado a lado nas 25 palavras; so a original no que desenha ou carrega.
SAVE = False
TELAS = True
EXE = {0x5e4560: 'original'}
ROTINAS = [('gui_init_palette_colours', 0x5e4560, [0xa1, 0x0c, 0xbe, 0xae, 0x00]),
           ('graphics_draw_rect', 0x5e3b50, [0xa1, 0x0c, 0xbe, 0xae, 0x00]),
           ('gui_greyscale_rect', 0x5e7f60, [0xa1, 0x0c, 0xbe, 0xae, 0x00]),
           ('gui_load_fonts', 0x5e4a60, [0xa1, 0x0c, 0xbe, 0xae, 0x00])]
DOCS = ('a base gráfica: cores da interface, retângulos, caixa cinzenta e fontes',
        'the graphics base: interface colours, rectangles, grey box and fonts',
        [('gui_init_palette_colours', 'gui_init_palette_colours', '0x5e4560', 'goldcup.cpp', 'cdecl',
          'acerta as palavras das cores da interface para o formato de pixel da tela (565 ou 555): nove fixas e dezesseis montadas de vermelho, verde e azul',
          'sets the interface colour words for the screen pixel format (565 or 555): nine fixed and sixteen packed from red, green and blue'),
         ('graphics_draw_rect', 'graphics_draw_rect', '0x5e3b50', 'goldcup.cpp', 'cdecl',
          'desenha o contorno de um retângulo com quatro linhas tracejadas (estilo 1 ou 2) ou o preenche com uma cor',
          'draws a rectangle outline as four dashed lines (style 1 or 2) or fills it with a colour'),
         ('gui_greyscale_rect', 'gui_greyscale_rect', '0x5e7f60', 'goldcup.cpp', 'cdecl',
          'deixa cinza cada pixel de uma caixa da tela pela média de vermelho, verde e azul',
          'turns every pixel of a screen box grey by averaging its red, green and blue'),
         ('gui_load_fonts', 'gui_load_fonts', '0x5e4a60', 'goldcup.cpp', 'cdecl',
          'carrega as fontes: o modo de fonte, os quatro tamanhos T2K e os sete arquivos de fonte de mapa de bits do SI_DATA, parando no primeiro erro',
          'loads the fonts: the font mode, the four T2K sizes and the seven bitmap font files of SI_DATA, stopping at the first error')])
CAMPOS = []
INTERESSA = []
