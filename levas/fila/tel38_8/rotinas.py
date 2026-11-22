# A base da interface (8): o grafico de linhas (plot.cpp; leitor tel38b, rodada das telas, 21/11/2026): construir e
# liberar o grafico, as series e os pontos, desenha-lo e a moldura com os eixos.
# Usa os auxiliares do tel01_1 (screens_current_display, DisplayAddItemFn, DisplayAddAreaFn), do tel26_1 (display_list,
# display_item_at) e do tel38_3 (gui_build_regions); show_error_at do motor. Os nomes evitam os plot_new,
# plot_add_series e plot_add_point privados do tel23_2 (que chama estas rotinas pelo endereco).
# Convencoes conferidas na desmontagem (0x7c6fc0..0x7c7d73), todas thiscall, ecx = o grafico (0x1c bytes): Plot::Plot
# ret 0x14 (x de, y de, x ate, y ate, series) -> o grafico; ~Plot ret 0; add_series ret 0xc (pontos, cor word,
# rotulo); add_point ret 0xc (serie, x, y); draw_line_graph ret 0x18 (x0, y0, x1, y1, titulo x, titulo y);
# draw_axes ret 0x1c (x0, y0, x1, y1, folga, titulo x, titulo y). Chamadas: malloc 0x944e46, free 0x9452ca, _strdup
# 0x952f5b (cdecl); Display::add_item 0x55f990 (as linhas: tipo 1, estilo 0x400) e add_area 0x55fba0 do canal atual;
# ScreenManager::add_text_item 0x803fd0 (os titulos e os numeros dos eixos); sprintf "%d".
# Iguais no cm0102-1080.exe (as rotinas inteiras; o add_item do 1080 escala as coordenadas por conta propria).
# Defeitos do original: ~Plot libera todas as vagas de serie (SERIES_CAPACITY), inclusive as nunca escritas (o vetor
# nao e zerado); add_point confere a serie contra a capacidade, nao contra quantas ha; uma faixa de valores 0 divide
# por zero.
# Comparadores: Plot::Plot e add_series so a original (alocam), conferindo o plano (a faixa, a capacidade, a serie
# nova); ~Plot so a original (libera); add_point lado a lado na serie e no ponto; draw_axes e draw_line_graph lado a
# lado na lista de exibicao (a area, os itens seguintes, a ordem, o pacote, o proprio grafico).
TELAS = True
ROTINAS = [('construct_plot', 0x7c6fc0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('destroy_plot', 0x7c70a0, [0x56, 0x8b, 0xf1, 0x83, 0x3e, 0x00]),
           ('plot_append_series', 0x7c7100, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('plot_append_point', 0x7c7240, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x51, 0x18]),
           ('plot_draw_line_graph', 0x7c7340, [0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x1c]),
           ('plot_draw_axes', 0x7c7580, [0x8b, 0x44, 0x24, 0x14, 0x83, 0xec, 0x18])]
DOCS = ('a base da interface (8): o gráfico de linhas: construir e liberar, as séries e os pontos, desenhar e a moldura com os eixos',
        'the interface base (8): the line graph: building and freeing it, its series and points, drawing it and its frame with the axes',
        [('construct_plot', 'Plot::Plot', '0x7c6fc0', 'plot.cpp', 'thiscall, ret 0x14',
          'um gráfico novo guarda a faixa de valores (de e até em cada eixo; "até" abaixo de "de" inverte o eixo) e o vetor das séries; sem memória, a caixa "plot.cpp 47"',
          'a new graph keeps its value range (from and to on each axis; "to" below "from" turns the axis around) and the series array; out of memory, the "plot.cpp 47" box'),
         ('destroy_plot', 'Plot::~Plot', '0x7c70a0', 'plot.cpp', 'thiscall',
          'libera os pontos e o rótulo de cada vaga de série (todas, inclusive as nunca escritas) e o vetor',
          'frees the points and the label of every series slot (all of them, even those never written) and the array'),
         ('plot_append_series', 'Plot::add_series', '0x7c7100', 'plot.cpp', 'thiscall, ret 0xc',
          'uma série nova com lugar para os pontos, a cor e uma cópia do rótulo; sem vaga, a caixa "plot.cpp 86"',
          'a new series with room for its points, a colour and a copy of its label; with no slot left, the "plot.cpp 86" box'),
         ('plot_append_point', 'Plot::add_point', '0x7c7240', 'plot.cpp', 'thiscall, ret 0xc',
          'um ponto (x, y) no fim de uma série; série fora da capacidade ou cheia, as caixas "plot.cpp 117" e "123"',
          'a point (x, y) at the end of a series; a series past the capacity or full, the "plot.cpp 117" and "123" boxes'),
         ('plot_draw_line_graph', 'Plot::draw_line_graph', '0x7c7340', 'plot.cpp', 'thiscall, ret 0x18',
          'desenha o gráfico: a moldura (20 pixels de margem à esquerda e em cima, 30 embaixo) e cada série como uma linha dupla pelos seus pontos, na cor dela',
          'draws the graph: the frame (20 pixels of margin left and top, 30 at the bottom) and each series as a double line through its points, in its colour'),
         ('plot_draw_axes', 'Plot::draw_axes', '0x7c7580', 'plot.cpp', 'thiscall, ret 0x1c',
          'a moldura: uma área, os quatro lados em linhas duplas (esquerda e topo azuis, base e direita pretas), os dois títulos e os números dos eixos (passo com mais de 15 pixels no x e 10 no y)',
          'the frame: an area, the four sides as double lines (left and top blue, bottom and right black), the two titles and the axis numbers (a step over 15 pixels apart on x and 10 on y)')])
