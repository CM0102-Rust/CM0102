# O ajustador automático da T2K: as zonas de alinhamento, as larguras de haste e a interpolação dos pontos não tocados (library_without_asserts, o autogrid da T2K; leitor rx01, rodada 6, 25/11/2026; o mapa em
# levas/reorganizacao/proximas/TRUETYPE.md).
# Item de TELAS. No jogo nao roda: o ajustador so entra no desenho com T2K_GRID_FIT (cmd & 0x281) ou na preparacao
# do tamanho com doSetUpNow, e o FONT do jogo chama o T2K_NewTransformation com doSetUpNow = 0 (0x5b1b79) e o
# T2K_RenderGlyph com cmd = 2 (0x5b1c8d). Caminho no jogo: nenhum; a prova e o emulador (rascunho rx01/check_hint.py):
# a original e a copia sobre o mesmo contexto e o mesmo glifo sorteados, memoria inteira comparada.
# Convencoes conferidas: todas cdecl (call direto ou pelo endereco num registrador, add esp do chamador).
# Iguais no cm0102-1080.exe (todas inteiras). Prologos sem call/jmp relativo.
# Comparadores: so a original nas que mudam o ajustador ou o glifo; valor contra valor nas puras.
# Depende de o rx01_1 e o rx01_5.
SAVE = False
TELAS = True
ROTINAS = [('glyph_hint_find_nearest_alignment_zone', 0x933b20, [0x83, 0xec, 0x28, 0x8b, 0x4c, 0x24, 0x2c]),
           ('glyph_hint_set_distance_by_stem_width', 0x933da0, [0x8b, 0x44, 0x24, 0x0c, 0x53]),
           ('glyph_hint_place_point_between_two_refs', 0x933e50, [0x8b, 0x54, 0x24, 0x04, 0x66, 0x83, 0x7c, 0x24, 0x0c, 0x00]),
           ('glyph_hint_adjust_x_stem_edges', 0x933fa0, [0x8b, 0x54, 0x24, 0x04, 0x8b, 0x44, 0x24, 0x08]),
           ('glyph_hint_interpolate_untouched_x', 0x9345e0, [0x83, 0xec, 0x18, 0x8b, 0x44, 0x24, 0x1c]),
           ('glyph_hint_interpolate_untouched_y', 0x9347a0, [0x83, 0xec, 0x18, 0x8b, 0x44, 0x24, 0x1c]),
           ('glyph_hint_set_distance_by_axis', 0x934960, [0x66, 0x83, 0x7c, 0x24, 0x18, 0x00]),
           ('glyph_hint_find_nearest_stem_width_index', 0x9349a0, [0x83, 0xec, 0x0c, 0x53, 0x55]),
           ('glyph_hint_max_x_stem_width', 0x934ab0, [0x8b, 0x4c, 0x24, 0x04, 0x56]),
           ('glyph_hint_max_y_stem_width', 0x934ae0, [0x8b, 0x4c, 0x24, 0x04, 0x56])]
_A = 'library_without_asserts'
DOCS = ('o ajustador automático da T2K: as zonas de alinhamento, as larguras de haste e a interpolação dos pontos não tocados',
        'the T2K automatic hinter: the alignment zones, the stem widths and the interpolation of untouched points',
        [('glyph_hint_find_nearest_alignment_zone', 'glyph_hint_find_nearest_alignment_zone', '0x933b20', _A, 'cdecl',
          'regra: a zona de alinhamento em que um ponto cai (a altura mais próxima a até 1/66 em; zonas de cima só para ponto que não sobe, de baixo só para o que não desce, salvo extremos; as zonas 2 e 6 só para algarismos), ou −1',
          'rule: the alignment zone a point falls on (the nearest height within 1/66 em; top zones only for a point that does not rise, bottom ones only for one that does not fall, extremes excepted; zones 2 and 6 only for figures), or −1'),
         ('glyph_hint_set_distance_by_stem_width', 'glyph_hint_set_distance_by_stem_width', '0x933da0', _A, 'cdecl',
          'regra: acerta a distância entre dois pontos pela largura de haste que casa com ela, ou pela distância arredondada (no mínimo um pixel)',
          'rule: sets the distance between two points by the stem width that matches it, or by the rounded distance (at least one pixel)'),
         ('glyph_hint_place_point_between_two_refs', 'glyph_hint_place_point_between_two_refs', '0x933e50', _A, 'cdecl',
          'regra: põe um ponto entre duas referências pelas distâncias originais escaladas (fora da busca, a candidata da primeira anda um pixel para a da segunda); no modo 2 a média, senão a um terço; arredonda ao pixel',
          'rule: places a point between two references by the scaled original distances (outside a search the first candidate moves one pixel toward the second); in mode 2 their average, otherwise a third of the way; rounded to the pixel'),
         ('glyph_hint_adjust_x_stem_edges', 'glyph_hint_adjust_x_stem_edges', '0x933fa0', _A, 'cdecl',
          'regra: acerta as bordas de uma haste em x: arredonda a direita e, numa haste de três pixels ou mais, move uma das bordas um pixel conforme as larguras ajustadas e originais',
          "rule: adjusts an x stem's edges: rounds the right one and, on a stem of three pixels or more, moves an edge one pixel by the hinted and original widths"),
         ('glyph_hint_interpolate_untouched_x', 'glyph_hint_interpolate_untouched_x', '0x9345e0', _A, 'cdecl',
          'regra: em cada contorno, os pontos não tocados em x entre dois tocados seguem a mudança deles na proporção dos originais escalados',
          'rule: on each contour, the points untouched in x between two touched ones follow their change in the proportion of the scaled originals'),
         ('glyph_hint_interpolate_untouched_y', 'glyph_hint_interpolate_untouched_y', '0x9347a0', _A, 'cdecl',
          'regra: o mesmo em y',
          'rule: the same in y'),
         ('glyph_hint_set_distance_by_axis', 'glyph_hint_set_distance_by_axis', '0x934960', _A, 'cdecl',
          'regra: acerta a distância entre dois pontos com uma vaga de largura de haste, pelo eixo',
          'rule: sets the distance between two points with a stem width slot, by axis'),
         ('glyph_hint_find_nearest_stem_width_index', 'glyph_hint_find_nearest_stem_width_index', '0x9349a0', _A, 'cdecl',
          'regra: a vaga da largura de haste mais próxima de uma distância (que caia entre metade e uma vez e meia a largura), ou −1',
          'rule: the stem width slot nearest a distance (falling between half and one and a half times the width), or −1'),
         ('glyph_hint_max_x_stem_width', 'glyph_hint_max_x_stem_width', '0x934ab0', _A, 'cdecl',
          'regra: a maior das 12 larguras de haste em x',
          'rule: the largest of the 12 x stem widths'),
         ('glyph_hint_max_y_stem_width', 'glyph_hint_max_y_stem_width', '0x934ae0', _A, 'cdecl',
          'regra: a maior das 12 larguras de haste em y',
          'rule: the largest of the 12 y stem widths')])
CAMPOS = []
INTERESSA = []
