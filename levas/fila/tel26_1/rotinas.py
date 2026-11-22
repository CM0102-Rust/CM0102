# The display list base (display.cpp; reader tel26, the screens round, 21/11/2026).
# Conventions checked in the disassembly (every push against esp):
# - 0x55f1c0 stdcall(short slots) ret 4; 0x55f270 cdecl; 0x55f290 stdcall(channel) ret 4; 0x55f2c0 thiscall;
#   0x55f300 thiscall(this, x1, y1, x2, y2) ret 0x10 -> 1; 0x55f620 thiscall -> ax; 0x55f990 thiscall(this, 18 dwords)
#   ret 0x48 -> ax; 0x5603e0 thiscall(this, short area) ret 4.
# - Guio::init 0x5ee150 thiscall(item, display + 4, display + 0xba95e, index, the 18 fields) ret 0x54; Area::add_item
#   0x403200 thiscall(area, short index) ret 4; Area::layout 0x403350 thiscall; Area::draw 0x402e70 thiscall(area, 1)
#   ret 4; Guio::draw 0x5ee590 thiscall(item, 1) ret 4; 0x7fee80 get_background_picture thiscall(SM); 0x5e40d0
#   cdecl(path, reuse); 0x5e42e0 / 0x5e2eb0 cdecl(x1, y1, x2, y2); 0x5e30e0 cdecl(x1, y1, x2, y2, 20); 0x569610
#   animate_widgets thiscall; the four input handlers thiscall -> ax.
# - The 1080 exe patches the screen edges (0x31f -> 0x77f, 0x257 -> 0x437) in 0x55f2c0 and 0x55f300: the copy reads
#   each immediate from the running exe. Display::add_item's prologue is a jump in the 1080 exe (to a routine of its
#   own): the entry is for the original exe only (EXE).
# Every other item of tel26 (tel26_2..tel26_4) uses the helpers of this one (display_list, display_item,
# tel26_error_box, display_item_at...): this item goes first.
# Defects of the original: none found (Display::add_item leaves the table at 1199 items, one short of the 1200 of
# the order list).
MOTOR = True
TELAS = True
EXE = {0x55f990: 'original'}
ROTINAS = [('display_alloc_guio_snapshot_pool', 0x55f1c0, [0xa1, 0x18, 0x29, 0xae, 0x00]),
           ('display_free_guio_snapshot_pool', 0x55f270, [0xa1, 0x18, 0x29, 0xae, 0x00]),
           ('display_clear_guio_snapshot', 0x55f290, [0x8b, 0x15, 0x18, 0x29, 0xae, 0x00]),
           ('display_clear_screen_once', 0x55f2c0, [0x8b, 0x81, 0xfa, 0xf4, 0x12, 0x00]),
           ('display_draw_screen', 0x55f300, [0x51, 0x55, 0x8b, 0xe9, 0x57]),
           ('display_poll_input', 0x55f620, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('display_add_item', 0x55f990, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('display_push_area_order', 0x5603e0, [0x66, 0x8b, 0x54, 0x24, 0x04])]
DOCS = ('a lista de exibição (1): o depósito dos instantâneos dos itens, limpar e desenhar a tela, a leitura da entrada, acrescentar um item e a ordem das áreas',
        'the display list (1): the widget snapshot pool, clearing and drawing the screen, polling the input, adding an item and the area order',
        [('display_alloc_guio_snapshot_pool', 'display_alloc_guio_snapshot_pool', '0x55f1c0', 'display.cpp', 'stdcall',
          'aloca o depósito global dos instantâneos dos itens (0x74042 bytes por canal); se a alocação falha, a caixa de erro de display.cpp 195 e o jogo sai',
          'allocates the global widget snapshot pool (0x74042 bytes per channel); a failed allocation shows the display.cpp 195 error box and the game exits'),
         ('display_free_guio_snapshot_pool', 'display_free_guio_snapshot_pool', '0x55f270', 'display.cpp', 'cdecl',
          'libera o depósito dos instantâneos e zera o ponteiro',
          'frees the snapshot pool and clears the pointer'),
         ('display_clear_guio_snapshot', 'display_clear_guio_snapshot', '0x55f290', 'display.cpp', 'stdcall',
          'zera a contagem de itens guardados de um canal do depósito',
          'clears the saved item count of one channel of the pool'),
         ('display_clear_screen_once', 'Display::clear_screen_once', '0x55f2c0', 'display.cpp', 'thiscall',
          'escurece e vira a tela inteira na primeira vez e marca a tela como escurecida (os desenhos seguintes a escurecem de novo)',
          'darkens and flips the whole screen the first time and marks it darkened (later draws darken it again)'),
         ('display_draw_screen', 'Display::draw_screen', '0x55f300', 'display.cpp', 'thiscall',
          'desenha a tela: a foto de fundo (carregada quando muda, com DEFAULT_PIC.RGN de reserva), a disposição das áreas no primeiro desenho, as áreas abertas, os itens soltos, as áreas que abrem por cima, o escurecimento, o esmaecer e a virada do retângulo pedido ou da tela inteira',
          'draws the screen: the background picture (loaded when it changes, DEFAULT_PIC.RGN as fallback), the area layout on the first draw, the open areas, the free widgets, the popup areas, the darkening, the fade-in and the flip of the given rectangle or the whole screen'),
         ('display_poll_input', 'Display::poll_input', '0x55f620', 'display.cpp', 'thiscall',
          'devolve o item escolhido: a tecla de atalho do foco, o atalho ou o clique na fila, ou o tratador do modo de entrada (teste do mouse, menu que abre por cima, arrastar a barra de rolagem, arrastar um item)',
          'returns the chosen widget: the focus hotkey, the queued hotkey or click, or the input mode handler (mouse hit test, popup hover, scroll thumb drag, widget drag)'),
         ('display_add_item', 'Display::add_item', '0x55f990', 'display.cpp', 'thiscall',
          'acrescenta um item à tabela (até 1199; depois marca o estouro e devolve -1), inicia o item e o põe na lista da sua área ou na ordem dos itens soltos (até 1200, senão a caixa de display.cpp 746)',
          'adds an item to the table (up to 1199; then marks the overflow and returns -1), initialises it and puts it in its area list or in the free widget order (up to 1200, else the display.cpp 746 box)'),
         ('display_push_area_order', 'Display::push_area_order', '0x5603e0', 'display.cpp', 'thiscall',
          'acrescenta uma área à ordem de desenho (ignora -1; a 251ª dá a caixa de display.cpp 770)',
          'appends an area to the draw order (ignores -1; the 251st shows the display.cpp 770 box)')])
