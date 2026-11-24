# A base grafica: a linha, a caixa escurecida, as figuras RGN, as fontes de mapa de bits, a largura do texto e o
# esmaecer (goldcup.cpp pelo mapa; a biblioteca grafica; leitor sbC, rodada 5, 24/11/2026, as rotinas que o cx07 deixou
# em levas/fila/cx07_14/FORA.md). Item de TELAS: rodam em todo desenho de tela (a linha nas molduras e nos
# sublinhados, a caixa escurecida atras das janelas, a largura em todo texto alinhado), na troca de fundo (as figuras
# RGN), na abertura do video (as fontes, pelo gui_load_fonts do cx07_13) e no esmaecer da tela (Display::draw_screen).
# Caminho no jogo: abrir o jogo comparando, navegar por algumas telas com fundo, abrir um menu ou uma janela por cima
# (a caixa escurecida) e voltar.
# Convencoes conferidas (todas cdecl, chamadas com add esp do chamador):
# - 0x5e3730 graphics_draw_line(x0, y0, x1, y1, estilo, cor): clip_rect_to_surface 0x5e3680; Lock (slot 0x64, 0x21) e
#   Unlock (0x80) do buffer de tras. Prologo mov eax, [0xaebe0c] (5 bytes).
# - 0x5e42e0 gui_darken_rect(x0, y0, x1, y1): a tabela de 0x10000 palavras em 0xaebe30 (60 por cento, a ultima palavra
#   zero = por montar). Prologo mov eax, [0xaebe0c].
# - 0x5e40d0 gui_load_rgn_picture(caminho, reaproveitar) -> imagem: fopen 0x945b00 ("rb" 0x98f928), fread 0x945bfe,
#   fclose 0x945a09, calloc 0x945501, free 0x9452ca. Prologo mov eax, [0xaebe0c].
# - 0x5e4ba0 gui_load_bitmap_font(vaga i16, caminho) -> 1/0: malloc 0x944e46; sprintf 0x944c9f so na falha do fopen
#   (num buffer local que ninguem le: a copia nao formata). Prologo sub esp, 0x238 (6 bytes).
# - 0x5e5ad0 font_text_width(fonte i16, texto) -> largura: text_decode_hash_escapes 0x5e5060 (cx07_12),
#   FONT::get_text_width 0x5b18a0 (thiscall(fonte, flags 0, tamanho, codigos) ret 0xc; o ax com sinal), a tabela de
#   salto 0x5e5cac (8 entradas, conferidas), font_glyph_gap 0x5e5d60 (cx07_12). Prologo mov eax, 0x2004 (5 bytes; o
#   call _chkstk vem depois).
# - 0x5e30e0 graphics_fade_rect(x0, y0, x1, y1, passos): gui_save_screen_image 0x5e3dd0, malloc, Lock 0x21,
#   graphics_flip_rect 0x5e2eb0, free. Prologo mov eax, 0x400c0 (5 bytes; o call _chkstk depois).
# Defeitos do original: (1) gui_load_bitmap_font deixa na vaga os ponteiros dos desenhos que liberou na falha (o
# desligamento os libera de novo) e nao confere o malloc da copia das letras acentuadas; (2) font_text_width decodifica
# o texto num buffer de pilha de 0x1000 codigos sem conferir o tamanho (a copia mede o texto); (3) graphics_fade_rect
# sai na falha de um malloc ou de um Lock sem liberar as tabelas e a imagem salva.
# Iguais no cm0102-1080.exe (todas inteiras).
# Depende do cx07_11 (DISPLAY_OFF, BACK_BUFFER, SCREEN_WIDTH/HEIGHT, FONT_TABLE, FONT_SLOT_SIZE, FONT_GLYPH_SIZE,
# clip_rect_to_surface, gui_free_image), do cx07_12 (SCREEN_PIXEL_FORMAT, SURFACE_DESC_*, lock_back_buffer,
# unlock_back_buffer, gui_scale_colour, text_decode_hash_escapes, font_glyph_gap, gui_save_screen_image,
# gui_restore_screen_image) e do cx07_13 (nada direto; a mesma leva de telas).
# Comparadores: a linha e a caixa escurecida lado a lado sobre o buffer de tras (compare_drawing_in_box: a caixa salva,
# a original desenha, a caixa volta, a copia desenha e os pixels sao comparados; na diferenca fica o desenho da
# original); a figura RGN lado a lado (a copia le o arquivo de novo e libera a sua imagem); a largura lado a lado; a
# carga da fonte e o esmaecer so a original. O compare_drawing_in_box e os auxiliares (lock_back_buffer_pixels,
# pixel_at, bitmap_font_glyph, T2K_FONT_SIZES) sao usados pelos sbC_2 e sbC_3.
SAVE = False
TELAS = True
ROTINAS = [('graphics_draw_line', 0x5e3730, [0xa1, 0x0c, 0xbe, 0xae, 0x00]),
           ('gui_darken_rect', 0x5e42e0, [0xa1, 0x0c, 0xbe, 0xae, 0x00]),
           ('gui_load_rgn_picture', 0x5e40d0, [0xa1, 0x0c, 0xbe, 0xae, 0x00]),
           ('gui_load_bitmap_font', 0x5e4ba0, [0x81, 0xec, 0x38, 0x02, 0x00, 0x00]),
           ('font_text_width', 0x5e5ad0, [0xb8, 0x04, 0x20, 0x00, 0x00]),
           ('graphics_fade_rect', 0x5e30e0, [0xb8, 0xc0, 0x00, 0x04, 0x00])]
