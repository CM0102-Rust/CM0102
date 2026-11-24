# main.cpp (leitor sx13, rodada 5, 24/11/2026): o procedimento da janela do jogo e o recomeço.
# - 0x68d630 main_window_proc: stdcall(hwnd, msg, wparam, lparam) ret 0x10 (registrado pelo graphics_init 0x5e2721;
#   a DLL subclassifica a janela em cm0102-base/src/apresentacao.rs e chama esta por CallWindowProcA: convivem).
#   Tabelas de salto conferidas: 0x68da60/0x68da78 (msg - 6, 0..0xfc: 6 e 0x1c ativação, 0xf pintura, 0x10 fechar,
#   0x100 tecla, 0x102 caractere, o resto DefWindowProcA), 0x68db78 (wparam - 0x21: PgUp 3, PgDn 2, End 4, Home 5,
#   setas esquerda/direita nada, cima 1, baixo 0) e 0x68db98 (msg - 0x201: 0x201, 0x202, 0x204, 0x205, 0x20a; o resto
#   DefWindowProcA). ScreenManager::press_scrollbar_key 0x803390 thiscall(0xb74450, tecla, contagem) ret 8 (já em Rust:
#   screen_manager_press_scrollbar_key, chamada direto); Display::draw_screen 0x55f300 thiscall([0xb74c4c], -1 x4);
#   0x5e2ea0/0x5e2e50/0x5e81e0/0x5a8f60 cdecl(); gui_show_message_box 0x5e8290 cdecl(título, texto, 0); langlib_sprintf
#   0x66f4e0 cdecl. A API pela tabela de importação (como o original: a DLL intercepta parte dela).
#   No cm0102-1080.exe só mudam os imediatos do modo de tela cheia (0x68d8d5 e 0x68d8e0: 800x600 -> 1920x1080); a cópia
#   lê os dois do código do exe que roda, e vale nos dois exes.
#   Defeito do original: o DEVMODE da reativação só tem dmSize, dmFields, dmBitsPerPel, dmPelsWidth e dmPelsHeight; o
#   resto (dmDriverExtra inclusive) é lixo da pilha (na cópia, zero).
# - 0x68dbc0 restart_game: cdecl(), não volta: 0x5cc7d0 game_fatal_cleanup, CloseHandle([0xb67a28]),
#   WinExec(GetCommandLineA(), 10), exit(0). Chamada pela barra de menu 0x7615d0 (0x762854, 0x762a94; já em Rust,
#   menubar.rs, pelo exe). Prólogo call rel32 (5 bytes): RELATIVAS (0x68dbc0, 0) no rust/src/detours.rs.
# Tipo: TELAS (a janela). Comparadores: main_window_proc lado a lado nos globais de entrada que a cópia grava
# (store_window_input; na tecla só o código da tecla, porque a tecla de rolagem mexe nos globais do mouse); pintura,
# ativação, a caixa de fechar, a roda e o DefWindowProcA só a original. restart_game só a original (o processo acaba).
TELAS = True
ROTINAS = [('main_window_proc', 0x68d630, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0xd8, 0x01, 0x00, 0x00]),
           ('restart_game', 0x68dbc0, [0xe8, 0x0b, 0xec, 0xf3, 0xff])]
DOCS = ('a janela do jogo: o procedimento das mensagens e o recomeço',
        'the game window: the message procedure and the restart',
        [('main_window_proc', 'main_window_proc', '0x68d630', 'main.cpp', 'stdcall, ret 0x10',
          'ativação: desativada em tela cheia volta o modo de vídeo e minimiza; ativada, com o jogo rodando, restaura as superfícies e redesenha a tela, e em tela cheia repõe o modo (16 bits, a largura e a altura do exe) e mostra a janela; pintura redesenha a tela; fechar mostra a caixa "Please use the Exit Game option"; tecla guarda o código e PgUp, PgDn, End, Home, cima e baixo apertam a tecla de rolagem 3, 2, 4, 5, 1 e 0; caractere guardado; mouse (sem tecla de rolagem segurando) guarda a posição e os botões, com o clique pendente; a roda aperta a tecla 1 (para cima) ou 0 com o número de entalhes (÷ 120) e devolve 1; o resto vai ao DefWindowProcA. Defeito do original: o DEVMODE só tem os campos usados, o resto é lixo da pilha',
          'activation: deactivated in full screen it restores the video mode and minimises; activated, while the game runs, it restores the surfaces and redraws the screen, and in full screen sets the mode again (16 bits, the width and height in the exe) and shows the window; paint redraws the screen; close shows the "Please use the Exit Game option" box; a key is kept and Page Up, Page Down, End, Home, Up and Down press scrollbar key 3, 2, 4, 5, 1 and 0; a character is kept; the mouse (with no scroll key holding it) keeps its position and the buttons, with the pending click; the wheel presses key 1 (up) or 0 with the notch count (÷ 120) and returns 1; the rest goes to DefWindowProcA. Defect of the original: the DEVMODE only holds the fields used, the rest is stack garbage'),
         ('restart_game', 'restart_game', '0x68dbc0', 'main.cpp', 'cdecl',
          'limpa o jogo, fecha o mutex de instância única, roda o programa de novo com a mesma linha de comando e termina este',
          'cleans the game up, closes the single instance mutex, runs the program again with the same command line and ends this one')])
CAMPOS = []
INTERESSA = []
