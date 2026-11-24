# Mandar uma secao da tela a um jogador em rede (display.cpp; leitor tel46, rodada 5, 23/11/2026). Uma rotina grande
# (7,4 KB) inteira, partida em pedacos com nome (cabecalho, widget, mascaras, retrato, linhas de rolagem). Usa
# NET_SEND_BUFFER, DISPLAY_PACKET_BUFFER, network_buffer_put_char/short/ushort/ulong/int (tel37_1),
# network_buffer_put_long (tel46_1: este item depende dele), network_buffer_cursor/advance, GuioFields e
# click_guio_fields (tel40_3), display_network_error, display_item_at, display_area_at, display_life, display_list,
# display (FOCUS_*), area_scroll::PENDING_ROW, DISPLAY_SNAPSHOT_POOL, c_string_len, c_string_bytes.
# Convencoes conferidas na desmontagem:
# - Display::send_screen_section 0x562370 thiscall(d, sessao, jogador, secao byte) ret 0xc -> int; quadro 0x28a4 com
#   __chkstk (0x945610): o prologo de 5 bytes e o mov eax, 0x28a4 (fronteira de instrucao; nao entra em RELATIVAS).
# - Guio::get_fields 0x5ee4a0 thiscall, 19 saidas, ret 0x4c: os mapas das vagas de pilha das duas chamadas (o widget
#   e o retrato) conferidos push a push (bandeiras +0x30/+0x88, estilo +0x34/+0x90, alvo +0x40/+0x9c, ...).
# - put_short 0x5601e0 (o tamanho do texto, chamado pelo endereco); as outras escritas sao em linha, com as linhas
#   do network.h: char 133, short 134, int 137, long 136, ushort 139, ulong 141 (iguais aos NetworkBuffer::put_*).
# - Network::send_packet 0x77ac30 thiscall(sessao, dados, tamanho, jogador) ret 0xc; caixa "Network Error" de
#   display.cpp nas linhas 2086 (0x826), 2101 (0x835), 2252 (0x8cc), 2287 (0x8ef) e 2304 (0x900).
# - O retrato dos widgets: pool [0xae2918] + slot atual (0xb77c7a) x 0x74042, contagem na word +0x74040.
# Chamada pelo Display::log_screen_sent (0x5622a0, secoes 0, 2, 1, 3) e pelo comando 5 do process_command_packet
# (0x565f6e): so numa partida em rede. Comparador: so a original, com o plano montado antes num buffer de rascunho.
# Fora da lista (ver o relatorio): 0x55f1a0 e 0x55f1b0 (thunks jmp Area::release / Guio::free_images, passados por
# valor ao vector_destructor_iterator; sem regra propria) e 0x567000 / 0x567260 (nao sao rotinas: 0x567000 cai no
# meio de uma instrucao, 0x567001 mov ax, [esp+0x20] e o epilogo do comando 6 do process_command_packet 0x564080,
# quadro 0x2af8, ret 0xc; 0x567260 e o or ax, 0xffff / epilogo de outro ramo da mesma rotina, ja Rust).
# Defeito do original: uma secao fora de 0..3 devolve um dword da pilha nunca escrito (a copia devolve 0); a soma
# do tamanho "igual a um anterior" desconta 17 bytes, embora o visual ocupe ate 19 (so escolhe a mascara).
# Iguais no cm0102-1080.exe (bytes conferidos).
TELAS = True
ROTINAS = [('display_send_screen_section', 0x562370, [0xb8, 0xa4, 0x28, 0x00, 0x00])]
DOCS = ('mandar uma seção da tela a um jogador em rede: o cabeçalho, os widgets com as mudanças, os pacotes das áreas e as linhas de rolagem',
        'sending one section of the screen to a network player: the header, the widgets with their changes, the area packets and the scroll rows',
        [('display_send_screen_section', 'Display::send_screen_section', '0x562370', 'display.cpp', 'thiscall, ret 0xc',
          'monta e manda uma seção: 0 o cabeçalho (o fundo, o formato de cor, as contagens, o foco); 1 cada widget com a menor máscara (só o que mudou desde o último envio, ou o visual de um dos 9 anteriores, ou tudo o que não é zero), guardando depois o retrato; 2 o buffer de pacotes das áreas como está; 3 um pacote por área com linha de rolagem pendente e o fim; devolve os bytes mandados, 0 se um envio falhar (com a caixa "Network Error")',
          'builds and sends one section: 0 the header (the background, the colour format, the counts, the focus); 1 each widget with the smallest mask (only what changed since the last send, or the look of one of the 9 before it, or everything that is not zero), then saving the snapshot; 2 the area packet buffer as it is; 3 one packet per area with a pending scroll row and the end; returns the bytes sent, 0 when a send fails (with the "Network Error" box)')])
