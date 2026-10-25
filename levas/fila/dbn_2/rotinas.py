# 0x5315a0 roda na carga do banco (load_database, uma vez na criação do .qst e outra ao carregar o save) e na gravação
# (save_database, depois de gravar as tabelas). 0x53db40 roda na criação (o vetor de clubes vai para o save), no
# carregar_jogo 0x82a9c0 quando o vetor falta e no laço diário 0x5cccd0. Por isso CARGA: a prova do save também carrega
# o FeriasTesteSave com e sem as rotinas.
# Sem SEH, sem sorteio, sem alocação; sem variável local lida sem valor. Comparadores de verdade nas duas (lado a lado
# com as 12 tabelas inteiras e com o vetor [0xae2d1c]).
# 0x53db40 entrou órfã da 232ª (cada agente achou que estava no item do outro) no lugar da 0x524d50; 0x601a40
# (set_key_ids, 41 KB com a cadeia das cores, mil chaves e as caixas) e 0x535a80 (12,6 KB, milhares de sorteios) ficam
# para depois.
SAVE = True
CARGA = True
ROTINAS = [('resolver_indices_do_banco', 0x5315a0, [0x81, 0xec, 0x04, 0x06, 0x00, 0x00]),
           ('ordenar_clubes_pela_reputacao', 0x53db40, [0x8a, 0x44, 0x24, 0x04, 0x84, 0xc0])]
DOCS = ('a carga do banco: trocar os índices por ponteiros e o vetor de clubes pela reputação',
        'database load: resolving indexes to pointers and the club vector by reputation',
        [('resolver_indices_do_banco', 'resolve_database_indexes', '0x5315a0', 'Database.cpp', 'cdecl',
          'troca cada campo de id das 12 tabelas pelo ponteiro ao registro, com os tetos, os registros em 0xde1f64, as caixas de pessoa sem jogador, preferências ou não jogador e os erros do original (+0x5b do clube zera +0x60; +0x48 do estádio zera a cidade)',
          'replaces every id field of the 12 tables with the record pointer, with the limits, the 0xde1f64 log lines, the boxes for staff with bad player, preferences or non-player data and the original bugs (club +0x5b clears +0x60; stadium +0x48 clears the city)'),
         ('ordenar_clubes_pela_reputacao', 'database_sort_clubs_by_reputation', '0x53db40', 'Database.cpp', 'cdecl',
          'com refazer, enche o vetor [0xae2d1c] com os ids dos clubes; depois o ordena pela reputação (qsort com 0x540020)',
          'with rebuild, fills the vector [0xae2d1c] with the club ids; then sorts it by reputation (qsort with 0x540020)')])
CAMPOS = [('TClub +0x53..+0xbb, +0xbf..+0xd3, +0xd7 (50), +0x19f, +0x1b3, +0x1cf, +0x1e0 (20); TNation +0x71..+0x118; árbitro +4..+0x1a; estádio +0x38, +0x48; preferências +4..+0x30; não jogador +0x23..+0x3f; TStaff +4..+0x24, +0x61, +0x65, +0x69; competição de pessoa +0x53..+0x5f; histórico de pessoas +4, +0xa; cidade +0x1f; histórico de competições +4..+0x16; competição de clube +0x59..+0x65: id → ponteiro; grava (resolver_indices_do_banco)', 'TClub +0x53..+0xbb, +0xbf..+0xd3, +0xd7 (50), +0x19f, +0x1b3, +0x1cf, +0x1e0 (20); TNation +0x71..+0x118; official +4..+0x1a; stadium +0x38, +0x48; preferences +4..+0x30; non-player +0x23..+0x3f; TStaff +4..+0x24, +0x61, +0x65, +0x69; staff comp +0x53..+0x5f; staff history +4, +0xa; city +0x1f; comp history +4..+0x16; club comp +0x59..+0x65: id → pointer; writes (resolver_indices_do_banco)'),
          ('[0xae2d1c]: vetor de ids de clube pela reputação; grava (ordenar_clubes_pela_reputacao)', '[0xae2d1c]: club id vector by reputation; writes (ordenar_clubes_pela_reputacao)')]
INTERESSA = ['SGE', 'CMRTE', 'CMScout']
