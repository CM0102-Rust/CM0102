# O interpretador TrueType da T2K (library_without_asserts; a T2K da Type Solutions, o rasterizador das fontes .t2k e
# .ttf do jogo): os estados (TtLocalState, TtZone, TtGlobalState), a tabela dos opcodes, os lacos das instrucoes e os
# chamados de ponto e de projecao (leitor rx01, rodada 6, 25/11/2026; o mapa em levas/reorganizacao/proximas/TRUETYPE.md).
# Item de TELAS: o texto desenhado e tela. No jogo, so roda na carga de cada fonte (FONT, 0x5b1aae -> tt_font_new ->
# tt_font_load_tables -> tt_exec_create 0x941830): tt_init_opcode_table sempre; tt_run_ins_range no programa da fonte
# (fpgm) do symbol.ttf, que so tem PUSHW[0] e FDEF. O desenho das letras (T2K_RenderGlyph com cmd = 2, sem T2K_GRID_FIT)
# nao chama o interpretador nem o ajustador: o resto so roda numa fonte com instrucoes desenhada com ajuste, o que o
# jogo nao faz. Caminho no jogo: abrir o jogo comparando (as fontes carregam no inicio do video).
# Convencoes conferidas: todas cdecl, chamadas pela tabela dos opcodes ou pelos campos de funcao do estado local
# (call dword ptr [..]; add esp do chamador): opcodes (estado); move_point (estado, zona, ponto, distancia);
# project/dual_project (estado, dx, dy); get_cvt (estado, indice); get_single_width (estado); interpreter (estado,
# inicio, fim); tt_init_opcode_table (estado global).
# - 0x9375d0 tt_init_opcode_table: os 256 enderecos (lidos da desmontagem e conferidos no emulador), os 39 dwords dos
#   angulos em +0x400 e as 20 palavras em +0x4a0, e +0xc0/+0xc4 do estado global; +0x49c fica como estava.
# - 0x937500 tt_run_ins_range e 0x937410 tt_run_ins_range_with_callback (o codigo do arredondamento em +0x3c, o gancho
#   em +0x2c; sem gancho a de gancho sai sem restaurar o ponteiro de instrucao, como a original).
# - 0x937d70 tt_op_invalid_opcode: longjmp 0x94791c (buffer em +0x5a do estado local, codigo 0x1101).
# - 0x937260 tt_move_point_and_touch, 0x937590 tt_direct_move_y, 0x937290 tt_project_x, 0x9375c0 tt_project_y (as duas
#   de 5 bytes: mov eax, [esp+n]; ret: o prologo e a rotina inteira), 0x9380c0 tt_project, 0x938250 tt_dual_project,
#   0x9372a0 tt_read_cvt, 0x937350 tt_single_width (no levantamento "tt_current_ppem": le +0x60 do estado global, o
#   valor que o SSW grava e que o tt_interpreter_run reescala; e o get_single_width do estado local, +0x56).
# - As contas de ponto fixo (fixed_mul 0x9446e0, f2dot14_mul 0x9447e0, fixed_div 0x944840, fixed_hypot 0x9448b0, do
#   rx03) chamadas pelo endereco (lib_fixed_mul...); o longjmp do CRT pelo tt_raise.
# Fora: 0x937090 tt_interpreter_run (prepara o estado local na pilha e roda o programa sob setjmp 0x947998: o setjmp
# nao tem como ser chamado do Rust; fica com o exe, que chama os opcodes ja em Rust).
# Iguais no cm0102-1080.exe (todas inteiras). Prologos sem call/jmp relativo.
# Comparadores: a tabela lado a lado (a original na tabela do jogo, a copia num bloco proprio); os projetores e os
# leitores valor contra valor; os lacos, a saida de erro e os que movem pontos so a original.
# Prova sem o jogo: o emulador (unicorn) roda a original e a copia sobre o mesmo mundo falso
# (rascunho rx01/check_rx01.py), com estados, zonas e pilhas sorteados.
# Os tipos TtLocalState, TtZone, TtGlobalState, os tipos de funcao Tt*Fn, lib_fixed_mul/lib_f2dot14_mul/lib_fixed_div/
# lib_fixed_hypot e tt_raise daqui sao usados pelos rx01_2..4 e pelas partes rx02 e rx03.
SAVE = False
TELAS = True
ROTINAS = [('tt_init_opcode_table', 0x9375d0, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('tt_run_ins_range', 0x937500, [0x8b, 0x44, 0x24, 0x08, 0x53]),
           ('tt_run_ins_range_with_callback', 0x937410, [0x55, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('tt_op_invalid_opcode', 0x937d70, [0x8b, 0x44, 0x24, 0x04, 0x68, 0x01, 0x11, 0x00, 0x00]),
           ('tt_move_point_and_touch', 0x937260, [0x8b, 0x4c, 0x24, 0x08, 0x8b, 0x54, 0x24, 0x0c]),
           ('tt_direct_move_y', 0x937590, [0x8b, 0x4c, 0x24, 0x08, 0x8b, 0x54, 0x24, 0x0c]),
           ('tt_project_x', 0x937290, [0x8b, 0x44, 0x24, 0x08, 0xc3]),
           ('tt_project_y', 0x9375c0, [0x8b, 0x44, 0x24, 0x0c, 0xc3]),
           ('tt_project', 0x9380c0, [0x8b, 0x4c, 0x24, 0x08, 0x56]),
           ('tt_dual_project', 0x938250, [0x8b, 0x4c, 0x24, 0x08, 0x56]),
           ('tt_read_cvt', 0x9372a0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x28]),
           ('tt_single_width', 0x937350, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x28])]
_A = 'library_without_asserts'
DOCS = ('o interpretador TrueType da T2K: a tabela dos opcodes, os laços das instruções e os chamados de ponto e de projeção',
        'the T2K TrueType interpreter: the opcode table, the instruction loops and the point and projection callbacks',
        [('tt_init_opcode_table', 'tt_init_opcode_table', '0x9375d0', _A, 'cdecl',
          'regra: preenche a tabela dos 256 opcodes do estado global com a rotina de cada um (os sem definição vão para a saída de erro) e, logo depois dela, as tabelas dos ângulos (39 vetores e 20 distâncias) do SANGW e do AA',
          'rule: fills the global state\'s table of the 256 opcodes with each one\'s handler (the undefined ones go to the error exit) and, right after it, the angle tables (39 vectors and 20 distances) of SANGW and AA'),
         ('tt_run_ins_range', 'tt_run_ins_range', '0x937500', _A, 'cdecl',
          'regra: roda as instruções de um trecho, byte a byte: guarda o opcode e chama a rotina dele pela tabela; no fim volta o ponteiro de instrução ao que era',
          'rule: runs the instructions of a range byte by byte: stores the opcode and calls its handler through the table; at the end restores the instruction pointer'),
         ('tt_run_ins_range_with_callback', 'tt_run_ins_range_with_callback', '0x937410', _A, 'cdecl',
          'regra: o mesmo laço com o gancho de depuração antes de cada opcode (com o arredondamento atual como número); o gancho pode se desligar e parar o laço; sem gancho não roda nada',
          'rule: the same loop with the debug hook before each opcode (with the current rounding as a number); the hook may switch itself off and stop the loop; without a hook nothing runs'),
         ('tt_op_invalid_opcode', 'tt_op_invalid_opcode', '0x937d70', _A, 'cdecl',
          'regra: opcode sem definição (e ENDF fora de uma definição): sai do programa pelo longjmp com o erro 0x1101',
          'rule: an undefined opcode (and ENDF outside a definition): leaves the program by longjmp with error 0x1101'),
         ('tt_move_point_and_touch', 'tt_move_point_and_touch', '0x937260', _A, 'cdecl',
          'regra: move um ponto da zona em x pela distância e o marca como tocado em x',
          'rule: moves a point of the zone in x by the distance and marks it touched in x'),
         ('tt_direct_move_y', 'tt_direct_move_y', '0x937590', _A, 'cdecl',
          'regra: move um ponto da zona em y pela distância e o marca como tocado em y',
          'rule: moves a point of the zone in y by the distance and marks it touched in y'),
         ('tt_project_x', 'tt_project_x', '0x937290', _A, 'cdecl',
          'regra: a projeção no eixo x: o dx', 'rule: the projection on the x axis: dx'),
         ('tt_project_y', 'tt_project_y', '0x9375c0', _A, 'cdecl',
          'regra: a projeção no eixo y: o dy', 'rule: the projection on the y axis: dy'),
         ('tt_project', 'tt_project', '0x9380c0', _A, 'cdecl',
          'regra: a projeção de (dx, dy) no vetor de projeção (2.14)',
          'rule: the projection of (dx, dy) on the projection vector (2.14)'),
         ('tt_dual_project', 'tt_dual_project', '0x938250', _A, 'cdecl',
          'regra: a projeção de (dx, dy) no vetor de projeção dual (o do contorno original)',
          'rule: the projection of (dx, dy) on the dual projection vector (the original outline\'s)'),
         ('tt_read_cvt', 'tt_read_cvt', '0x9372a0', _A, 'cdecl',
          'regra: o valor n da tabela de valores de controle, sem escala (pixels quadrados)',
          'rule: value n of the control value table, unscaled (square pixels)'),
         ('tt_single_width', 'tt_single_width', '0x937350', _A, 'cdecl',
          'regra: a largura única (single width) já em pixels do estado global (pixels quadrados)',
          'rule: the global state\'s single width, already in pixels (square pixels)')])
CAMPOS = [('estado local do interpretador (TtLocalState, 0xa0 bytes na pilha do tt_interpreter_run): +0 ponteiro de instrução, +4 opcode, +8/+0xc/+0x10 zonas 0-2, +0x14 projeção, +0x18 liberdade, +0x1c dual (2.14), +0x20 pilha, +0x24 zonas, +0x28 estado global, +0x2c gancho, +0x30-0x38 pontos de referência, +0x3c código do arredondamento, +0x3e laço, +0x40 produto projeção·liberdade, +0x42..+0x56 funções (mover, projetar, dual, laço, cvt, single width), +0x5a jmp_buf',
           'interpreter local state (TtLocalState, 0xa0 bytes on tt_interpreter_run\'s stack): +0 instruction pointer, +4 opcode, +8/+0xc/+0x10 zones 0-2, +0x14 projection, +0x18 freedom, +0x1c dual (2.14), +0x20 stack, +0x24 zones, +0x28 global state, +0x2c hook, +0x30-0x38 reference points, +0x3c round code, +0x3e loop, +0x40 projection·freedom, +0x42..+0x56 functions (move, project, dual, loop, cvt, single width), +0x5a jmp_buf'),
          ('zona (TtZone, 0x2c bytes): +0 x, +4 y, +8 x original, +0xc y original (i32), +0x28 marcas (bit 0 tocado em x, bit 1 em y)',
           'zone (TtZone, 0x2c bytes): +0 x, +4 y, +8 original x, +0xc original y (i32), +0x28 flags (bit 0 touched in x, bit 1 in y)'),
          ('estado global (TtGlobalState): +8 cvt, +0x58 cvt cut-in, +0x5c single width cut-in, +0x60 single width, +0x6c distância mínima, +0x70 arredondamento, +0x74 máscara do período, +0x78 período 45, +0x7e fase, +0x80 limiar, +0x88 single width em unidades, +0x8c tabela dos opcodes, +0x98 escala, +0xb8/+0xbc esticamento x/y, +0xc0/+0xc4 tabelas dos ângulos',
           'global state (TtGlobalState): +8 cvt, +0x58 cvt cut-in, +0x5c single width cut-in, +0x60 single width, +0x6c minimum distance, +0x70 rounding, +0x74 period mask, +0x78 period 45, +0x7e phase, +0x80 threshold, +0x88 single width in units, +0x8c opcode table, +0x98 scaler, +0xb8/+0xbc x/y stretch, +0xc0/+0xc4 angle tables')]
INTERESSA = []
