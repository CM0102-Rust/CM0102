# A T2K (library_without_asserts): o escalador (t2k.c da T2K): a preparacao do ajuste, a limpeza da memoria, a escala
# das coordenadas e o filtro de cinco tomadas (leitor rx03, rodada 6, 25/11/2026; o mapa em
# levas/reorganizacao/proximas/TRUETYPE.md).
# Item de TELAS: o texto desenhado e tela. No jogo rodam no desenho de cada letra nova (t2k_purge_memory_internal no
# comeco e no fim do t2k_render_glyph_internal; t2k_scale_coordinates de cada glifo simples) e na limpeza
# (T2K_PurgeMemory e T2K_DeleteT2K, do exe, chamam a interna). t2k_set_up_hinting_for_size so com o ajuste a grade e
# o filtro so com cmd & 0x100, que o jogo nao usa (cmd = 2). Caminho no jogo: abrir o jogo comparando.
# Convencoes conferidas: todas cdecl: (escalador); t2k_purge_memory_internal (escalador, nivel); t2k_scale_coordinates
# (origem i16, destino i32, n, registro de escala). O ajustador automatico do rx01/rx02 (0x9364c0, 0x9365d0, 0x936840,
# 0x9368e0, 0x93b300, 0x93b370) pelo endereco.
# Iguais no cm0102-1080.exe (todas inteiras). Prologos sem call/jmp relativo.
# Comparadores: lado a lado no destino da escala; so a original nas que alocam, liberam ou preparam o ajustador.
# Prova sem o jogo: o emulador, o caminho inteiro do texto (rascunho rx03/e2e.py: cmd 2 do jogo; cmd 1/3 com o
# ajustador e o ajuste TrueType; 0x102/0x103 com o filtro; a limpeza nos niveis 1 e 2).
# Duvida: os 27 dwords de parametros do ajustador comecam zerados na copia; na original sao pilha, preenchida pelo
# 0x93b300 ou 0x93b370 (rx02; se algum campo ficar sem escrever, a original copia lixo de pilha).
# Depende do rx03_2, rx03_4 (tt_font, os leitores), rx03_5 (tt_exec_set_ppem, t2k_stream_close) e rx03_7
# (glyph_outline_free).
SAVE = False
TELAS = True
ROTINAS = [('t2k_set_up_hinting_for_size', 0x93e5b0, [0x83, 0xec, 0x6c, 0x53, 0x8b, 0x5c, 0x24, 0x74]),
           ('t2k_purge_memory_internal', 0x93e7d0, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('t2k_scale_coordinates', 0x93f9a0, [0x8b, 0x44, 0x24, 0x10, 0x53]),
           ('t2k_apply_five_tap_bitmap_filter', 0x93fb70, [0x83, 0xec, 0x1c, 0x53, 0x8b, 0x5c, 0x24, 0x24])]
_A = 'library_without_asserts'
DOCS = ('a T2K: o escalador, a preparação do ajuste, a limpeza da memória, a escala das coordenadas e o filtro de cinco tomadas',
        'the T2K: the scaler, the hinting set up, the memory purge, the coordinate scaling and the five-tap filter',
        [('t2k_set_up_hinting_for_size', 't2k_set_up_hinting_for_size', '0x93e5b0', _A, 'cdecl',
          'regra: cria o ajustador no primeiro uso (modo 2 com 1000 glifos ou mais), com os parâmetros da fonte (da tabela T2KG sem função de estilo, senão medidos; guardados na fonte uma vez); com tamanhos novos, o tamanho do ajustador e o do contexto TrueType (roda o prep); erro do ajustador sai por longjmp',
          'rule: creates the hinter on first use (mode 2 with 1000 glyphs or more), with the font parameters (from the T2KG table without a style function, else measured; kept in the font once); with new sizes, the hinter\'s size and the TrueType context\'s (the prep runs); a hinter error longjmps'),
         ('t2k_purge_memory_internal', 't2k_purge_memory_internal', '0x93e7d0', _A, 'cdecl',
          'regra: libera o glifo atual sempre; do nível 1 o mapa de bits (devolvendo a vaga 5) e o segundo mapa; do nível 100 o buffer da vaga 5 maior que 0x100 bytes; do nível 2 o ajustador',
          'rule: frees the current glyph always; from level 1 the bitmap (giving back slot 5) and the second bitmap; from level 100 slot 5\'s buffer when larger than 0x100 bytes; from level 2 the hinter'),
         ('t2k_scale_coordinates', 't2k_scale_coordinates', '0x93f9a0', _A, 'cdecl',
          'regra: n coordenadas de 16 bits em unidades da fonte para 26.6 de 32 bits pelo modo do registro: deslocamento, divisão simétrica ou fixed_mul',
          'rule: n 16-bit font unit coordinates to 32-bit 26.6 by the record\'s mode: shift, symmetric division or fixed_mul'),
         ('t2k_apply_five_tap_bitmap_filter', 't2k_apply_five_tap_bitmap_filter', '0x93fb70', _A, 'cdecl',
          'regra: o mapa ganha 4 colunas (recuo e avanço voltam 2 pixels) num bloco novo zerado, e cada cinza v se espalha em 5 bytes (a, b − a, b, b − a, a), a = v>>3 − v>>6, b = (v+1)>>1 − (v+15)>>5 − (v+4)>>3 − v>>6, em 8 bits; o mapa velho é liberado',
          'rule: the bitmap gains 4 columns (bearing and advance moved back 2 pixels) in a new zeroed block, and each grey v spreads over 5 bytes (a, b − a, b, b − a, a), a = v>>3 − v>>6, b = (v+1)>>1 − (v+15)>>5 − (v+4)>>3 − v>>6, in 8 bits; the old bitmap is freed')])
CAMPOS = []
INTERESSA = []
MUTACOES = []
