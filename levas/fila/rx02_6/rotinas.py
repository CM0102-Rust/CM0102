# O interpretador TrueType da T2K: NROUND, MDRP, MIRP, as definições de funções e instruções, UTP, SDB, SDS e DELTAP1.
# library_without_asserts; leitor rx02, rodada 6, 25/11/2026; o mapa em levas/reorganizacao/proximas/TRUETYPE.md.
# Item de TELAS: o texto desenhado e tela. O interpretador so roda na carga das fontes (o programa da fonte do
# symbol.ttf, PUSHW e FDEF); o resto so roda numa fonte com instrucoes desenhada com ajuste a grade, o que o jogo
# nao faz (ver o rx01_1). Caminho no jogo: abrir o jogo comparando (as fontes carregam no inicio do video).
# - No jogo roda o FDEF (programa da fonte do symbol.ttf, na carga das fontes); o resto nao roda.
# Convencoes conferidas na desmontagem: todas cdecl; os opcodes pela tabela dos opcodes (estado local); as outras
# pelo ponto de chamada (tt_set_super_round_params: estado, byte, grade; tt_compute_rp_displacement: estado, &dx,
# &dy, &ponto -> zona; tt_delta_scan_apply: estado, retorno, base, shift; tt_delta_cvt_add como o move_point;
# glyph_hint_read_font_params: parametros, fluxo; glyph_hint_measure_font_params: fonte, ajustador, parametros,
# amostrar glifos).
# Iguais no cm0102-1080.exe (todas inteiras). Prologos sem call/jmp relativo.
# Comparadores: so a original (mudam o estado do interpretador ou leem uma fonte). Prova sem o jogo: o emulador
# (rascunho rx02/check_rx02.py), a original e a copia sobre o mesmo mundo sorteado, memoria inteira comparada.
# Depende de: o rx01_1, o rx01_3 (TT_PROJECT_X/Y), o rx01_5, o rx02_1, o rx02_2, o rx02_5 (tt_fetch_opcode, tt_skip_push_data) e o rx02_7 (tt_delta_scan_apply).
SAVE = False
TELAS = True
ROTINAS = [('tt_op_nround', 0x93a7d0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x20]),
           ('tt_op_mdrp', 0x93a7e0, [0x83, 0xec, 0x0c, 0x53, 0x55]),
           ('tt_op_mirp', 0x93a960, [0x83, 0xec, 0x10, 0x53, 0x55]),
           ('tt_op_call', 0x93abf0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('tt_op_fdef', 0x93ac30, [0x8b, 0x44, 0x24, 0x04, 0x53]),
           ('tt_op_loopcall', 0x93ad80, [0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10]),
           ('tt_op_call_user_defined', 0x93adf0, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('tt_op_idef', 0x93ae60, [0x51, 0x8b, 0x44, 0x24, 0x08]),
           ('tt_op_utp', 0x93aff0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x20]),
           ('tt_op_sdb', 0x93b030, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x20]),
           ('tt_op_sds', 0x93b050, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x20]),
           ('tt_op_deltap1', 0x93b070, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x41, 0x28])]
_A = 'library_without_asserts'
DOCS = ('o interpretador TrueType da T2K: NROUND, MDRP, MIRP, as definições de funções e instruções, UTP, SDB, SDS e DELTAP1',
        'the T2K TrueType interpreter: NROUND, MDRP, MIRP, the function and instruction definitions, UTP, SDB, SDS and DELTAP1',
        [('tt_op_nround', 'tt_op_nround', '0x93a7d0', _A, 'cdecl',
          'regra: NROUND: deixa o topo como está (sem compensação na T2K)',
          'rule: NROUND: leaves the top value as it is (no compensation in T2K)'),
         ('tt_op_mdrp', 'tt_op_mdrp', '0x93a7e0', _A, 'cdecl',
          'regra: MDRP: tira um ponto da zona 1 e o põe à distância original do ponto de referência 0 (original, ou em unidades escalada fora da zona do crepúsculo), com a largura única, o arredondamento (bit 2) e a distância mínima (bit 3); atualiza os pontos de referência',
          'rule: MDRP: pops a point of zone 1 and puts it at its original distance from reference point 0 (original, or in font units scaled outside the twilight zone), with the single width, the rounding (bit 2) and the minimum distance (bit 3); updates the reference points'),
         ('tt_op_mirp', 'tt_op_mirp', '0x93a960', _A, 'cdecl',
          'regra: MIRP: tira um índice do cvt e um ponto da zona 1 e o põe à distância do valor de controle do ponto de referência 0 (com largura única, inversão automática, cut-in e arredondamento no bit 2, distância mínima no bit 3); atualiza os pontos de referência',
          "rule: MIRP: pops a control value index and a point of zone 1 and puts it at the control value's distance from reference point 0 (with single width, auto flip, cut-in and rounding on bit 2, minimum distance on bit 3); updates the reference points"),
         ('tt_op_call', 'tt_op_call', '0x93abf0', _A, 'cdecl',
          'regra: CALL: tira o número da função e roda as instruções dela',
          'rule: CALL: pops the function number and runs its instructions'),
         ('tt_op_fdef', 'tt_op_fdef', '0x93ac30', _A, 'cdecl',
          'regra: FDEF: tira o número da função e grava o programa, o início e o tamanho do corpo até o ENDF (roda no jogo, no programa da fonte do symbol.ttf)',
          'rule: FDEF: pops the function number and records the program, the start and the length of the body up to ENDF (runs in the game, in the font program of symbol.ttf)'),
         ('tt_op_loopcall', 'tt_op_loopcall', '0x93ad80', _A, 'cdecl',
          'regra: LOOPCALL: tira a função e a contagem e roda a função essas vezes',
          'rule: LOOPCALL: pops the function and the count and runs the function that many times'),
         ('tt_op_call_user_defined', 'tt_op_call_user_defined', '0x93adf0', _A, 'cdecl',
          'regra: opcodes sem sentido no TrueType: roda a definição de instrução do opcode, ou sai com o erro 0x1101',
          "rule: opcodes with no TrueType meaning: runs the opcode's instruction definition, or leaves with error 0x1101"),
         ('tt_op_idef', 'tt_op_idef', '0x93ae60', _A, 'cdecl',
          'regra: IDEF: tira um opcode e grava a definição de instrução (reaproveitando a do opcode, senão uma nova no fim)',
          "rule: IDEF: pops an opcode and records its instruction definition (reusing the opcode's entry, else a new one at the end)"),
         ('tt_op_utp', 'tt_op_utp', '0x93aff0', _A, 'cdecl',
          'regra: UTP: tira um ponto da zona 0 e apaga a marca de tocado nos eixos que o vetor de liberdade move',
          'rule: UTP: pops a point of zone 0 and clears its touched mark on the axes the freedom vector moves'),
         ('tt_op_sdb', 'tt_op_sdb', '0x93b030', _A, 'cdecl',
          'regra: SDB: tira a base dos deltas',
          'rule: SDB: pops the delta base'),
         ('tt_op_sds', 'tt_op_sds', '0x93b050', _A, 'cdecl',
          'regra: SDS: tira o deslocamento dos deltas',
          'rule: SDS: pops the delta shift'),
         ('tt_op_deltap1', 'tt_op_deltap1', '0x93b070', _A, 'cdecl',
          'regra: DELTAP1: as exceções de ponto para os tamanhos a partir da base',
          'rule: DELTAP1: the point exceptions for the sizes from the base')])
CAMPOS = []
INTERESSA = []
