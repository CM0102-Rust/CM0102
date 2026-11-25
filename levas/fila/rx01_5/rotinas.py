# O ajustador automático da T2K: o contexto, o glifo e as operações de ponto (library_without_asserts, o autogrid da T2K; leitor rx01, rodada 6, 25/11/2026; o mapa em
# levas/reorganizacao/proximas/TRUETYPE.md).
# Item de TELAS. No jogo nao roda: o ajustador so entra no desenho com T2K_GRID_FIT (cmd & 0x281) ou na preparacao
# do tamanho com doSetUpNow, e o FONT do jogo chama o T2K_NewTransformation com doSetUpNow = 0 (0x5b1b79) e o
# T2K_RenderGlyph com cmd = 2 (0x5b1c8d). Caminho no jogo: nenhum; a prova e o emulador (rascunho rx01/check_hint.py):
# a original e a copia sobre o mesmo contexto e o mesmo glifo sorteados, memoria inteira comparada.
# Convencoes conferidas: todas cdecl (call direto ou pelo endereco num registrador, add esp do chamador).
# Os tipos GlyphHinter (o contexto ag_DataType, 0x450 bytes) e HintGlyph (o glifo ag_ElementType), os auxiliares at_i16,
# at_i32, set_i32, approach, round_to_pixel, NO_HINT_SLOT, HINTER_MAGIC* e o tipo GlyphHintDistanceFn daqui sao usados
# pelos rx01_6..8 e pela parte rx03 (glyph_hint_fit_outline_both_axes, o ajuste que chama estas operacoes).
# Iguais no cm0102-1080.exe (todas inteiras). Prologos sem call/jmp relativo.
# Comparadores: so a original nas que mudam o ajustador ou o glifo; valor contra valor nas puras.
# Depende de o rx01_1 (lib_fixed_mul...) e o rx01_6 (glyph_hint_set_distance_by_axis...).
SAVE = False
TELAS = True
ROTINAS = [('glyph_hint_reset_scale_search', 0x9340a0, [0x8b, 0x44, 0x24, 0x04, 0xc7, 0x80, 0x26, 0x04, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00]),
           ('glyph_hint_refine_scale_step', 0x9340d0, [0x66, 0x83, 0x7c, 0x24, 0x0c, 0x00]),
           ('glyph_hint_raise_to_max_of_pair', 0x9341e0, [0x66, 0x83, 0x7c, 0x24, 0x0c, 0x00]),
           ('glyph_hint_lower_to_min_of_pair', 0x934220, [0x66, 0x83, 0x7c, 0x24, 0x0c, 0x00]),
           ('glyph_hint_clamp_point_and_shift_companion', 0x934260, [0x66, 0x83, 0x7c, 0x24, 0x0c, 0x00]),
           ('glyph_hint_set_x_distance_rounded', 0x9342c0, [0x8b, 0x4c, 0x24, 0x04, 0x53]),
           ('glyph_hint_set_y_distance_rounded', 0x934380, [0x53, 0x55, 0x8b, 0x6c, 0x24, 0x2c]),
           ('glyph_hint_round_x_to_pixel_if_flag', 0x934440, [0x66, 0x83, 0x7c, 0x24, 0x0c, 0x00]),
           ('glyph_hint_round_y_to_pixel_if_flag', 0x934460, [0x66, 0x83, 0x7c, 0x24, 0x0c, 0x00]),
           ('glyph_hint_snap_x_toward_target', 0x934480, [0x66, 0x83, 0x7c, 0x24, 0x0c, 0x00]),
           ('glyph_hint_snap_y_toward_target', 0x9344e0, [0x66, 0x83, 0x7c, 0x24, 0x0c, 0x00]),
           ('glyph_hint_interpolate_x_between_points', 0x934540, [0x53, 0x8b, 0x5c, 0x24, 0x18]),
           ('glyph_hint_interpolate_y_between_points', 0x934590, [0x53, 0x8b, 0x5c, 0x24, 0x18])]
