# O interpretador TrueType da T2K: os PUSHW, o armazenamento, a tabela dos valores de controle e as medidas.
# library_without_asserts; leitor rx02, rodada 6, 25/11/2026; o mapa em levas/reorganizacao/proximas/TRUETYPE.md.
# Item de TELAS: o texto desenhado e tela. O interpretador so roda na carga das fontes (o programa da fonte do
# symbol.ttf, PUSHW e FDEF); o resto so roda numa fonte com instrucoes desenhada com ajuste a grade, o que o jogo
# nao faz (ver o rx01_1). Caminho no jogo: abrir o jogo comparando (as fontes carregam no inicio do video).
# - No jogo roda o PUSHW (programa da fonte do symbol.ttf, na carga das fontes); o resto nao roda.
# Convencoes conferidas na desmontagem: todas cdecl; os opcodes pela tabela dos opcodes (estado local); as outras
# pelo ponto de chamada (tt_set_super_round_params: estado, byte, grade; tt_compute_rp_displacement: estado, &dx,
# &dy, &ponto -> zona; tt_delta_scan_apply: estado, retorno, base, shift; tt_delta_cvt_add como o move_point;
# glyph_hint_read_font_params: parametros, fluxo; glyph_hint_measure_font_params: fonte, ajustador, parametros,
# amostrar glifos).
# Iguais no cm0102-1080.exe (todas inteiras). Prologos sem call/jmp relativo.
# Comparadores: so a original (mudam o estado do interpretador ou leem uma fonte). Prova sem o jogo: o emulador
# (rascunho rx02/check_rx02.py), a original e a copia sobre o mesmo mundo sorteado, memoria inteira comparada.
# Depende de: o rx01_1, o rx01_2 (projection_stretch), o rx01_5, o rx02_1 e o rx02_2.
SAVE = False
TELAS = True
ROTINAS = [('tt_op_pushw', 0x939d90, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('tt_op_npushw', 0x939de0, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('tt_op_ws', 0x939e30, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x20]),
           ('tt_op_rs', 0x939e60, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('tt_op_wcvtp', 0x939e90, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('tt_op_wcvtf', 0x939ef0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('tt_op_rcvt', 0x939f20, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('tt_op_gc', 0x939f50, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('tt_op_scfs', 0x939fc0, [0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10]),
           ('tt_op_md', 0x93a020, [0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10]),
           ('tt_op_mppem', 0x93a0d0, [0x55, 0x8b, 0x6c, 0x24, 0x08]),
           ('tt_op_mps', 0x93a190, [0x8b, 0x44, 0x24, 0x04, 0x33, 0xd2])]
_A = 'library_without_asserts'
DOCS = ('o interpretador TrueType da T2K: os PUSHW, o armazenamento, a tabela dos valores de controle e as medidas',
        'the T2K TrueType interpreter: the word pushes, the storage, the control value table and the measures',
        [('tt_op_pushw', 'tt_op_pushw', '0x939d90', _A, 'cdecl',
          'regra: PUSHW[n]: empilha as n + 1 palavras big-endian depois do opcode, com sinal (roda no jogo, no programa da fonte do symbol.ttf)',
          'rule: PUSHW[n]: pushes the n + 1 big-endian words after the opcode, signed (runs in the game, in the font program of symbol.ttf)'),
         ('tt_op_npushw', 'tt_op_npushw', '0x939de0', _A, 'cdecl',
          'regra: NPUSHW: lê a contagem e empilha essas palavras',
          'rule: NPUSHW: reads the count and pushes those words'),
         ('tt_op_ws', 'tt_op_ws', '0x939e30', _A, 'cdecl',
          'regra: WS: tira um valor e um índice e grava no armazenamento',
          'rule: WS: pops a value and an index and writes it to the storage'),
         ('tt_op_rs', 'tt_op_rs', '0x939e60', _A, 'cdecl',
          'regra: RS: troca o índice do topo pelo valor armazenado',
          'rule: RS: replaces the index on top with the stored value'),
         ('tt_op_wcvtp', 'tt_op_wcvtp', '0x939e90', _A, 'cdecl',
          'regra: WCVTP: tira um valor em pixels e um índice e grava no cvt; se a leitura (esticada) der outro valor não nulo r, grava v × (v ÷ r) para que leia v',
          'rule: WCVTP: pops a value in pixels and an index and writes it to the cvt; when the (stretched) reading gives another non-zero r, writes v × (v ÷ r) so that it reads back v'),
         ('tt_op_wcvtf', 'tt_op_wcvtf', '0x939ef0', _A, 'cdecl',
          'regra: WCVTF: tira um valor em unidades e um índice e grava no cvt o valor escalado',
          'rule: WCVTF: pops a value in font units and an index and writes the scaled value to the cvt'),
         ('tt_op_rcvt', 'tt_op_rcvt', '0x939f20', _A, 'cdecl',
          'regra: RCVT: troca o índice do topo pelo valor do cvt',
          'rule: RCVT: replaces the index on top with the control value'),
         ('tt_op_gc', 'tt_op_gc', '0x939f50', _A, 'cdecl',
          'regra: GC: troca o ponto (zona 2) do topo pela sua coordenada pelo vetor de projeção, atual ou original',
          'rule: GC: replaces the point (zone 2) on top with its coordinate along the projection vector, current or original'),
         ('tt_op_scfs', 'tt_op_scfs', '0x939fc0', _A, 'cdecl',
          'regra: SCFS: tira uma coordenada e um ponto da zona 2 e o move até ela; na zona do crepúsculo o original segue',
          'rule: SCFS: pops a coordinate and a point of zone 2 and moves it there; in the twilight zone the original follows'),
         ('tt_op_md', 'tt_op_md', '0x93a020', _A, 'cdecl',
          'regra: MD: tira dois pontos (zona 1 e zona 0) e empilha a distância pelo vetor de projeção, atual ou original em unidades escalada',
          'rule: MD: pops two points (zone 1 and zone 0) and pushes their distance along the projection vector, current or original in font units scaled'),
         ('tt_op_mppem', 'tt_op_mppem', '0x93a0d0', _A, 'cdecl',
          'regra: MPPEM: empilha os pixels por em (esticados pelo vetor de projeção com pixels não quadrados)',
          'rule: MPPEM: pushes the pixels per em (stretched along the projection vector with non-square pixels)'),
         ('tt_op_mps', 'tt_op_mps', '0x93a190', _A, 'cdecl',
          'regra: MPS: empilha o tamanho em pontos',
          'rule: MPS: pushes the point size')])
CAMPOS = []
INTERESSA = []
