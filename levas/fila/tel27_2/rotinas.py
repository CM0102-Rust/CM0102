# The file screens (file_screens.cpp; reader tel27, the screens round, 21/11/2026): the file box (opening, building,
# its clicks), the save game info screen (building, its click) and the confirmation box (opening with a message or a
# Text, building, its answer). build_save_game_summary (0x591f80) of the same file is already Rust (file_screens.rs).
# Conventions checked in the disassembly (every push against esp and the ret of each routine):
# - 0x58f500 open_file_dialog cdecl(14: title, subtitle, mode, directory, extension, name, filter, colour 1, colour 2,
#   command, directories only, fixed directory, no cancel, info); 0x58f7c0 file_dialog_build cdecl(); 0x58ff80
#   file_dialog_input cdecl(short widget pushed as a dword) -> eax (jump table 0x590b7c + byte table 0x590b88 for the
#   keys inside the region); 0x590c00 create_save_file_summary_screen cdecl() (SEH for the local Text, destroyed on
#   every exit; _chkstk 0x11a4); 0x5917f0 save_game_info_input cdecl(widget) -> eax; 0x591840 confirm_push cdecl(7);
#   0x5919b0 confirm_push_text_object cdecl(7) (SEH for the new Text); 0x591b70 confirm_build cdecl(); 0x591dc0
#   confirm_input cdecl(widget) -> eax (jump table 0x591f00 + byte table 0x591f0c).
# - Called: 0x945bd2 _mkdir cdecl; 0x90bf90 utils_is_directory cdecl; 0x946c40 localtime cdecl; 0x54d3b0
#   date_get_month_name and 0x54d170 date_get_weekday_name cdecl; 0x4ae5c0 Text::Text(copy) thiscall ret 4; 0x4aeaf0
#   Text::draw_wrapped thiscall ret 0x20; 0x5ed200 gui_add_text_area_from_text cdecl(10); 0x8ac600 draw_pitch cdecl(11);
#   the filter callback cdecl(path) -> flags; the version string 0x90d130 with a fourth argument (the message) for the
#   "v%s %s.%s %d (%s)" load error boxes.
# Uses: tel26_1, tel01_1, tel29_1, tel22_1 (as tel27_1) and tel27_1 (screen_nodes_built_by, screen_newly_opened).
# The 1080 exe patches the save game info screen's full-screen panel (0x31f/0x257 at 0x590f2b/0x590f23): read from the
#   running exe (exe_immediate).
# Defects of the original: file_dialog_input's Info closes general.dat twice when the save is too old (zclose of the
#   cleared handle); confirm_push_text_object leaks the Text copy when the screen does not open; confirm_build with a
#   yes/no value other than 0/1 puts the buttons at (0, 0)-(3, 1).
TELAS = True
ROTINAS = [('open_file_dialog', 0x58f500, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('file_dialog_build', 0x58f7c0, [0x81, 0xec, 0x40, 0x02, 0x00, 0x00]),
           ('file_dialog_input', 0x58ff80, [0xa1, 0x84, 0x7c, 0xb7, 0x00]),
           ('create_save_file_summary_screen', 0x590c00, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('save_game_info_input', 0x5917f0, [0x66, 0x8b, 0x4c, 0x24, 0x04]),
           ('confirm_push', 0x591840, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('confirm_push_text_object', 0x5919b0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('confirm_build', 0x591b70, [0x83, 0xec, 0x10, 0x53, 0x55]),
           ('confirm_input', 0x591dc0, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c])]
DOCS = ('as telas de arquivo: a caixa de arquivo (abrir, montar, cliques), a tela de informações do jogo salvo e a caixa de confirmação',
        'the file screens: the file box (opening, building, clicks), the save game info screen and the confirmation box',
        [('open_file_dialog', 'open_file_dialog', '0x58f500', 'file_screens.cpp', 'cdecl',
          'abre a caixa de arquivo: copia o diretório aceito, o nome do arquivo (o dado, só ao salvar) e um resultado vazio, e grava os 19 parâmetros (título, subtítulo, modo, diretório, extensão, filtro, cores, comando, só diretórios, diretório fixo, sem cancelar, botão de informações)',
          'opens the file box: copies the accepted directory, the file name (the given one, only when saving) and an empty result, and writes the 19 parameters (title, subtitle, mode, directory, extension, filter, colours, command, directories only, fixed directory, no cancel, info button)'),
         ('file_dialog_build', 'file_dialog_build', '0x58f7c0', 'file_screens.cpp', 'cdecl',
          'monta a caixa de arquivo: o diretório (editável salvo se fixo), o nome do arquivo, a lista dos arquivos que o filtro deixa (ou dos subdiretórios), sem a extensão, em seis linhas visíveis, e Cancelar, Informações e Ok',
          'builds the file box: the directory (editable unless fixed), the file name, the list of the files the filter keeps (or the subdirectories), without the extension, in six visible rows, and Cancel, Info and Ok'),
         ('file_dialog_input', 'file_dialog_input', '0x58ff80', 'file_screens.cpp', 'cdecl',
          'trata os cliques da caixa de arquivo: o diretório digitado (aceito se existe; ao salvar, pergunta se cria), um arquivo da lista, Cancelar, Informações (a tela do jogo salvo, se a versão serve) e Ok conforme o modo (nome em branco, arquivo inexistente para carregar ou apagar, pergunta se substitui ou apaga); as respostas das perguntas voltam na montagem seguinte; Enter/S é Ok e Esc/N é Cancelar',
          "handles the file box's clicks: the typed directory (accepted when it exists; when saving, asks whether to create it), a file of the list, Cancel, Info (the save game screen, when its version is good) and Ok by the mode (blank name, missing file to load or delete, the question whether to replace or delete); the questions' answers come back on the next build; Enter/Y is Ok and Esc/N Cancel"),
         ('create_save_file_summary_screen', 'create_save_file_summary_screen', '0x590c00', 'file_screens.cpp', 'cdecl',
          'monta a tela de informações do jogo salvo pelo resumo do general.dat: nome do arquivo, quando foi salvo, versão, data do jogo, tempo de jogo (com o "grau de vício" em inglês), as ligas em primeiro e segundo plano e os treinadores humanos; erro de leitura mostra a caixa de jogo corrompido',
          "builds the save game info screen from general.dat's summary: file name, when it was saved, version, game date, play time (with the \"addictedness rating\" in English), the foreground and background leagues and the human managers; a read error shows the corrupted save box"),
         ('save_game_info_input', 'save_game_info_input', '0x5917f0', 'file_screens.cpp', 'cdecl',
          'o clique da tela do jogo salvo: Ok (1) fecha (−9); o resto, 0',
          "the save game info screen's click: Ok (1) closes it (−9); the rest 0"),
         ('confirm_push', 'confirm_push', '0x591840', 'file_screens.cpp', 'cdecl',
          'abre a caixa de confirmação com título, cabeçalho e mensagem (copiados), sim/não, as cores e o comando da resposta',
          'opens the confirmation box with a title, a heading and a message (copied), yes/no, the colours and the answer\'s command'),
         ('confirm_push_text_object', 'confirm_push_text_object', '0x5919b0', 'file_screens.cpp', 'cdecl',
          'abre a caixa de confirmação com uma cópia de um Text (com ligações) no lugar da mensagem',
          'opens the confirmation box with a copy of a Text (with links) instead of the message'),
         ('confirm_build', 'confirm_build', '0x591b70', 'file_screens.cpp', 'cdecl',
          'monta a caixa de confirmação: a mensagem ou o Text e "Ok", ou "Não" e "Sim"',
          'builds the confirmation box: the message or the Text and "Ok", or "No" and "Yes"'),
         ('confirm_input', 'confirm_input', '0x591dc0', 'file_screens.cpp', 'cdecl',
          'a resposta da caixa de confirmação (Não 1, Sim 2, Ok 3; Enter/S e Esc/N): libera o Text, grava o comando pendente com a resposta e fecha (−9)',
          "the confirmation box's answer (No 1, Yes 2, Ok 3; Enter/Y and Esc/N): frees the Text, sets the pending command with the answer and closes (−9)")])
