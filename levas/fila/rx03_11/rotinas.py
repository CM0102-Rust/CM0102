# A T2K (library_without_asserts): o conversor em mapa de bits (scanconv.c da T2K): as listas de cruzamentos, retas,
# curvas quadraticas e o preenchimento (leitor rx03, rodada 6, 25/11/2026; o mapa em
# levas/reorganizacao/proximas/TRUETYPE.md).
# Item de TELAS: o texto desenhado e tela. No jogo rodam a cada letra nova (t2k_render_glyph_internal com cmd & 2): o
# conversor com 5 niveis de cinza (as listas de colunas ligadas pelo cinza), as retas e curvas de cada contorno, o
# preenchimento em cinza com a mistura das colunas e a liberacao. O caminho de 1 bit por pixel com o controle de perda
# so com cinza 0, que o jogo nao usa. Caminho no jogo: abrir o jogo comparando (qualquer texto novo).
# Convencoes conferidas: todas cdecl: t2k_scan_converter_new (contexto, contornos, inicios, fins, x, y, na curva,
# cinza, ordem da curva, perda, perda nas colunas, planura: os bytes lidos so no byte baixo); t2k_scan_edge_pool_grow
# (conversor) -> o pedaco novo; t2k_scan_add_line (conversor, x0, y0, x1, y1); t2k_scan_add_quadratic_curve
# (conversor, 3 pontos); t2k_scan_convert_to_bitmap (conversor, nao usado, so os limites); t2k_scan_converter_delete
# (conversor).
# Iguais no cm0102-1080.exe (todas inteiras). Prologos sem call/jmp relativo.
# Comparadores: so a original (alocam, liberam ou enchem o mapa da vaga de rascunho 5).
# Prova sem o jogo: o emulador, o caminho inteiro do texto com as fontes do jogo, em cinza e em 1 bit (rascunho
# rx03/e2e.py).
# Depende do rx03_1 (fixed_mul, fixed_div) e do rx03_2 (o contexto e os buffers de rascunho).
SAVE = False
TELAS = True
ROTINAS = [('t2k_scan_converter_new', 0x93fd10, [0x51, 0x53, 0x55, 0x56, 0x57]),
           ('t2k_scan_edge_pool_grow', 0x940160, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('t2k_scan_add_line', 0x940290, [0x83, 0xec, 0x20, 0x8b, 0x44, 0x24, 0x28]),
           ('t2k_scan_add_quadratic_curve', 0x940820, [0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0xc4, 0x01, 0x00, 0x00]),
           ('t2k_scan_convert_to_bitmap', 0x9409b0, [0x83, 0xec, 0x40, 0x53, 0x55]),
           ('t2k_scan_converter_delete', 0x941480, [0x56, 0x8b, 0x74, 0x24, 0x08])]
_A = 'library_without_asserts'
DOCS = ('a T2K: o conversor em mapa de bits, as listas de cruzamentos, retas, curvas e o preenchimento',
        'the T2K: the scan converter, the crossing lists, lines, curves and the bitmap fill',
        [('t2k_scan_converter_new', 't2k_scan_converter_new', '0x93fd10', _A, 'cdecl',
          'regra: o conversor (vaga de rascunho 4): as marcas, os limites do contorno, as listas por coluna (só com cinza ou perda nas colunas) e por linha (até 32 no próprio conversor, senão num bloco), o primeiro pedaço de nós; cada contorno entra com retas entre pontos na curva e curvas pelos pontos fora (com os pontos do meio entre dois fora); não com a ordem 3',
          'rule: the converter (scratch slot 4): the flags, the outline bounds, the column lists (only with grey or column dropout) and the row lists (up to 32 inside the converter, else a block), the first node chunk; each contour goes in with lines between on curve points and curves through the off ones (with the middle points between two off ones); not for order 3'),
         ('t2k_scan_edge_pool_grow', 't2k_scan_edge_pool_grow', '0x940160', _A, 'cdecl',
          'regra: a primeira vez aponta para o pedaço do próprio conversor (1024 nós); depois um pedaço de (vão em y − x mín + x máx)/64 + 256 nós, o vetor dos pedaços dobrado quando cheio',
          'rule: the first time it points at the converter\'s own chunk (1024 nodes); later a chunk of (y span − x min + x max)/64 + 256 nodes, the chunk array doubled when full'),
         ('t2k_scan_add_line', 't2k_scan_add_line', '0x940290', _A, 'cdecl',
          'regra: os cruzamentos da reta com cada centro de coluna (com as listas de colunas) e de linha, a outra coordenada interpolada (passos inteiros, ou fixed_mul/fixed_div num contorno grande), em dobro com o bit do sentido, na lista ordenada do pixel',
          'rule: the line\'s crossings with each column (with column lists) and row centre, the other coordinate interpolated (integer steps, or fixed_mul/fixed_div for a large outline), doubled with the direction bit, into the pixel\'s sorted list'),
         ('t2k_scan_add_quadratic_curve', 't2k_scan_add_quadratic_curve', '0x940820', _A, 'cdecl',
          'regra: a curva dividida ao meio até o desvio |p0 − 2p1 + p2|/4 caber na planura (cada divisão divide o desvio por 4), numa pilha de até 16, e cada pedaço plano entra como reta',
          'rule: the curve halved until the deviation |p0 − 2p1 + p2|/4 fits the flatness (each halving quarters it), on a stack of at most 16, and each flat piece goes in as a line'),
         ('t2k_scan_convert_to_bitmap', 't2k_scan_convert_to_bitmap', '0x9409b0', _A, 'cdecl',
          'regra: os limites uma vez (as linhas e colunas com cruzamentos, os cruzamentos extremos); o mapa (vaga 5, zerado) de 1 bit ou 1 byte de cinza (0..0x7e) por pixel; 1 bit: os trechos de cada linha pela regra do enrolamento, depois o controle de perda nas linhas e colunas; cinza: os trechos com a cobertura fracionária nas pontas, depois as pontas das colunas misturadas',
          'rule: the bounds once (the rows and columns with crossings, the extreme crossings); the bitmap (slot 5, zeroed) of 1 bit or 1 grey byte (0..0x7e) per pixel; 1 bit: each row\'s spans by the winding rule, then the dropout control on rows and columns; grey: the spans with fractional end coverage, then the columns\' ends blended in'),
         ('t2k_scan_converter_delete', 't2k_scan_converter_delete', '0x941480', _A, 'cdecl',
          'regra: libera o mapa (devolvendo a vaga 5), os pedaços de nós depois do primeiro, o vetor dos pedaços e os blocos das listas quando fora do conversor, e o conversor (vaga 4); nada com nulo',
          'rule: frees the bitmap (giving back slot 5), the node chunks after the first, the chunk array and the list blocks when outside the converter, and the converter (slot 4); nothing for null')])
CAMPOS = []
INTERESSA = []
MUTACOES = []
