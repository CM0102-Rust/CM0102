# A base grafica: a abertura do video (goldcup.cpp pelo mapa; a biblioteca grafica; leitor sbC, rodada 5, 24/11/2026,
# a rotina que o cx07 deixou em levas/fila/cx07_14/FORA.md). Item de TELAS: roda na abertura do jogo (game_config
# 0x5d72c0 e 0x5d8480) e ao trocar o modo de video nas preferencias ou na rede (0x8aee20).
# Caminho no jogo: abrir o jogo comparando (a primeira abertura: a janela, os cursores, o DirectDraw); para a segunda
# (a janela de novo), trocar nas preferencias do jogo uma opcao que reabre o video (por exemplo a fonte) e confirmar.
# Convencao conferida: cdecl(largura, altura, flags, modo de fonte, tela cheia, titulo) -> 1/0 (sx11_4).
# A API pela tabela de importacao, como o original (a DLL intercepta parte dela): GetCursor 0x9671c8, LoadCursorA
# 0x9671c4, ShowCursor 0x9671c0, SetCursor 0x9671bc, LoadIconA 0x9671b8, RegisterClassA 0x9671b4, GetSystemMetrics
# 0x9671b0, GetDesktopWindow 0x9671ac, GetWindowRect 0x9671a8, CreateWindowExA 0x9671a4, UpdateWindow 0x9671a0,
# ShowWindow 0x967204, SetWindowLongA 0x9671ec, MoveWindow 0x9671e8; DirectDrawCreate pelo salto 0x944c1c (jmp
# [0x96700c]); os metodos COM stdcall: IDirectDraw SetCooperativeLevel 0x50, SetDisplayMode 0x54 (3 argumentos, a
# versao 1), CreateSurface 0x18, CreateClipper 0x10, Release 8; IDirectDrawSurface GetSurfaceDesc 0x58, SetClipper 0x70,
# Release 8; IDirectDrawClipper SetHWnd 0x20. Depois gui_init_palette_colours 0x5e4560 (ALTERADA no 1080: pelo
# endereco), gui_load_fonts 0x5e4a60, graphics_draw_rect 0x5e3b50, graphics_flip_rect 0x5e2eb0 e, nas falhas,
# gui_show_message_box 0x5e8290 ("Error", com o video desligado vira MessageBoxA). Prologo mov eax, [0x9cd808] (5
# bytes).
# Defeitos do original: (1) na segunda abertura em janela o lugar nao e o centro da primeira: soma a altura da moldura
# em vez de subtrair, e soma duas larguras de moldura em vez de subtrair; (2) o DDSURFACEDESC so tem os campos usados,
# o resto e lixo da pilha (na copia, zero); (3) em janela o modo de video e posto no tamanho da area de trabalho a 16
# bits (SetDisplayMode mesmo sem tela cheia).
# Igual no cm0102-1080.exe (inteira).
# Depende dos cx07_11 (os globais do video, os cursores, user32_*, com_call0), cx07_12 (WINDOW_FRAME_X,
# WINDOW_CAPTION_Y, SCREEN_PIXEL_FORMAT, SURFACE_DESC_SIZE) e cx07_13 (a mesma leva).
# Convivencia: o cm0102-base (apresentacao.rs) subclassifica a janela e o iat_hooks.rs intercepta a API pela tabela de
# importacao; a copia chama pela mesma tabela, entao os dois continuam vendo tudo.
# Comparador: so a original (cria a janela e os objetos do DirectDraw).
SAVE = False
TELAS = True
ROTINAS = [('graphics_init_display', 0x5e2620, [0xa1, 0x08, 0xd8, 0x9c, 0x00])]
DOCS = ('a base gráfica: a abertura do vídeo',
        'the graphics base: the display init',
        [('graphics_init_display', 'graphics_init_display', '0x5e2620', 'goldcup.cpp', 'cdecl',
          'cria (na primeira vez: cursores, classe, janela centrada e DirectDraw) ou acerta de novo a janela do jogo, põe o modo de vídeo, cria a superfície primária, a de trás (flag 2) e o recortador em janela, guarda o formato do pixel, carrega as cores e as fontes e limpa a tela de preto; numa falha, a caixa de erro e 0; defeito: na segunda vez a janela não fica no centro',
          'creates (the first time: cursors, class, centred window and DirectDraw) or sets again the game window, sets the video mode, creates the primary surface, the back buffer (flag 2) and the clipper when windowed, keeps the pixel format, loads the colours and the fonts and clears the screen to black; on a failure, the error box and 0; defect: the second time the window is not centred')])
CAMPOS = []
INTERESSA = []
