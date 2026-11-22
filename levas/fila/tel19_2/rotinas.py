# The player and staff search screen (search_screens.cpp, part 2; reader tel19, the screens round, 21/11/2026): its
# answer. Uses the helpers of tel19_1 (finder_*, FINDER_*): it goes after it.
# 0x80eb00 cdecl(short item), ret 0, no SEH, 0x794 bytes of locals; returns -4 (a screen or a box opened), -9 (the
# screen closes), -11 (redraw) or the menu bar's code. 17980 bytes up to its two jump tables (0x81306c with the byte
# table 0x813074, the views of id 0x12; 0x8130b4 with 0x8130bc, the sort columns of id 0x1e); split into named pieces
# (finder_pending, finder_assign, finder_recall, finder_list_command, finder_view_command), the whole routine hooked.
# Calls checked at the call sites: confirm_push 0x591840 cdecl (7: title, heading, text, buttons, the colours of
# params 1 and 2, the pending code); open_file_dialog 0x58f500 cdecl (14: title, heading, mode 0 load / 1 save / 2
# delete, folder, ".pls", default name, the kind's check 0x7bd010/0x7bd0d0/0x7bd190/0x7bd250, the colours, the pending
# code, four zeros); HUMAN_MANAGER::get_folder_path 0x5fc5b0 ret 8 (0, 2); the SearchEditSession methods (0x8042e0
# .. 0x804e10, 0x813140) and PLAYER_SEARCH::set_assigned_club 0x7b3d50 thiscall; the screens they open (0x453d60,
# 0x5a0650, 0x7961b0, 0x8702a0, 0x813210) cdecl; menubar_handle_command / handle_object_link cdecl(short).
# Kept from the original: the "Recall scout .. from finding youth players" box copies the name of the nation the
# assignment holds (-1: a read before the nation table, unused); the pending answers clear the argument before the
# action as the original does; a pick-mode click on the background (item -1) goes on to the other commands.
TELAS = True
ROTINAS = [('search_screens_handle_player_search_input', 0x80eb00, [0x81, 0xec, 0x94, 0x07, 0x00, 0x00])]
DOCS = ('a tela de busca de jogadores e funcionários (2): as respostas',
        'the player and staff search screen (2): its answers',
        [('search_screens_handle_player_search_input', 'search_screens_handle_player_search_input', '0x80eb00', 'search_screens.cpp', 'cdecl',
          'responde à tela de busca: primeiro a resposta de uma caixa ou diálogo de arquivo (designar o olheiro a uma nação, região, competição, juvenis, próximo adversário ou clube; chamá-lo de volta; carregar, salvar ou apagar o arquivo escolhido, com a caixa quando outro humano o travou ou falha; salvar antes de travar; reiniciar; limpar; carregar um arquivo recente; cancelar as mudanças; tirar uma pessoa da lista); depois o clique: no modo de escolha, tirar a pessoa da lista; a nota, o elenco, outra aba, as designações e a volta do olheiro, carregar, salvar, salvar como e para outro humano, apagar, travar e destravar, reiniciar, limpar, os arquivos recentes, as vistas, as colunas de ordenação e as páginas, os filtros, o perfil da pessoa, Cancelar e Ok; o resto vai à barra de menus',
          'answers the search screen: first the answer of a box or file dialog (assign the scout to a nation, region, competition, youth players, next opposition or club; recall him; load, save or delete the chosen file, with the box when another human locked it or it fails; save before locking; reset; clear; load a recent file; cancel the changes; remove a person from the list); then the click: in pick mode, remove the person from the list; the note, the squad, another tab, the scout\'s assignments and recall, load, save, save as and for another human, delete, lock and unlock, reset, clear, the recent files, the views, the sort columns and the pages, the filters, the person\'s profile, Cancel and Ok; the rest goes to the menu bar')])
