# A T2K (library_without_asserts): a fonte TrueType (truetype.c da T2K): o mapa de caracteres, a carga dos glifos, as
# tabelas da fonte e a mediana dos extremos (leitor rx03, rodada 6, 25/11/2026; o mapa em
# levas/reorganizacao/proximas/TRUETYPE.md).
# Item de TELAS: o texto desenhado e tela. No jogo rodam na carga de cada fonte (tt_font_load_tables pelo tt_font_new
# 0x5b1aae) e no desenho de cada letra nova (tt_font_cmap_lookup com o cmap 3/1 que o FONT pede, carregado no primeiro
# uso pelo tt_cmap_load; tt_font_load_glyph com o T2K comprimido das .t2k ou o glyf do symbol.ttf pelo
# tt_glyph_decode_outline). tt_font_is_digit_glyph so com cmd & 8 e glyph_hint_median_char_extreme_y so na medicao dos
# parametros do ajustador (rx02), que o jogo nao usa. Caminho no jogo: abrir o jogo comparando.
# Convencoes conferidas: todas cdecl: tt_font_cmap_lookup (fonte, caractere; quem chama le os 16 bits baixos);
# tt_cmap_load (contexto, plataforma, codificacao, fluxo); tt_font_is_digit_glyph (fonte, glifo); tt_font_load_glyph
# (fonte, indice, ler instrucoes, &avanco, &avanco vertical); tt_glyph_decode_outline (contexto, fluxo, ler instrucoes,
# lsb, aw, tsb, ah); glyph_hint_median_char_extreme_y (fonte, texto, alto); tt_font_load_tables (fonte, fluxo, indice
# na colecao). A funcao de estilo da fonte (+0x3c, glyph_outline_embolden; +0x40, font_embolden_advance_widths) e
# chamada pelo ponteiro, cdecl (glifo ou metricas, contexto, unidades, parametros).
# Iguais no cm0102-1080.exe (todas inteiras). Prologos sem call/jmp relativo.
# Comparadores: valor contra valor nos mapas (os 16 bits baixos; o cmap ja carregado pela original); so a original nos
# que carregam ou alocam.
# Prova sem o jogo: o emulador, o caminho inteiro do texto com as fontes do jogo (rascunho rx03/e2e.py).
# Depende do rx03_2, rx03_3, rx03_4 (tt_font, as tags, tt_find_table_by_tag, os leitores), rx03_5 (tt_exec_create,
# t2k_stream_close) e rx03_7 (t2k_glyph, read_components, write_phantom_points, as rotinas do contorno).
SAVE = False
TELAS = True
ROTINAS = [('tt_font_cmap_lookup', 0x942a40, [0x55, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('tt_cmap_load', 0x942c00, [0x53, 0x55, 0x8b, 0x6c, 0x24, 0x0c]),
           ('tt_font_is_digit_glyph', 0x942f50, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('tt_font_load_glyph', 0x942350, [0x8b, 0x44, 0x24, 0x10, 0x8b, 0x4c, 0x24, 0x14]),
           ('tt_glyph_decode_outline', 0x942560, [0x83, 0xec, 0x0c, 0x53, 0x55]),
           ('glyph_hint_median_char_extreme_y', 0x93ba30, [0x83, 0xec, 0x50, 0x53, 0x55]),
           ('tt_font_load_tables', 0x9430e0, [0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10])]
_A = 'library_without_asserts'
DOCS = ('a T2K: o mapa de caracteres, a carga dos glifos e as tabelas da fonte',
        'the T2K: the character map, the glyph loading and the font tables',
        [('tt_font_cmap_lookup', 'tt_font_cmap_lookup', '0x942a40', _A, 'cdecl',
          'regra: o índice do glifo de um caractere pelo cmap (carregado no primeiro uso): formato 0 (bytes, código < 0x100), 6 (faixa densa) ou 4 (segmentos: o primeiro fim não menor que o caractere, delta ou índice pelo deslocamento); 0 sem glifo',
          'rule: a character\'s glyph index through the cmap (loaded on first use): format 0 (bytes, code < 0x100), 6 (dense range) or 4 (segments: the first end not below the character, a delta or an index through the offset); 0 when unmapped'),
         ('tt_cmap_load', 'tt_cmap_load', '0x942c00', _A, 'cdecl',
          'regra: os registros das subtabelas e a tabela inteira copiada; a escolha em quatro passadas (a plataforma e codificação pedidas, −1 vale qualquer, entre os formatos 0, 4 e 6; senão o primeiro formato 0, senão 6, senão 4; a primeira quando nenhuma); outro formato dá o erro 0x2725; os glifos dos algarismos 0 a 9 guardados (0xffff sem glifo)',
          'rule: the subtable records and the whole table copied; the choice in four passes (the platform and encoding asked for, −1 matching any, among formats 0, 4 and 6; else the first format 0, else 6, else 4; the first one when none); another format raises 0x2725; the glyphs of the digits 0 to 9 cached (0xffff when unmapped)'),
         ('tt_font_is_digit_glyph', 'tt_font_is_digit_glyph', '0x942f50', _A, 'cdecl',
          'regra: 1 se o glifo é um dos dez algarismos (o cmap carregado antes se preciso)',
          'rule: 1 when the glyph is one of the ten digits (the cmap loaded first if needed)'),
         ('tt_font_load_glyph', 'tt_font_load_glyph', '0x942350', _A, 'cdecl',
          'regra: com glyf, loca, métricas e índice válido: o avanço e o recuo (hmtx, senão 0x800 e 0), a altura e o recuo vertical (vmtx, senão a altura de linha do hhea e um décimo dela), o contorno pelos bytes do glifo (T2K comprimido com o formato 2000 do head, senão TrueType) e a função de estilo; glifo vazio sem bytes; sem as tabelas, glifo vazio sem métricas',
          'rule: with glyf, loca, metrics and a valid index: the advance and bearing (hmtx, else 0x800 and 0), the height and top bearing (vmtx, else the hhea line height and a tenth of it), the outline from the glyph bytes (T2K compressed with head\'s format 2000, else TrueType) and the style function; an empty glyph without bytes; without the tables, an empty glyph without metrics'),
         ('tt_glyph_decode_outline', 'tt_glyph_decode_outline', '0x942560', _A, 'cdecl',
          'regra: um registro glyf: a caixa; o glifo simples com os fins dos contornos, as instruções (guardadas ou puladas), as marcas com repetição e os deltas curtos ou longos; o composto com os registros dos componentes e as instruções da marca 0x100; os quatro pontos fantasmas depois',
          'rule: one glyf record: the box; a simple glyph with its contour ends, its instructions (kept or skipped), its flags with repeats and its short or word deltas; a composite one with its component records and the instructions of flag 0x100; the four phantom points after'),
         ('glyph_hint_median_char_extreme_y', 'glyph_hint_median_char_extreme_y', '0x93ba30', _A, 'cdecl',
          'regra: a mediana, sobre até 32 caracteres do texto, do menor (ou maior) y dos contornos de mais de um ponto de cada glifo (o composto trocado pelo primeiro componente); 0 sem nenhum',
          'rule: the median, over up to 32 characters of the text, of each glyph\'s lowest (or highest) y over its contours of more than one point (a composite glyph replaced by its first component); 0 when none'),
         ('tt_font_load_tables', 'tt_font_load_tables', '0x9430e0', _A, 'cdecl',
          'regra: libera as tabelas de antes e lê o diretório (a entrada da coleção com ttcf), head, hhea, vhea, maxp, loca (16 bits dobrados ou 32), hmtx e vmtx (as horizontais pela função de métricas do estilo), kern (não com fvar), o contexto de ajuste e o sublinhado do post (0 sem post)',
          'rule: frees the earlier tables and reads the directory (the collection entry with ttcf), head, hhea, vhea, maxp, loca (16 bits doubled or 32), hmtx and vmtx (the horizontal ones through the style\'s metrics function), kern (not with fvar), the hinting context and post\'s underline (0 without post)')])
CAMPOS = []
INTERESSA = []
MUTACOES = []
