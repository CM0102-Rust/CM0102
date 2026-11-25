# O interpretador TrueType da T2K: os opcodes dos vetores (SVTCA, SPVTCA, SFVTCA, SPVTL, SDPVTL, SFVTL, SPVFS, SFVFS,
# GPV, GFV) e o movedor de ponto pelo vetor de liberdade (library_without_asserts; leitor rx01, rodada 6, 25/11/2026;
# o mapa em levas/reorganizacao/proximas/TRUETYPE.md).
# Item de TELAS. No jogo nao rodam (so numa fonte com instrucoes desenhada com ajuste: ver o rx01_1). Caminho no jogo:
# abrir o jogo comparando.
# Convencoes conferidas: cdecl; os opcodes pela tabela (estado local); 0x937e50 tt_direct_move pelo campo move_point
# (+0x42: estado, zona, ponto, distancia).
# - Os opcodes gravam os vetores (2.14, +0x14 projecao, +0x18 liberdade, +0x1c dual), o produto projecao.liberdade
#   (+0x40; |v| < 0x400 vira +-0x4000) e os enderecos das funcoes (+0x42 mover, +0x46 projetar, +0x4a dual).
# - SPVTL/SDPVTL/SFVTL: os dois pontos tirados da pilha (o de cima na zona 2, o de baixo na zona 1), a diferenca << 10,
#   fixed_hypot e fixed_div; >> 2 para 2.14; o bit 0 do opcode gira 90 graus.
# Iguais no cm0102-1080.exe (todas inteiras). Prologos sem call/jmp relativo.
# Comparadores: so a original (mudam o estado do interpretador). Prova sem o jogo: o emulador (rascunho
# rx01/check_rx01.py) com estados e zonas sorteados.
# As constantes TT_MOVE_*, TT_PROJECT*, TT_ONE e os auxiliares tt_set_proj_dot_free e tt_dot_proj_free daqui sao
# usados pelo rx01_4 e pela parte rx02.
# Depende do rx01_1.
SAVE = False
TELAS = True
ROTINAS = [('tt_op_svtca_y', 0x937550, [0x8b, 0x44, 0x24, 0x04, 0x33, 0xc9]),
           ('tt_op_svtca_x', 0x937d90, [0x8b, 0x44, 0x24, 0x04, 0x33, 0xd2]),
           ('tt_op_spvtca', 0x937dd0, [0x8b, 0x44, 0x24, 0x04, 0xf6, 0x40, 0x04, 0x01]),
           ('tt_direct_move', 0x937e50, [0x8b, 0x4c, 0x24, 0x04, 0x53]),
           ('tt_op_sfvtca', 0x937f60, [0x8b, 0x44, 0x24, 0x04, 0xf6, 0x40, 0x04, 0x01]),
           ('tt_op_spvtl', 0x937fd0, [0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10]),
           ('tt_op_sdpvtl', 0x9380f0, [0x51, 0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x14]),
           ('tt_op_sfvtl', 0x938280, [0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10]),
           ('tt_op_spvfs', 0x938360, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('tt_op_sfvfs', 0x9383f0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('tt_op_gpv', 0x938470, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('tt_op_gfv', 0x9384a0, [0x8b, 0x44, 0x24, 0x04, 0x56])]
_A = 'library_without_asserts'
_P = ' (o produto projeção·liberdade guardado, com o mínimo de 1/16 em módulo)'
_PE = ' (the projection·freedom product stored, with a 1/16 minimum magnitude)'
DOCS = ('o interpretador TrueType da T2K: os opcodes dos vetores e o movedor pelo vetor de liberdade',
        'the T2K TrueType interpreter: the vector opcodes and the mover along the freedom vector',
        [('tt_op_svtca_y', 'tt_op_svtca_y', '0x937550', _A, 'cdecl',
          'regra: SVTCA[y] (0x00): projeção e liberdade no eixo y, com o movedor e os projetores do eixo',
          'rule: SVTCA[y] (0x00): projection and freedom along y, with the axis mover and projectors'),
         ('tt_op_svtca_x', 'tt_op_svtca_x', '0x937d90', _A, 'cdecl',
          'regra: SVTCA[x] (0x01): projeção e liberdade no eixo x', 'rule: SVTCA[x] (0x01): projection and freedom along x'),
         ('tt_op_spvtca', 'tt_op_spvtca', '0x937dd0', _A, 'cdecl',
          'regra: SPVTCA (0x02 y, 0x03 x): a projeção num eixo' + _P, 'rule: SPVTCA (0x02 y, 0x03 x): the projection along an axis' + _PE),
         ('tt_direct_move', 'tt_direct_move', '0x937e50', _A, 'cdecl',
          'regra: move um ponto pela distância (medida na projeção) ao longo do vetor de liberdade: cada eixo com parte não nula anda distância × parte (dividida pelo produto projeção·liberdade quando ele não é 1) e fica marcado',
          'rule: moves a point by the distance (measured along the projection) along the freedom vector: each axis with a non-zero part moves distance × part (divided by the projection·freedom product when it is not 1) and is marked'),
         ('tt_op_sfvtca', 'tt_op_sfvtca', '0x937f60', _A, 'cdecl',
          'regra: SFVTCA (0x04 y, 0x05 x): a liberdade num eixo' + _P, 'rule: SFVTCA (0x04 y, 0x05 x): the freedom along an axis' + _PE),
         ('tt_op_spvtl', 'tt_op_spvtl', '0x937fd0', _A, 'cdecl',
          'regra: SPVTL (0x06, 0x07 girado 90°): a projeção na direção da reta entre dois pontos tirados da pilha' + _P,
          'rule: SPVTL (0x06, 0x07 turned 90°): the projection along the line between two points popped from the stack' + _PE),
         ('tt_op_sdpvtl', 'tt_op_sdpvtl', '0x9380f0', _A, 'cdecl',
          'regra: SDPVTL (0x86, 0x87 girado): a projeção pelos pontos atuais e a projeção dual pelos originais' + _P,
          'rule: SDPVTL (0x86, 0x87 turned): the projection from the current points and the dual projection from the original ones' + _PE),
         ('tt_op_sfvtl', 'tt_op_sfvtl', '0x938280', _A, 'cdecl',
          'regra: SFVTL (0x08, 0x09 girado): a liberdade na direção da reta entre dois pontos' + _P,
          'rule: SFVTL (0x08, 0x09 turned): the freedom along the line between two points' + _PE),
         ('tt_op_spvfs', 'tt_op_spvfs', '0x938360', _A, 'cdecl',
          'regra: SPVFS (0x0a): a projeção tirada da pilha (y, depois x)' + _P, 'rule: SPVFS (0x0a): the projection popped from the stack (y, then x)' + _PE),
         ('tt_op_sfvfs', 'tt_op_sfvfs', '0x9383f0', _A, 'cdecl',
          'regra: SFVFS (0x0b): a liberdade tirada da pilha' + _P, 'rule: SFVFS (0x0b): the freedom popped from the stack' + _PE),
         ('tt_op_gpv', 'tt_op_gpv', '0x938470', _A, 'cdecl',
          'regra: GPV (0x0c): empilha o x e o y da projeção', 'rule: GPV (0x0c): pushes the projection\'s x and y'),
         ('tt_op_gfv', 'tt_op_gfv', '0x9384a0', _A, 'cdecl',
          'regra: GFV (0x0d): empilha o x e o y da liberdade', 'rule: GFV (0x0d): pushes the freedom\'s x and y')])
CAMPOS = []
INTERESSA = []