DOCS = ('a base gráfica: a linha, a caixa escurecida, as figuras RGN, as fontes de mapa de bits, a largura do texto e o esmaecer',
        'the graphics base: the line, the darkened box, the RGN pictures, the bitmap fonts, the text width and the fade',
        [('graphics_draw_line', 'graphics_draw_line', '0x5e3730', 'goldcup.cpp', 'cdecl',
          'desenha uma linha cheia (bit 1 do estilo) ou tracejada (4 pixels sim, 2 não) na superfície de trás, só quando a caixa dela cabe inteira na tela; a horizontal e a vertical percorrem a caixa, as outras por Bresenham a partir da ponta de menor coordenada',
          'draws a solid line (bit 1 of the style) or a dashed one (4 pixels on, 2 off) on the back buffer, only when its box fits whole on the screen; horizontal and vertical lines walk the box, the others by Bresenham from their lower end'),
         ('gui_darken_rect', 'gui_darken_rect', '0x5e42e0', 'goldcup.cpp', 'cdecl',
          'escurece uma caixa da tela para 60 por cento, pela tabela de todas as cores montada no primeiro uso',
          'darkens a screen box to 60 percent, through the table of every colour built on the first use'),
         ('gui_load_rgn_picture', 'gui_load_rgn_picture', '0x5e40d0', 'goldcup.cpp', 'cdecl',
          'lê uma figura RGN numa imagem (reaproveitando a dada quando o tamanho bate) e converte os pixels entre 555 e 565 quando o formato do arquivo não é o da tela',
          'reads an RGN picture into an image (reusing the given one when the size matches) and converts the pixels between 555 and 565 when the file format is not the screen\'s'),
         ('gui_load_bitmap_font', 'gui_load_bitmap_font', '0x5e4ba0', 'goldcup.cpp', 'cdecl',
          'carrega um arquivo de fonte de mapa de bits na vaga dela (a altura e, de 0x20 a 0xff, a largura, os bytes da linha, os dois apoios e o desenho de cada letra) e, nas duas fontes Trade, copia letras para as acentuadas; defeito: na falha os ponteiros liberados ficam na vaga',
          'loads a bitmap font file into its slot (the height and, from 0x20 to 0xff, each glyph\'s width, row bytes, two bearings and bitmap) and, in the two Trade fonts, copies glyphs to the accented characters; defect: on a failure the freed pointers stay in the slot'),
         ('font_text_width', 'font_text_width', '0x5e5ad0', 'goldcup.cpp', 'cdecl',
          'a largura em pixels de um texto numa fonte: pela fonte T2K e o tamanho da fonte de base, ou somando a largura e o espaço de cada letra da fonte de mapa de bits (a barra conta como espaço)',
          'the width in pixels of a text in a font: through the T2K font and size of the built-in font, or adding each glyph\'s width and gap in the bitmap font (the bar counts as a space)'),
         ('graphics_fade_rect', 'graphics_fade_rect', '0x5e30e0', 'goldcup.cpp', 'cdecl',
          'faz uma caixa da tela surgir do preto (passos positivos) ou sumir no preto (negativos), quadro a quadro, com uma tabela de tons por cor',
          'fades a screen box in from black (positive steps) or out to black (negative), frame by frame, with a table of shades per colour')])
CAMPOS = []
INTERESSA = []
