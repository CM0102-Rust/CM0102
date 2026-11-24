# O objeto FONT: carregar um arquivo TrueType/T2K, o estilo e o tamanho, o cache de glifos e a largura do texto (o
# codigo de fontes que o mapa poe em fog_of_war.cpp; leitor sx10, rodada 5, 23/11/2026). Item de TELAS: as quatro
# fontes da interface (o vetor global FONT de 0xaebdf4: pequena, media, grande e simbolos; no modo -2 "Futuristic",
# o padrao, os arquivos small/medium/large.t2k e symbol.ttf do SI_DATA), usadas por todo texto das telas.
# Convencoes conferidas na desmontagem: FONT::FONT thiscall() -> this (construtor de vetor 0x9463af em 0x5e25d0, antes
# do main); FONT::~FONT thiscall (atexit 0x5e2600); load_font_file thiscall(qual) ret 4 (0x5e4a60, com 3, 0, 1, 2);
# unload thiscall; set_style_and_size thiscall(flags, tamanho) ret 8; get_cached_glyph thiscall(flags, tamanho,
# caractere) ret 0xc; get_text_width thiscall(flags, tamanho, texto) ret 0xc (0x5e5ad0 usa so o ax).
# A biblioteca T2K no exe (cdecl, todas com &erro no fim; nomes [inferencia]): 0x9441a0, 0x944300, 0x9416c0,
# 0x941760, 0x942ff0, 0x943f30, 0x93dd90, 0x93fa90, 0x93de70, 0x93e8c0, 0x93e780; _ftol 0x945640 (a inclinacao, no
# x87 como o original: fld pi, fmul, fsin, fimul, fmul 65536).
# RELATIVAS: o prologo do FONT::~FONT (0x5b0b40) tem a chamada ao unload no 4o byte: (0x5b0b40, 3) em
# rust/src/detours.rs. Sem tabelas de salto. Mesmos bytes no cm0102-1080.exe.
# Depende do sx10_12 (font_files, font_find_folder_once, font_is_font_extension); o sx10_14 depende deste.
TELAS = True
ROTINAS = [('font_constructor', 0x5b0ab0, [0xa1, 0x68, 0x2c, 0xae, 0x00]),
           ('font_destructor', 0x5b0b40, [0x56, 0x8b, 0xf1, 0xe8, 0xf8, 0x0d, 0x00, 0x00]),
           ('font_load_font_file', 0x5b0b60, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00]),
           ('font_unload', 0x5b1940, [0x51, 0x53, 0x57, 0x8b, 0x39]),
           ('font_set_style_and_size', 0x5b1a00, [0x83, 0xec, 0x2c, 0x53, 0x56]),
           ('font_get_cached_glyph', 0x5b1ba0, [0x83, 0xec, 0x20, 0x53, 0x8b, 0x19]),
           ('font_get_text_width', 0x5b18a0, [0x51, 0x53, 0x8b, 0xd9, 0x56])]
DOCS = ('o objeto FONT: carregar um arquivo TrueType/T2K, o estilo e o tamanho, o cache de glifos e a largura do texto',
        'the FONT object: loading a TrueType/T2K file, the style and size, the glyph cache and the text width',
        [('font_constructor', 'FONT::FONT', '0x5b0ab0', 'fog_of_war.cpp', 'thiscall',
          'acha a pasta FONTS uma vez e aloca o estado zerado de 0x23 bytes da fonte',
          "finds the FONTS folder once and allocates the font's zeroed 0x23-byte state"),
         ('font_destructor', 'FONT::~FONT', '0x5b0b40', 'fog_of_war.cpp', 'thiscall',
          'descarrega a fonte e libera o estado',
          'unloads the font and frees its state'),
         ('font_load_font_file', 'FONT::load_font_file', '0x5b0b60', 'fog_of_war.cpp', 'thiscall, ret 4',
          'descarrega, escolhe o arquivo (com uma fonte escolhida, a da pasta FONTS para pequena, média e grande; senão, e sempre para os símbolos, small/medium/large.t2k ou symbol.ttf do SI_DATA), exige extensão .ttf/.ttc/.t2k e face 0, lê o arquivo inteiro para a memória e abre um fluxo T2K; 1, ou 0 em qualquer falha; defeito: se a leitura falha o buffer fica alocado até o próximo descarregamento',
          'unloads, picks the file (with a chosen font, the FONTS file for small, medium and large; otherwise, and always for the symbols, small/medium/large.t2k or symbol.ttf in SI_DATA), requires a .ttf/.ttc/.t2k extension and face 0, reads the whole file into memory and opens a T2K stream; 1, or 0 on any failure; defect: when the read fails the buffer stays allocated until the next unload'),
         ('font_unload', 'FONT::unload', '0x5b1940', 'fog_of_war.cpp', 'thiscall',
          'libera os glifos do cache e seus bitmaps, o vetor do cache, os objetos T2K (escalador, fonte, fluxo, memória) e o buffer do arquivo, e zera o estado',
          "frees the cached glyphs and their bitmaps, the cache array, the T2K objects (scaler, font, stream, memory) and the file buffer, and zeroes the state"),
         ('font_set_style_and_size', 'FONT::set_style_and_size', '0x5b1a00', 'fog_of_war.cpp', 'thiscall, ret 8',
          'refaz o escalador T2K quando os bits de negrito (0x1 ×1,25, 0x2 ×1,5) mudam e lhe dá nova transformação a 96 dpi quando a inclinação (0x4 π/12, 0x8 π/6) ou o tamanho em pixels mudam',
          'rebuilds the T2K scaler when the bold bits (0x1 ×1.25, 0x2 ×1.5) change and gives it a new 96 dpi transformation when the slant (0x4 π/12, 0x8 π/6) or the pixel size change'),
         ('font_get_cached_glyph', 'FONT::get_cached_glyph', '0x5b1ba0', 'fog_of_war.cpp', 'thiscall, ret 0xc',
          'procura o glifo (caractere, tamanho, 5 bits de flags) num cache de 256 com o último usado na frente; sem ele, reaproveita o mais antigo ou cresce o cache, desenha pelo T2K e copia o bitmap, ou com a flag 0x10 uma cópia borrada 5×5 quatro pixels maior (a sombra); defeitos: a janela do borrado para no primeiro pixel fora em vez de pulá-lo (as duas primeiras linhas e colunas ficam vazias) e o bitmap borrado é gravado com a largura como passo de linha, mas lido com os bytes por linha',
          'looks the glyph up (character, size, 5 flag bits) in a 256-entry cache with the last used first; without it, reuses the oldest or grows the cache, renders through T2K and copies the bitmap, or with flag 0x10 a 5×5 blurred copy four pixels bigger (the shadow); defects: the blur window stops at the first pixel outside instead of skipping it (the first two rows and columns stay empty) and the blurred bitmap is written with the width as row stride but read with the row bytes'),
         ('font_get_text_width', 'FONT::get_text_width', '0x5b18a0', 'fog_of_war.cpp', 'thiscall, ret 0xc',
          'a largura do texto em pixels: o avanço de cada glifo e, no último, a extensão da tinta (esquerda + largura), arredondada; 0 sem fonte carregada',
          "the text's width in pixels: each glyph's advance and, for the last one, its ink extent (left + width), rounded; 0 without a loaded font")])
