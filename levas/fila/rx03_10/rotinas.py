# A T2K (library_without_asserts): o desenho do glifo (t2k.c da T2K): contorno, compostos, escala, ajuste e mapa de bits
# (leitor rx03, rodada 6, 25/11/2026; o mapa em levas/reorganizacao/proximas/TRUETYPE.md). Rotina grande (4 KB) num
# item so, partida em pedacos com nome.
# Item de TELAS: o texto desenhado e tela. No jogo roda a cada letra nova: FONT 0x5b1ca3 -> T2K_RenderGlyph 0x93e8c0
# (fica no exe: setjmp) -> esta, com cmd = 2 (o mapa de bits, sem ajuste a grade) e 5 niveis de cinza; os compostos
# (letras acentuadas) chamam esta de novo, uma vez por componente. Caminho no jogo: abrir o jogo comparando (qualquer
# texto novo, com acentos e negrito).
# Convencoes conferidas: cdecl (escalador, codigo, profundidade, dx, dy, cinza, cmd); a chamada recursiva passa o cmd
# com so o byte baixo (& 0x9d | 0xc; & 0xfdfe no modo 2 ou x so) e os 16 bits altos de pilha, que nao sao lidos. O
# ajustador (0x936510, rx01) pelo endereco, com a vista HintGlyph do glifo numa estrutura local.
# Iguais no cm0102-1080.exe (inteira). Prologo sem call/jmp relativo.
# Comparador: so a original (carrega, aloca, recursa e enche o mapa).
# Prova sem o jogo: o emulador, o caminho inteiro do texto com as fontes do jogo (rascunho rx03/e2e.py: cmd 2 como o
# jogo, normal e negrito, quatro tamanhos e inclinacao; cmd 1/3 e 0x83 com os ajustadores, 0x102/0x103 com o filtro,
# 1 bit por pixel com o controle de perda).
# Defeito do original: o controle de perda nas colunas e decidido pelo conjunto de bits das linhas (+0x10) tambem; o
# das colunas (+0x14) e gravado e nunca lido.
# Duvida: a vista HintGlyph comeca zerada da +0x20 em diante; na original e pilha (o ajustador a escreve).
# Depende do rx03_1, rx03_2, rx03_4, rx03_5, rx03_6, rx03_7, rx03_9 e rx03_11.
SAVE = False
TELAS = True
ROTINAS = [('t2k_render_glyph_internal', 0x93e970, [0x81, 0xec, 0x84, 0x00, 0x00, 0x00])]
_A = 'library_without_asserts'
DOCS = ('a T2K: o desenho do glifo, compostos, escala, ajuste e mapa de bits',
        'the T2K: the glyph renderer, composite glyphs, scaling, hinting and the bitmap',
        [('t2k_render_glyph_internal', 't2k_render_glyph_internal', '0x93e970', _A, 'cdecl',
          'regra: os bits do comando decidem o ajuste (0x80 instruções TrueType, só numa fonte TrueType no seu tamanho mínimo legível ou acima; 0x281 o ajustador automático; 0x200 só em x), 8 índice de glifo em vez de caractere, 0x10 deslocamento inteiro, 2 o mapa de bits, 0x20 só os limites, 4 manter o glifo; o composto desenha cada componente (com escala ou matriz e deslocamento ou âncoras) e os junta; o contorno é escalado e ajustado, os fantasmas arredondados e levados à origem, os avanços calculados (pela matriz quando não é a identidade), e o conversor faz o mapa (defeito: a perda nas colunas é decidida pelo conjunto das linhas)',
          'rule: the command bits decide the hinting (0x80 TrueType instructions, only for a TrueType font at or above its smallest readable size; 0x281 the automatic hinter; 0x200 x only), 8 a glyph index instead of a character, 0x10 integer offset, 2 the bitmap, 0x20 bounds only, 4 keep the glyph; a composite glyph renders each component (with its scale or matrix and its offset or anchors) and joins them; the outline is scaled and hinted, the phantoms rounded and moved to the origin, the advances computed (through the matrix when not the identity), and the scan converter builds the bitmap (defect: the column dropout is decided by the rows\' bit set)')])
CAMPOS = []
INTERESSA = []
MUTACOES = []
