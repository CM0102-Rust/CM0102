# A base da interface (6): os comandos de impressão do gerente de telas (scrman.cpp; leitor tel38b, rodada das telas,
# 21/11/2026): começar e terminar um trabalho de impressão, as linhas e as quebras de página, imprimir o trabalho e
# imprimir a tela.
# Usa os auxiliares do tel29_1 (screen_record, screen_history, c_string_len, STR_EMPTY), do tel38_4 (scrman_net_put) e
# do tel38_5 (scrman_command_packet, scrman_net_put_text, scrman_send_packet, scrman_check_packet): vem depois dele.
# Convencoes conferidas na desmontagem (0x8033e0..0x803d45), todas thiscall, ecx = o gerente (0xb74450):
# begin_print_job ret 4 (colunas, word); add_print_line ret 4 (texto; nulo -> ""); end_print_job, add_print_page_break,
# print_job e print_screen ret 0. O pacote: o comando (byte; 0x17, 0x18, 0x19, 0x1a, 0x16, 0x15) e o valor; a entrega:
# registro local -> Display::process_command_packet 0x564080 thiscall (Display local +0x3844, sessao, 0, buffer);
# remoto -> Network::send_packet 0x77ac30, falha -> "scrman.cpp 3280/3303/3330/3353/3376/3399". Sem o Display de eco dos
# pacotes de widget.
# Iguais no cm0102-1080.exe (as rotinas inteiras).
# Defeito do original: add_print_line copia o texto para o buffer de rede sem conferir o espaco.
# Comparadores: so a original (o Display imprime ou mexe no trabalho de impressao, ou o pacote vai pela rede),
# conferindo o pacote no buffer quando havia espaco.
TELAS = True
ROTINAS = [('screen_manager_begin_print_job', 0x8033e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('screen_manager_end_print_job', 0x8035b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('screen_manager_add_print_line', 0x803710, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('screen_manager_add_print_page_break', 0x803930, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('screen_manager_print_job', 0x803a90, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('screen_manager_print_screen', 0x803bf0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a base da interface (6): os comandos de impressão: começar e terminar o trabalho, as linhas, as quebras de página, imprimir o trabalho e a tela',
        'the interface base (6): the print commands: starting and ending the job, its lines, page breaks, printing the job and the screen',
        [('screen_manager_begin_print_job', 'ScreenManager::begin_print_job', '0x8033e0', 'scrman.cpp', 'thiscall, ret 4',
          'começa um trabalho de impressão com as colunas dadas: o comando 0x17 aplicado ao Display local ou mandado ao jogador remoto',
          'starts a print job with the given columns: command 0x17 applied to the local Display or sent to the remote player'),
         ('screen_manager_end_print_job', 'ScreenManager::end_print_job', '0x8035b0', 'scrman.cpp', 'thiscall',
          'termina o trabalho de impressão (o Display libera as linhas): o comando 0x18',
          'ends the print job (the Display frees its lines): command 0x18'),
         ('screen_manager_add_print_line', 'ScreenManager::add_print_line', '0x803710', 'scrman.cpp', 'thiscall, ret 4',
          'acrescenta uma linha ao trabalho de impressão (vazia para o ponteiro nulo): o comando 0x19 com o texto',
          'adds a line to the print job (an empty one for a null pointer): command 0x19 with the text'),
         ('screen_manager_add_print_page_break', 'ScreenManager::add_print_page_break', '0x803930', 'scrman.cpp', 'thiscall',
          'acrescenta uma quebra de página (o Display põe uma linha nula): o comando 0x1a',
          'adds a page break (the Display adds a null line): command 0x1a'),
         ('screen_manager_print_job', 'ScreenManager::print_job', '0x803a90', 'scrman.cpp', 'thiscall',
          'imprime as linhas do trabalho de impressão: o comando 0x16',
          "prints the print job's lines: command 0x16"),
         ('screen_manager_print_screen', 'ScreenManager::print_screen', '0x803bf0', 'scrman.cpp', 'thiscall',
          'imprime a própria tela (o Display chama a impressão sem linhas): o comando 0x15',
          'prints the screen itself (the Display calls the print routine with no lines): command 0x15')])
