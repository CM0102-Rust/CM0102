# A T2K (library_without_asserts): o glifo (glyph.c da T2K): os contornos comprimidos, o glifo vazio, os glifos
# compostos, os vetores de pontos e o estilo negrito (leitor rx03, rodada 6, 25/11/2026; o mapa em
# levas/reorganizacao/proximas/TRUETYPE.md).
# Item de TELAS: o texto desenhado e tela. No jogo rodam no desenho de cada letra nova: as fontes .t2k do jogo usam o
# formato comprimido (glyph_outline_read_from_stream e glyph_outline_read_compressed_point_delta), o espaco vira
# glifo vazio, as letras acentuadas sao compostas (glyph_outline_append_component), e o negrito do FONT passa pelo
# glyph_outline_embolden e pelo font_embolden_advance_widths (o estilo com o parametro 0x18000). O
# glyph_hint_heap_sift_down_by_coord so com o ajustador automatico (glyph_hint_fit_outline_both_axes). Caminho no
# jogo: abrir o jogo comparando (texto em negrito e acentos).
# Convencoes conferidas: todas cdecl: glyph_outline_read_compressed_point_delta (fluxo, &dx, &dy) -> na curva;
# glyph_outline_new_empty (contexto, lsb, aw, tsb, ah); glyph_outline_read_from_stream (contexto, fluxo, instrucoes,
# lsb, aw, tsb, ah, +0x60 da fonte); glyph_outline_append_component (&base, componente, marcas, arg1, arg2);
# glyph_outline_alloc_point_arrays (glifo, n); glyph_outline_free (glifo); glyph_hint_heap_sift_down_by_coord
# (registros, no, ultimo, chaves); as duas do negrito (glifo ou metricas, contexto, unidades, parametros), chamadas
# pelo ponteiro do estilo.
# Iguais no cm0102-1080.exe (todas inteiras). Prologos sem call/jmp relativo.
# Comparadores: lado a lado nas puras (o passo comprimido na posicao do fluxo e nas duas saidas; o monte no vetor dos
# registros; os avancos alargados); so a original nas que alocam ou liberam.
# Prova sem o jogo: o emulador, o caminho inteiro do texto com as fontes do jogo, normal e negrito (rascunho
# rx03/e2e.py), e o monte valor a valor (check_rx03.py rx03_7).
# Defeito do original: glyph_outline_read_from_stream com um glifo sem contornos (e com bytes) escreve os pontos
# fantasmas por vetores nulos (endereco 0).
# Depende do rx03_1 (fixed_mul, fixed_div), rx03_2 (o contexto e os buffers de rascunho) e rx03_3 (o fluxo).
SAVE = False
TELAS = True
ROTINAS = [('glyph_outline_read_compressed_point_delta', 0x93bbd0, [0x8b, 0x44, 0x24, 0x04, 0x55]),
           ('glyph_outline_new_empty', 0x93bd70, [0x53, 0x8b, 0x5c, 0x24, 0x08]),
           ('glyph_outline_read_from_stream', 0x93be40, [0x83, 0xec, 0x24, 0x53, 0x55]),
           ('glyph_outline_append_component', 0x93c200, [0x83, 0xec, 0x20, 0x53, 0x55]),
           ('glyph_outline_alloc_point_arrays', 0x93c590, [0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('glyph_outline_free', 0x93c5e0, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('glyph_hint_heap_sift_down_by_coord', 0x93d780, [0x53, 0x55, 0x8b, 0x5c, 0x24, 0x18]),
           ('font_embolden_advance_widths', 0x93d820, [0x8b, 0x44, 0x24, 0x10, 0x56]),
           ('glyph_outline_embolden', 0x93d870, [0x83, 0xec, 0x48, 0x8b, 0x44, 0x24, 0x58])]
_A = 'library_without_asserts'
DOCS = ('a T2K: o glifo, os contornos comprimidos, os compostos, os vetores de pontos e o negrito',
        'the T2K: the glyph, the compressed outlines, the composite glyphs, the point arrays and the bold style',
        [('glyph_outline_read_compressed_point_delta', 'glyph_outline_read_compressed_point_delta', '0x93bbd0', _A, 'cdecl',
          'regra: um passo comprimido: palavra com o quadrante nos 2 bits altos e o código nos 14 baixos (código 0 com quadrante 0 ou 1: dx e dy explícitos de 16 bits); faixas < 0x898, < 0x313c, < 0x393c (mais 1 byte) e o resto (mais 2), divididas por 102, 724 ou 10650; o quadrante dá sinais e ordem; dx é o primeiro dividido por 2 e o bit baixo zero quer dizer na curva',
          'rule: one compressed step: a word with the quadrant in its 2 top bits and the code in its 14 low bits (code 0 with quadrant 0 or 1: explicit 16-bit dx and dy); ranges < 0x898, < 0x313c, < 0x393c (one more byte) and the rest (two more), split by 102, 724 or 10650; the quadrant gives the signs and order; dx is the first halved and a clear low bit means on curve'),
         ('glyph_outline_new_empty', 'glyph_outline_new_empty', '0x93bd70', _A, 'cdecl',
          'regra: glifo vazio com só os quatro pontos fantasmas: (0, 0), (aw, 0), (0, tsb), (0, tsb − ah)',
          'rule: an empty glyph with only the four phantom points: (0, 0), (aw, 0), (0, tsb), (0, tsb − ah)'),
         ('glyph_outline_read_from_stream', 'glyph_outline_read_from_stream', '0x93be40', _A, 'cdecl',
          'regra: o glifo T2K comprimido: os fins dos contornos em varint de 7 bits, os pontos por passos comprimidos (o x mínimo e o y máximo guardados na caixa), o composto com os registros dos componentes; depois os pontos fantasmas (defeito: sem contornos e com bytes, escreve por vetores nulos)',
          'rule: a T2K compressed glyph: the contour ends as 7-bit varints, the points as compressed steps (the x minimum and y maximum kept in the box), a composite one with its component records; then the phantom points (defect: with no contours and some bytes, it writes through null arrays)'),
         ('glyph_outline_append_component', 'glyph_outline_append_component', '0x93c200', _A, 'cdecl',
          'regra: o primeiro componente vira a base (movido pelo deslocamento, arredondado com a marca 4); os seguintes vão para vetores novos com os pontos da base, os dois primeiros fantasmas da base (sem a marca 0x200), os pontos do componente deslocados (pelo deslocamento ou pela distância entre o ponto arg1 da base e o arg2 do componente) e os contornos; os vetores velhos são liberados',
          'rule: the first component becomes the base (moved by the offset, rounded with flag 4); the next ones go into new arrays with the base\'s points, the base\'s first two phantom points (without flag 0x200), the component\'s points moved (by the offset or by the distance between base point arg1 and component point arg2) and the contours; the old arrays are freed'),
         ('glyph_outline_alloc_point_arrays', 'glyph_outline_alloc_point_arrays', '0x93c590', _A, 'cdecl',
          'regra: um bloco só (vaga de rascunho 1) com x e y em 26.6, x e y em unidades da fonte e as marcas na curva, para n pontos mais 4 fantasmas',
          'rule: one block (scratch slot 1) with x and y in 26.6, x and y in font units and the on curve flags, for n points plus 4 phantoms'),
         ('glyph_outline_free', 'glyph_outline_free', '0x93c5e0', _A, 'cdecl',
          'regra: libera os pontos e as instruções (devolvendo as vagas de rascunho 1 e 3 quando são delas), os contornos fora do glifo, os componentes e o glifo (vaga 0); nada com nulo',
          'rule: frees the points and the instructions (giving back scratch slots 1 and 3 when theirs), the contours when not inline, the components and the glyph (slot 0); nothing for null'),
         ('glyph_hint_heap_sift_down_by_coord', 'glyph_hint_heap_sift_down_by_coord', '0x93d780', _A, 'cdecl',
          'regra: refaz a ordem do monte (o maior em cima) abaixo de um nó: o nó desce até a folha pelos filhos maiores e sobe de volta enquanto for maior que o pai; a chave é a coordenada do primeiro i16 do registro',
          'rule: restores the heap order (largest on top) below a node: the node sinks to a leaf along the larger children and climbs back while larger than its parent; the key is the coordinate of the record\'s first i16'),
         ('font_embolden_advance_widths', 'font_embolden_advance_widths', '0x93d820', _A, 'cdecl',
          'regra: cada avanço cresce o dobro do deslocamento do negrito, fixed_mul(força, unidades/12) − unidades/12',
          'rule: each advance grows by twice the bold offset, fixed_mul(strength, units/12) − units/12'),
         ('glyph_outline_embolden', 'glyph_outline_embolden', '0x93d870', _A, 'cdecl',
          'regra: cada ponto sai das suas duas arestas pelo deslocamento do negrito (pelo sentido do passo em y) e vai ao encontro das arestas movidas, limitado a |2d| de onde estava; depois o ponto que inverteu a ordem com o anterior toma a coordenada dele; depois as sequências de x original igual têm a média',
          'rule: each point leaves its two edges by the bold offset (by the sign of the y step) and goes to where the moved edges meet, kept within |2d| of where it was; then a point that reversed its order against the previous one takes its coordinate; then runs of equal original x are averaged')])
CAMPOS = []
INTERESSA = []
MUTACOES = []