_A = 'library_without_asserts'
DOCS = ('o ajustador automático da T2K: o contexto, o glifo e as operações de ponto',
        'the T2K automatic hinter: its context, the glyph and the point operations',
        [('glyph_hint_reset_scale_search', 'glyph_hint_reset_scale_search', '0x9340a0', _A, 'cdecl',
          'regra: volta a busca da escala ao início: fator 0x1000, limites 0x800 e 0x2000, nenhum passo',
          'rule: puts the scale search back at its start: factor 0x1000, bounds 0x800 and 0x2000, no step'),
         ('glyph_hint_refine_scale_step', 'glyph_hint_refine_scale_step', '0x9340d0', _A, 'cdecl',
          'regra: um passo da busca da escala de um ponto: o original escalado, puxado até 0x60 para o valor de dica, arredondado ao pixel; se não bate com o ponto ajustado (e menos de 14 passos), corta o intervalo do fator ao meio na direção do erro; se bate (ou no 14º passo), grava e reinicia a busca',
          "rule: one step of a point's scale search: the scaled original, pulled up to 0x60 toward the hint value, rounded to the pixel; when it misses the hinted point (under 14 steps) it halves the factor's interval toward the miss; on a hit (or the 14th step) it stores it and resets the search"),
         ('glyph_hint_raise_to_max_of_pair', 'glyph_hint_raise_to_max_of_pair', '0x9341e0', _A, 'cdecl',
          'regra: sobe a coordenada de um ponto até a de outro quando a outra é maior',
          "rule: raises a point's coordinate to another's when that is larger"),
         ('glyph_hint_lower_to_min_of_pair', 'glyph_hint_lower_to_min_of_pair', '0x934220', _A, 'cdecl',
          'regra: desce a coordenada de um ponto até a de outro quando a outra é menor',
          "rule: lowers a point's coordinate to another's when that is smaller"),
         ('glyph_hint_clamp_point_and_shift_companion', 'glyph_hint_clamp_point_and_shift_companion', '0x934260', _A, 'cdecl',
          'regra: quando um ponto passou do limite, volta-o pelo excesso (16 bits) e move o companheiro do mesmo tanto',
          'rule: when a point passed its limit, moves it back by the excess (16 bits) and the companion by the same amount'),
         ('glyph_hint_set_x_distance_rounded', 'glyph_hint_set_x_distance_rounded', '0x9342c0', _A, 'cdecl',
          'regra: põe um ponto à distância original escalada de outro em x: puxada até 42 para a largura de haste da vaga, arredondada ao pixel (no mínimo um, se pedido); no modo sem arredondar, distância de menos de meio pixel dobra (até meio pixel)',
          "rule: places a point at its scaled original distance from another in x: pulled up to 42 toward the slot's stem width, rounded to the pixel (at least one when asked); in the unrounded mode a distance under half a pixel doubles (up to half a pixel)"),
         ('glyph_hint_set_y_distance_rounded', 'glyph_hint_set_y_distance_rounded', '0x934380', _A, 'cdecl',
          'regra: o mesmo em y; no modo sem arredondar só arredonda distância de 0x24 ou mais fora do modo 2, e o resto fica como medido',
          'rule: the same in y; in the unrounded mode only a distance of 0x24 or more outside mode 2 is rounded, the rest stays as measured'),
         ('glyph_hint_round_x_to_pixel_if_flag', 'glyph_hint_round_x_to_pixel_if_flag', '0x934440', _A, 'cdecl',
          'regra: arredonda o x de um ponto ao pixel, se pedido',
          "rule: rounds a point's x to the pixel, when asked"),
         ('glyph_hint_round_y_to_pixel_if_flag', 'glyph_hint_round_y_to_pixel_if_flag', '0x934460', _A, 'cdecl',
          'regra: arredonda o y de um ponto ao pixel, se pedido',
          "rule: rounds a point's y to the pixel, when asked"),
         ('glyph_hint_snap_x_toward_target', 'glyph_hint_snap_x_toward_target', '0x934480', _A, 'cdecl',
          'regra: puxa o x de um ponto até 0x60 para o valor de dica e arredonda ao pixel, se pedido',
          "rule: pulls a point's x up to 0x60 toward the hint value and rounds it, when asked"),
         ('glyph_hint_snap_y_toward_target', 'glyph_hint_snap_y_toward_target', '0x9344e0', _A, 'cdecl',
          'regra: o mesmo em y',
          'rule: the same in y'),
         ('glyph_hint_interpolate_x_between_points', 'glyph_hint_interpolate_x_between_points', '0x934540', _A, 'cdecl',
          'regra: põe o x de um ponto entre dois outros na proporção dos x originais',
          "rule: puts a point's x between two others in the proportion of the original x"),
         ('glyph_hint_interpolate_y_between_points', 'glyph_hint_interpolate_y_between_points', '0x934590', _A, 'cdecl',
          'regra: o mesmo em y',
          'rule: the same in y')])
CAMPOS = []
INTERESSA = []
