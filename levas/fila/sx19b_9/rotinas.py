# A sessao de edicao das buscas dos olheiros, segunda parte (search_edit_session.cpp; leitor sx19b, rodada 5,
# 24/11/2026). Rotinas inteiras: 0x804810..0x8048d5 (ret), 0x8048e0..0x80499c (ret 0x14), 0x8049a0..0x804a3b (ret),
# 0x804a40..0x804aa7 (ret 0xc), 0x804ab0..0x804b5b (ret 4), 0x804b60..0x804c0b (ret 4), 0x804c10..0x804ccb (ret 4),
# 0x804cd0..0x804e08 (ret 4) e 0x804e10..0x804efa (ret 8).
# DEPENDE DO sx19b_8 (session_take_undo, session_mark_changed, session_change_recorded, session_current_search,
# search_assign, session_search, as constantes SESSION_* e FN_PLAYER_SEARCH_REMOVE): integrar junto ou depois.
# Quem chama: a tela de busca de olheiros (0x80f616, 0x80f661, 0x80ec4b.., 0x80ee5d, 0x808e47, 0x8100b4, 0x80fdd2,
# 0x80f214, 0x80ff28, 0x8123fd, 0x80f626, 0x8116d6) e a tela de filtros (0x8186df). A carreira automatica nao passa.
# TELAS: o caminho no jogo: Olheiros (a tela de busca de jogadores): trancar uma lista com senha, mandar o olheiro de
# uma lista para uma nacao / regiao / competicao / juniores / proximo adversario e chama-lo de volta, a lista de
# resultados (ordenar pelas colunas), tirar um jogador da lista, limpar a lista, os filtros, carregar e gravar uma
# lista em arquivo (com e sem senha).
# Convencoes conferidas na desmontagem: todas thiscall sobre a busca atual (sessao + 1 + 0x2b0 x atual); 0x7ae7d0
# set_password thiscall(busca, dono) ret 4; 0x7ae780 set_manager_name thiscall(busca, 0) ret 4; 0x7ae9c0
# start_scout_assignment ret 0x14 e 0x7aee80 clear_scout_assignment (ja Rust); 0x7b14a0 clear_results ret 4, 0x7af160
# get_result_list ret 0xc, 0x7b2bf0 load_from_file ret 8 e 0x7b3660 save_to_file ret 8 (sx17_*, pelo endereco); 0x5fcb60
# HUMAN_MANAGER::add_recent_file thiscall([0xb63c98], arquivo, 0, 2) ret 0xc. Os filtros da busca: 0x99 bytes em
# +0x1fa; a contagem guardada em +0x2a4.
# Comparadores: get_current_results lado a lado quando os resultados ja estao frescos (so le); os outros so a original,
# conferindo depois as marcas, o desfazer, os resultados por refazer, os filtros e a lista vazia.
TELAS = True
ROTINAS = [('lock_current_search', 0x804810, [0x51, 0x56, 0x8b, 0xf1, 0x8a, 0x16]),
           ('set_search_scope', 0x8048e0, [0x51, 0x53, 0x55, 0x56, 0x8b, 0xf1]),
           ('reset_scout_fields', 0x8049a0, [0x51, 0x53, 0x55, 0x56, 0x8b, 0xf1]),
           ('get_current_results', 0x804a40, [0x56, 0x8b, 0xf1, 0x8b, 0x4c, 0x24, 0x08]),
           ('remove_player_from_current_search', 0x804ab0, [0x51, 0x53, 0x55, 0x56, 0x8b, 0xf1]),
           ('clear_current_results', 0x804b60, [0x51, 0x53, 0x55, 0x56, 0x8b, 0xf1]),
           ('set_current_filters', 0x804c10, [0x51, 0x53, 0x8b, 0xd9, 0x55]),
           ('load_search_file', 0x804cd0, [0x83, 0xec, 0x08, 0x53, 0x55]),
           ('save_current_search', 0x804e10, [0x53, 0x56, 0x8b, 0xf1, 0x57])]
