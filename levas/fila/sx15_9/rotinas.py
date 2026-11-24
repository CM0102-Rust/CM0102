# O objeto das seleções gravado no jogo salvo e destruído (national_teams.cpp; leitor sx15, rodada 5, 23/11/2026):
# NationalTeams::save_dat 0x776a20 (passo 0x16 do save_game_data, 0x82f05b; SaveStep::Caminho no setup.rs) e o
# destrutor NationalTeams::~NationalTeams 0x76a710 (free_game_data, 0x8301ca: fechar o jogo, carregar outro, voltar ao
# menu).
# Convenções conferidas na desmontagem:
# - 0x776a20: thiscall(this, save), ret 4; o eax de saída é o do FIFA_RANKINGS::save (0x58dae0 thiscall(r, save) ret 4,
#   que devolve o lixo do zclose) e quem chama não o lê: a cópia é void; zopen(save, "national_teams.dat", 1, 0x16, 4),
#   zwrite(buf, tam, n, z) cdecl; as caixas pelo show_error_at, sem fechar o item; os pools gravados como ids (pessoa
#   -> [pessoa+0], nula -> -1) e voltam a pessoas (-1 -> 0); a conta das convocações num dword zerado (só 2 bytes
#   gravados); o cursor da lista (+0xa) volta à cabeça (+2) antes da conta e antes de percorrer (LLIST::get_next
#   0x68d1b0).
# - 0x76a710: thiscall(this), ret 0; FIFA_RANKINGS::~FIFA_RANKINGS 0x58cf20 + operator delete; LLIST::~LLIST 0x68d040
#   + delete; free de +4, +8, +0xc, +0x10, dos vetores dos registros dos clubes (a conta relida a cada passo) e da
#   tabela, de +0x24 e +0x36; por último LLIST::~LLIST no +0x28 embutido; não zera os campos.
# Comparadores: só a original nas duas (gravam no contêiner, liberam memória). A prova é a do save: o .qst gravado com e
# sem a leva; o destrutor roda quando a prova carrega um save por cima de um jogo (se ela não fizer isso, fica sem prova
# na cadeia e roda no jogo do Lucas ao carregar um save).
# Usa NATIONAL_TEAMS_DAT do sx15_8: integrar junto ou depois dele.
SAVE = True
ROTINAS = [('nationalteams_save_dat', 0x776a20, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('nationalteams_destructor', 0x76a710, [0x6a, 0xff, 0x68, 0xcb, 0xdc, 0x95, 0x00])]
DOCS = ('o objeto das seleções gravado no jogo salvo e destruído',
        'the national teams object written to the saved game and destroyed',
        [('nationalteams_save_dat', 'NationalTeams::save_dat', '0x776a20', 'national_teams.cpp', 'thiscall, ret 4',
          'regra: no item national_teams.dat grava, nesta ordem, os registros das nações, o pool de reservas principal e o sub-21 (tamanho e as pessoas como ids, -1 nenhuma; voltam a pessoas depois), as convocações (conta de 2 bytes, 0 sem lista, e os registros de 0x12 bytes da cabeça ao fim), os registros de convocados por clube e o vetor de cada um com entradas, as próximas competições por nação e as marcas olímpicas; fecha o item e o ranking da FIFA grava o seu; cada falha mostra a caixa de national_teams.cpp da sua linha e para sem fechar o item',
          'rule: into the national_teams.dat item it writes, in this order, the nation records, the senior and the under-21 reserve pools (size and the persons as ids, -1 none; turned back into persons afterwards), the call-ups (a 2-byte count, 0 without a list, and the 0x12-byte records from head to end), the club call-up records and the vector of each one with entries, the next competitions per nation and the Olympic marks; closes the item and the FIFA rankings write their own; each failure shows the national_teams.cpp box of its line and stops without closing the item'),
         ('nationalteams_destructor', 'NationalTeams::~NationalTeams', '0x76a710', 'national_teams.cpp', 'thiscall',
          'regra: libera o ranking da FIFA, a lista de convocações, os registros das nações, as próximas competições, os dois pools, o vetor de cada registro de clube e os registros, as marcas dos clubes e as olímpicas, e por fim a lista embutida dos novos classificados; não zera os campos',
          "rule: frees the FIFA rankings, the call-up list, the nation records, the next competitions, both pools, each club record's vector and the records, the club marks and the Olympic ones, and last the inline new qualifiers list; does not clear the fields")])
INTERESSA = ['SGE', 'CMRTE']
