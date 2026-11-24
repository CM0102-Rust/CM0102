# O gerente de olheiros para as telas: a lista curta, os jogadores de comparacao, o clube da busca e as competicoes
# de selecao do momento (scout_manager.cpp; leitor sx19b, rodada 5, 24/11/2026). Rotinas inteiras: 0x7f75f0..0x7f7777
# (ret 8), 0x7f7780..0x7f7917 (ret 0xc), 0x7f7920..0x7f7a1a (ret 4), 0x7f7a20..0x7f7ba7 (ret 8), 0x7f82f0..0x7f8413
# (ret 4) e 0x7f7010..0x7f7267 (ret 4); o resto ate a fronteira e enchimento.
# Quem chama: as telas do jogador (lista curta: 0x8898e5, 0x88b0a8, 0x8be9ad, 0x8f49a9, 0x8c9d11, 0x8f4c24), a
# comparacao de jogadores (0x8f45c3; get_compare_player em 0x87065e, 0x8765af, 0x879071, 0x888f20, 0x88c520...), o
# comando de olheiro da tela do clube (0x482d72) e a tela de busca (0x80925b). A carreira automatica nao passa.
# TELAS: o caminho no jogo: na ficha de um jogador, "Adicionar a lista curta" / "Tirar da lista curta" (com e sem a
# tela de busca aberta antes, para passar pela sessao de edicao); "Comparar" dois jogadores (jogador 1 e 2, e trocar);
# na tela de um clube, mandar o olheiro da busca atual observar o clube; na tela de busca de olheiros, a lista das
# competicoes de selecao em andamento.
# Convencoes conferidas na desmontagem:
# - 0x7f75f0 / 0x7f7a20 thiscall(gerente, pessoa, tecnico) ret 8 -> 1/0; 0x804590 / 0x804670 SearchEditSession
#   thiscall(sessao, pessoa) ret 4 (sx19b_8, pelo endereco); 0x7f6cd0 scout_record thiscall(gerente, k, tecnico) ret 8
#   (ja Rust; o k num dword da pilha com so o byte baixo escrito); 0x7af720 / 0x7b1260 PLAYER_SEARCH::add_player /
#   remove_player thiscall(busca, pessoa) ret 4 (sx17_2, pelo endereco);
# - 0x7f7780 thiscall(gerente, pessoa, tecnico, qual) ret 0xc -> 1/0; 0x7f7920 thiscall(gerente, qual) ret 4 -> pessoa;
# - 0x7f82f0 thiscall(gerente, &clube) ret 4 (o eax nao e usado pelo chamador); 0x7b3d50 PLAYER_SEARCH::set_assigned_club
#   thiscall(busca, clube) ret 4 (sx17_3, pelo endereco);
# - 0x7f7010 thiscall(gerente, lista) ret 4 (o ecx nao e lido); quadro SEH so para as datas locais; 0x68d080
#   LLIST::clear; 0x775580 NationalTeams::collect_duty_competitions thiscall([0xb74318], lista) ret 4 (ja Rust);
#   0x68d1b0 get_next; 0x68d1f0 remove; 0x48d500 Comp::build_match_date_list thiscall(comp, &total) -> vetor de 20 bytes
#   (a data em +0xc) liberado pelo 0x9452ca; 0x549ef0 Date::Date(dia, ano); 0x417c10 operator=; 0x54c8e0
#   Date::sub_days thiscall(data, saida, 7) ret 8.
# Comparadores: get_compare_player lado a lado; set_compare_player planeja o par e confere depois da original;
# add/remove so a original, conferindo depois a lista curta (is_player_shortlisted); scope so a original;
# collect_current_duty_comps lado a lado (a copia numa lista local, liberada depois), os ids em ordem.
TELAS = True
ROTINAS = [('add_to_shortlist', 0x7f75f0, [0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('set_compare_player', 0x7f7780, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('get_compare_player', 0x7f7920, [0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00]),
           ('remove_from_shortlist', 0x7f7a20, [0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('scope_current_search_to_club', 0x7f82f0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('collect_current_duty_comps', 0x7f7010, [0x6a, 0xff, 0x68, 0x57, 0x0f, 0x96, 0x00])]
DOCS = ('o gerente de olheiros para as telas: lista curta, comparação, clube da busca e competições de seleção',
        'the scout manager for the screens: shortlist, comparison, the search club and duty competitions',
        [('add_to_shortlist', 'SCOUT_MANAGER::add_to_shortlist', '0x7f75f0', 'scout_manager.cpp', 'thiscall, ret 8',
          'põe o jogador na lista curta do técnico humano (sem técnico, o da vez): com a sessão de edição aberta, pela sessão; senão na primeira das 10 buscas dele do tipo 1 (devolve 1; nenhuma, 0). Sem pessoa, sem técnico ou vaga fora de 0..15: a caixa e 0',
          'puts the player on the human manager\'s shortlist (no manager: the current one): with the edit session open, through it; otherwise on the first of his 10 searches of type 1 (returns 1; none, 0). No person, no manager or a slot outside 0..15: the box and 0'),
         ('set_compare_player', 'SCOUT_MANAGER::set_compare_player', '0x7f7780', 'scout_manager.cpp', 'thiscall, ret 0xc',
          'guarda o jogador como o de comparação 1 ou 2 do técnico humano; se ele já ocupa o outro lugar, os dois trocam; devolve 1',
          'stores the player as compare player 1 or 2 of the human manager; when he already holds the other place, the two swap; returns 1'),
         ('get_compare_player', 'SCOUT_MANAGER::get_compare_player', '0x7f7920', 'scout_manager.cpp', 'thiscall, ret 4',
          'devolve o jogador de comparação 1 (ou 2) do técnico humano da vez',
          'returns compare player 1 (or 2) of the current human manager'),
         ('remove_from_shortlist', 'SCOUT_MANAGER::remove_from_shortlist', '0x7f7a20', 'scout_manager.cpp', 'thiscall, ret 8',
          'tira o jogador da lista curta do técnico humano, pela sessão de edição aberta ou pela primeira busca do tipo 1',
          'takes the player off the human manager\'s shortlist, through the open edit session or the first search of type 1'),
         ('scope_current_search_to_club', 'SCOUT_MANAGER::scope_current_search_to_club', '0x7f82f0', 'scout_manager.cpp', 'thiscall, ret 4',
          'com a sessão de edição do técnico da vez aberta, a busca atual sem clube (−2) passa a observar o clube',
          'with the current manager\'s edit session open, the current search without a club (−2) takes the club'),
         ('collect_current_duty_comps', 'SCOUT_MANAGER::collect_current_duty_comps', '0x7f7010', 'scout_manager.cpp', 'thiscall, ret 4',
          'enche a lista com as competições de seleção e tira as de id negativo, sem objeto ou fora do momento: a primeira data de jogo menos 7 dias depois de hoje, ou a última menos 7 dias antes de hoje; devolve 1',
          'fills the list with the national team duty competitions and drops those with a negative id, no object or not current: the first match date less 7 days after today, or the last less 7 days before today; returns 1')])
CAMPOS = []
INTERESSA = []
