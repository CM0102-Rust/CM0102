# O registro da busca de jogadores no save: o construtor, a gravacao e a carga (PLAYER_SEARCH::PLAYER_SEARCH 0x7ae390,
# PLAYER_SEARCH::save 0x7b2360, PLAYER_SEARCH::load 0x7b17e0, player_search.cpp; leitor sx17, rodada 5, 24/11/2026).
# As tres rotinas inteiras (0x7ae390..0x7ae4ef, ret; 0x7b2360..0x7b2be1, ret 4; 0x7b17e0..0x7b2356, ret 4; o resto ate
# a proxima fronteira e enchimento).
# Chamadas: o construtor pelo iterador de vetor do SCOUT_MANAGER (0x7f68eb, 0x7f7d0f: criacao do jogo e carga) e pelas
# sessoes da tela de busca (0x8041e2, 0x804200); a gravacao pelo SCOUT_MANAGER::save (0x7f8177, no save_game_data); a
# carga pelo SCOUT_MANAGER na carga do save (0x7f7dc2).
# Convencoes conferidas na desmontagem:
# - 0x7ae390 thiscall(busca) ret, devolve a busca (moldura SEH so para os destrutores dos membros);
# - 0x7b2360 / 0x7b17e0 thiscall(busca, item do save) ret 4 -> 1 ou 0;
# - 0x54a110 Date::Date thiscall(data); 0x549f70 Date::Date thiscall(data, 0x1b, 4, 0x7b1, -1) ret 0x10; 0x417c10
#   Date::operator= thiscall(destino, &tmp, origem) ret 8; 0x806450 SearchFilters::set_defaults thiscall; 0x81b800
#   search_filter_record_init thiscall (ja em Rust, search_screens.rs);
# - 0x932e10 zwrite / 0x932c30 zread cdecl(buf, tamanho, quantos, item) -> quantos; 0x7b1520 purge_invalid_results
#   thiscall(busca, 0) ret 4; 0x944e46 malloc, 0x9452ca free, 0x944cf1 operator new cdecl; 0x68d0d0 LLIST::add_tail
#   thiscall([0xb74438], registro, 4) ret 8; a caixa de erro: 0x944cff _splitpath, 0x90d130, sprintf "v%s %s.%s %d",
#   0x5e8290 ("Error", texto, 0) e [0xb67a34] = 0 (show_error_box); na falta de memoria 0x5cc7d0, 0x5a8f60 e exit(-1).
# Defeitos do original: na carga dos resultados antigos (dados ate 0x3e032) o malloc do buffer antigo nao e conferido e,
# na leitura curta, o buffer fica alocado; na carga, a leitura curta do registro do clube observado mostra a caixa mas
# guarda o registro (meio lido) sem o pôr na lista global; nos erros o item fica como esta (sem fechar).
SAVE = True
CARGA = True  # a carga so roda ao carregar: a prova do save carrega o save de ferias com e sem a leva
ROTINAS = [('new_player_search', 0x7ae390, [0x6a, 0xff, 0x68, 0xea, 0xf2, 0x95, 0x00]),
           ('player_search_save', 0x7b2360, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('player_search_load', 0x7b17e0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a busca de jogadores no save: o construtor, a gravacao e a carga',
        'the player search in the saved game: the constructor, saving and loading',
        [('new_player_search', 'PLAYER_SEARCH::PLAYER_SEARCH', '0x7ae390', 'player_search.cpp', 'thiscall',
          'monta uma busca vazia: as datas (+0x184, inicio e fim da observacao, +0x1b6, data da competicao) e os dois registros de filtros construidos; dono −1, vaga −1, o numero magico dos arquivos de busca +5 = 0x24a9e e a versao +9 = 1, o tipo de lista 4, +0xb 0, nome do tecnico (+0x70) e senha (+0x139) vazios, +0x16c a semente do jogo, sem olheiro e sem tarefa (nacao, clube, regiao, competicao e clube-alvo −1; base e adversario 0), o inicio da observacao 27/04/1969 [inferencia], sem relatorios, sem resultados (vetor, capacidade e contagem 0), a marca de relatorio 1, +0x2a8 e +0x2ac 0 e sem registro de clube observado; devolve a busca',
          'builds an empty search: the dates (+0x184, scouting start and end, +0x1b6, competition date) and both filter records constructed; owner −1, slot −1, the search file magic +5 = 0x24a9e and version +9 = 1, list type 4, +0xb 0, manager name (+0x70) and password (+0x139) empty, +0x16c the game seed, no scout and no assignment (nation, club, region, competition and target club −1; base and opponent 0), the scouting start 27/04/1969 [inference], no reports, no results (array, capacity and count 0), the report mark 1, +0x2a8 and +0x2ac 0 and no scouted club record; returns the search'),
         ('player_search_save', 'PLAYER_SEARCH::save', '0x7b2360', 'player_search.cpp', 'thiscall, ret 4',
          'grava a busca no save, campo a campo (dono, vaga, criterios, olheiro, tarefa, datas, os dez dwords de +0x1c6); tira os resultados invalidos (purge_invalid_results sem religar), grava capacidade, contagem e os resultados (0x2d bytes cada), os filtros, a marca, +0x2a8 e +0x2ac; com clube atribuido e registro, o registro do clube observado (0x51a8 bytes). Sem item ou gravacao curta: a caixa "Error" do player_search.cpp com a linha do campo e 0; a gravacao curta do registro do clube mostra a caixa e devolve 1',
          'writes the search into the save field by field (owner, slot, criteria, scout, assignment, dates, the ten dwords at +0x1c6); drops the invalid results (purge_invalid_results without relinking), writes capacity, count and the results (0x2d bytes each), the filters, the mark, +0x2a8 and +0x2ac; with an assigned club and its record, the scouted club record (0x51a8 bytes). No item or a short write: the player_search.cpp "Error" box with the field line and 0; a short write of the club record shows the box and returns 1'),
         ('player_search_load', 'PLAYER_SEARCH::load', '0x7b17e0', 'player_search.cpp', 'thiscall, ret 4',
          'le a busca do save na mesma ordem: o clube atribuido abaixo de −1 vira −1; a base so com dados acima de 0x3e010, o adversario e o clube-alvo acima de 0x3e039, o inicio da observacao acima de 0x3e010; libera os resultados antigos e aloca a capacidade (sem memoria: a caixa, a limpeza fatal e exit(-1)); os resultados de 0x2d bytes acima de 0x3e032, senao os antigos de 0x2c convertidos (o byte novo +0x2c = 0xff); os filtros, a marca, +0x2a8 e +0x2ac; com clube atribuido, um registro de clube observado novo (0x51a8 bytes) lido e posto na lista global (leitura curta: a caixa, e o registro fica sem ir para a lista). Leitura curta: a caixa com a linha do campo e 0; senao 1',
          'reads the search from the save in the same order: an assigned club below −1 becomes −1; the base only with data above 0x3e010, the opponent and the target club above 0x3e039, the scouting start above 0x3e010; frees the old results and allocates the capacity (out of memory: the box, the fatal cleanup and exit(-1)); 0x2d-byte results above 0x3e032, else the old 0x2c-byte ones converted (the new byte +0x2c = 0xff); the filters, the mark, +0x2a8 and +0x2ac; with an assigned club, a new scouted club record (0x51a8 bytes) read and added to the global list (short read: the box, and the record is kept off the list). A short read: the box with the field line and 0; otherwise 1')])
