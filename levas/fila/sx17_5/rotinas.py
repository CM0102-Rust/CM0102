# Encher a lista de resultados da busca de jogadores (player_search.cpp; leitor sx17, rodada 5, 24/11/2026):
# get_result_list (0x7af160), collect_known_people (0x7afa70) e add_own_squad (0x7b0e40). Rotinas inteiras
# (0x7af160..0x7af574 ret 0xc; 0x7afa70..0x7b0049 ret; 0x7b0e40..0x7b11a8 ret, o fim e a saida fatal da realocacao; o
# resto e enchimento).
# DEPENDE DO sx17_2 (integrar junto ou depois): usa as rotinas da lista de resultados de la (search_result_copy,
# add_player, add_result, remove_own_squad, a ordem por pessoa) e o comparador player_search_compare_results.
# Quem chama: a tela de busca (0x804a7f: get_result_list, o que a tela lista), e o proprio get_result_list
# (collect_known_people nas buscas rapida e de comissao; add_own_squad sempre). A carreira automatica nao passa.
# TELAS: a prova e o Lucas jogando comparando: a tela de busca de jogadores (busca rapida, busca de comissao, a lista
# de um olheiro com clube observado), ordenando pelas colunas de situacao e de nota.
# Convencoes conferidas na desmontagem:
# - 0x7af160 thiscall(busca, LLIST, &fora_do_elenco, ordem) ret 0xc -> quantos resultados foram vistos; 0x7afa70 e
#   0x7b0e40 thiscall(busca) ret -> 1/0;
# - 0x68d080 LLIST::clear thiscall; 0x68d0d0 LLIST::add_tail thiscall(lista, resultado, 0x2d) ret 8; 0x4a7f20
#   Date::operator== thiscall(data, dia/ano, bissexto) ret 8; 0x807450 SearchFilters::player_matches_filter
#   thiscall(filtros, resultado) ret 4; 0x4849f0 get_player_status_code cdecl(pessoa, 0, 0); 0x795750 Notes::find_note;
#   0x4cf5a0 get_contract thiscall([0xae1050], pessoa) ret 4 (o resultado vai para [0xae1080]); 0x5407e0
#   database_get_staff_reputation_for_viewer cdecl(pessoa, tecnico, 0/1) -> ax; 0x7f8d70 has_scout_request
#   thiscall([0xb74448], pessoa, clube, 0) ret 0xc; 0x544520 / 0x544590 cdecl(pessoa, 0); 0x76d9b0
#   national_teams_player_is_eligible stdcall(pessoa, tecnico) ret 8 (o ecx = [0xb74318] nao e lido); 0x7f6be0
#   get_edit_session thiscall([0xb74448], tecnico) ret 4 (10 buscas de 0x2b0 bytes a partir de +1); 0x540a50
#   club_get_linked_b_team cdecl(clube, 0, 1); 0x541f90 database_collect_club_squad cdecl(clube, vetor de 50, 0);
#   0x542740 database_club_has_reserve_squad cdecl(clube); 0x946464 bsearch, 0x945333 qsort, 0x945667 realloc.
# Defeitos do original: add_own_squad e collect_known_people nao iniciam o byte +0x2b dos resultados (vai o lixo da
# pilha para a lista; a copia escreve 0 e os comparadores o ignoram); collect_known_people pede o contrato de novo e
# descarta o resultado.
TELAS = True
ROTINAS = [('player_search_get_result_list', 0x7af160, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00]),
           ('player_search_collect_known_people', 0x7afa70, [0x6a, 0xff, 0x68, 0xc6, 0xf3, 0x95, 0x00]),
           ('player_search_add_own_squad', 0x7b0e40, [0x6a, 0xff, 0x68, 0x36, 0xf4, 0x95, 0x00])]
DOCS = ('encher a lista de resultados da busca de jogadores',
        'filling the player search result list',
        [('player_search_get_result_list', 'PLAYER_SEARCH::get_result_list', '0x7af160', 'player_search.cpp', 'thiscall, ret 0xc',
          'atualiza os resultados e os entrega a tela: sem lista, a caixa (0x29a) e 0; esvazia a lista; nas buscas rapida e de comissao, recolhe as pessoas conhecidas de novo se estiver vazia ou o primeiro resultado nao for de hoje; com clube observado, tira do registro quem saiu do clube e esvazia os resultados; poe o proprio elenco e, com o registro, cada jogador dele; com resultados: tira os invalidos, copia os que passam nos filtros (marca se algum nao e do proprio elenco), prepara e ordena pela ordem pedida (situacao em +0x2b, nota em +0x2c) e poe cada um na lista; no fim tira o proprio elenco; devolve quantos resultados viu',
          'refreshes the results and hands them to the screen: without a list, the box (0x29a) and 0; empties the list; in quick and staff searches, collects the known people again when empty or when the first result is not from today; with a watched club, drops from the record those who left the club and empties the results; adds the own squad and, with the record, each of its players; with results: drops the invalid, copies those passing the filters (flags when one is not of the own squad), prepares and sorts by the asked order (status at +0x2b, note at +0x2c) and adds each to the list; at the end removes the own squad; returns how many results it looked at'),
         ('player_search_collect_known_people', 'PLAYER_SEARCH::collect_known_people', '0x7afa70', 'player_search.cpp', 'thiscall',
          'refaz os resultados com as pessoas que o tecnico conhece: sem tecnico (0x3eb) ou sem nacao de onde olha (a do clube, a da selecao ou a propria; 0x3fd), a caixa e 0; esvazia; cada pessoa fora do clube do tecnico e ainda nao listada entra: na busca de comissao todo nao-jogador; nas outras o jogador com reputacao vista de 4000 ou mais, pedido por um olheiro do clube, ou do mesmo continente e na lista de transferencias ou de emprestimos ou elegivel para a selecao do tecnico; a nota e a reputacao vista do nao-jogador; ordena; poe os resultados das outras listas do tecnico (fora as rapidas e de comissao) que nao sao do clube dele e ordena; devolve 1',
          'rebuilds the results with the people the manager knows: without a manager (0x3eb) or a nation to look from (his club one, his national side one or his own; 0x3fd), the box and 0; empties; each person outside the manager club and not listed yet goes in: in a staff search every non-player; in the others a player with a seen reputation of 4000 or more, asked for by a club scout, or from the same continent and transfer or loan listed or eligible for the manager national side; the rating is the seen reputation of a non-player; sorts; adds the results of the manager other lists (but quick and staff ones) that are not of his club and sorts; returns 1'),
         ('player_search_add_own_squad', 'PLAYER_SEARCH::add_own_squad', '0x7b0e40', 'player_search.cpp', 'thiscall',
          'poe os jogadores do elenco do clube do tecnico como resultados do proprio elenco (bandeira 2), salvo se os filtros tiram o proprio clube (0) ou o tecnico nao tem clube (0); com elenco reserva os filtros podem tirar a reserva ou o time principal, senao, com time B, os contratados pelo B ou pelo clube; quem ainda nao esta listado entra e a lista e ordenada; devolve 1',
          'adds the players of the manager club squad as own squad results (flag 2), unless the filters leave the own club out (0) or the manager has no club (0); with a reserve squad the filters may leave out the reserves or the first team, otherwise, with a B team, those contracted to the B team or to the club; those not listed yet go in and the list is sorted; returns 1')])
