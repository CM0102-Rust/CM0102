# O interpretador TrueType da T2K: GETINFO, as marcas, a aritmética, IF e ELSE, os saltos e ROUND.
# library_without_asserts; leitor rx02, rodada 6, 25/11/2026; o mapa em levas/reorganizacao/proximas/TRUETYPE.md.
# Item de TELAS: o texto desenhado e tela. O interpretador so roda na carga das fontes (o programa da fonte do
# symbol.ttf, PUSHW e FDEF); o resto so roda numa fonte com instrucoes desenhada com ajuste a grade, o que o jogo
# nao faz (ver o rx01_1). Caminho no jogo: abrir o jogo comparando (as fontes carregam no inicio do video).
# Convencoes conferidas na desmontagem: todas cdecl; os opcodes pela tabela dos opcodes (estado local); as outras
# pelo ponto de chamada (tt_set_super_round_params: estado, byte, grade; tt_compute_rp_displacement: estado, &dx,
# &dy, &ponto -> zona; tt_delta_scan_apply: estado, retorno, base, shift; tt_delta_cvt_add como o move_point;
# glyph_hint_read_font_params: parametros, fluxo; glyph_hint_measure_font_params: fonte, ajustador, parametros,
# amostrar glifos).
# Iguais no cm0102-1080.exe (todas inteiras). Prologos sem call/jmp relativo.
# Comparadores: so a original (mudam o estado do interpretador ou leem uma fonte). Prova sem o jogo: o emulador
# (rascunho rx02/check_rx02.py), a original e a copia sobre o mesmo mundo sorteado, memoria inteira comparada.
# lib_f26dot6_mul (0x944760, rx03) e os auxiliares tt_fetch_opcode e tt_skip_push_data daqui sao usados pelo rx02_6.
# Depende de: o rx01_1, o rx01_2 (tt_round_to_grid), o rx01_5 e o rx02_1.
SAVE = False
TELAS = True
ROTINAS = [('tt_op_getinfo', 0x93a1b0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('tt_op_flipon', 0x93a210, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x28]),
           ('tt_op_flipoff', 0x93a220, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x28]),
           ('tt_op_debug', 0x93a230, [0x8b, 0x44, 0x24, 0x04, 0x83, 0x40, 0x20, 0xfc]),
           ('tt_op_binary_arith_compare', 0x93a240, [0x55, 0x8b, 0x6c, 0x24, 0x08]),
           ('tt_op_unary_math', 0x93a400, [0x55, 0x8b, 0x6c, 0x24, 0x08]),
           ('tt_op_if', 0x93a4d0, [0x8b, 0x4c, 0x24, 0x04, 0x55]),
           ('tt_op_else', 0x93a600, [0x8b, 0x4c, 0x24, 0x04, 0x55]),
           ('tt_op_jmpr', 0x93a720, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x20]),
           ('tt_op_jrot', 0x93a740, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x41, 0x20]),
           ('tt_op_jrof', 0x93a770, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x41, 0x20]),
           ('tt_op_round', 0x93a7a0, [0x56, 0x8b, 0x74, 0x24, 0x08])]
_A = 'library_without_asserts'
DOCS = ('o interpretador TrueType da T2K: GETINFO, as marcas, a aritmética, IF e ELSE, os saltos e ROUND',
        'the T2K TrueType interpreter: GETINFO, the flags, the arithmetic, IF and ELSE, the jumps and ROUND',
        [('tt_op_getinfo', 'tt_op_getinfo', '0x93a1b0', _A, 'cdecl',
          'regra: GETINFO: tira o seletor e empilha a versão 1 e as marcas de girado e esticado',
          'rule: GETINFO: pops the selector and pushes the version 1 and the rotated and stretched flags'),
         ('tt_op_flipon', 'tt_op_flipon', '0x93a210', _A, 'cdecl',
          'regra: FLIPON: liga a inversão automática',
          'rule: FLIPON: sets the auto flip'),
         ('tt_op_flipoff', 'tt_op_flipoff', '0x93a220', _A, 'cdecl',
          'regra: FLIPOFF: desliga a inversão automática',
          'rule: FLIPOFF: clears the auto flip'),
         ('tt_op_debug', 'tt_op_debug', '0x93a230', _A, 'cdecl',
          'regra: DEBUG: tira um valor e o ignora',
          'rule: DEBUG: pops a value and ignores it'),
         ('tt_op_binary_arith_compare', 'tt_op_binary_arith_compare', '0x93a240', _A, 'cdecl',
          'regra: LT a NEQ, AND, OR, ADD, SUB, DIV, MUL, MAX e MIN: tira dois valores e empilha o resultado; defeito do original: DIV por zero não é conferido',
          'rule: LT to NEQ, AND, OR, ADD, SUB, DIV, MUL, MAX and MIN: pops two values and pushes the result; defect of the original: DIV by zero is not checked'),
         ('tt_op_unary_math', 'tt_op_unary_math', '0x93a400', _A, 'cdecl',
          'regra: ODD, EVEN, NOT, ABS, NEG, FLOOR e CEILING sobre o valor do topo',
          'rule: ODD, EVEN, NOT, ABS, NEG, FLOOR and CEILING on the top value'),
         ('tt_op_if', 'tt_op_if', '0x93a4d0', _A, 'cdecl',
          'regra: IF: tira a condição; se 0, pula até o ELSE do mesmo nível ou o EIF, contando os IF aninhados e pulando os dados dos PUSH',
          'rule: IF: pops the condition; when 0 it skips to the ELSE of the same level or the EIF, counting nested IFs and skipping push data'),
         ('tt_op_else', 'tt_op_else', '0x93a600', _A, 'cdecl',
          'regra: ELSE: pula até o EIF do mesmo nível',
          'rule: ELSE: skips to the EIF of the same level'),
         ('tt_op_jmpr', 'tt_op_jmpr', '0x93a720', _A, 'cdecl',
          'regra: JMPR: tira um deslocamento e salta',
          'rule: JMPR: pops an offset and jumps'),
         ('tt_op_jrot', 'tt_op_jrot', '0x93a740', _A, 'cdecl',
          'regra: JROT: tira a condição e o deslocamento e salta se a condição não é 0',
          'rule: JROT: pops the condition and the offset and jumps when the condition is not 0'),
         ('tt_op_jrof', 'tt_op_jrof', '0x93a770', _A, 'cdecl',
          'regra: JROF: tira a condição e o deslocamento e salta se a condição é 0',
          'rule: JROF: pops the condition and the offset and jumps when the condition is 0'),
         ('tt_op_round', 'tt_op_round', '0x93a7a0', _A, 'cdecl',
          'regra: ROUND: arredonda o topo pela função atual',
          'rule: ROUND: rounds the top value by the current function')])
CAMPOS = []
INTERESSA = []
