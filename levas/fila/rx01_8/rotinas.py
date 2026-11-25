# O ajustador automático da T2K: a lista de restrições, as larguras, o tamanho e a vida do ajustador (library_without_asserts, o autogrid da T2K; leitor rx01, rodada 6, 25/11/2026; o mapa em
# levas/reorganizacao/proximas/TRUETYPE.md).
# Item de TELAS. No jogo nao roda: o ajustador so entra no desenho com T2K_GRID_FIT (cmd & 0x281) ou na preparacao
# do tamanho com doSetUpNow, e o FONT do jogo chama o T2K_NewTransformation com doSetUpNow = 0 (0x5b1b79) e o
# T2K_RenderGlyph com cmd = 2 (0x5b1c8d). Caminho no jogo: nenhum; a prova e o emulador (rascunho rx01/check_hint.py):
# a original e a copia sobre o mesmo contexto e o mesmo glifo sorteados, memoria inteira comparada.
# Convencoes conferidas: todas cdecl (call direto ou pelo endereco num registrador, add esp do chamador).
# lib_mem_alloc e lib_mem_free (pelo endereco, 0x9443a0 e 0x9445d0, rx03) daqui sao usados pela parte rx03.
# Iguais no cm0102-1080.exe (todas inteiras). Prologos sem call/jmp relativo.
# Comparadores: so a original nas que mudam o ajustador ou o glifo; valor contra valor nas puras.
# Depende de o rx01_1, o rx01_5 e o rx01_7.
SAVE = False
TELAS = True
ROTINAS = [('glyph_hint_build_sorted_constraint_list', 0x936060, [0x83, 0xec, 0x0c, 0x8b, 0x54, 0x24, 0x10]),
           ('glyph_hint_collect_stem_widths', 0x9362b0, [0x83, 0xec, 0x18, 0x53, 0x55]),
           ('glyph_hint_set_font_params', 0x9364c0, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0]),
           ('glyph_hint_outline_and_get_advance', 0x936510, [0x33, 0xc0, 0x53, 0x66, 0x39, 0x44, 0x24, 0x1c]),
           ('glyph_hint_create', 0x9365d0, [0x51, 0x55, 0x8b, 0x6c, 0x24, 0x0c]),
           ('glyph_hint_destroy', 0x936840, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('glyph_hint_set_size', 0x9368e0, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0]),
           ('glyph_hint_scale_font_zones', 0x936930, [0x83, 0xec, 0x0c, 0x53, 0x55])]
_A = 'library_without_asserts'
DOCS = ('o ajustador automático da T2K: a lista de restrições, as larguras, o tamanho e a vida do ajustador',
        "the T2K automatic hinter: the constraint list, the widths, the size and the hinter's life",
        [('glyph_hint_build_sorted_constraint_list', 'glyph_hint_build_sorted_constraint_list', '0x936060', _A, 'cdecl',
          'regra: monta a lista de restrições (pontos nas zonas e pares de haste mútuos), classifica cada uma pelo eixo e a ordena pela prioridade',
          'rule: builds the constraint list (points on zones and mutual stem pairs), classes each by axis and sorts it by priority'),
         ('glyph_hint_collect_stem_widths', 'glyph_hint_collect_stem_widths', '0x9362b0', _A, 'cdecl',
          'regra: junta as larguras das hastes de até 1/3 em em duas listas novas (x e y), com as contagens; −1 se faltar memória',
          'rule: collects the stem widths up to 1/3 em into two new lists (x and y), with their counts; −1 when memory fails'),
         ('glyph_hint_set_font_params', 'glyph_hint_set_font_params', '0x9364c0', _A, 'cdecl',
          'regra: guarda o modo e os 27 parâmetros de dica da fonte no ajustador válido e os marca como novos',
          "rule: stores the mode and the font's 27 hint parameters in a valid hinter and marks them new"),
         ('glyph_hint_outline_and_get_advance', 'glyph_hint_outline_and_get_advance', '0x936510', _A, 'cdecl',
          'regra: ajusta o contorno de um glifo (no modo 0 copiando antes os pontos e o avanço) e guarda o avanço ajustado em pixels',
          "rule: hints a glyph's outline (in mode 0 first copying the points and the advance) and keeps the hinted advance in pixels"),
         ('glyph_hint_create', 'glyph_hint_create', '0x9365d0', _A, 'cdecl',
          'regra: cria o ajustador (0x450 bytes, as palavras mágicas, os padrões) e os oito blocos de trabalho para o número de pontos; −1 se faltar memória',
          'rule: creates the hinter (0x450 bytes, the magic words, the defaults) and its eight work blocks for the point count; −1 when memory fails'),
         ('glyph_hint_destroy', 'glyph_hint_destroy', '0x936840', _A, 'cdecl',
          'regra: libera o ajustador e seus blocos',
          'rule: frees the hinter and its blocks'),
         ('glyph_hint_set_size', 'glyph_hint_set_size', '0x9368e0', _A, 'cdecl',
          'regra: guarda os pixels por em e reescala as zonas e as larguras',
          'rule: stores the pixels per em and rescales the zones and widths'),
         ('glyph_hint_scale_font_zones', 'glyph_hint_scale_font_zones', '0x936930', _A, 'cdecl',
          'regra: escala as zonas de alinhamento e as larguras de haste ao tamanho (arredondadas ao pixel, a altura do x 9 unidades acima no modo 1), monta as faixas de altura do modo 3 e diz se a primeira haste tem menos de 1,5 pixel',
          'rule: scales the alignment zones and stem widths to the size (rounded to the pixel, the x height 9 units up in mode 1), builds the height bands of mode 3 and tells whether the first stem is under 1.5 pixels')])
CAMPOS = []
INTERESSA = []
