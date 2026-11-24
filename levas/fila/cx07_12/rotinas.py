# A base grafica: a passagem para a tela, as conversoes de cor, as medidas das fontes, os escapes de texto, uma letra e
# as imagens da tela (goldcup.cpp pelo mapa; a biblioteca grafica; leitor cx07, rodada 5, 24/11/2026). Item de TELAS:
# rodam em todo desenho de tela. Caminho no jogo: abrir o jogo comparando e navegar pelas telas (as conversoes e as
# medidas sao comparadas lado a lado a cada texto; as caixas de mensagem salvam e restauram a tela).
# Convencoes conferidas: todas cdecl.
# - 0x5e2eb0 graphics_flip_rect(x0, y0, x1, y1): clip_rect_to_surface, gui_greyscale_rect 0x5e7f60 (com [0xaebe28]),
#   ShowCursor em volta com o cursor por software ([0xaebe14]) sobre o retangulo, BltFast (slot 0x1c, 0x10) em tela cheia
#   ou GetWindowRect (0x9671a8) e Blt (slot 0x14, 0x1000000) em janela; [0xaebe10] = 1. Prologo mov eax, [0xaebe0c].
# - 0x5e4800 graphics_rgb_to_pixel(r, g, b, formato) -> u16; 0x5e49f0 gui_colour_to_rgb(cor, &r, &g, &b, formato);
#   0x5e4890 gui_scale_colour(cor, por_cento, formato) -> u16 (a divisao por 100 com sinal pelo 0x51eb851f). Os formatos
#   nulos usam a copia do DDPIXELFORMAT da tela em 0xae3158 (+0x10 vermelho, +0x14 verde, +0x18 azul).
# - 0x5e5cd0 font_get_height(fonte) -> u32: com [0x9cc2c4] != -1 a tabela de salto 0x5e5d38 (0xf, 0xf, 0x12, 0x15, 0x18,
#   0x1b, 0x27, 0x2d); senao +0 da fonte na tabela 0xae1dac. ALTERADA no cm0102-1080.exe (50 bytes: as alturas da
#   resolucao): EXE original (so no cm0102.exe). Prologo cmp dword [0x9cc2c4], -1 (7 bytes).
# - 0x5e5d60 font_glyph_gap(fonte, texto, indice) -> i32: +0x284 da fonte x 3 / 4 menos +0 da letra e +4 da anterior
#   (as letras em +0xc + 0x14c; '|' conta como espaco). Prologo mov cx, [esp+4] (5 bytes).
# - 0x5e5aa0 text_caret_glyph_callback(x, ...) -> 0: a chamada de volta da linha T2K (sx10_14). Prologo mov eax,
#   [0xae3178] (5 bytes).
# - 0x5e5060 text_decode_hash_escapes(saida, texto): strlen inline, isxdigit 0x947063 e sscanf 0x946fd8 (cdecl) com
#   "#%4x#". Prologo sub esp, 8; push ebp; mov ebp, [esp+0x14] (8 bytes).
# - 0x5e3ce0 gui_draw_glyph(x, y, cor, letra); 0x5e3dd0 gui_save_screen_image(x0, y0, x1, y1, imagem) -> imagem;
#   0x5e3fd0 gui_restore_screen_image(x, y, imagem): Lock (slot 0x64, 0x21 ou 0x11) e Unlock (0x80) do buffer de tras;
#   calloc 0x945501 e free 0x9452ca (cdecl). Prologo mov eax, [0xaebe0c] (5 bytes).
# Defeitos do original: (1) text_decode_hash_escapes devolve a contagem, mas o ponteiro de saida quando o texto e vazio;
# (2) gui_colour_to_rgb devolve o ultimo quociente (ou a marca do video desligado) sem que ninguem o use.
# Iguais no cm0102-1080.exe, menos o font_get_height.
# Depende do cx07_11 (os globais do video, FONT_TABLE e companhia, user32_show_cursor, clip_rect_to_surface,
# rect_overlap_area, gui_free_image): integrar depois dele (a mesma leva de telas).
# Comparadores: lado a lado nas conversoes, nas medidas, na chamada de volta (o contador e o x restaurados entre as
# duas) e nos escapes; so a original no que desenha ou aloca.
SAVE = False
TELAS = True
EXE = {0x5e5cd0: 'original'}
ROTINAS = [('graphics_flip_rect', 0x5e2eb0, [0xa1, 0x0c, 0xbe, 0xae, 0x00]),
           ('graphics_rgb_to_pixel', 0x5e4800, [0xa1, 0x0c, 0xbe, 0xae, 0x00]),
           ('gui_colour_to_rgb', 0x5e49f0, [0xa1, 0x0c, 0xbe, 0xae, 0x00]),
           ('gui_scale_colour', 0x5e4890, [0x51, 0xa1, 0x0c, 0xbe, 0xae, 0x00]),
           ('font_get_height', 0x5e5cd0, [0x83, 0x3d, 0xc4, 0xc2, 0x9c, 0x00, 0xff]),
           ('font_glyph_gap', 0x5e5d60, [0x66, 0x8b, 0x4c, 0x24, 0x04]),
           ('text_caret_glyph_callback', 0x5e5aa0, [0xa1, 0x78, 0x31, 0xae, 0x00]),
           ('text_decode_hash_escapes', 0x5e5060, [0x83, 0xec, 0x08, 0x55, 0x8b, 0x6c, 0x24, 0x14]),
           ('gui_draw_glyph', 0x5e3ce0, [0xa1, 0x0c, 0xbe, 0xae, 0x00]),
           ('gui_save_screen_image', 0x5e3dd0, [0xa1, 0x0c, 0xbe, 0xae, 0x00]),
           ('gui_restore_screen_image', 0x5e3fd0, [0xa1, 0x0c, 0xbe, 0xae, 0x00])]
