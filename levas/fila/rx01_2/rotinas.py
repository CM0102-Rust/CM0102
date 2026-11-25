# O interpretador TrueType da T2K: as funcoes de arredondamento e os leitores esticados (pixels nao quadrados)
# (library_without_asserts; leitor rx01, rodada 6, 25/11/2026; o mapa em levas/reorganizacao/proximas/TRUETYPE.md).
# Item de TELAS. No jogo nao rodam (so numa fonte com instrucoes desenhada com ajuste, o que o jogo nao faz: ver o
# rx01_1); o tt_run_ins_range_with_callback compara o endereco delas (o codigo do arredondamento) e o tt_exec_create
# (rx03, 0x941a7c) grava o tt_round_to_grid no estado global. Caminho no jogo: abrir o jogo comparando.
# Convencoes conferidas: todas cdecl, chamadas pelo campo de funcao do estado global (+0x70: estado local, valor 26.6,
# compensacao) ou do estado local (+0x52 get_cvt: estado, indice; +0x56 get_single_width: estado).
# - 0x936e80 tt_round_to_double_grid, 0x936eb0 tt_round_down_to_grid, 0x936ee0 tt_round_up_to_grid, 0x936f10
#   tt_round_to_grid, 0x936f40 tt_round_to_half_grid, 0x936f80 tt_round_off (mov eax, [esp+8]; ret: 5 bytes, o prologo e
#   a rotina inteira), 0x936f90 tt_super_round (+0x74 mascara, +0x7e fase, +0x80 limiar do estado global), 0x936ff0
#   tt_super_round_45 (+0x78 >> 14 o periodo, fixed_div e fixed_mul).
# - 0x9372c0 tt_read_cvt_stretched e 0x937360 tt_single_width_stretched (no levantamento "tt_current_ppem_stretched":
#   le a single width, +0x60): o esticamento pela direcao da projecao (+0xb8 x, +0xbc y do estado global).
# Iguais no cm0102-1080.exe (todas inteiras). Prologos sem call/jmp relativo.
# Comparadores: valor contra valor (puras). Prova sem o jogo: o emulador (rascunho rx01/check_rx01.py).
# Depende do rx01_1 (TtLocalState, TtGlobalState, TtReadFn, TtValueFn, lib_fixed_mul, lib_fixed_div, lib_f2dot14_mul).
SAVE = False
TELAS = True
ROTINAS = [('tt_round_to_double_grid', 0x936e80, [0x8b, 0x4c, 0x24, 0x08, 0x8b, 0x44, 0x24, 0x0c]),
           ('tt_round_down_to_grid', 0x936eb0, [0x8b, 0x4c, 0x24, 0x08, 0x8b, 0x44, 0x24, 0x0c]),
           ('tt_round_up_to_grid', 0x936ee0, [0x8b, 0x4c, 0x24, 0x08, 0x8b, 0x44, 0x24, 0x0c]),
           ('tt_round_to_grid', 0x936f10, [0x8b, 0x4c, 0x24, 0x08, 0x8b, 0x44, 0x24, 0x0c]),
           ('tt_round_to_half_grid', 0x936f40, [0x8b, 0x4c, 0x24, 0x08, 0x85, 0xc9]),
           ('tt_round_off', 0x936f80, [0x8b, 0x44, 0x24, 0x08, 0xc3]),
           ('tt_super_round', 0x936f90, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('tt_super_round_45', 0x936ff0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('tt_read_cvt_stretched', 0x9372c0, [0x53, 0x55, 0x8b, 0x6c, 0x24, 0x0c]),
           ('tt_single_width_stretched', 0x937360, [0x53, 0x55, 0x8b, 0x6c, 0x24, 0x0c])]
_A = 'library_without_asserts'
_R = 'simétrico em torno de zero (o negativo arredonda o módulo e volta o sinal), com a compensação somada antes; quando o sinal virar, '
_E = 'symmetric about zero (a negative value rounds its magnitude and gets its sign back), with the compensation added first; when the sign would flip, '
DOCS = ('o interpretador TrueType da T2K: os arredondamentos e os leitores esticados',
        'the T2K TrueType interpreter: the round functions and the stretched readers',
        [('tt_round_to_double_grid', 'tt_round_to_double_grid', '0x936e80', _A, 'cdecl',
          'regra: arredonda ao meio pixel mais próximo (múltiplo de 32), ' + _R + '0', 'rule: rounds to the nearest half pixel (multiple of 32), ' + _E + '0'),
         ('tt_round_down_to_grid', 'tt_round_down_to_grid', '0x936eb0', _A, 'cdecl',
          'regra: arredonda para baixo ao pixel (múltiplo de 64), ' + _R + '0', 'rule: rounds down to the pixel (multiple of 64), ' + _E + '0'),
         ('tt_round_up_to_grid', 'tt_round_up_to_grid', '0x936ee0', _A, 'cdecl',
          'regra: arredonda para cima ao pixel, ' + _R + '0', 'rule: rounds up to the pixel, ' + _E + '0'),
         ('tt_round_to_grid', 'tt_round_to_grid', '0x936f10', _A, 'cdecl',
          'regra: arredonda ao pixel mais próximo, ' + _R + '0', 'rule: rounds to the nearest pixel, ' + _E + '0'),
         ('tt_round_to_half_grid', 'tt_round_to_half_grid', '0x936f40', _A, 'cdecl',
          'regra: leva ao centro do pixel (64n + 32), ' + _R + '±32 com o sinal do valor',
          'rule: moves to the centre of the pixel (64n + 32), ' + _E + '±32 with the value\'s sign'),
         ('tt_round_off', 'tt_round_off', '0x936f80', _A, 'cdecl',
          'regra: sem arredondamento: devolve o valor', 'rule: no rounding: returns the value'),
         ('tt_super_round', 'tt_super_round', '0x936f90', _A, 'cdecl',
          'regra: o arredondamento do SROUND: (limiar − fase + valor) cortado pela máscara do período, mais a fase, ' + _R + 'a fase com o sinal do valor',
          'rule: SROUND rounding: (threshold − phase + value) cut by the period mask, plus the phase, ' + _E + 'the phase with the value\'s sign'),
         ('tt_super_round_45', 'tt_super_round_45', '0x936ff0', _A, 'cdecl',
          'regra: o arredondamento do S45ROUND: como o SROUND com o período de √2 (divide pelo período, corta ao pixel e multiplica de volta), ' + _R + 'a fase com o sinal do valor',
          'rule: S45ROUND rounding: like SROUND with the √2 period (divides by the period, cuts to the pixel and multiplies back), ' + _E + 'the phase with the value\'s sign'),
         ('tt_read_cvt_stretched', 'tt_read_cvt_stretched', '0x9372c0', _A, 'cdecl',
          'regra: o valor de controle n vezes o esticamento da direção de projeção (pixels não quadrados: o de x no eixo x, o de y no eixo y, senão a mistura pelos quadrados do vetor)',
          'rule: control value n times the stretch of the projection direction (non-square pixels: the x one along x, the y one along y, else the mix by the vector\'s squares)'),
         ('tt_single_width_stretched', 'tt_single_width_stretched', '0x937360', _A, 'cdecl',
          'regra: a largura única vezes o esticamento da direção de projeção',
          'rule: the single width times the stretch of the projection direction')])
CAMPOS = []
INTERESSA = []
