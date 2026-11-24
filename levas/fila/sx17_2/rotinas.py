# A lista de resultados da busca de jogadores (player_search.cpp; leitor sx17, rodada 5, 24/11/2026): o destrutor
# (0x7ae4f0), a copia de um resultado (0x7af580), pôr um jogador (0x7af720) ou um resultado (0x7af870), tirar o proprio
# elenco (0x7b11b0) ou um jogador (0x7b1260), esvaziar (0x7b14a0), tirar os invalidos (0x7b1520) e a ordem por pessoa
# (0x7bce50). Rotinas inteiras (0x7ae4f0..0x7ae56b ret; 0x7af580..0x7af5f4 ret 4; 0x7af720..0x7af85e ret 4;
# 0x7af870..0x7afa5f ret 4; 0x7b11b0..0x7b1258 ret; 0x7b1260..0x7b1491 ret 4; 0x7b14a0..0x7b1517 ret 4;
# 0x7b1520..0x7b17d2 ret 4; 0x7bce50..0x7bce72 ret; o resto ate a proxima fronteira e enchimento).
# Quem chama: as sessoes e as telas da busca (search_edit_session 0x804xxx, search_screens), o SCOUT_MANAGER
# (0x7f76f9, 0x7f7b29), o dia de trabalho do olheiro (0x7b40b8, ja em Rust) e a gravacao da busca (sx17_1). A carreira
# automatica (tecnico de ferias, sem olheiros) nao passa por nenhuma: contadores 0 em docs/cobertura_simulacao.txt.
# TELAS: a prova e o Lucas jogando comparando: a tela de busca de jogadores e as listas de olheiros (pôr e tirar
# jogadores, abrir e fechar a tela, carregar uma busca gravada), e um olheiro trabalhando.
# Convencoes conferidas na desmontagem:
# - thiscall: 0x7ae4f0 (ret), 0x7af580 (destino, origem) ret 4 -> destino, 0x7af720 (busca, pessoa) ret 4, 0x7af870
#   (busca, resultado) ret 4, 0x7b11b0 (ret), 0x7b1260 (busca, pessoa) ret 4, 0x7b14a0 (busca, tirar o clube) ret 4,
#   0x7b1520 (busca, religar) ret 4; 0x7bce50 cdecl(a, b);
# - 0x7e4920 fill_staff_name_ref cdecl(pessoa, resultado); 0x795750 Notes::find_note thiscall([0xb7433c], tecnico,
#   pessoa) ret 8; 0x53e990 database_staff_matches_names cdecl(pessoa, nome, sobrenome, apelido, &data); 0x7a7eb0
#   thiscall([0xb7434c], pessoa) ret 4 -> al; 0x946464 bsearch, 0x945333 qsort, 0x945667 realloc, 0x9452ca free,
#   0x944c94 operator delete cdecl; 0x68d1f0 LLIST::remove thiscall(0xb74438, registro) ret 4; 0x417c10 Date::operator=.
# Defeito do original: add_player nao inicia o byte +0x2b do resultado (vai o lixo da pilha para a lista; a copia
# escreve 0 e o comparador o ignora).
TELAS = True
ROTINAS = [('drop_player_search', 0x7ae4f0, [0x6a, 0xff, 0x68, 0x62, 0xf3, 0x95, 0x00]),
           ('search_result_copy', 0x7af580, [0x83, 0xec, 0x08, 0x56, 0x57]),
           ('player_search_add_player', 0x7af720, [0x6a, 0xff, 0x68, 0xa6, 0xf3, 0x95, 0x00]),
           ('player_search_add_result', 0x7af870, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('player_search_remove_own_squad', 0x7b11b0, [0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x86, 0xf6, 0x01, 0x00, 0x00]),
           ('player_search_remove_player', 0x7b1260, [0x6a, 0xff, 0x68, 0x56, 0xf4, 0x95, 0x00]),
           ('player_search_clear_results', 0x7b14a0, [0x53, 0x56, 0x8b, 0xf1, 0x33, 0xdb]),
           ('player_search_purge_invalid_results', 0x7b1520, [0x83, 0xec, 0x24, 0x53, 0x8b, 0xd9]),
           ('player_search_compare_results_by_person', 0x7bce50, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0])]
