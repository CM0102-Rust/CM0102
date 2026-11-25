# A T2K (library_without_asserts): o fluxo de entrada (t2kstrm.c da T2K): as leituras big-endian e a posicao de
# leitura (leitor rx03, rodada 6, 25/11/2026; o mapa em levas/reorganizacao/proximas/TRUETYPE.md).
# Item de TELAS: o texto desenhado e tela. No jogo rodam na carga de cada fonte (as tabelas sfnt lidas pelo
# tt_font_load_tables e o tt_exec_create) e no desenho de cada letra nova (o glyf de cada glifo pelo tt_font_load_glyph,
# o cmap no primeiro uso). Caminho no jogo: abrir o jogo comparando.
# Convencoes conferidas: todas cdecl (call direto, add esp do chamador): (fluxo); input_stream_read_bytes (fluxo,
# destino, n); input_stream_seek (fluxo, deslocamento). O input_stream_read_int16 devolve em ax (os 16 bits altos de eax
# ficam com a metade alta do ponteiro dos dados; quem chama le ax).
# - 0x941580 input_stream_read_int32, 0x9415c0 input_stream_read_int16, 0x9415e0 input_stream_read_bytes, 0x941620
#   input_stream_get_size, 0x941730 input_stream_rewind, 0x941740 input_stream_seek, 0x9417f0 input_stream_read_varint7.
# Fora: 0x941630 input_stream_new, 0x9416c0 input_stream_new_from_buffer e 0x941760 input_stream_free chamam o setjmp
# 0x947998 quando recebem o ponteiro do status (o setjmp nao tem como ser chamado do Rust; ficam com o exe).
# Iguais no cm0102-1080.exe (todas inteiras). Prologos sem call/jmp relativo.
# Comparadores: lado a lado na posicao de leitura (e nos bytes copiados), o valor contra o valor.
# Prova sem o jogo: o emulador (rascunho rx03/check_rx03.py rx03_3).
SAVE = False
TELAS = True
ROTINAS = [('input_stream_read_int32', 0x941580, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x50, 0x04]),
           ('input_stream_read_int16', 0x9415c0, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x51, 0x04]),
           ('input_stream_read_bytes', 0x9415e0, [0x53, 0x8b, 0x5c, 0x24, 0x10]),
           ('input_stream_get_size', 0x941620, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x41, 0x08]),
           ('input_stream_rewind', 0x941730, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x0c]),
           ('input_stream_seek', 0x941740, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('input_stream_read_varint7', 0x9417f0, [0x53, 0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14])]
_A = 'library_without_asserts'
DOCS = ('a T2K: o fluxo de entrada, as leituras big-endian e a posição de leitura',
        'the T2K: the input stream, the big-endian reads and the read position',
        [('input_stream_read_int32', 'input_stream_read_int32', '0x941580', _A, 'cdecl',
          'regra: lê 32 bits big-endian e avança a posição 4',
          'rule: reads a big-endian 32-bit value and moves the position on by 4'),
         ('input_stream_read_int16', 'input_stream_read_int16', '0x9415c0', _A, 'cdecl',
          'regra: lê 16 bits big-endian e avança a posição 2',
          'rule: reads a big-endian 16-bit value and moves the position on by 2'),
         ('input_stream_read_bytes', 'input_stream_read_bytes', '0x9415e0', _A, 'cdecl',
          'regra: copia n bytes para o destino e avança a posição n; nada com n <= 0',
          'rule: copies n bytes to the destination and moves the position on by n; nothing for n <= 0'),
         ('input_stream_get_size', 'input_stream_get_size', '0x941620', _A, 'cdecl',
          'regra: o tamanho do fluxo, o fim menos o começo',
          'rule: the stream size, the end minus the start'),
         ('input_stream_rewind', 'input_stream_rewind', '0x941730', _A, 'cdecl',
          'regra: a posição volta ao começo',
          'rule: the position goes back to the start'),
         ('input_stream_seek', 'input_stream_seek', '0x941740', _A, 'cdecl',
          'regra: a posição vai ao começo mais o deslocamento',
          'rule: the position goes to the start plus the offset'),
         ('input_stream_read_varint7', 'input_stream_read_varint7', '0x9417f0', _A, 'cdecl',
          'regra: lê um inteiro sem sinal de 7 bits por byte, o grupo baixo primeiro, enquanto o bit alto do byte estiver ligado',
          'rule: reads an unsigned value of seven bits a byte, low group first, while the byte\'s top bit is set')])
CAMPOS = []
INTERESSA = []
MUTACOES = []
