# The Display's life: the two command buffers, building and destroying a Display, resetting it, adding an area and the
# network status packet (display.cpp; reader tel37, the screens round, 21/11/2026).
# Conventions checked in the disassembly:
# - init_net_send_buffer 0x55eef0 / init_display_packet_buffer 0x55ef20 cdecl (static initialisers: NetworkBuffer ctor
#   0x77b630 thiscall(capacity) ret 4, then atexit(free)); free_net_send_buffer 0x55ef10 / free_display_packet_buffer
#   0x55ef40 cdecl (mov ecx, buffer; jmp NetworkBuffer dtor 0x77b6e0).
# - Display::Display 0x55ef50 thiscall(channel) ret 4 -> this; Display::~Display 0x55efe0 thiscall (exception frame,
#   vector_destructor_iterator stdcall(ptr, size, count, dtor) with the thunks 0x55f1b0 / 0x55f1a0); Display::reset
#   0x55f7c0 thiscall(release) ret 4; Display::add_area 0x55fba0 thiscall(x1, y1, x2, y2, columns, column weights,
#   rows, row weights, marks, colour, owner) ret 0x2c -> ax (Area::init 0x402ac0 thiscall(areas, items, index, the
#   eleven) ret 0x38; push_area_order 0x5603e0 thiscall(short) ret 4; 0x569540 thiscall(ptr, short n) ret 8 = put
#   the count and the bytes); display_send_status_packet 0x568290 stdcall(session, player, status) ret 0xc (the
#   session's send 0x77ac30 thiscall(bytes, length, player) ret 0xc -> 0 on failure).
# - The 1080 exe replaces the first 4 bytes of Display::add_area with a jump to a routine of its own (coordinate
#   scaling): its entry is for the original exe only (EXE). Nothing else of this item differs in the 1080 exe.
# Where they run: the buffers before WinMain and at exit; the Display constructor with the screen manager (startup),
# the destructor at exit; reset on every new screen (ScreenManager and process_command_packet command 0); add_area
# on every screen (172 callers); the status packet only for network players.
# Depends on tel26_1 (display_list, display_area_at, display_item_at, display_method, display_method1,
# DISPLAY_SNAPSHOT_POOL, DISPLAY_CPP, first_differences) and tel37_1 (network_buffer, the put functions,
# NET_SEND_BUFFER, DISPLAY_PACKET_BUFFER).
# Defects of the original: add_area copies the column weights into the packet buffer without a room check; the
# destructor frees the media list's entry array but not its 8-byte holder [inference].
TELAS = True
EXE = {0x55fba0: 'original'}
ROTINAS = [('init_net_send_buffer', 0x55eef0, [0x68, 0x50, 0xc3, 0x00, 0x00]),
           ('free_net_send_buffer', 0x55ef10, [0xb9, 0xf8, 0x28, 0xae, 0x00]),
           ('init_display_packet_buffer', 0x55ef20, [0x68, 0x00, 0x0c, 0x00, 0x00]),
           ('free_display_packet_buffer', 0x55ef40, [0xb9, 0x08, 0x29, 0xae, 0x00]),
           ('display_build', 0x55ef50, [0x8b, 0x54, 0x24, 0x04, 0x8b, 0xc1]),
           ('display_destroy', 0x55efe0, [0x6a, 0xff, 0x68, 0x6a, 0x80, 0x95, 0x00]),
           ('display_reset', 0x55f7c0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('display_add_area', 0x55fba0, [0xa1, 0x0c, 0x29, 0xae, 0x00]),
           ('display_send_status_packet', 0x568290, [0xa1, 0x00, 0x29, 0xae, 0x00])]
DOCS = ('a vida da exibição: os dois buffers de comando, construir e destruir uma exibição, recomeçá-la, acrescentar uma área e o pacote de estado da rede',
        "the Display's life: the two command buffers, building and destroying a Display, resetting it, adding an area and the network status packet",
        [('init_net_send_buffer', 'init_net_send_buffer', '0x55eef0', 'display.cpp', 'cdecl',
          'constrói o buffer de envio da rede (50000 bytes) e registra a liberação no atexit (antes do WinMain)',
          'builds the network send buffer (50000 bytes) and registers its freeing with atexit (before WinMain)'),
         ('free_net_send_buffer', 'free_net_send_buffer', '0x55ef10', 'display.cpp', 'cdecl',
          'libera o buffer de envio da rede (na saída)',
          'frees the network send buffer (at exit)'),
         ('init_display_packet_buffer', 'init_display_packet_buffer', '0x55ef20', 'display.cpp', 'cdecl',
          'constrói o buffer de pacotes da exibição (3072 bytes) e registra a liberação no atexit',
          'builds the display packet buffer (3072 bytes) and registers its freeing with atexit'),
         ('free_display_packet_buffer', 'free_display_packet_buffer', '0x55ef40', 'display.cpp', 'cdecl',
          'libera o buffer de pacotes da exibição (na saída)',
          'frees the display packet buffer (at exit)'),
         ('display_build', 'Display::Display', '0x55ef50', 'display.cpp', 'thiscall, ret 4',
          'constrói a exibição: o canal, sem imagens, sem lista de mídia, sem áreas nem itens, nada pendente, sem tecla nem item na fila (-1) e o nome do fundo "\\x01" para o primeiro desenho carregar a foto',
          'builds the Display: the channel, no images, no media list, no areas or widgets, nothing pending, no key or queued widget (-1) and the background name "\\x01" so that the first draw loads the picture'),
         ('display_destroy', 'Display::~Display', '0x55efe0', 'display.cpp', 'thiscall',
          'destrói a exibição: libera a lista de mídia, o depósito dos instantâneos, as áreas e as imagens dos itens, as imagens de arrastar e de fundo e (sem mais usuários) a imagem compartilhada, e roda os destrutores das tabelas de itens (1200) e de áreas (250)',
          "destroys the Display: frees the media list, the snapshot pool, the areas and the widgets' images, the drag and background images and (with no more users) the shared image, and runs the destructors of the widget (1200) and area (250) tables"),
         ('display_reset', 'Display::reset', '0x55f7c0', 'display.cpp', 'thiscall, ret 4',
          'recomeça a exibição para uma tela nova: com o pedido (e sem pacote de manutenção enviado) libera as áreas e as imagens dos itens, esvazia os dois buffers, põe o comando 2 (tela nova) no de pacotes e zera a lista (contagens, arrasto, estouros; suspensa, rolagem, arrasto e foco em -1)',
          'resets the Display for a new screen: when asked (and no keepalive sent) releases the areas and the widget images, empties both buffers, puts command 2 (new screen) in the packet buffer and empties the list (counts, drag, overflows; popup, scroll, drag and focus at -1)'),
         ('display_add_area', 'Display::add_area', '0x55fba0', 'display.cpp', 'thiscall, ret 0x2c',
          'acrescenta uma área: o comando no buffer de pacotes (retângulo, colunas e pesos, linhas e pesos, marcas, cor, dono) e, até 249 áreas, uma área zerada iniciada por Area::init e posta na ordem de desenho; devolve o índice, ou -1 (marca o estouro) com a tabela cheia',
          'adds an area: its command in the packet buffer (rectangle, columns and weights, rows and weights, marks, colour, owner) and, up to 249 areas, a cleared area initialised by Area::init and put in the draw order; returns the index, or -1 (marking the overflow) when the table is full'),
         ('display_send_status_packet', 'display_send_status_packet', '0x568290', 'display.cpp', 'stdcall, ret 0xc',
          'manda um pacote de estado a um jogador de rede: comando 5 e o byte de estado no buffer de envio, enviados pela sessão; falha dá a caixa "Network Error" (display.cpp 3154)',
          'sends a status packet to a network player: command 5 and the status byte in the send buffer, sent by the session; a failure shows the "Network Error" box (display.cpp 3154)')])
