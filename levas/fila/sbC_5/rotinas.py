# A caixa de mensagem do jogo (goldcup.cpp pelo mapa; a biblioteca grafica; leitor sbC, rodada 5, 24/11/2026, a rotina
# que o cx07 deixou em levas/fila/cx07_14/FORA.md). Item de TELAS, a parte (pedido do Lucas): a rotina ja tem o desvio
# so de observacao do rust/src/detours.rs (install_box: cada caixa vira a linha "msgbox ..." do ddraw.log). Como os dois
# convivem esta no PRONTO.txt.
# Caminho no jogo (com o cm0102.exe): abrir o jogo comparando e fechar pelo X da janela ("Please use the Exit Game
# option", do main_window_proc); OK com o clique e com Enter.
# Convencao conferida: cdecl(titulo, texto, permite_sair) (centenas de chamadores, todos com add esp, 0xc; o retorno so
# e definido no MessageBoxA e na caixa ja aberta, e ninguem o le).
# Chama: MessageBoxA pela tabela de importacao (0x9671d4) antes do jogo rodar ([0xb67a30] = 0) ou com o video
# desligado; font_text_width 0x5e5ad0 (fonte 1), gui_save_screen_image 0x5e3dd0, gui_draw_panel 0x5e5df0,
# gui_draw_text_box 0x5e6d80, langlib_sprintf 0x66f4e0 ("Ok" 0x990f7c, "Quit <%s - ...>" 0x9cda80 com ""),
# graphics_flip_rect 0x5e2eb0, pump_messages 0x68d520, ShowCursor/SetCursor, game_fatal_cleanup 0x5cc7d0,
# shared_noop_ret0_5a8f60, exit 0x945934 (-1), gui_restore_screen_image 0x5e3fd0, free 0x9452ca. O mouse em
# [0xb67a20] (x) e [0xb67a14] (y), o botao esquerdo em [0xb67a34], a tecla em [0xb67a04] (Enter 0xd), a caixa aberta em
# [0xb0be30]. Prologo sub esp, 0x6c; mov ecx, [0xb67a30] (9 bytes, os mesmos ENTRY_BYTES do install_box).
# ALTERADA no cm0102-1080.exe (6 bytes: o teto da largura 600 -> 1080 em 0x5e82f5 e 0x5e8305, a largura da tela 800 ->
# 1920 em 0x5e830a): a copia le os tres imediatos do codigo do exe que roda (vale nos dois), mas a assinatura so deixa
# desviar num exe, entao EXE = original.
# Defeitos do original: (1) o rotulo dos botoes vai para um buffer de pilha de 0x48 bytes sem conferir o tamanho da
# traducao (a copia usa 0x100); (2) o retorno fica indefinido ao fechar a caixa (a copia devolve 0).
# Depende dos cx07_11 (DISPLAY_OFF, MAIN_WINDOW, CURSOR_MODE, cursor_of_mode, user32_show_cursor, user32_set_cursor,
# gui_free_image) e cx07_12 (SCREEN_UPDATED), e do display.rs (exe_immediate, DISPLAY_MOUSE_X/Y, DISPLAY_KEY_CODE).
# Comparador: so a original (espera o clique do jogador).
SAVE = False
TELAS = True
EXE = {0x5e8290: 'original'}
ROTINAS = [('gui_show_message_box', 0x5e8290, [0x83, 0xec, 0x6c, 0x8b, 0x0d, 0x30, 0x7a, 0xb6, 0x00])]
DOCS = ('a caixa de mensagem do jogo',
        'the game\'s message box',
        [('gui_show_message_box', 'gui_show_message_box', '0x5e8290', 'goldcup.cpp', 'cdecl',
          'mostra a caixa de mensagem do próprio jogo (título, texto quebrado em linhas, OK e, se permitido, Sair) centrada na tela e espera o OK (clique solto sobre ele ou Enter); Sair limpa o jogo e termina; antes do jogo rodar ou sem vídeo usa a caixa do Windows; uma segunda caixa enquanto a primeira está aberta volta na hora',
          'shows the game\'s own message box (title, word-wrapped text, OK and, when allowed, Quit) centred on the screen and waits for OK (a click released on it or Enter); Quit cleans up the game and exits; before the game runs or without a display it uses the Windows box; a second box while the first is open returns at once')])
CAMPOS = []
INTERESSA = []