DOCS = ('a sessão de edição das buscas dos olheiros (2ª parte)',
        'the scout search edit session (part 2)',
        [('lock_current_search', 'SearchEditSession::lock_current_search', '0x804810', 'search_edit_session.cpp', 'thiscall',
          'sem senha na busca atual: guarda o desfazer, põe a senha do técnico e marca a busca e a sessão como mudadas; devolve 1',
          'without a password on the current search: takes the undo, sets the manager\'s password and marks the search and the session changed; returns 1'),
         ('set_search_scope', 'SearchEditSession::set_search_scope', '0x8048e0', 'search_edit_session.cpp', 'thiscall, ret 0x14',
          'guarda o desfazer, manda o olheiro da busca atual para o alvo (nação, região, competição, juniores ou próximo adversário) e marca como mudada; devolve 1',
          'takes the undo, sends the current search\'s scout to the target (nation, region, competition, youth or next opponent) and marks it changed; returns 1'),
         ('reset_scout_fields', 'SearchEditSession::reset_scout_fields', '0x8049a0', 'search_edit_session.cpp', 'thiscall',
          'guarda o desfazer, chama de volta o olheiro da busca atual e marca como mudada; devolve 1',
          'takes the undo, recalls the current search\'s scout and marks it changed; returns 1'),
         ('get_current_results', 'SearchEditSession::get_current_results', '0x804a40', 'search_edit_session.cpp', 'thiscall, ret 0xc',
          'entrega a lista de resultados da sessão; se os resultados não estão frescos, a busca atual a enche na ordem pedida e eles ficam frescos; devolve quantos resultados ela viu e a marca de alguém fora do próprio elenco',
          'hands out the session\'s result list; when the results are not fresh, the current search fills it in the asked order and they become fresh; returns how many results it saw and the someone-outside-the-own-squad flag'),
         ('remove_player_from_current_search', 'SearchEditSession::remove_player_from_current_search', '0x804ab0', 'search_edit_session.cpp', 'thiscall, ret 4',
          'guarda o desfazer, tira o jogador dos resultados da busca atual, marca como mudada e os resultados por refazer; devolve 1',
          'takes the undo, removes the player from the current search\'s results, marks it changed and the results to redo; returns 1'),
         ('clear_current_results', 'SearchEditSession::clear_current_results', '0x804b60', 'search_edit_session.cpp', 'thiscall, ret 4',
          'guarda o desfazer, esvazia os resultados da busca atual, marca como mudada e os resultados por refazer; devolve 1',
          'takes the undo, empties the current search\'s results, marks it changed and the results to redo; returns 1'),
         ('set_current_filters', 'SearchEditSession::set_current_filters', '0x804c10', 'search_edit_session.cpp', 'thiscall, ret 4',
          'guarda o desfazer, copia os 0x99 bytes de filtros para a busca atual, zera a contagem guardada dela, marca como mudada e os resultados por refazer',
          'takes the undo, copies the 0x99 filter bytes into the current search, clears its cached count, marks it changed and the results to redo'),
         ('load_search_file', 'SearchEditSession::load_search_file', '0x804cd0', 'search_edit_session.cpp', 'thiscall, ret 4',
          'guarda o desfazer e carrega o arquivo na busca atual: conseguindo, marca como mudada, resultados por refazer, o arquivo nos recentes e 1; falhando, volta tudo do desfazer (sem desfazer depois) e 0',
          'takes the undo and loads the file into the current search: on success, marks it changed, results to redo, the file in the recent files and 1; on failure, everything back from the undo (no undo afterwards) and 0'),
         ('save_current_search', 'SearchEditSession::save_current_search', '0x804e10', 'search_edit_session.cpp', 'thiscall, ret 8',
          'grava a busca atual no arquivo com o nome do técnico e, com dono, a senha dele só durante a gravação; gravando, sem desfazer, o arquivo nos recentes e 1; senão 0',
          'saves the current search to the file with the manager\'s name and, with an owner, his password only while saving; saved, no undo, the file in the recent files and 1; otherwise 0')])
CAMPOS = [('busca de jogadores +0x1fa: os filtros (0x99 bytes); +0x2a4: a contagem guardada dos resultados filtrados',
           'player search +0x1fa: the filters (0x99 bytes); +0x2a4: the cached count of filtered results')]
INTERESSA = []
