# A T2K (library_without_asserts): o contexto de ajuste TrueType (truetype.c da T2K): as escalas, os programas da fonte
# e de controle e o programa do glifo (leitor rx03, rodada 6, 25/11/2026; o mapa em
# levas/reorganizacao/proximas/TRUETYPE.md).
# Item de TELAS. No jogo roda o tt_exec_create (na carga de cada fonte, pelo tt_font_load_tables: monta a tabela dos
# opcodes e roda o fpgm do symbol.ttf) e o tt_exec_destroy (na liberacao); o resto so com o ajuste a grade, que o jogo
# nao usa (T2K_RenderGlyph cmd = 2): tt_exec_set_ppem e as escalas pelo t2k_set_up_hinting_for_size, tt_exec_hint_glyph
# pelo desenho com cmd & 0x80. Caminho no jogo: abrir o jogo comparando (a carga das fontes).
# Convencoes conferidas: todas cdecl: as escalas (estado global, valor), chamadas pelo campo +0x98 do estado global (os
# enderecos 0x941e80/0x941ea0/0x941ef0 gravados pelo tt_exec_set_ppem); tt_compute_scale_factor (tamanho, unidades,
# registro); tt_exec_create (contexto, fluxo, fonte); tt_exec_set_ppem (contexto de ajuste, x, y); tt_exec_hint_glyph
# (contexto de ajuste, glifo); tt_exec_destroy (contexto de ajuste).
# O tt_interpreter_run 0x937090 (setjmp: fica no exe) e o input_stream_free 0x941760 (setjmp com status; aqui sem)
# pelo endereco; o tt_init_opcode_table do rx01_1 direto. O arredondamento padrao (0x936f10) e as escalas ficam no
# estado pelo endereco do exe, como a original grava (o rx01 compara o endereco; os desvios levam ao Rust).
# Iguais no cm0102-1080.exe (todas inteiras). Prologos sem call/jmp relativo.
# Comparadores: valor contra valor nas escalas; lado a lado no registro do tt_compute_scale_factor; so a original nos
# que alocam, liberam ou rodam programas.
# Prova sem o jogo: o emulador, o caminho inteiro do texto (rascunho rx03/e2e.py: o symbol.ttf com o ajuste a grade,
# cmd 1/3/0x83, roda o tt_exec_set_ppem com o prep e o tt_exec_hint_glyph) e as escalas valor a valor
# (check_rx03.py rx03_5).
# Defeitos do original: tt_compute_scale_factor com tamanho e unidades zero nao sai do laco; tt_exec_set_ppem com um
# tamanho cujo size << 6 nao cabe em 16 bits copia para o estado bytes de pilha que o tt_compute_scale_factor nao
# escreveu (multiplicador, deslocamento, divisor, arredondamento; zero na copia).
# Depende do rx01_1 (TtGlobalState, tt_init_opcode_table), do rx03_1 (fixed_mul, fixed_div), do rx03_2, do rx03_3 e do
# rx03_4 (tt_font, tt_find_table_by_tag, t2k_stream_open).
SAVE = False
TELAS = True
ROTINAS = [('tt_scale_by_shift', 0x941e80, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x81, 0xa8, 0x00, 0x00, 0x00]),
           ('tt_scale_by_division', 0x941ea0, [0x8b, 0x54, 0x24, 0x08, 0x83, 0xfa, 0xff]),
           ('tt_scale_by_fixed_mul', 0x941ef0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('tt_compute_scale_factor', 0x942200, [0x53, 0x8b, 0x5c, 0x24, 0x0c]),
           ('tt_exec_destroy', 0x942140, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('tt_exec_create', 0x941830, [0x83, 0xec, 0x10, 0x53, 0x55]),
           ('tt_exec_set_ppem', 0x941ba0, [0x83, 0xec, 0x20, 0x53, 0x8b, 0x5c, 0x24, 0x28]),
           ('tt_exec_hint_glyph', 0x941f10, [0x53, 0x8b, 0x5c, 0x24, 0x08])]
_A = 'library_without_asserts'
DOCS = ('a T2K: o contexto de ajuste TrueType, as escalas, os programas da fonte e de controle e o programa do glifo',
        'the T2K: the TrueType hinting context, the scale factors, the font and control programs and the glyph program',
        [('tt_scale_by_shift', 'tt_scale_by_shift', '0x941e80', _A, 'cdecl',
          'regra: (valor × multiplicador + arredondamento) deslocado à direita',
          'rule: (value × multiplier + rounding) shifted right'),
         ('tt_scale_by_division', 'tt_scale_by_division', '0x941ea0', _A, 'cdecl',
          'regra: (valor × multiplicador + arredondamento) / divisor, simétrico em torno de zero',
          'rule: (value × multiplier + rounding) / divisor, symmetric about zero'),
         ('tt_scale_by_fixed_mul', 'tt_scale_by_fixed_mul', '0x941ef0', _A, 'cdecl',
          'regra: o valor vezes a razão 16.16 do registro (fixed_mul)',
          'rule: the value times the record\'s 16.16 ratio (fixed_mul)'),
         ('tt_compute_scale_factor', 'tt_compute_scale_factor', '0x942200', _A, 'cdecl',
          'regra: n = tamanho × 64 e d = unidades, os dois divididos por 2 enquanto ambos forem pares; a razão n/d e o modo 2; com n em 16 bits, o multiplicador n, o divisor d, o arredondamento d/2, o deslocamento pelos zeros finais de d e o modo 0 (d potência de 2) ou 1 (defeito: com os dois zero o laço não acaba)',
          'rule: n = size × 64 and d = units, both halved while both even; the ratio n/d and mode 2; with n in 16 bits, the multiplier n, the divisor d, the rounding d/2, the shift by d\'s trailing zeros and mode 0 (d a power of 2) or 1 (defect: with both zero the loop never ends)'),
         ('tt_exec_destroy', 'tt_exec_destroy', '0x942140', _A, 'cdecl',
          'regra: libera os buffers do contexto de ajuste e o contexto; nada com nulo',
          'rule: frees the hinting context\'s buffers and the context; nothing for null'),
         ('tt_exec_create', 'tt_exec_create', '0x941830', _A, 'cdecl',
          'regra: o contexto de ajuste: a tabela dos opcodes, a pilha, o armazenamento zerado, o buffer de trabalho, os valores de controle (cvt), as definições, os programas prep e fpgm, o estado gráfico padrão e a zona do crepúsculo; depois roda o fpgm',
          'rule: the hinting context: the opcode table, the stack, the zeroed storage, the work buffer, the control values (cvt), the definitions, the prep and fpgm programs, the default graphics state and the twilight zone; then the fpgm runs'),
         ('tt_exec_set_ppem', 'tt_exec_set_ppem', '0x941ba0', _A, 'cdecl',
          'regra: com tamanhos novos: as razões de esticamento, o registro de escala, os valores de controle escalados pelo modo (com a função de escala gravada), a zona do crepúsculo limpa e o prep rodado sobre uma cópia do estado padrão (defeito: no modo 2, bytes de pilha não escritos vão para o estado)',
          'rule: with new sizes: the stretch ratios, the scale record, the control values scaled by its mode (with the scale function stored), the twilight zone cleared and the prep run on a copy of the default state (defect: in mode 2, unwritten stack bytes go into the state)'),
         ('tt_exec_hint_glyph', 'tt_exec_hint_glyph', '0x941f10', _A, 'cdecl',
          'regra: com instruções: a zona do glifo aponta para os vetores do glifo, os originais esticados pelas razões, os pontos fantasmas arredondados ao pixel, e o programa do glifo roda',
          'rule: with instructions: the glyph zone points at the glyph\'s arrays, the originals stretched by the ratios, the phantom points rounded to the pixel, and the glyph program runs')])
CAMPOS = []
INTERESSA = []
MUTACOES = []
