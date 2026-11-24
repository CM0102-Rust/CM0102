# Desenhar um texto TrueType/T2K numa superficie de 15 ou 16 bits (FONT::draw_text, o codigo de fontes que o mapa poe
# em fog_of_war.cpp; leitor sx10, rodada 5, 23/11/2026). A rotina inteira (0x5b0d80..0x5b1893, ret 0x20; 2,8 KB, sem
# tabela de saltos) num item so, em pedacos com nome: o recorte do glifo (font_clip_glyph), o percurso do bitmap
# (font_walk_glyph) e as tres pinturas (font_paint_blend, font_paint_tint com a flag 0x40, font_paint_emboss com 0x20),
# o formato do pixel (font_unpack/font_pack, 5-6-5 quando a mascara verde 0xae316c e 0x7e0, senao 5-5-5).
# Item de TELAS: e o desenho de todo texto da interface no modo de fonte -2 (o padrao) e com uma fonte escolhida.
# Convencao conferida na desmontagem: thiscall(fonte, superficie, x, y, flags, tamanho, cor, texto, callback) ret 0x20;
# a flag 0x80 chama a si mesma com (flags & 0xff7f) | 0x50, cor 0, sem callback (a sombra) e segue com flags & 0xff7f;
# o callback e cdecl(x0, y0, x1, y1) -> int (nao zero pula o caractere). Chamada por 0x5e5160 (ecx = 0xaebdf4..0xaebe00,
# o vetor global FONT; flags 0x280 no texto da barra) e por ela mesma (0x5b0dd6). Mesmos bytes no cm0102-1080.exe.
# Depende do sx10_13 (font_state, font_glyph, t2k_scaler, font_set_style_via_exe, font_glyph_via_exe).
TELAS = True
ROTINAS = [('font_draw_text', 0x5b0d80, [0x83, 0xec, 0x50, 0x55, 0x57])]
DOCS = ('desenhar um texto TrueType/T2K numa superfície de 15 ou 16 bits',
        'drawing a TrueType/T2K text on a 15 or 16-bit surface',
        [('font_draw_text', 'FONT::draw_text', '0x5b0d80', 'fog_of_war.cpp', 'thiscall, ret 0x20',
          'desenha um texto de 16 bits terminado em zero, glifo a glifo do cache, misturando cada pixel pela cobertura (0..126): com a cor, com a média da cor e do pixel (0x40, a sombra) ou em relevo (0x20: clareia o pixel para 7/5 e escurece o de cima à esquerda para 3/5); 0x80 desenha antes a sombra borrada em preto; 0x100 espelha cada glifo; 0x200 troca | por espaço; caracteres abaixo de 0x20 não desenham nem andam; um callback pode recusar cada caractere pela caixa dele e recebe a caixa vazia do fim; 1, ou 0 sem fonte carregada; defeito: o glifo espelhado cortado na borda direita sai deslocado',
          'draws a zero-terminated 16-bit text, glyph by glyph from the cache, mixing each pixel by the coverage (0..126): with the colour, with the average of the colour and the pixel (0x40, the shadow) or embossed (0x20: lightens the pixel towards 7/5 and darkens the one up and left towards 3/5); 0x80 first draws the blurred black shadow; 0x100 mirrors each glyph; 0x200 turns | into a space; characters below 0x20 neither draw nor move; a callback may refuse each character by its box and gets the empty box at the end; 1, or 0 without a loaded font; defect: a mirrored glyph cut by the right edge comes out shifted')])
