# A T2K (library_without_asserts): a fonte TrueType (truetype.c da T2K): o diretorio das tabelas, os cabecalhos das
# metricas, o kern e os leitores pequenos (leitor rx03, rodada 6, 25/11/2026; o mapa em
# levas/reorganizacao/proximas/TRUETYPE.md).
# Item de TELAS: o texto desenhado e tela. No jogo rodam na carga de cada fonte (tt_font_load_tables: o diretorio, hhea,
# hmtx, o kern; tt_font_get_units_per_em, a contagem dos glifos e as metricas de linha no T2K_NewTransformation) e no
# desenho (tt_find_table_by_tag no glyf de cada glifo). Caminho no jogo: abrir o jogo comparando.
# Convencoes conferidas: todas cdecl (call direto, add esp do chamador): tt_find_table_by_tag e tt_open_table_stream
# (fonte, tag); os leitores (contexto, fluxo); tt_read_hmtx_or_vmtx (contexto, fluxo, glifos, metricas: o chamador
# estende o u16 com zero); tt_font_get_line_metrics (fonte, horizontal, vertical); os outros (fonte) ou (tabela).
# tt_font_get_units_per_em devolve o u16 estendido com zero.
# O input_stream_new 0x941630 (fica no exe: setjmp com o ponteiro do status; aqui sem) pelo t2k_stream_open.
# Iguais no cm0102-1080.exe (todas inteiras). Prologos sem call/jmp relativo.
# Comparadores: valor contra valor nos leitores puros (tabela, contagem, unidades, pontos); lado a lado nos dois
# registros das metricas de linha; so a original nos que alocam ou liberam.
# Prova sem o jogo: o emulador, o caminho inteiro do texto com as fontes do jogo (rascunho rx03/e2e.py).
# Defeito do original: tt_read_hmtx_or_vmtx sem nenhum par de metricas le o "ultimo avanco" antes do bloco (o
# cabecalho da guarda).
# Depende do rx03_2 (lib_ctx_alloc, lib_ctx_free) e do rx03_3 (o fluxo).
SAVE = False
TELAS = True
ROTINAS = [('tt_find_table_by_tag', 0x942290, [0x8b, 0x4c, 0x24, 0x04, 0x53]),
           ('tt_open_table_stream', 0x9422d0, [0x53, 0x8b, 0x5c, 0x24, 0x08]),
           ('tt_font_get_glyph_count', 0x942330, [0x8b, 0x4c, 0x24, 0x04, 0x33, 0xc0]),
           ('tt_read_table_directory_entry', 0x943b60, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('tt_read_hhea_or_vhea', 0x943bb0, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('tt_read_hmtx_or_vmtx', 0x943c80, [0x53, 0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14]),
           ('tt_kern_free_format0_table', 0x943960, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('tt_read_kern_table', 0x943990, [0x83, 0xec, 0x0c, 0x53, 0x55]),
           ('tt_font_get_units_per_em', 0x943d30, [0x8b, 0x4c, 0x24, 0x04, 0xb8, 0x00, 0x08, 0x00, 0x00]),
           ('tt_font_get_max_glyph_points', 0x943d50, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x40, 0x10]),
           ('tt_font_get_line_metrics', 0x943d70, [0x8b, 0x54, 0x24, 0x0c, 0x8b, 0x44, 0x24, 0x08]),
           ('tt_font_free_cmap', 0x943ec0, [0x53, 0x8b, 0x5c, 0x24, 0x08])]
_A = 'library_without_asserts'
DOCS = ('a T2K: a fonte TrueType, o diretório das tabelas, os cabeçalhos das métricas, o kern e os leitores pequenos',
        'the T2K: the TrueType font, the table directory, the metrics headers, the kerning and the small getters',
        [('tt_find_table_by_tag', 'tt_find_table_by_tag', '0x942290', _A, 'cdecl',
          'regra: a primeira entrada do diretório com a tag pedida; nulo quando não há',
          'rule: the first directory entry with the tag; null when absent'),
         ('tt_open_table_stream', 'tt_open_table_stream', '0x9422d0', _A, 'cdecl',
          'regra: um fluxo sobre os bytes da tabela (deslocamento e tamanho da entrada); nulo sem a tabela',
          'rule: a stream over the table\'s bytes (the entry\'s offset and length); null without the table'),
         ('tt_font_get_glyph_count', 'tt_font_get_glyph_count', '0x942330', _A, 'cdecl',
          'regra: a contagem do maxp, cortada para a contagem do loca menos 1 quando esta não for maior',
          'rule: maxp\'s count, cut to loca\'s entry count minus 1 when that is not larger'),
         ('tt_read_table_directory_entry', 'tt_read_table_directory_entry', '0x943b60', _A, 'cdecl',
          'regra: uma entrada do diretório: tag, soma, deslocamento e tamanho, 32 bits big-endian cada',
          'rule: one directory entry: tag, checksum, offset and length, 32-bit big-endian each'),
         ('tt_read_hhea_or_vhea', 'tt_read_hhea_or_vhea', '0x943bb0', _A, 'cdecl',
          'regra: o cabeçalho das métricas: a versão de 32 bits e dezesseis campos de 16 bits (o último é a contagem das métricas)',
          'rule: the metrics header: the 32-bit version and sixteen 16-bit fields (the last is the metrics count)'),
         ('tt_read_hmtx_or_vmtx', 'tt_read_hmtx_or_vmtx', '0x943c80', _A, 'cdecl',
          'regra: os pares (avanço, recuo) das métricas, e os glifos restantes com o último avanço e só o recuo lido (defeito: sem nenhum par, o último avanço vem de antes do bloco)',
          'rule: the (advance, bearing) pairs, and the remaining glyphs with the last advance and only the bearing read (defect: with no pair, the last advance comes from before the block)'),
         ('tt_kern_free_format0_table', 'tt_kern_free_format0_table', '0x943960', _A, 'cdecl',
          'regra: libera os pares e o bloco de uma subtabela kern de formato 0; nada com nulo',
          'rule: frees the pairs and the block of a kern format 0 subtable; nothing for null'),
         ('tt_read_kern_table', 'tt_read_kern_table', '0x943990', _A, 'cdecl',
          'regra: o kern com cabeçalho Microsoft (versão e contagem de 16 bits) ou Apple (versão 1 e contagem 0: contagem de 32 bits); cada subtabela com o formato, o tamanho e a cobertura; a de formato 0 e tamanho positivo lê os pares (32 bits do par, 16 do valor)',
          'rule: the kern table with the Microsoft header (16-bit version and count) or Apple\'s (version 1 and count 0: a 32-bit count); each subtable with its format, length and coverage; a format 0 one with a positive length reads its pairs (32-bit pair, 16-bit value)'),
         ('tt_font_get_units_per_em', 'tt_font_get_units_per_em', '0x943d30', _A, 'cdecl',
          'regra: as unidades por em do head, ou 0x800 sem head',
          'rule: head\'s units per em, or 0x800 without head'),
         ('tt_font_get_max_glyph_points', 'tt_font_get_max_glyph_points', '0x943d50', _A, 'cdecl',
          'regra: o maior entre os limites de pontos simples e compostos do maxp',
          'rule: the larger of maxp\'s simple and composite point limits'),
         ('tt_font_get_line_metrics', 'tt_font_get_line_metrics', '0x943d70', _A, 'cdecl',
          'regra: os registros horizontal e vertical das métricas de linha (ascendente, descendente, entrelinha, avanço máximo, inclinação do cursor dobrada até 16 vezes enquanto couber em ±0x10000; o sublinhado só no horizontal)',
          'rule: the horizontal and vertical line metrics records (ascender, descender, line gap, maximum advance, the caret slope doubled up to 16 times while within ±0x10000; the underline only in the horizontal one)'),
         ('tt_font_free_cmap', 'tt_font_free_cmap', '0x943ec0', _A, 'cdecl',
          'regra: libera o mapa de caracteres (os registros, o vetor, os dados e o bloco) e zera o campo',
          'rule: frees the character map (the records, their array, the data and the block) and clears the field')])
CAMPOS = []
INTERESSA = []
MUTACOES = []
