# O interpretador TrueType da T2K: ALIGNRP, ALIGNPTS, as marcas do contorno, o controle de varredura e de instruções, AA e os PUSHB.
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
# Depende de: o rx01_1, o rx01_3 (tt_set_proj_dot_free, tt_dot_proj_free, tt_direct_move), o rx01_5, o rx02_1 e o rx02_2.
SAVE = False
TELAS = True
ROTINAS = [('tt_op_alignrp', 0x939750, [0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10]),
           ('tt_op_alignpts', 0x9397d0, [0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10]),
           ('tt_op_sangw', 0x939840, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x20]),
           ('tt_op_flippt', 0x939860, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('tt_op_fliprgon', 0x9398a0, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('tt_op_fliprgoff', 0x9398e0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('tt_op_scanctrl', 0x939920, [0x8b, 0x4c, 0x24, 0x04, 0x56]),
           ('tt_op_scantype', 0x939950, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x20]),
           ('tt_op_instctrl', 0x9399a0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('tt_op_aa', 0x939a00, [0x83, 0xec, 0x3c, 0x53, 0x55]),
           ('tt_op_pushb', 0x939d10, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('tt_op_npushb', 0x939d50, [0x8b, 0x44, 0x24, 0x04, 0x56])]
_A = 'library_without_asserts'
DOCS = ('o interpretador TrueType da T2K: ALIGNRP, ALIGNPTS, as marcas do contorno, o controle de varredura e de instruções, AA e os PUSHB',
        'the T2K TrueType interpreter: ALIGNRP, ALIGNPTS, the outline flags, the scan and instruction control, AA and the byte pushes',
        [('tt_op_alignrp', 'tt_op_alignrp', '0x939750', _A, 'cdecl',
          'regra: ALIGNRP: tira pontos da zona 1 (o laço) e os leva ao ponto de referência 0 pelo vetor de projeção',
          'rule: ALIGNRP: pops loop-count points of zone 1 and moves them onto reference point 0 along the projection vector'),
         ('tt_op_alignpts', 'tt_op_alignpts', '0x9397d0', _A, 'cdecl',
          'regra: ALIGNPTS: tira dois pontos e move cada um metade da distância entre eles; defeito do original: mede com o primeiro na zona 0 e move-o na zona 1 (e o segundo ao contrário)',
          'rule: ALIGNPTS: pops two points and moves each by half their distance; defect of the original: measures with the first in zone 0 and moves it in zone 1 (the second the other way)'),
         ('tt_op_sangw', 'tt_op_sangw', '0x939840', _A, 'cdecl',
          'regra: SANGW: tira o peso dos ângulos',
          'rule: SANGW: pops the angle weight'),
         ('tt_op_flippt', 'tt_op_flippt', '0x939860', _A, 'cdecl',
          'regra: FLIPPT: tira pontos da zona 0 (o laço) e inverte a marca de ponto na curva de cada um',
          "rule: FLIPPT: pops loop-count points of zone 0 and flips each one's on-curve mark"),
         ('tt_op_fliprgon', 'tt_op_fliprgon', '0x9398a0', _A, 'cdecl',
          'regra: FLIPRGON: tira uma faixa de pontos da zona 0 e os põe na curva',
          'rule: FLIPRGON: pops a range of points of zone 0 and puts them on the curve'),
         ('tt_op_fliprgoff', 'tt_op_fliprgoff', '0x9398e0', _A, 'cdecl',
          'regra: FLIPRGOFF: tira uma faixa de pontos da zona 0 e os tira da curva',
          'rule: FLIPRGOFF: pops a range of points of zone 0 and takes them off the curve'),
         ('tt_op_scanctrl', 'tt_op_scanctrl', '0x939920', _A, 'cdecl',
          'regra: SCANCTRL: tira o valor da metade baixa da palavra de controle de varredura (somado com OU inteiro, como no original)',
          "rule: SCANCTRL: pops the low half of the scan control word (OR'ed in whole, as in the original)"),
         ('tt_op_scantype', 'tt_op_scantype', '0x939950', _A, 'cdecl',
          'regra: SCANTYPE: tira o tipo 0, 1 ou 2 para a metade alta da palavra de controle de varredura',
          'rule: SCANTYPE: pops the type 0, 1 or 2 into the high half of the scan control word'),
         ('tt_op_instctrl', 'tt_op_instctrl', '0x9399a0', _A, 'cdecl',
          'regra: INSTCTRL: tira um seletor e um valor; só durante o programa do cvt, o seletor 1 ou 2 grava o bit 0 ou 1 do controle de instruções',
          'rule: INSTCTRL: pops a selector and a value; only while the control value program runs, selector 1 or 2 sets instruction control bit 0 or 1'),
         ('tt_op_aa', 'tt_op_aa', '0x939a00', _A, 'cdecl',
          'regra: AA (versão da T2K): tira um ponto e escolhe, entre os 20 vetores da tabela dos ângulos, o de menor custo (o custo da tabela + o peso × a distância/64); move o ponto pela distância ao ponto de referência 0 medida por ele e volta o vetor de projeção; defeitos do original: troca as zonas do ponto e do ponto de referência entre a escolha e a medida, e lê os pontos 2.30 da tabela como valores de 16 bits, de modo que com a tabela do jogo o primeiro vetor, (0, 0), sempre vence e o AA só toca o ponto',
          "rule: AA (T2K's version): pops a point and picks, among the 20 angle table vectors, the cheapest (the table cost + the weight × the distance/64); moves the point by its distance to reference point 0 measured along it and restores the projection vector; defects of the original: swaps the zones of the point and the reference point between the choice and the measurement, and reads the table's 2.30 points as 16-bit values, so with the game's table the first vector, (0, 0), always wins and AA only touches the point"),
         ('tt_op_pushb', 'tt_op_pushb', '0x939d10', _A, 'cdecl',
          'regra: PUSHB[n]: empilha os n + 1 bytes depois do opcode',
          'rule: PUSHB[n]: pushes the n + 1 bytes after the opcode'),
         ('tt_op_npushb', 'tt_op_npushb', '0x939d50', _A, 'cdecl',
          'regra: NPUSHB: lê a contagem e empilha esses bytes',
          'rule: NPUSHB: reads the count and pushes those bytes')])
CAMPOS = []
INTERESSA = []
