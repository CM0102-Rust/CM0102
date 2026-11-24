# O buffer de rede (o dword e o bloco com tamanho) e a animacao dos widgets (network.h; leitor tel46, rodada 5,
# 23/11/2026). Usa network_buffer_room/cursor/advance e compare_network_buffer (tel37_1/tel40), network_buffer_put_short
# (tel37_1), display_item_at, display_item, guio_fields, display_list, screen_record, c_string_bytes, Tel28FormatFn.
# Convencoes conferidas na desmontagem:
# - put_long 0x5694a0 thiscall(buf, valor) ret 4: espaco para 4 (caixa network.h 136 = 0x88), grava e avanca; os
#   chamadores sao o send_widget_click (0x569163..0x56939a, ja Rust, chama pelo endereco).
# - put_sized_block 0x569540 thiscall(buf, dados, tamanho short) ret 8: o tamanho [esp+8] como word (caixa 134 = 0x86),
#   depois os bytes (movsx do tamanho) sem conferir o espaco, copiados mesmo depois da caixa.
# - animate_widgets 0x569610 thiscall(display): utils_get_time_ms 0x90c510; os relogios 0xae2a28 (1000 ms) e 0xae2a2c
#   (100 ms) com comparacao sem sinal; flag 0x800 gira o texto ("%s%c%s" 0x98b4c8 com o resto, 1, a cabeca;
#   Guio::set_text_and_refresh 0x5eed50 thiscall(item, texto, 0) ret 8; Display::draw_screen 0x55f300
#   thiscall(d, x1, y1, x2, y2) ret 0x10); flag 0x1000 troca as words +0x72 e +0x78 e redesenha se o Display do slot
#   atual (0xb74c4c + 0x300 x slot) tiver LAID_OUT (+0x12f545). Chamada so pelo Display::poll_input (0x55f6ef; o porte
#   tel26_1 a chama pelo endereco).
# Convivencia com o simulation.rs: 0x569610 esta em CLOCK_CALLERS (um relogio por rotina, escolhido pelo endereco de
# retorno). A copia le o relogio da rotina sem avancar (clock_peek): uma consulta da DLL avancaria o relogio do
# 0x81b890 (o da espera do dia de jogo; a 318a travou assim). Na simulacao a copia nao anima (so tela).
# Iguais no cm0102-1080.exe (bytes conferidos).
TELAS = True
ROTINAS = [('network_buffer_put_long', 0x5694a0, [0x8b, 0x41, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('network_buffer_put_sized_block', 0x569540, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('display_animate_widgets', 0x569610, [0x81, 0xec, 0x14, 0x01, 0x00, 0x00])]
DOCS = ('o buffer de rede (o dword e o bloco com tamanho) e a animação dos widgets (o letreiro que gira e as cores que piscam)',
        'the network buffer (the dword and the sized block) and the widget animation (the turning marquee and the blinking colours)',
        [('network_buffer_put_long', 'NetworkBuffer::put_long', '0x5694a0', 'network.h', 'thiscall, ret 4',
          'grava um dword na posição do buffer e avança 4; sem espaço, a caixa de erro do network.h linha 136',
          'writes a dword at the buffer position and advances 4; with no room, the network.h line 136 error box'),
         ('network_buffer_put_sized_block', 'NetworkBuffer::put_sized_block', '0x569540', 'network.h', 'thiscall, ret 8',
          'grava o tamanho (word, caixa da linha 134 sem espaço) e depois os bytes, sem conferir o espaço (defeito do original: copia mesmo depois da caixa)',
          'writes the length (a word, the line 134 box with no room) and then the bytes, with no room check (defect of the original: it copies even after the box)'),
         ('display_animate_widgets', 'Display::animate_widgets', '0x569610', 'network.h', 'thiscall',
          'a cada segundo os widgets com a marca 0x800 giram o texto ("cabeça\\x01resto" vira "resto\\x01cabeça") e são redesenhados; a cada 100 ms os com 0x1000 trocam a cor e a cor do texto e são redesenhados se o Display do slot atual estiver montado',
          'every second the widgets with flag 0x800 turn their text ("head\\x01rest" becomes "rest\\x01head") and are redrawn; every 100 ms those with 0x1000 swap their colour and text colour and are redrawn when the current slot\'s Display is laid out')])
