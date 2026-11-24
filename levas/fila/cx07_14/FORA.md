# cx07_14: as rotinas graficas grandes do grupo cx07 que ficaram para a proxima rodada

O leitor cx07 (rodada 5, 24/11/2026) escreveu os itens cx07_1 a cx07_13 (as competicoes, a League e 27 rotinas da base
grafica) e parou aqui pelo tamanho: as 14 rotinas abaixo somam cerca de 18 KB de codigo de maquina de desenho e nao
couberam com a fidelidade pedida nesta rodada. Nenhuma foi escrita; nenhuma esta em `rust/src` nem em outro item da fila
(conferido em 24/11/2026). Todas sao de TELAS (rodam em todo desenho de tela). Os globais do video, os auxiliares de
travar e destravar o buffer de tras, `pack_pixel`/`unpack_pixel`, `clip_rect_to_surface`, `rect_overlap_area` e
`user32_*` ja estao nos cx07_11, cx07_12 e cx07_13: reaproveitar.

Notas para quem pegar (enderecos provados pela desmontagem; prologos na fronteira de instrucao):

| Endereco | Bytes | Nome | Notas |
|---|---|---|---|
| 0x5e2620 | 1696 | graphics_init_display | cdecl(largura, altura, 2, modo de fonte, tela cheia, titulo) (sx11_4). Win32 pela tabela de importacao (LoadCursorA, RegisterClassA, GetSystemMetrics, CreateWindowExA, ShowWindow, MoveWindow, SetWindowLongA) e DirectDraw (0x944c1c, provavelmente o DirectDrawCreate; SetCooperativeLevel 0x50, SetDisplayMode 0x54, CreateSurface 0x18, GetSurfaceDesc 0x58 com o formato de pixel copiado para 0xae3158, CreateClipper 0x10, SetHWnd 0x20, SetClipper 0x70); depois gui_init_palette_colours, gui_load_fonts, graphics_draw_rect e graphics_flip_rect. Prologo mov eax, [0x9cd808] (5 bytes). Igual no 1080. |
| 0x5e30e0 | 1440 | graphics_fade_rect | Prologo mov eax, 0x400c0 (5 bytes) seguido de call rel32 ao _chkstk: o desvio de 5 bytes serve. |
| 0x5e3730 | 1056 | graphics_draw_line | Bresenham com tracejado; chamada pelo graphics_draw_rect (cx07_13) pelo endereco. |
| 0x5e40d0 | 480 | gui_load_rgn_picture | Le o .rgn e converte 555/565. |
| 0x5e42e0 | 640 | gui_darken_rect | Tabela de 64K cores montada no primeiro uso (60 por cento). |
| 0x5e4ba0 | 1216 | gui_load_bitmap_font | cdecl(vaga, caminho); a vaga de 0x1404 bytes na tabela 0xae1dac (a letra c em +0xc + 0x14c: +0 largura, +4, +8 o desenho); copia as acentuadas das duas Trade. |
| 0x5e5160 | 2368 | gui_draw_text_line | Prologo push -1; mov eax, fs:[0] (8 bytes); chama a linha T2K com text_caret_glyph_callback (cx07_12). |
| 0x5e5ad0 | 512 | font_text_width | Prologo mov eax, 0x2004 + call _chkstk. |
| 0x5e5df0 | 3984 | gui_draw_panel | Rotina grande: em pedacos com nome, inteira num item. |
| 0x5e6d80 | 2416 | gui_draw_text_box | Prologo push -1; push imm32 (7 bytes). |
| 0x5e76f0 | 336 | gui_draw_progress_box | ALTERADA no cm0102-1080.exe (24 bytes: as coordenadas da caixa): EXE original. |
| 0x5e7840 | 608 | gui_draw_circle | ALTERADA no 1080 (6 bytes): EXE original. |
| 0x5e7aa0 | 1120 | circle_plot_points | ALTERADA no 1080 (72 bytes: o passo 800 do buffer): EXE original. |
| 0x5e8290 | 1552 | gui_show_message_box | ALTERADA no 1080 (6 bytes). Ja tem um desvio so de observacao no `rust/src/detours.rs` (o registro das caixas no ddraw.log, `ENTRY_BYTES`): portar exige decidir com o Lucas como os dois convivem. |