DOCS = ('a lista de resultados da busca de jogadores',
        'the player search result list',
        [('drop_player_search', 'PLAYER_SEARCH::~PLAYER_SEARCH', '0x7ae4f0', 'player_search.cpp', 'thiscall',
          'libera o vetor de resultados (ponteiro zerado) e o registro do clube observado, que sai da lista global e e apagado; capacidade e contagem ficam',
          'frees the result array (pointer zeroed) and the scouted club record, which leaves the global list and is deleted; capacity and count stay'),
         ('search_result_copy', 'search_result_copy', '0x7af580', 'player_search.cpp', 'thiscall, ret 4',
          'copia um resultado de 0x2d bytes campo a campo (nomes, as duas datas, bandeiras, as duas notas e os bytes +0x29..+0x2c); devolve o destino',
          'copies a 0x2d-byte result field by field (names, both dates, flags, both ratings and the bytes +0x29..+0x2c); returns the destination'),
         ('player_search_add_player', 'PLAYER_SEARCH::add_player', '0x7af720', 'player_search.cpp', 'thiscall, ret 4',
          'poe um jogador com a data de hoje: os nomes, bandeiras e notas zeradas e, em +0x2c, a nota da anotacao do tecnico dono sobre ele (0xff sem anotacao), pelo add_result; sem jogador, a caixa "Error" (linha 0x369) e 0',
          'adds a player dated today: the names, flags and ratings zeroed and, at +0x2c, the rating of the owning manager note on him (0xff without a note), through add_result; without a player, the "Error" box (line 0x369) and 0'),
         ('player_search_add_result', 'PLAYER_SEARCH::add_result', '0x7af870', 'player_search.cpp', 'thiscall, ret 4',
          'poe ou atualiza um resultado na lista ordenada por pessoa: ja listado, e sobrescrito; senao, cheio, a capacidade cresce 250 (realloc; sem memoria, a caixa e exit(-1)), sem anotacao do tecnico o +0x2c do resultado vira 0xff, ele vai para o fim e a lista e reordenada; a marca de relatorio zera; sem resultado, a caixa (0x394) e 0',
          'adds or updates a result in the list sorted by person: when listed it is overwritten; otherwise, when full, the capacity grows by 250 (realloc; out of memory, the box and exit(-1)), without a manager note the result +0x2c becomes 0xff, it goes to the end and the list is sorted again; the report mark is cleared; without a result, the box (0x394) and 0'),
         ('player_search_remove_own_squad', 'PLAYER_SEARCH::remove_own_squad', '0x7b11b0', 'player_search.cpp', 'thiscall',
          'tira os resultados do proprio elenco (bit 2 das bandeiras), cada um trocado pelo ultimo; reordena ou, vazia, libera a lista',
          'removes the own squad results (flag bit 2), each swapped with the last; sorts again or, empty, frees the list'),
         ('player_search_remove_player', 'PLAYER_SEARCH::remove_player', '0x7b1260', 'player_search.cpp', 'thiscall, ret 4',
          'tira uma pessoa da lista (busca binaria), trocando pelo ultimo; reordena ou libera; zera a marca de relatorio; sem pessoa, lista vazia ou nao listada: a caixa (0x733, 0x73d, 0x747) e 0',
          'removes a person from the list (binary search), swapping with the last; sorts again or frees; clears the report mark; without a person, with an empty list or not listed: the box (0x733, 0x73d, 0x747) and 0'),
         ('player_search_clear_results', 'PLAYER_SEARCH::clear_results', '0x7b14a0', 'player_search.cpp', 'thiscall, ret 4',
          'esvazia a lista (libera o vetor, zera capacidade, contagem e marca); com a bandeira e um clube atribuido com registro, o clube vira −1 e o registro sai da lista global e e apagado',
          'empties the list (frees the array, zeroes capacity, count and mark); with the flag and an assigned club with a record, the club becomes −1 and the record leaves the global list and is deleted'),
         ('player_search_purge_invalid_results', 'PLAYER_SEARCH::purge_invalid_results', '0x7b1520', 'player_search.cpp', 'thiscall, ret 4',
          'tira os resultados cuja pessoa nao confere mais (indice abaixo do total de pessoas − 16, os nomes, jogador ou, na busca de comissao, nao-jogador fora de tecnico/auxiliar/treinador, e nao aposentado); com religar, a data de hoje em cada um e a primeira pessoa da tabela que confere passa a ser a do resultado; reordena ou libera',
          'drops the results whose person no longer fits (index below the staff count − 16, the names, a player or, in a staff search, a non-player who is not manager/assistant/coach, and not retired); with relink, today goes into each and the first person in the table who fits becomes the result person; sorts again or frees'),
         ('player_search_compare_results_by_person', 'player_search_compare_results_by_person', '0x7bce50', 'player_search.cpp', 'cdecl',
          'a ordem da lista de resultados: pelo indice da pessoa (+0xc); o nulo depois de todos',
          'the order of the result list: by person index (+0xc); a null after everything')])
