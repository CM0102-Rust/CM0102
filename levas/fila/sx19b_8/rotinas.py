# A sessao de edicao das buscas dos olheiros, primeira parte (search_edit_session.cpp; leitor sx19b, rodada 5,
# 24/11/2026). Rotinas inteiras: 0x8041c0..0x8042b8 (ret 4), 0x8042e0..0x8043b4 (ret), 0x8043c0..0x804440 (ret),
# 0x804450..0x80449f (ret), 0x8044a0..0x80453b (ret 4), 0x804540..0x804585 (ret 4), 0x804590..0x804665 (ret 4),
# 0x804670..0x804745 (ret 4) e 0x804750..0x80480b (ret).
# Os outros itens usam as pecas deste (session_take_undo, session_mark_changed, session_change_recorded, as constantes
# SESSION_*): o sx19b_9 integra junto ou depois.
# Fora da lista: 0x8042c0 SearchEditSession::unwind_destroy_working_criteria (push 0x7ae4f0; push 0xa; inc ecx; push
# 0x2b0; push ecx; call vector_destructor_iterator; ret): o funclet de desenrolar do construtor, chamado so pelos
# manipuladores de excecao (jmp de 0x961123, 0x961131, 0x961233, 0x961241) com o ecx do quadro; sem excecao C++ nunca
# roda e nao ha comparador possivel. Fica original.
# Quem chama: a tela de busca de olheiros (0x8085a0 o construtor; 0x80fda4, 0x811ef7, 0x812dda, 0x8131d5, 0x81056c,
# 0x80f60f...), o gerente de olheiros (0x7f7403 sync_scout_lists; 0x7f7585, 0x7f76a8, 0x7f7ad8 a lista curta) e a
# ficha (0x80d7c3). A carreira automatica nao passa.
# TELAS: o caminho no jogo: Olheiros (a tela de busca de jogadores): abrir, trocar de lista, desfazer a lista atual,
# destrancar uma lista com senha, gravar e sair (as 10 listas voltam ao gerente); na ficha de um jogador com a tela de
# busca aberta, pôr e tirar da lista curta.
# Convencoes conferidas na desmontagem: todas thiscall; 0x8041c0 (sessao, lista) ret 4 -> sessao (quadro SEH para os
# vetores), 0x9463af vector_constructor_iterator(+1 e +0x1b0e, 0x2b0, 10, 0x7ae390, 0x7ae4f0), 0x68d020 LLIST::LLIST em
# +0x361e, 0x7f6e30 get_selected_list thiscall([0xb74448], 0) (ja Rust); 0x7f6cd0 scout_record thiscall([0xb74448], k,
# 0) (ja Rust); 0x7b7690 PLAYER_SEARCH::operator= thiscall(destino, origem) ret 4 (sx17_3, pelo endereco); 0x7f6f20
# set_selected_list thiscall([0xb74448], k, 0) ret 8 (ja Rust); 0x7ae950 get_scout e 0x7ae8a0 set_scout (ja Rust);
# 0x7af600 contains_player (ja Rust); 0x7af720 add_player / 0x7b1260 remove_player thiscall(busca, pessoa) ret 4 (sx17_2,
# pelo endereco). A senha da busca: +0x139 (o primeiro byte).
# Comparadores: is_player_on_short_list lado a lado; refresh_scout_assignments roda a copia de novo sobre a sessao e
# confere os 20 olheiros; os outros so a original, conferindo depois o que garantem (lista atual, marcas, desfazer,
# senha, lista escolhida).
TELAS = True
ROTINAS = [('new_search_edit_session', 0x8041c0, [0x6a, 0xff, 0x68, 0x44, 0x11, 0x96, 0x00]),
           ('revert_current_slot', 0x8042e0, [0x51, 0x53, 0x55, 0x56, 0x8b, 0xf1]),
           ('refresh_scout_assignments', 0x8043c0, [0x51, 0x53, 0x56, 0x32, 0xdb]),
           ('save_all_slots', 0x804450, [0x51, 0x53, 0x56, 0x8b, 0xf1]),
           ('set_current_search', 0x8044a0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('is_player_on_short_list', 0x804540, [0x53, 0x56, 0x8b, 0xf1, 0x32, 0xdb]),
           ('add_player_to_short_list', 0x804590, [0x83, 0xec, 0x0c, 0x32, 0xd2]),
           ('remove_player_from_short_list', 0x804670, [0x83, 0xec, 0x0c, 0x32, 0xd2]),
           ('unlock_current_search', 0x804750, [0x51, 0x56, 0x8b, 0xf1, 0x8a, 0x16])]
DOCS = ('a sessão de edição das buscas dos olheiros (1ª parte)',
        'the scout search edit session (part 1)',
        [('new_search_edit_session', 'SearchEditSession::SearchEditSession', '0x8041c0', 'search_edit_session.cpp', 'thiscall, ret 4',
          'monta a sessão de edição do técnico da vez: os dois blocos de 10 buscas e a lista de resultados; a busca atual é a pedida (0..9) ou a lista escolhida do técnico; as 10 buscas copiadas das do gerente de olheiros, nenhuma mudada, sem desfazer, resultados por refazer',
          'builds the current manager\'s edit session: both blocks of 10 searches and the result list; the current search is the asked one (0..9) or the manager\'s selected list; the 10 searches copied from the scout manager\'s, none changed, no undo, results to redo'),
         ('revert_current_slot', 'SearchEditSession::revert_current_slot', '0x8042e0', 'search_edit_session.cpp', 'thiscall',
          'guarda o desfazer, recarrega a busca atual do gerente de olheiros, desmarca a mudança dela e refaz a marca geral pelas 10; resultados por refazer; devolve 1',
          'takes the undo, reloads the current search from the scout manager, clears its changed mark and recomputes the overall mark from the 10; results to redo; returns 1'),
         ('refresh_scout_assignments', 'SearchEditSession::refresh_scout_assignments', '0x8043c0', 'search_edit_session.cpp', 'thiscall',
          'para cada lista de olheiro (tipo 4) do gerente, o olheiro dela vai para a busca de trabalho e para a do desfazer; devolve 1',
          'for each scout list (type 4) of the manager, its scout goes to the working and the undo search; returns 1'),
         ('save_all_slots', 'SearchEditSession::save_all_slots', '0x804450', 'search_edit_session.cpp', 'thiscall',
          'grava a busca atual como a lista escolhida do técnico e copia as 10 buscas de volta no gerente de olheiros; devolve 1',
          'stores the current search as the manager\'s selected list and copies the 10 searches back into the scout manager; returns 1'),
         ('set_current_search', 'SearchEditSession::set_current_search', '0x8044a0', 'search_edit_session.cpp', 'thiscall, ret 4',
          'escolhe a busca atual (0..9) e marca os resultados por refazer; fora disso, a caixa e 0',
          'selects the current search (0..9) and marks the results to redo; otherwise the box and 0'),
         ('is_player_on_short_list', 'SearchEditSession::is_player_on_short_list', '0x804540', 'search_edit_session.cpp', 'thiscall, ret 4',
          'pergunta à primeira busca do tipo lista curta se o jogador está nela; sem lista curta, 0',
          'asks the first short list search whether the player is on it; no short list, 0'),
         ('add_player_to_short_list', 'SearchEditSession::add_player_to_short_list', '0x804590', 'search_edit_session.cpp', 'thiscall, ret 4',
          'na primeira busca do tipo lista curta: guarda o desfazer, põe o jogador, marca a busca e a sessão como mudadas e os resultados por refazer; devolve 1 (sem lista curta, 0)',
          'on the first short list search: takes the undo, adds the player, marks the search and the session changed and the results to redo; returns 1 (no short list, 0)'),
         ('remove_player_from_short_list', 'SearchEditSession::remove_player_from_short_list', '0x804670', 'search_edit_session.cpp', 'thiscall, ret 4',
          'o mesmo, tirando o jogador da lista curta',
          'the same, taking the player off the short list'),
         ('unlock_current_search', 'SearchEditSession::unlock_current_search', '0x804750', 'search_edit_session.cpp', 'thiscall',
          'com senha na busca atual: guarda o desfazer, apaga a senha e marca a busca e a sessão como mudadas; devolve 1',
          'with a password on the current search: takes the undo, clears the password and marks the search and the session changed; returns 1')])
CAMPOS = [('sessão de edição das buscas (0x3638 bytes): +0 a busca atual; +1 as 10 buscas de trabalho (0x2b0); +0x1ae1 as marcas de mudança (10 dwords); +0x1b09 a marca geral; +0x1b0d o bloco do desfazer (atual, buscas +0x1b0e, marcas +0x35ee, marca geral +0x3616); +0x361a se há desfazer; +0x361e a lista de resultados (LLIST); +0x362c resultados vistos; +0x3630 a marca de fora do próprio elenco; +0x3634 resultados frescos',
           'search edit session (0x3638 bytes): +0 the current search; +1 the 10 working searches (0x2b0); +0x1ae1 the changed marks (10 dwords); +0x1b09 the overall mark; +0x1b0d the undo block (current, searches +0x1b0e, marks +0x35ee, overall +0x3616); +0x361a whether an undo is held; +0x361e the result list (LLIST); +0x362c results seen; +0x3630 the outside-own-squad flag; +0x3634 results fresh'),
          ('busca de jogadores +0x139: a senha da lista (texto; o primeiro byte 0 = sem senha)',
           'player search +0x139: the list password (text; first byte 0 = no password)')]
INTERESSA = []
