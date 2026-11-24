# A base grafica: os cursores, as marcas do vidro, o recorte e a sobreposicao de retangulos, a liberacao de uma imagem
# e o desligamento do video (goldcup.cpp pelo mapa; e a biblioteca grafica do jogo, compilada logo depois da Copa
# America; leitor cx07, rodada 5, 24/11/2026). Item de TELAS: rodam nas telas (o cursor, o recorte e a sobreposicao em
# todo desenho), na abertura (as fontes T2K estaticas, antes do main) e na saida do jogo (o desligamento e o destrutor
# das fontes). Caminho no jogo: abrir o jogo comparando, navegar por algumas telas (o cursor de espera aparece ao
# carregar) e sair pelo menu.
# Convencoes conferidas: todas cdecl (chamadas com call e add esp do chamador; set_cursor_mode, set_software_cursor_flag,
# gui_free_image com 1 argumento; clip_rect_to_surface com 5; rect_overlap_area com 8, dois RECT por valor).
# - 0x5e25d0 t2k_fonts_static_init: o eh vector constructor iterator do CRT 0x9463af (stdcall, 5 argumentos: 0xaebdf4, 4,
#   4, FONT::FONT 0x5b0ab0, FONT::~FONT 0x5b0b40) e atexit 0x94627a (cdecl) com 0x5e2600. Prologo push imm32 (5 bytes).
# - 0x5e2600 t2k_fonts_static_destroy: o vector destructor iterator 0x9462bb (stdcall, 4 argumentos). Prologo push imm32.
# - 0x5e2e50 graphics_restore_lost_surfaces: IsLost (slot 0x60) e Restore (0x6c) do primario e do buffer de tras (COM,
#   stdcall). Prologo mov eax, [0xaebe0c] (5 bytes).
# - 0x5e2ea0 graphics_is_full_screen: mov eax, [0xae319c]; ret. Prologo de 5 bytes (a instrucao inteira).
# - 0x5e3680 clip_rect_to_surface(x0, y0, x1, y1, saida) -> 1/0. Prologo push ecx; mov eax, [0xaebe0c] (6 bytes).
# - 0x5e7f00 rect_overlap_area(8 inteiros) -> area. Prologo mov eax, [esp+0xc]; mov ecx, [esp+0x1c] (8 bytes).
# - 0x5e42b0 gui_free_image(imagem): free 0x9452ca de +0xc (zerado) e da imagem. Prologo push esi; mov esi, [esp+8].
# - 0x5e8160 get_cursor_mode: mov eax, [0x9cd804]; ret (5 bytes). 0x5e8170 set_cursor_mode(modo) e 0x5e81e0
#   reapply_cursor_mode: ShowCursor, SetCursor e GetCursor pela tabela de importacao (0x9671c0, 0x9671bc, 0x9671c8).
#   Prologo mov eax, [0x9cd804] (5 bytes).
# - 0x5e8280 set_software_cursor_flag(v): mov eax, [esp+4]; mov [0xaebe14], eax (9 bytes).
# - 0x5e2cc0 graphics_shutdown_display: SetCursor, DestroyCursor (0x9671d0), Release (slot 8) do recortador,
#   RestoreDisplayMode (0x4c) e Release do DirectDraw, ShowWindow (0x967204) e DestroyWindow (0x9671cc), free dos
#   desenhos das letras 0x20 a 0xff das 7 fontes de mapa de bits (a tabela 0xae1dac, 0x1404 bytes por fonte, fontes 1 a
#   7; a letra c em +0xc + 0x14c, o desenho em +8; os ponteiros ficam). Prologo mov eax, [0xaebe18] (5 bytes).
# Defeito do original: o desligamento libera os desenhos das letras sem zerar os ponteiros (uma segunda carga das fontes
# ou um segundo desligamento os liberaria de novo).
# Iguais no cm0102-1080.exe (todas inteiras).
# Os nomes dos globais do video (DISPLAY_OFF, DIRECTDRAW, PRIMARY_SURFACE, BACK_BUFFER, SCREEN_WIDTH...) e os auxiliares
# user32_* e com_call0 daqui sao usados pelos cx07_12 e cx07_13.
# Comparadores: so a original no que desenha, mexe no cursor, solta o DirectDraw ou libera memoria; lado a lado nas
# puras (o recorte, a sobreposicao, os leitores).
SAVE = False
TELAS = True
ROTINAS = [('t2k_fonts_static_init', 0x5e25d0, [0x68, 0x40, 0x0b, 0x5b, 0x00]),
           ('t2k_fonts_static_destroy', 0x5e2600, [0x68, 0x40, 0x0b, 0x5b, 0x00]),
           ('graphics_restore_lost_surfaces', 0x5e2e50, [0xa1, 0x0c, 0xbe, 0xae, 0x00]),
           ('graphics_is_full_screen', 0x5e2ea0, [0xa1, 0x9c, 0x31, 0xae, 0x00]),
           ('clip_rect_to_surface', 0x5e3680, [0x51, 0xa1, 0x0c, 0xbe, 0xae, 0x00]),
           ('rect_overlap_area', 0x5e7f00, [0x8b, 0x44, 0x24, 0x0c, 0x8b, 0x4c, 0x24, 0x1c]),
           ('gui_free_image', 0x5e42b0, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('get_cursor_mode', 0x5e8160, [0xa1, 0x04, 0xd8, 0x9c, 0x00]),
           ('set_cursor_mode', 0x5e8170, [0xa1, 0x04, 0xd8, 0x9c, 0x00]),
           ('reapply_cursor_mode', 0x5e81e0, [0xa1, 0x04, 0xd8, 0x9c, 0x00]),
           ('set_software_cursor_flag', 0x5e8280, [0x8b, 0x44, 0x24, 0x04, 0xa3, 0x14, 0xbe, 0xae, 0x00]),
           ('graphics_shutdown_display', 0x5e2cc0, [0xa1, 0x18, 0xbe, 0xae, 0x00])]
DOCS = ('a base gráfica: cursores, marcas do vídeo, recorte e sobreposição de retângulos, liberação de imagem e desligamento',
        'the graphics base: cursors, display flags, rectangle clipping and overlap, image release and shutdown',
        [('t2k_fonts_static_init', 't2k_fonts_static_init', '0x5e25d0', 'goldcup.cpp', 'cdecl',
          'monta o vetor estático das quatro fontes T2K e registra o seu destrutor para a saída do programa',
          'builds the static array of the four T2K fonts and registers its destructor for program exit'),
         ('t2k_fonts_static_destroy', 't2k_fonts_static_destroy', '0x5e2600', 'goldcup.cpp', 'cdecl',
          'desmonta o vetor estático das quatro fontes T2K na saída do programa',
          'destroys the static array of the four T2K fonts at program exit'),
         ('graphics_restore_lost_surfaces', 'graphics_restore_lost_surfaces', '0x5e2e50', 'goldcup.cpp', 'cdecl',
          'restaura a superfície primária e a de trás quando o DirectDraw as dá por perdidas, salvo com o vídeo desligado',
          'restores the primary surface and the back buffer when DirectDraw reports them lost, unless the display is off'),
         ('graphics_is_full_screen', 'graphics_is_full_screen', '0x5e2ea0', 'goldcup.cpp', 'cdecl',
          'diz se o vídeo está em tela cheia',
          'tells whether the display is full screen'),
         ('clip_rect_to_surface', 'clip_rect_to_surface', '0x5e3680', 'goldcup.cpp', 'cdecl',
          'ordena os cantos de um retângulo e o prende à tela; devolve 1 se ele cabia inteiro, 0 se foi cortado ou sem vídeo',
          'orders a rectangle\'s corners and clamps it to the screen; returns 1 when it fitted whole, 0 when clamped or without a display'),
         ('rect_overlap_area', 'rect_overlap_area', '0x5e7f00', 'goldcup.cpp', 'cdecl',
          'a área da sobreposição de dois retângulos, ou 0',
          'the area of the overlap of two rectangles, or 0'),
         ('gui_free_image', 'gui_free_image', '0x5e42b0', 'goldcup.cpp', 'cdecl',
          'libera os pixels de uma imagem e a imagem',
          'frees an image\'s pixels and the image'),
         ('get_cursor_mode', 'get_cursor_mode', '0x5e8160', 'goldcup.cpp', 'cdecl',
          'o modo atual do cursor do mouse (0 normal, 1 espera, 2 apontar)',
          'the current mouse cursor mode (0 normal, 1 busy, 2 pointing)'),
         ('set_cursor_mode', 'set_cursor_mode', '0x5e8170', 'goldcup.cpp', 'cdecl',
          'troca o cursor do mouse para o modo pedido quando o modo muda, escondendo-o e mostrando-o em volta',
          'switches the mouse cursor to the given mode when the mode changes, hiding and showing it around'),
         ('reapply_cursor_mode', 'reapply_cursor_mode', '0x5e81e0', 'goldcup.cpp', 'cdecl',
          'põe de novo o cursor do modo atual quando o sistema mostra outro',
          'sets the current mode\'s cursor again when the system shows another one'),
         ('set_software_cursor_flag', 'set_software_cursor_flag', '0x5e8280', 'goldcup.cpp', 'cdecl',
          'liga ou desliga o esconder e redesenhar do cursor em cada atualização da tela',
          'turns on or off hiding and redrawing the cursor around each screen update'),
         ('graphics_shutdown_display', 'graphics_shutdown_display', '0x5e2cc0', 'goldcup.cpp', 'cdecl',
          'desliga o vídeo: o cursor do sistema de volta, os cursores do jogo destruídos, o recortador e o DirectDraw soltos (com o modo de vídeo restaurado), a janela escondida e destruída e os desenhos das letras das fontes liberados',
          'shuts the display down: the system cursor back, the game cursors destroyed, the clipper and DirectDraw released (with the display mode restored), the window hidden and destroyed and the fonts\' glyph bitmaps freed')])
CAMPOS = []
INTERESSA = []
