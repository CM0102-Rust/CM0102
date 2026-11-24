# A base grafica: a caixa de progresso de baixo, o circulo e os pontos dele (goldcup.cpp pelo mapa; a biblioteca
# grafica; leitor sbC, rodada 5, 24/11/2026, as rotinas que o cx07 deixou em levas/fila/cx07_14/FORA.md). Item de TELAS,
# so no cm0102.exe: as tres sao ALTERADAS no cm0102-1080.exe (a caixa chama copias deslocadas 0x601f00/0x601f0a/
# 0x601f14/0x601f1e; o circulo trava 1920 x 1080 e comeca os vaos em 1920; os pontos usam o passo 1920 por imul em vez
# de lea/lea/shl), entao EXE = original nas tres e os chamadores as chamam sempre pelo endereco (o gui_draw_panel do
# sbC_2 chama o circulo por 0x5e7840). A caixa de progresso roda ao salvar e carregar o jogo e na rede; o circulo, nos
# paineis de elipse (flag 0x10000) da tela de jogo (Display, 0x561210).
# Caminho no jogo (com o cm0102.exe, nao o 1080): abrir o jogo comparando, salvar o jogo (a caixa de progresso com a
# barra) e abrir uma partida com a tela de jogo (os circulos dos paineis de elipse).
# Convencoes conferidas (todas cdecl):
# - 0x5e76f0 gui_draw_progress_box(texto, passo, total) -> 1/0: pump_messages 0x68d520; com passo 0
#   Display::clear_screen_once 0x55f2c0 (thiscall, ja em Rust: display_clear_screen_once) do Display do humano atual
#   ([0xb74c4c + 0x300 x [0xb77c7a]]); gui_draw_panel 0x5e5df0 (7 pushes), gui_draw_text_box 0x5e6d80 (9),
#   graphics_draw_rect 0x5e3b50 (6), graphics_flip_rect 0x5e2eb0 (4). Prologo mov eax, [0xaebe0c] (5 bytes).
# - 0x5e7840 gui_draw_circle(x1, y1, x2, y2, estilo, cor): operator new 0x944cf1 e delete 0x944c94 (os vaos de 8
#   bytes), Lock 0x21 de (0, 0, 800, 600), circle_plot_points 0x5e7aa0 (9 pushes), graphics_draw_line 0x5e3730.
#   Prologo sub esp, 0x94 (6 bytes).
# - 0x5e7aa0 circle_plot_points(pixels, dx, dy, esquerda, topo, direita, base, cor, vaos). Prologo sub esp, 0x18;
#   push ebx; push ebp (5 bytes).
# Defeitos do original: (1) os pontos do circulo usam o passo fixo de 800 pixels, nao o da superficie; (2) o circulo nao
# confere o video nem o buffer de tras antes do Lock e, na falha do Lock, nao libera os vaos.
# Depende do sbC_1 (compare_drawing_in_box, lock_back_buffer_pixels) e do sbC_2 (nada direto: a caixa chama o painel e a
# caixa de texto pelo endereco), dos cx07_11/cx07_12 (DISPLAY_OFF, unlock_back_buffer) e do display.rs
# (display_clear_screen_once, current_human_display, ja publicados).
# Comparadores: a caixa de progresso so a original (bombeia as mensagens e passa para a tela); o circulo lado a lado
# sobre o buffer de tras (compare_drawing_in_box; a copia chama a copia dos pontos, provada junto); os pontos so a
# original (escrevem numa superficie travada).
SAVE = False
TELAS = True
EXE = {0x5e76f0: 'original', 0x5e7840: 'original', 0x5e7aa0: 'original'}
ROTINAS = [('gui_draw_progress_box', 0x5e76f0, [0xa1, 0x0c, 0xbe, 0xae, 0x00]),
           ('gui_draw_circle', 0x5e7840, [0x81, 0xec, 0x94, 0x00, 0x00, 0x00]),
           ('circle_plot_points', 0x5e7aa0, [0x83, 0xec, 0x18, 0x53, 0x55])]
DOCS = ('a base gráfica: a caixa de progresso de baixo, o círculo e os pontos dele',
        'the graphics base: the bottom progress box, the circle and its points',
        [('gui_draw_progress_box', 'gui_draw_progress_box', '0x5e76f0', 'goldcup.cpp', 'cdecl',
          'desenha a caixa de mensagem de baixo com o texto e, quando 0 < total e passo <= total, a barra de progresso de passo ÷ total da largura, e passa a caixa para a tela; com passo 0 escurece antes a tela atual uma vez',
          'draws the bottom message box with its text and, when 0 < total and step <= total, a progress bar of step ÷ total of its width, and flips the box to the screen; with step 0 it first darkens the current screen once'),
         ('gui_draw_circle', 'gui_draw_circle', '0x5e7840', 'goldcup.cpp', 'cdecl',
          'desenha um círculo dentro de uma caixa pelo algoritmo do ponto médio, cheio, tracejado ou preenchido (linha a linha pelos vãos de cada linha)',
          'draws a circle inside a box by the midpoint algorithm, solid, dashed or filled (row by row through each row\'s span)'),
         ('circle_plot_points', 'circle_plot_points', '0x5e7aa0', 'goldcup.cpp', 'cdecl',
          'pinta os pontos simétricos de um passo do círculo dentro da caixa (com um pixel de folga) e alarga o vão de cada linha; defeito: o passo da tela é fixo em 800 pixels',
          'plots the symmetric points of one circle step inside the box (with one pixel of slack) and widens each row\'s span; defect: the screen pitch is fixed at 800 pixels')])
CAMPOS = []
INTERESSA = []
