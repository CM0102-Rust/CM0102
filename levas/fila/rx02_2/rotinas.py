# O interpretador TrueType da T2K: ROLL e os opcodes que movem pontos (MDAP, MIAP, IUP, SHP, SHC, SHZ, SHPIX, IP, MSIRP).
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
# Os auxiliares tt_current, tt_original, tt_original_units, tt_project_between e tt_dual_project_between daqui sao
# usados pelos rx02_3..6.
# Depende de: o rx01_1, o rx01_3 (TT_ONE), o rx01_5 e o rx02_1.
SAVE = False
TELAS = True
ROTINAS = [('tt_op_roll', 0x938ca0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x40, 0x20]),
           ('tt_op_mdap', 0x938cc0, [0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10]),
           ('tt_op_miap', 0x938d40, [0x51, 0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x14]),
           ('tt_op_iup', 0x938e20, [0x83, 0xec, 0x44, 0x8b, 0x44, 0x24, 0x48]),
           ('tt_op_shp', 0x939110, [0x83, 0xec, 0x0c, 0x8d, 0x44, 0x24, 0x08]),
           ('tt_compute_rp_displacement', 0x9391a0, [0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10]),
           ('tt_op_shc', 0x9392a0, [0x83, 0xec, 0x0c, 0x53, 0x55]),
           ('tt_op_shz', 0x939360, [0x83, 0xec, 0x0c, 0x53, 0x55]),
           ('tt_op_shpix', 0x9394a0, [0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10]),
           ('tt_op_ip', 0x939540, [0x83, 0xec, 0x0c, 0x53, 0x55]),
           ('tt_op_msirp', 0x939670, [0x51, 0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x14])]
_A = 'library_without_asserts'
DOCS = ('o interpretador TrueType da T2K: ROLL e os opcodes que movem pontos (MDAP, MIAP, IUP, SHP, SHC, SHZ, SHPIX, IP, MSIRP)',
        'the T2K TrueType interpreter: ROLL and the point moving opcodes (MDAP, MIAP, IUP, SHP, SHC, SHZ, SHPIX, IP, MSIRP)',
        [('tt_op_roll', 'tt_op_roll', '0x938ca0', _A, 'cdecl',
          'regra: ROLL: o terceiro valor a partir do topo vai para o topo',
          'rule: ROLL: the third value from the top moves to the top'),
         ('tt_op_mdap', 'tt_op_mdap', '0x938cc0', _A, 'cdecl',
          'regra: MDAP: tira um ponto da zona 0, que vira os pontos de referência 0 e 1; com o bit 0 vai à posição arredondada pelo vetor de projeção, senão só é tocado',
          'rule: MDAP: pops a point of zone 0, which becomes reference points 0 and 1; with bit 0 it moves to its rounded position along the projection vector, else it is only touched'),
         ('tt_op_miap', 'tt_op_miap', '0x938d40', _A, 'cdecl',
          'regra: MIAP: tira um índice do cvt e um ponto da zona 0 (pontos de referência 0 e 1); na zona do crepúsculo o ponto é posto no valor; vai ao valor pelo vetor de projeção, que com o bit 0 vira a posição atual se diferir mais que o cut-in, e é arredondado',
          'rule: MIAP: pops a control value index and a point of zone 0 (reference points 0 and 1); in the twilight zone the point is put at the value; it moves to the value along the projection vector, which with bit 0 becomes the current position when they differ by more than the cut-in, and is rounded'),
         ('tt_op_iup', 'tt_op_iup', '0x938e20', _A, 'cdecl',
          'regra: IUP: em cada contorno da zona 2, os pontos não tocados no eixo entre dois tocados os seguem: abaixo do menor e acima do maior (pela coordenada em unidades) andam como eles, entre eles são interpolados (divisão arredondada, ou razão 16.16 nos vãos grandes)',
          'rule: IUP: on each contour of zone 2, the points untouched on the axis between two touched ones follow them: below the lower and above the higher (by font unit coordinate) they move as those, between them they are interpolated (a rounded division, or a 16.16 ratio on large spans)'),
         ('tt_op_shp', 'tt_op_shp', '0x939110', _A, 'cdecl',
          'regra: SHP: tira pontos da zona 2 (o laço) e os desloca pelo deslocamento do ponto de referência, marcando-os tocados nos eixos que o vetor de liberdade move',
          "rule: SHP: pops loop-count points of zone 2 and shifts them by the reference point's displacement, marking them touched on the axes the freedom vector moves"),
         ('tt_compute_rp_displacement', 'tt_compute_rp_displacement', '0x9391a0', _A, 'cdecl',
          'regra: quanto o ponto de referência 1 (zona 0) ou 2 (zona 1) andou da posição original, medido pelo vetor de projeção e repartido pelo de liberdade (cada parte só escrita se o vetor de liberdade tem aquele componente)',
          'rule: how far reference point 1 (zone 0) or 2 (zone 1) moved from its original position, measured along the projection vector and split along the freedom vector (each part written only when the freedom vector has that component)'),
         ('tt_op_shc', 'tt_op_shc', '0x9392a0', _A, 'cdecl',
          'regra: SHC: tira um contorno da zona 2 e desloca seus pontos (menos o de referência) pelo deslocamento; eixo com deslocamento 0 nem move nem toca',
          'rule: SHC: pops a contour of zone 2 and shifts its points (except the reference point) by the displacement; an axis with a zero shift is neither moved nor touched'),
         ('tt_op_shz', 'tt_op_shz', '0x939360', _A, 'cdecl',
          'regra: SHZ: tira uma zona e desloca todos os seus pontos (menos o de referência); defeito do original: não marca tocado em x, e em y os pontos antes do de referência não ganham a marca de y',
          'rule: SHZ: pops a zone and shifts all its points (except the reference point); defect of the original: no touched mark along x, and along y the points before the reference point miss the y mark'),
         ('tt_op_shpix', 'tt_op_shpix', '0x9394a0', _A, 'cdecl',
          'regra: SHPIX: tira uma distância e pontos da zona 2 (o laço) e os desloca por ela pelo vetor de liberdade',
          'rule: SHPIX: pops a distance and loop-count points of zone 2 and shifts them by it along the freedom vector'),
         ('tt_op_ip', 'tt_op_ip', '0x939540', _A, 'cdecl',
          'regra: IP: tira pontos da zona 2 (o laço) e os mantém, pelo vetor de projeção, na mesma proporção entre os pontos de referência 1 e 2 que no contorno original',
          'rule: IP: pops loop-count points of zone 2 and keeps them, along the projection vector, in the same proportion between reference points 1 and 2 as in the original outline'),
         ('tt_op_msirp', 'tt_op_msirp', '0x939670', _A, 'cdecl',
          'regra: MSIRP: tira uma distância e um ponto da zona 1 e o põe àquela distância do ponto de referência 0; na zona do crepúsculo também no contorno original; atualiza os pontos de referência',
          'rule: MSIRP: pops a distance and a point of zone 1 and puts it at that distance from reference point 0; in the twilight zone in the original outline too; updates the reference points')])
CAMPOS = []
INTERESSA = []
