# O interpretador TrueType da T2K: os DELTA; o ajustador automático: os parâmetros de dica da fonte, lidos ou medidos.
# library_without_asserts; leitor rx02, rodada 6, 25/11/2026; o mapa em levas/reorganizacao/proximas/TRUETYPE.md.
# Item de TELAS: o texto desenhado e tela. O interpretador so roda na carga das fontes (o programa da fonte do
# symbol.ttf, PUSHW e FDEF); o resto so roda numa fonte com instrucoes desenhada com ajuste a grade, o que o jogo
# nao faz (ver o rx01_1). Caminho no jogo: abrir o jogo comparando (as fontes carregam no inicio do video).
# - glyph_hint_read_font_params e glyph_hint_measure_font_params sao chamadas so pelo t2k_set_up_hinting_for_size
#   (0x93e656 e 0x93e684), que o jogo nao roda (doSetUpNow = 0 e desenho sem ajuste); a segunda so para fonte
#   sem a tabela T2KG (as .t2k do jogo tem).
# Convencoes conferidas na desmontagem: todas cdecl; os opcodes pela tabela dos opcodes (estado local); as outras
# pelo ponto de chamada (tt_set_super_round_params: estado, byte, grade; tt_compute_rp_displacement: estado, &dx,
# &dy, &ponto -> zona; tt_delta_scan_apply: estado, retorno, base, shift; tt_delta_cvt_add como o move_point;
# glyph_hint_read_font_params: parametros, fluxo; glyph_hint_measure_font_params: fonte, ajustador, parametros,
# amostrar glifos).
# Iguais no cm0102-1080.exe (todas inteiras). Prologos sem call/jmp relativo.
# Comparadores: so a original (mudam o estado do interpretador ou leem uma fonte). Prova sem o jogo: o emulador
# (rascunho rx02/check_rx02.py), a original e a copia sobre o mesmo mundo sorteado, memoria inteira comparada.
# lib_read_int16 e lib_sort_i16 (0x9415c0 e 0x9446a0, rx03) daqui podem ser usados pela parte rx03.
# Depende de: o rx01_1, o rx01_2 (projection_stretch), o rx01_5 (HintGlyph, GlyphHinter), o rx01_7 (glyph_hint_collect_stem_widths_checked, chamada pelo endereco do exe), o rx01_8 (lib_mem_free) e o rx02_1.
SAVE = False
TELAS = True
ROTINAS = [('tt_delta_scan_apply', 0x93b0a0, [0x51, 0x53, 0x55, 0x56, 0x57]),
           ('tt_op_deltap2', 0x93b1f0, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x41, 0x28]),
           ('tt_op_deltap3', 0x93b220, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x41, 0x28]),
           ('tt_op_deltac1', 0x93b250, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x41, 0x28]),
           ('tt_delta_cvt_add', 0x93b280, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x28]),
           ('tt_op_deltac2', 0x93b2a0, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x41, 0x28]),
           ('tt_op_deltac3', 0x93b2d0, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x41, 0x28]),
           ('glyph_hint_read_font_params', 0x93b300, [0x53, 0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x18]),
           ('glyph_hint_measure_font_params', 0x93b370, [0x81, 0xec, 0xd8, 0x00, 0x00, 0x00])]
_A = 'library_without_asserts'
DOCS = ('o interpretador TrueType da T2K: os DELTA; o ajustador automático: os parâmetros de dica da fonte, lidos ou medidos',
        "the T2K TrueType interpreter: the DELTA opcodes; the automatic hinter: the font's hint parameters, read or measured",
        [('tt_delta_scan_apply', 'tt_delta_scan_apply', '0x93b0a0', _A, 'cdecl',
          'regra: a varredura dos DELTA: tira n pares (argumento, alvo); o tamanho (ppem, esticado, menos a base) só vale de 0 a 15; cada par daquele tamanho chama o retorno com o passo de −8 a 8 pixels (sem o 0), deslocado pelo shift; busca por metades e varre em ordem',
          'rule: the DELTA scan: pops n (argument, target) pairs; the size (ppem, stretched, minus the base) applies only from 0 to 15; each pair of that size calls the callback with the step −8 to 8 pixels (no 0), shifted by the shift; a halving search, then an ordered scan'),
         ('tt_op_deltap2', 'tt_op_deltap2', '0x93b1f0', _A, 'cdecl',
          'regra: DELTAP2: as exceções de ponto a partir da base + 16',
          'rule: DELTAP2: the point exceptions from the base + 16'),
         ('tt_op_deltap3', 'tt_op_deltap3', '0x93b220', _A, 'cdecl',
          'regra: DELTAP3: as exceções de ponto a partir da base + 32',
          'rule: DELTAP3: the point exceptions from the base + 32'),
         ('tt_op_deltac1', 'tt_op_deltac1', '0x93b250', _A, 'cdecl',
          'regra: DELTAC1: as exceções do cvt a partir da base',
          'rule: DELTAC1: the control value exceptions from the base'),
         ('tt_delta_cvt_add', 'tt_delta_cvt_add', '0x93b280', _A, 'cdecl',
          'regra: o retorno dos DELTAC: soma o passo ao valor de controle',
          'rule: the DELTAC callback: adds the step to the control value'),
         ('tt_op_deltac2', 'tt_op_deltac2', '0x93b2a0', _A, 'cdecl',
          'regra: DELTAC2: as exceções do cvt a partir da base + 16',
          'rule: DELTAC2: the control value exceptions from the base + 16'),
         ('tt_op_deltac3', 'tt_op_deltac3', '0x93b2d0', _A, 'cdecl',
          'regra: DELTAC3: as exceções do cvt a partir da base + 32',
          'rule: DELTAC3: the control value exceptions from the base + 32'),
         ('glyph_hint_read_font_params', 'glyph_hint_read_font_params', '0x93b300', _A, 'cdecl',
          'regra: lê da tabela T2KG os parâmetros de dica da fonte: as 10 zonas de alinhamento (altura plana, plana + overshoot, overshoot) e as 12 larguras de haste x e y',
          "rule: reads the font's hint parameters from the T2KG table: the 10 alignment zones (flat height, flat + overshoot, overshoot) and the 12 x and y stem widths"),
         ('glyph_hint_measure_font_params', 'glyph_hint_measure_font_params', '0x93b370', _A, 'cdecl',
          'regra: mede os parâmetros de dica quando a fonte não tem T2KG: as zonas pelas medianas dos extremos de letras de amostra, os overshoots simetrizados pela mediana ou pela média do par, e as larguras de haste da letra "o" ou de até 32 glifos; defeitos do original: com menos de 32 glifos e nenhum contorno nos cinco primeiros a amostragem não termina; sem amostra lê pilha não iniciada',
          'rule: measures the hint parameters when the font has no T2KG: the zones by the medians of the extremes of sample letters, the overshoots made symmetric by the median or the pair\'s average, and the stem widths of the letter "o" or of up to 32 glyphs; defects of the original: with fewer than 32 glyphs and no contour in the first five the sampling never ends; with no sample it reads uninitialised stack')])
CAMPOS = []
INTERESSA = []
