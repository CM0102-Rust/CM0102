# O dono, o clube observado, a copia e os arquivos gravados da busca de jogadores (player_search.cpp; leitor sx17,
# rodada 5, 24/11/2026): set_manager_name (0x7ae780), set_password (0x7ae7d0), set_assigned_club (0x7b3d50), operator=
# (0x7b7690), read_file_header (0x7bcee0) e os quatro filtros do dialogo de carga (0x7bd010 lista curta, 0x7bd0d0 busca
# rapida, 0x7bd190 busca de comissao, 0x7bd250 busca de olheiro). Rotinas inteiras (0x7ae780..0x7ae7c9 ret 4;
# 0x7ae7d0..0x7ae894 ret 4; 0x7b3d50..0x7b3e1c ret 4; 0x7b7690..0x7b7832 ret 4; 0x7bcee0..0x7bd00a ret; cada filtro
# 0xc0 bytes com ret; o resto e enchimento).
# Quem chama: as sessoes de edicao da busca (search_edit_session 0x804xxx: nome, senha, copia das listas), a tela de
# busca (0x80eccf, 0x81094f: o clube observado), o SCOUT_MANAGER (0x7f839a), e os dialogos de abrir arquivo da tela de
# busca (os filtros vao por endereco: 0x80eec8, 0x80fc30, 0x811346, 0x811954, 0x812f06...). A carreira automatica nao
# passa por nenhuma.
# TELAS: a prova e o Lucas jogando comparando: a tela de busca de jogadores, gravar e abrir uma busca ou lista curta
# (o dialogo de abrir filtra os arquivos), atribuir a um olheiro um clube para observar.
# Convencoes conferidas na desmontagem:
# - thiscall ret 4: 0x7ae780 (busca, pessoa) -> 1; 0x7ae7d0 (busca, pessoa) -> 1/0; 0x7b3d50 (busca, clube) (eax sem
#   valor: os chamadores nao o leem); 0x7b7690 (busca, outra) (idem); cdecl: 0x7bcee0 (caminho, saida, tipo) -> 0/1,
#   os filtros (caminho) -> 2, 1, 5 ou 9 (o 2 sai por `mov ax, si` com o eax zerado pela 0x7bcee0);
# - 0x53dba0 database_format_staff_name cdecl(pessoa, buf, 0xc9, 1); 0x5fde40 HUMAN_MANAGER::get_password
#   thiscall([0xb63c98], pessoa) ret 4; 0x945b00 fopen, 0x945bfe fread, 0x945a09 fclose cdecl; 0x944cf1 operator new,
#   0x944c94 operator delete, 0x944e46 malloc, 0x9452ca free cdecl; 0x68d0d0 LLIST::add_tail thiscall(0xb74438,
#   registro, 4) ret 8; 0x68d1f0 LLIST::remove thiscall(0xb74438, registro) ret 4.
# Defeito do original: operator= com capacidade 0 fica com o ponteiro dos resultados da outra busca (as duas liberam o
# mesmo vetor depois).
TELAS = True
ROTINAS = [('player_search_set_manager_name', 0x7ae780, [0x8b, 0x44, 0x24, 0x04, 0x6a, 0x01]),
           ('player_search_set_password', 0x7ae7d0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('player_search_set_assigned_club', 0x7b3d50, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('player_search_assign', 0x7b7690, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('player_search_read_file_header', 0x7bcee0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('player_search_filter_short_list_file', 0x7bd010, [0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x6c, 0x01, 0x00, 0x00]),
           ('player_search_filter_quick_search_file', 0x7bd0d0, [0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x6c, 0x01, 0x00, 0x00]),
           ('player_search_filter_staff_search_file', 0x7bd190, [0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x6c, 0x01, 0x00, 0x00]),
           ('player_search_filter_scout_search_file', 0x7bd250, [0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x6c, 0x01, 0x00, 0x00])]
_FILTRO = ('responde ao dialogo de abrir para um arquivo de busca do tipo %d (%s): 2 se nao for um arquivo desse tipo, 1 sem senha do dono, 9 com a senha do tecnico humano atual e 5 com a de outro',
           'answers the load dialog for a search file of list type %d (%s): 2 when it is not a file of that type, 1 without an owner password, 9 with the current human manager password and 5 with someone else')
DOCS = ('o dono, o clube observado, a copia e os arquivos gravados da busca de jogadores',
        'the player search owner, watched club, copy and saved files',
        [('player_search_set_manager_name', 'PLAYER_SEARCH::set_manager_name', '0x7ae780', 'player_search.cpp', 'thiscall, ret 4',
          'escreve o nome formatado da pessoa, ou do tecnico dono quando nula, no campo de 201 bytes em +0x70; devolve 1',
          'writes the formatted name of the person, or of the owning manager when null, into the 201-byte field at +0x70; returns 1'),
         ('player_search_set_password', 'PLAYER_SEARCH::set_password', '0x7ae7d0', 'player_search.cpp', 'thiscall, ret 4',
          'copia a senha do tecnico humano para +0x139 e devolve 1; sem senha (nula ou vazia), a caixa "Error" (linha 0x131) e 0',
          'copies the human manager password to +0x139 and returns 1; without one (null or empty), the "Error" box (line 0x131) and 0'),
         ('player_search_set_assigned_club', 'PLAYER_SEARCH::set_assigned_club', '0x7b3d50', 'player_search.cpp', 'thiscall, ret 4',
          'troca o clube que a lista observa: o registro de outro clube sai da lista global e e apagado; guarda o clube; com clube e sem registro, um registro novo (o id do clube e as 50 vagas livres, −1) entra na lista global',
          'changes the club the list watches: the record of another club leaves the global list and is deleted; stores the club; with a club and no record, a new record (the club id and the 50 slots free, −1) joins the global list'),
         ('player_search_assign', 'PLAYER_SEARCH::operator=', '0x7b7690', 'player_search.cpp', 'thiscall, ret 4',
          'copia uma busca inteira: libera os resultados e o registro do clube proprios, copia os 0x2b0 bytes e, com capacidade, aloca e copia os resultados (sem memoria, a caixa e exit(-1)); com registro de clube, uma copia nova na lista global',
          'copies a whole search: frees the own results and club record, copies the 0x2b0 bytes and, with a capacity, allocates and copies the results (out of memory, the box and exit(-1)); with a club record, a new copy on the global list'),
         ('player_search_read_file_header', 'player_search_read_file_header', '0x7bcee0', 'player_search.cpp', 'cdecl',
          'le o cabecalho de 0x16b bytes de um arquivo de busca (zerado antes) e diz se e uma busca do tipo pedido: o numero magico 0x24a9e, a versao 1 e o byte do tipo; sem caminho, a caixa (0x1ef6) e 0; arquivo que nao abre ou leitura curta, 0',
          'reads the 0x16b-byte header of a search file (zeroed first) and tells whether it is a search of the asked type: the magic 0x24a9e, version 1 and the type byte; without a path, the box (0x1ef6) and 0; a file that does not open or a short read, 0'),
         ('player_search_filter_short_list_file', 'player_search_filter_short_list_file', '0x7bd010', 'player_search.cpp', 'cdecl',
          _FILTRO[0] % (1, 'lista curta'), _FILTRO[1] % (1, 'short list')),
         ('player_search_filter_quick_search_file', 'player_search_filter_quick_search_file', '0x7bd0d0', 'player_search.cpp', 'cdecl',
          _FILTRO[0] % (2, 'busca rapida'), _FILTRO[1] % (2, 'quick search')),
         ('player_search_filter_staff_search_file', 'player_search_filter_staff_search_file', '0x7bd190', 'player_search.cpp', 'cdecl',
          _FILTRO[0] % (3, 'busca de comissao'), _FILTRO[1] % (3, 'staff search')),
         ('player_search_filter_scout_search_file', 'player_search_filter_scout_search_file', '0x7bd250', 'player_search.cpp', 'cdecl',
          _FILTRO[0] % (4, 'busca de olheiro'), _FILTRO[1] % (4, 'scout search'))])
