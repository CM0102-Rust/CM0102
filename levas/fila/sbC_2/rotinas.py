# A base grafica: uma linha de texto, a caixa de texto e o painel (goldcup.cpp pelo mapa; a biblioteca grafica; leitor
# sbC, rodada 5, 24/11/2026, as rotinas que o cx07 deixou em levas/fila/cx07_14/FORA.md). Item de TELAS: o painel e a
# caixa de texto desenham as caixas de mensagem, a barra de progresso de baixo e a caixa da rede; a linha de texto e
# chamada so pela caixa de texto. Caminho no jogo: abrir o jogo comparando, salvar o jogo (a barra de progresso de baixo
# com o texto) e provocar uma caixa de mensagem (por exemplo, sair pelo X da janela: "Please use the Exit Game option").
# Convencoes conferidas (todas cdecl):
# - 0x5e5160 gui_draw_text_line(x, y, fonte i16, cor, texto, cursor): text_decode_hash_escapes 0x5e5060 (cx07_12);
#   Lock 0x21 da tela inteira e FONT::draw_text 0x5b0d80 (thiscall(fonte, &alvo, x, y + d, flags, tamanho, cor, codigos,
#   callback) ret 0x20; o alvo {pixels, passo em pixels (o ax com sinal), largura i16, altura i16}; tabela de salto
#   0x5e5a3c, 8 entradas conferidas: fontes 0..7 = (0xaebdf4, +3, 0x200, 9), (0xaebdf8, +3, 0x200, 9), (0xaebdf8, +4,
#   0x200, 10), (0xaebdfc, +4, 0x200, 14/16/18), (0xaebe00, +9, 0x280, 20), (0xaebe00, +0xb, 0x280, 22));
#   text_caret_glyph_callback 0x5e5aa0 (cx07_12) como callback; o cursor por graphics_draw_line 0x5e3730 (estilo 2) com
#   as alturas das tabelas 0x5e5a5c/0x5e5a7c (0xf, 0xf, 0x12, 0x15, 0x18, 0x1b, 0x27, 0x2d; iguais no 1080, ao contrario
#   do font_get_height); no modo -1 font_glyph_gap 0x5e5d60. Prologo push -1; mov eax, fs:[0] (8 bytes, fronteira de
#   instrucao; o quadro SEH do original nao tem objeto a destruir na copia).
# - 0x5e6d80 gui_draw_text_box(x1, y1, x2, y2, flags, fonte i16, cor, texto, cursor): operator new 0x944cf1 (0xe),
#   LLIST::LLIST 0x68d020, add_tail 0x68d0d0 (thiscall ret 8), get_next 0x68d1b0, ~LLIST 0x68d040, operator delete
#   0x944c94; sprintf 0x944c9f ("%c", "%c%c%c%c%c%c"), font_text_width 0x5e5ad0, gui_draw_text_line 0x5e5160; tabelas
#   de salto das alturas 0x5e76ac e 0x5e76cc (as mesmas). Prologo push -1; push 0x95a91e (7 bytes).
# - 0x5e5df0 gui_draw_panel(x1, y1, x2, y2, flags, cor, extra): gui_darken_rect 0x5e42e0, gui_draw_circle 0x5e7840
#   (ALTERADA no 1080: chamada sempre pelo endereco), graphics_draw_rect 0x5e3b50, graphics_draw_line 0x5e3730; Lock
#   0x11 para ler o pixel (flag 0x1000). Prologo mov eax, [0xaebe0c] (5 bytes). Rotina grande (3,9 KB) em pedacos com
#   nome: o preenchimento, o contorno, o chanfro.
# Defeitos do original: (1) gui_draw_text_box procura o corte da linha pela posicao no texto, nao na linha: da segunda
# linha em diante le alem da linha, bytes de pilha que o original nunca escreveu (na copia o buffer e zero ali, e a
# busca volta ao ultimo espaco da linha); (2) gui_draw_text_line e o font_text_width decodificam num buffer de pilha de
# 0x1000 codigos sem conferir o tamanho; (3) a mistura da fonte de mapa de bits guarda a cor resultante e reaproveita o
# fundo anterior quando o pixel seguinte e igual a ela; (4) o degrade do painel com uma coluna (ou linha) so divide por
# zero; (5) o chanfro e o sulco do painel clareiam a cor dada (ou o azul escuro quando ela e zero), mas escurecem sempre
# a cor dada (preto quando ela e zero); (6) o chanfro de 4 pixels exige o painel acima da linha 100 (y2 < 0x64).
# Iguais no cm0102-1080.exe (todas inteiras).
# Depende do sbC_1 (compare_drawing_in_box, lock_back_buffer_pixels, bitmap_font_glyph, FONT_GLYPH_RECORDS,
# GLYPH_WIDTH, GLYPH_BITMAP, T2K_FONT_SIZES) e dos cx07_11, cx07_12 (CARET_*, BUILT_IN_FONT_HEIGHTS, pack_pixel,
# unpack_pixel, gui_scale_colour, text_decode_hash_escapes, font_glyph_gap) e cx07_13.
# Comparadores: a linha de texto e a caixa de texto so a original (o texto nao tem caixa fixa, e um texto misturado
# desenhado duas vezes sobre si mesmo muda); o painel lado a lado sobre o buffer de tras (compare_drawing_in_box, um
# pixel em volta pela moldura de foco).
SAVE = False
TELAS = True
ROTINAS = [('gui_draw_text_line', 0x5e5160, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('gui_draw_text_box', 0x5e6d80, [0x6a, 0xff, 0x68, 0x1e, 0xa9, 0x95, 0x00]),
           ('gui_draw_panel', 0x5e5df0, [0xa1, 0x0c, 0xbe, 0xae, 0x00])]
DOCS = ('a base gráfica: uma linha de texto, a caixa de texto e o painel',
        'the graphics base: a text line, the text box and the panel',
        [('gui_draw_text_line', 'gui_draw_text_line', '0x5e5160', 'goldcup.cpp', 'cdecl',
          'desenha uma linha de texto numa posição, pela fonte T2K da fonte de base ou letra a letra pela fonte de mapa de bits (misturando a cor com o fundo pela cobertura de 4 bits), com o cursor de texto como uma linha da altura da fonte; defeito: a mistura reaproveita o fundo anterior quando o pixel é igual à cor que ela acabou de produzir',
          'draws a text line at a position, through the T2K font of the built-in font or glyph by glyph in the bitmap font (mixing the colour with the background by the 4-bit coverage), with the text caret as a line of the font\'s height; defect: the mix reuses the previous background when the pixel equals the colour it just produced'),
         ('gui_draw_text_box', 'gui_draw_text_box', '0x5e6d80', 'goldcup.cpp', 'cdecl',
          'quebra um texto em linhas que cabem na largura da caixa (cortando no último espaço, com 0x100 seguindo na linha de baixo, sem ele parando na borda até a próxima quebra de linha) e desenha cada linha alinhada à esquerda (1), à direita (0x40) ou ao centro, do alto (2) ou centrada na vertical, com asteriscos (0x80) ou em relevo (0x20); defeito: o corte da segunda linha em diante é procurado pela posição no texto, não na linha',
          'word-wraps a text into lines that fit the box width (cut at the last space, going on below with 0x100, without it stopping at the edge until the next newline) and draws each line aligned left (1), right (0x40) or centred, from the top (2) or vertically centred, as asterisks (0x80) or engraved (0x20); defect: from the second line on the cut is searched by the position in the text, not in the line'),
         ('gui_draw_panel', 'gui_draw_panel', '0x5e5df0', 'goldcup.cpp', 'cdecl',
          'desenha um painel pelas flags: o recuo das bordas, a cor tirada da tela, o preenchimento (escurecido, cheio, elipse ou degradê), o contorno (elipse, retângulo ou chanfro de 2 ou 4 pixels, sublinhado ou linha no meio), o sulco no meio e a moldura amarela de foco; defeitos: o degradê de uma coluna divide por zero, e o chanfro escurece a cor dada mesmo quando clareia o azul escuro no lugar dela',
          'draws a panel by its flags: the edges pulled in, the colour taken from the screen, the fill (darkened, solid, ellipse or gradient), the outline (ellipse, rectangle or a 2 or 4-pixel bevel, underline or a line across), the groove halfway down and the yellow focus frame; defects: a one-column gradient divides by zero, and the bevel darkens the given colour even when it lightens the dark blue in its place')])
CAMPOS = []
INTERESSA = []
