# O ajustador automático da T2K: a análise do contorno (library_without_asserts, o autogrid da T2K; leitor rx01, rodada 6, 25/11/2026; o mapa em
# levas/reorganizacao/proximas/TRUETYPE.md).
# Item de TELAS. No jogo nao roda: o ajustador so entra no desenho com T2K_GRID_FIT (cmd & 0x281) ou na preparacao
# do tamanho com doSetUpNow, e o FONT do jogo chama o T2K_NewTransformation com doSetUpNow = 0 (0x5b1b79) e o
# T2K_RenderGlyph com cmd = 2 (0x5b1c8d). Caminho no jogo: nenhum; a prova e o emulador (rascunho rx01/check_hint.py):
# a original e a copia sobre o mesmo contexto e o mesmo glifo sorteados, memoria inteira comparada.
# Convencoes conferidas: todas cdecl (call direto ou pelo endereco num registrador, add esp do chamador).
# Iguais no cm0102-1080.exe (todas inteiras). Prologos sem call/jmp relativo.
# Comparadores: so a original nas que mudam o ajustador ou o glifo; valor contra valor nas puras.
# Depende de o rx01_1, o rx01_5, o rx01_6 e o rx01_8.
SAVE = False
TELAS = True
ROTINAS = [('glyph_hint_collect_stem_widths_checked', 0x934b10, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0]),
           ('glyph_hint_run_pass', 0x934b60, [0x83, 0xec, 0x08, 0x66, 0x8b, 0x4c, 0x24, 0x14]),
           ('glyph_hint_outline_compute_point_tangents', 0x934e00, [0x83, 0xec, 0x38, 0x8b, 0x44, 0x24, 0x3c]),
           ('fixed_normalise_vector_2dot14', 0x935080, [0x8b, 0x4c, 0x24, 0x04, 0x53]),
           ('glyph_hint_outline_flag_extrema_points', 0x935140, [0x83, 0xec, 0x4c, 0x53, 0x55]),
           ('glyph_hint_outline_pair_stem_edges', 0x935cb0, [0x83, 0xec, 0x40, 0x53, 0x55]),
           ('glyph_hint_stem_edge_faces_point', 0x935f90, [0x53, 0x8b, 0x5c, 0x24, 0x0c])]
_A = 'library_without_asserts'
DOCS = ('o ajustador automático da T2K: a análise do contorno',
        'the T2K automatic hinter: the outline analysis',
        [('glyph_hint_collect_stem_widths_checked', 'glyph_hint_collect_stem_widths_checked', '0x934b10', _A, 'cdecl',
          'regra: com o ajustador válido, roda a análise no modo 2 (as larguras de haste); senão −1',
          'rule: with a valid hinter, runs the analysis in mode 2 (the stem widths); otherwise −1'),
         ('glyph_hint_run_pass', 'glyph_hint_run_pass', '0x934b60', _A, 'cdecl',
          'regra: roda o ajustador num glifo: no modo 3 só a escala vertical por faixas de altura; nos outros, os elos dos contornos, as direções, os extremos, as bordas de haste, as zonas e a lista de restrições, e então o ajuste (0), as larguras (2) ou nada (1)',
          'rule: runs the hinter on a glyph: mode 3 only the vertical scaling by height bands; the others the contour links, the directions, the extrema, the stem edges, the zones and the constraint list, then the fitting (0), the widths (2) or nothing (1)'),
         ('glyph_hint_outline_compute_point_tangents', 'glyph_hint_outline_compute_point_tangents', '0x934e00', _A, 'cdecl',
          'regra: a direção unitária de cada ponto para o seguinte e do anterior para ele (fora da curva, a corda entre os meios), e a posição do ponto para a busca das hastes',
          "rule: each point's unit direction toward the next and from the previous (off the curve, the chord between the midpoints), and the point's position for the stem search"),
         ('fixed_normalise_vector_2dot14', 'fixed_normalise_vector_2dot14', '0x935080', _A, 'cdecl',
          'regra: normaliza um vetor a comprimento 1 em 2.14 (raiz quadrada por Newton)',
          'rule: normalises a vector to unit length in 2.14 (Newton square root)'),
         ('glyph_hint_outline_flag_extrema_points', 'glyph_hint_outline_flag_extrema_points', '0x935140', _A, 'cdecl',
          'regra: marca nos pontos os extremos em x e y, os trechos lisos, as direções retas, os extremos planos, as inflexões e os segmentos longos',
          "rule: flags the points' x and y extrema, smooth stretches, straight directions, flat extrema, inflections and long segments"),
         ('glyph_hint_outline_pair_stem_edges', 'glyph_hint_outline_pair_stem_edges', '0x935cb0', _A, 'cdecl',
          'regra: emparelha as bordas que se olham através de uma haste: para cada uma, a mais próxima compatível na direção para a frente e na para trás',
          'rule: pairs the edges that face each other across a stem: for each, the nearest compatible one along its forward and its backward direction'),
         ('glyph_hint_stem_edge_faces_point', 'glyph_hint_stem_edge_faces_point', '0x935f90', _A, 'cdecl',
          'regra: se um ponto encara outro através de uma haste numa direção (está à esquerda e uma das direções dele vira contra)',
          'rule: whether a point faces another across a stem along a direction (it lies to the left and one of its directions turns against it)')])
CAMPOS = []
INTERESSA = []
