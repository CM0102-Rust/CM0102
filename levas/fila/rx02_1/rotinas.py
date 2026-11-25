# O interpretador TrueType da T2K: mais campos dos estados, SZP, os opcodes do arredondamento e os da pilha.
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
# Os metodos acrescentados a TtLocalState, TtZone e TtGlobalState (outro bloco impl, sem mexer nos do rx01_1), as
# constantes TT_ROUND_* e o auxiliar tt_round daqui sao usados pelos rx02_2..7.
# Depende de: o rx01_1 (os tipos do interpretador), o rx01_2 (TtRoundFn) e o rx01_5 (at_i16, at_i32, set_i32).
SAVE = False
TELAS = True
ROTINAS = [('tt_op_szp', 0x938880, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x20]),
           ('tt_op_sround', 0x9388d0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('tt_op_s45round', 0x938900, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('tt_op_set_round_mode', 0x938930, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x41, 0x28]),
           ('tt_set_super_round_params', 0x938a10, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x0c]),
           ('tt_op_push_advance_width', 0x938b80, [0x8b, 0x4c, 0x24, 0x04, 0x56]),
           ('tt_op_dup', 0x938bc0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x20]),
           ('tt_op_clear', 0x938be0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x28]),
           ('tt_op_swap', 0x938bf0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x40, 0x20]),
           ('tt_op_depth', 0x938c10, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('tt_op_cindex', 0x938c40, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x40, 0x20]),
           ('tt_op_mindex', 0x938c60, [0x56, 0x8b, 0x74, 0x24, 0x08])]
_A = 'library_without_asserts'
DOCS = ('o interpretador TrueType da T2K: mais campos dos estados, SZP, os opcodes do arredondamento e os da pilha',
        'the T2K TrueType interpreter: more state fields, SZP, the round state opcodes and the stack opcodes',
        [('tt_op_szp', 'tt_op_szp', '0x938880', _A, 'cdecl',
          'regra: SZP0, SZP1, SZP2 e SZPS: tira um número de zona e aponta o ponteiro de zona 0, 1 ou 2, ou os três, para aquela zona',
          'rule: SZP0, SZP1, SZP2 and SZPS: pops a zone number and points zone pointer 0, 1 or 2, or all three, at that zone'),
         ('tt_op_sround', 'tt_op_sround', '0x9388d0', _A, 'cdecl',
          'regra: SROUND: tira o byte do super arredondamento, decodifica-o com período na grade e instala o super arredondamento',
          'rule: SROUND: pops the super round byte, decodes it with a grid period and installs the super round'),
         ('tt_op_s45round', 'tt_op_s45round', '0x938900', _A, 'cdecl',
          'regra: S45ROUND: o mesmo com período a 45 graus',
          'rule: S45ROUND: the same with a 45 degree period'),
         ('tt_op_set_round_mode', 'tt_op_set_round_mode', '0x938930', _A, 'cdecl',
          'regra: RTG, RTHG, RTDG, ROFF, RUTG e RDTG: instalam a função de arredondamento do opcode',
          "rule: RTG, RTHG, RTDG, ROFF, RUTG and RDTG: install the opcode's round function"),
         ('tt_set_super_round_params', 'tt_set_super_round_params', '0x938a10', _A, 'cdecl',
          'regra: decodifica o byte do super arredondamento: o período (meio, um ou dois pixels na grade; raiz de 2 sobre 2, raiz de 2 ou 2 raiz de 2 a 45 graus; 999 no valor reservado), a fase (0, 1/4, 1/2 ou 3/4 do período) e o limiar (período − 1 para 0, senão (n − 4) × período / 8)',
          'rule: decodes the super round byte: the period (half, one or two pixels on the grid; √2/2, √2 or 2√2 at 45°; 999 for the reserved value), the phase (0, 1/4, 1/2 or 3/4 of the period) and the threshold (period − 1 for 0, else (n − 4) × period / 8)'),
         ('tt_op_push_advance_width', 'tt_op_push_advance_width', '0x938b80', _A, 'cdecl',
          'regra: opcode 0x28 próprio da T2K: empilha a largura de avanço sem ajuste, a distância entre os dois pontos fantasmas do glifo',
          "rule: T2K's own opcode 0x28: pushes the unhinted advance width, the distance between the glyph's two phantom points"),
         ('tt_op_dup', 'tt_op_dup', '0x938bc0', _A, 'cdecl',
          'regra: DUP: duplica o valor do topo',
          'rule: DUP: duplicates the top value'),
         ('tt_op_clear', 'tt_op_clear', '0x938be0', _A, 'cdecl',
          'regra: CLEAR: esvazia a pilha',
          'rule: CLEAR: empties the stack'),
         ('tt_op_swap', 'tt_op_swap', '0x938bf0', _A, 'cdecl',
          'regra: SWAP: troca os dois valores do topo',
          'rule: SWAP: swaps the top two values'),
         ('tt_op_depth', 'tt_op_depth', '0x938c10', _A, 'cdecl',
          'regra: DEPTH: empilha quantos valores há na pilha',
          'rule: DEPTH: pushes the number of values on the stack'),
         ('tt_op_cindex', 'tt_op_cindex', '0x938c40', _A, 'cdecl',
          'regra: CINDEX: troca o topo n por uma cópia do valor n lugares abaixo',
          'rule: CINDEX: replaces the top n with a copy of the value n places below'),
         ('tt_op_mindex', 'tt_op_mindex', '0x938c60', _A, 'cdecl',
          'regra: MINDEX: tira n e leva ao topo o valor n lugares abaixo, descendo os de cima',
          'rule: MINDEX: pops n and moves the value n places below to the top, the ones above sliding down')])
CAMPOS = []
INTERESSA = []
