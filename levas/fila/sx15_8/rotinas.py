# O objeto das seleções montado para um jogo novo ou a partir do jogo salvo (national_teams.cpp; leitor sx15, rodada 5,
# 23/11/2026): NationalTeams::load_dat 0x7763b0 (chamada só pelo construtor, 0x76a6da) e o construtor
# NationalTeams::NationalTeams 0x76a4e0 (chamado pelo create_game em 0x82938e e pela carga em 0x82c3c2; ambos portados
# em setup.rs, que chamam o endereço). O construtor roda em toda carreira de teste (jogo novo, save 0), mas o item é
# SAVE+CARGA: a prova do save cria o jogo (o caminho novo) e carrega o FeriasTesteSave (o caminho do save); um item não
# mistura tipos e a carga só se prova por ela.
# Convenções conferidas na desmontagem:
# - 0x76a4e0: thiscall(this, save), ret 4, devolve this; SEH sem efeito; LLIST::LLIST (0x68d020) no +0x28 embutido;
#   a checagem de CD (GetLogicalDriveStringsA/GetDriveTypeA pela IAT, "%s%c..%c" = <raiz>cm0102.exe, file_find_first
#   0x4c0aa0 devolve 0 quando acha, file_find_close 0x4c0c10, a caixa "Insert CD" 0x5e8290(título, texto, 1)) pelo
#   find_game_cd do sx02b_1 (o cm0102-1080 troca o jne em 0x76a543 por nops e o test em 0x76a586 por xor: a cópia lê
#   esses bytes); save 0: 0x7a30c0 cdecl(4, 0); operator new(0xd) e FIFA_RANKINGS::FIFA_RANKINGS 0x58cea0
#   thiscall(r, save) ret 4; calloc(clubes - 2 x nações, 6) cdecl; a caixa da linha 0x2163 e segue; load_dat e
#   init_tables (0x76a840, já em Rust) pelo endereço do exe.
# - 0x7763b0: thiscall(this, save), ret 4, devolve 0/1; zopen(save, "national_teams.dat", 0, 0x16, 4), zread(buf, tam,
#   n, z) cdecl; malloc(n x 0x6e) conferido contra nulo (como os pools e os vetores dos clubes), malloc(n x 4) e
#   calloc(1, n) não; as caixas pelo show_error_at (o original monta o mesmo "v%s %s.%s %d" com os buffers do
#   _splitpath em ordens diferentes, mesmo resultado); Date::Date 0x54a110 no +8 do buffer local de 0x12 antes da
#   lista (lido por cima); operator new(0xe) + LLIST::LLIST, LLIST::add_tail 0x68d0d0 thiscall(lista, buf, 0x12);
#   leitura curta dos registros dos clubes pula os vetores sem caixa; leitura curta das próximas competições enche
#   todas com -1 (rep stosd com a conta de nações); a das marcas olímpicas não é conferida; em erro o item fica aberto.
# Defeito do original: o registro de clube sem entradas guarda o ponteiro do vetor da sessão que gravou (o destrutor
# libera todo ponteiro não nulo) [inferência: se o jogo deixa ponteiro não nulo com conta 0].
# Comparadores: só a original nas duas (alocam e leem o contêiner); a prova é a do save (CARGA).
# Integrar depois do sx02b_1 (usa o find_game_cd dele).
SAVE = True
CARGA = True
ROTINAS = [('nationalteams_load_dat', 0x7763b0, [0x6a, 0xff, 0x68, 0x19, 0xdf, 0x95, 0x00]),
           ('nationalteams_constructor', 0x76a4e0, [0x6a, 0xff, 0x68, 0xac, 0xdc, 0x95, 0x00])]
DOCS = ('o objeto das seleções montado para um jogo novo ou a partir do jogo salvo',
        'the national teams object built for a new game or from the saved game',
        [('nationalteams_load_dat', 'NationalTeams::load_dat', '0x7763b0', 'national_teams.cpp', 'thiscall, ret 4',
          'regra: sem save, 0 sem caixa; do item national_teams.dat lê, nesta ordem, os registros das nações (0x6e bytes cada), o pool de reservas principal e o sub-21 (tamanho e ids das pessoas, que viram as pessoas; -1 nenhuma), as convocações (conta de 2 bytes e registros de 0x12 bytes numa lista nova), os registros de convocados por clube (6 bytes por clube sem as seleções; cada um com entradas ganha capacidade = conta e um vetor novo de entradas de 9 bytes; leitura curta pula os vetores), as próximas competições por nação (leitura curta enche com -1) e as marcas olímpicas; fecha o item e devolve 1; cada falha mostra a caixa de national_teams.cpp da sua linha e devolve 0 (o construtor então monta as tabelas do zero)',
          'rule: no save, 0 without a box; from the national_teams.dat item it reads, in this order, the nation records (0x6e bytes each), the senior and the under-21 reserve pools (size and person ids, turned into the persons; -1 none), the call-ups (a 2-byte count and 0x12-byte records into a new list), the club call-up records (6 bytes per club without the national sides; each one with entries gets capacity = count and a new vector of 9-byte entries; a short read skips the vectors), the next competitions per nation (a short read fills them with -1) and the Olympic marks; closes the item and returns 1; each failure shows the national_teams.cpp box of its line and returns 0 (the constructor then builds the tables from scratch)'),
         ('nationalteams_constructor', 'NationalTeams::NationalTeams', '0x76a4e0', 'national_teams.cpp', 'thiscall, ret 4',
          'regra: monta a lista dos novos classificados; confere o CD (o primeiro drive de CD com o cm0102.exe; senão a caixa "Insert CD" e de novo); jogo novo carrega a configuração dos jogadores; o ranking da FIFA (do save, quando há), a lista de convocações e as marcas dos clubes zeradas, os registros de convocados por clube (6 bytes por clube, zerados; sem memória, a caixa e segue); com save, lê do save, e sem save ou se a leitura falhar monta as tabelas do zero; devolve o objeto',
          'rule: builds the new qualifiers list; checks the CD (the first CD drive holding cm0102.exe; otherwise the "Insert CD" box and again); a new game loads the player setup; the FIFA rankings (from the save when there is one), the call-up list and the club marks cleared, the club call-up records (6 bytes per club, zeroed; out of memory, the box and it goes on); with a save it reads from it, and without one or when the read fails it builds the tables from scratch; returns the object')])
CAMPOS = [('NationalTeams +0x14 e +0x18: o tamanho do pool de reservas principal e do sub-21 (dword; no jogo novo os dois com o total do principal); valor; grava do save (nationalteams_load_dat)',
           'NationalTeams +0x14 and +0x18: the size of the senior and of the under-21 reserve pool (dword; in a new game both with the senior total); value; writes from the save (nationalteams_load_dat)')]
INTERESSA = ['SGE', 'CMRTE']