DOCS = ('a base gráfica: passagem para a tela, cores, medidas das fontes, escapes de texto, letras e imagens da tela',
        'the graphics base: screen flip, colours, font metrics, text escapes, glyphs and screen images',
        [('graphics_flip_rect', 'graphics_flip_rect', '0x5e2eb0', 'goldcup.cpp', 'cdecl',
          'copia um retângulo da superfície de trás para a tela (em cinza com a marca do fundo cinzento), escondendo o cursor por software sobre ele, e marca a tela como atualizada',
          'copies a rectangle of the back buffer to the screen (grey with the greyscale flag), hiding the software cursor over it, and marks the screen as updated'),
         ('graphics_rgb_to_pixel', 'graphics_rgb_to_pixel', '0x5e4800', 'goldcup.cpp', 'cdecl',
          'junta vermelho, verde e azul num pixel de 16 bits no formato 565 ou 555',
          'packs red, green and blue into a 16-bit pixel in the 565 or 555 format'),
         ('gui_colour_to_rgb', 'gui_colour_to_rgb', '0x5e49f0', 'goldcup.cpp', 'cdecl',
          'separa uma cor de 16 bits em vermelho, verde e azul de 8 bits pelas máscaras do formato',
          'splits a 16-bit colour into 8-bit red, green and blue through the format masks'),
         ('gui_scale_colour', 'gui_scale_colour', '0x5e4890', 'goldcup.cpp', 'cdecl',
          'escala cada canal de uma cor por uma porcentagem, limitado a 255, e junta de novo',
          'scales each channel of a colour by a percentage, capped at 255, and packs it again'),
         ('font_get_height', 'font_get_height', '0x5e5cd0', 'goldcup.cpp', 'cdecl',
          'a altura da linha de uma fonte: fixa por fonte com as fontes de base, senão a da fonte carregada',
          'the line height of a font: fixed per font with the built-in fonts, otherwise the loaded font\'s'),
         ('font_glyph_gap', 'font_glyph_gap', '0x5e5d60', 'goldcup.cpp', 'cdecl',
          'o espaço entre uma letra e a anterior numa fonte de mapa de bits, nunca negativo',
          'the spacing between a glyph and the one before it in a bitmap font, never negative'),
         ('text_caret_glyph_callback', 'text_caret_glyph_callback', '0x5e5aa0', 'goldcup.cpp', 'cdecl',
          'conta as letras de uma linha T2K e guarda o x da letra do cursor de texto',
          'counts the glyphs of a T2K line and records the x of the caret\'s glyph'),
         ('text_decode_hash_escapes', 'text_decode_hash_escapes', '0x5e5060', 'goldcup.cpp', 'cdecl',
          'converte um texto de bytes em códigos de 16 bits, trocando cada #xxxx# pelo código hexadecimal',
          'converts a byte string into 16-bit codes, turning each #xxxx# into its hexadecimal code'),
         ('gui_draw_glyph', 'gui_draw_glyph', '0x5e3ce0', 'goldcup.cpp', 'cdecl',
          'desenha uma letra de uma cor na superfície de trás, pondo a cor onde a máscara não é zero',
          'draws a one-colour glyph on the back buffer, putting the colour where the mask is non-zero'),
         ('gui_save_screen_image', 'gui_save_screen_image', '0x5e3dd0', 'goldcup.cpp', 'cdecl',
          'copia uma caixa da tela para uma imagem, reaproveitando a imagem dada quando o tamanho bate',
          'copies a screen box into an image, reusing the given image when its size matches'),
         ('gui_restore_screen_image', 'gui_restore_screen_image', '0x5e3fd0', 'goldcup.cpp', 'cdecl',
          'devolve uma imagem salva à superfície de trás numa posição',
          'copies a saved image back onto the back buffer at a position')])
CAMPOS = []
INTERESSA = []
