# Um ano do calendario de jogos: construir, destruir e gravar (fix_man.cpp; leitor sbA, rodada 5, 24/11/2026). Carreira
# ENG (sem LIGAS).
# - 0x5a9cf0 FIXTURE_MANAGER::FIXTURE_MANAGER thiscall(ano, outro ano ou 0, principal) ret 0xc -> this: na criacao (o
#   principal) e quando o calendario precisa de outro ano. O CD so no principal sem outro ano (0x5a9d65 e 0x5a9da7: o
#   patch sem CD do cm0102-1080 e do portatil troca esses 3 bytes; find_game_cd le os bytes do exe que roda; a rotina e
#   desviada nos dois exes pelos dois hashes); club_dates 0x5aee90 e leg checks 0x5af390 (versao >= 254000);
#   date_is_leap_year 0x54cfe0; malloc 0x944e46 dos 3 vetores de cada dia; operator new(0xe) + LLIST::LLIST 0x68d020;
#   load_fixtures 0x5ac3b0. Caixas (fix_man.cpp) 0xaf e 0xba, sem fim do jogo.
# - 0x5aa0d0 FIXTURE_MANAGER::~FIXTURE_MANAGER thiscall() ret 0: a fila de noticias, save_fixtures(0) (falhando, 0xe3),
#   as listas e os vetores; o principal destroi o outro ano e libera a tabela das datas dos clubes (0xae2a5c).
# - 0x5ac060 FIXTURE_MANAGER::save_fixtures thiscall(tudo) ret 4 -> 0/1: CM3_TEMP\fixtures_<ano>.tmp ("wb", 0x90bde0),
#   a contagem, os jogos de 0x4f bytes e a contagem de novo no comeco; com tudo, o outro ano, club_dates 0x5af040 e leg
#   checks 0x5af4a0. Caixas 0x69f, 0x6ab, 0x6d2, 0x6d8.
# Convencoes conferidas: CRT cdecl; LLIST thiscall; remove 0x94643a (cdecl). Prologos: push -1; push imm32 (7 bytes);
# sub esp, 0x210 (6); sub esp, 0x210 (6). Sem relativas. 0x5aa0d0 e 0x5ac060 iguais no cm0102-1080.exe; 0x5a9cf0 difere
# so nos 3 bytes do patch sem CD.
# Defeitos do original: o save_fixtures apaga o arquivo vazio pelo nome sem a pasta (remove("fixtures_<ano>.tmp")),
# entao o vazio fica em CM3_TEMP; nao confere a gravacao de cada jogo nem lista nula.
# Comparadores: o construtor e o destrutor so a original (alocam e liberam), conferindo o bloco (ano, bissexto,
# principal, outro ano, as listas do primeiro dia) e, no destrutor do principal, a tabela das datas liberada; o
# save_fixtures as duas gravam o mesmo arquivo (a segunda substitui a primeira) e o arquivo depois de cada uma e o
# resultado tem de bater.
# Depende do sx02b_1 (find_game_cd, leva 330): integrar antes.
SAVE = False
ROTINAS = [('new_fixture_manager', 0x5a9cf0, [0x6a, 0xff, 0x68, 0xaa, 0x95, 0x95, 0x00]),
           ('drop_fixture_manager', 0x5aa0d0, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00]),
           ('save_fixtures', 0x5ac060, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00])]
DOCS = ('um ano do calendário de jogos: construir, destruir e gravar',
        'a year of the fixture calendar: building it, destroying it and saving it',
        [('new_fixture_manager', 'FIXTURE_MANAGER::FIXTURE_MANAGER', '0x5a9cf0', 'fix_man.cpp', 'thiscall, ret 0xc',
          'monta o bloco de um ano do calendário: o bloco do outro ano, o CD no principal, as datas dos clubes e os confrontos de ida e volta no principal, o ano e se é bissexto, as listas de cada dia, faixa e nação-chave, e carrega os jogos do ano',
          "builds a year block of the calendar: the other year's block, the CD in the main one, the club dates and the leg checks in the main one, the year and whether it is a leap year, the lists of each day, slice and key nation, and loads the year's fixtures"),
         ('drop_fixture_manager', 'FIXTURE_MANAGER::~FIXTURE_MANAGER', '0x5aa0d0', 'fix_man.cpp', 'thiscall, ret 0',
          'destrói o bloco de um ano: grava os jogos, libera a fila de notícias, as listas e os vetores; o principal destrói também o outro ano e a tabela das datas dos clubes',
          "destroys a year block: saves the fixtures, frees the news queue, the lists and the arrays; the main one also destroys the other year and the club dates table"),
         ('save_fixtures', 'FIXTURE_MANAGER::save_fixtures', '0x5ac060', 'fix_man.cpp', 'thiscall, ret 4',
          'grava os jogos do ano em fixtures_<ano>.tmp na pasta temporária (a contagem e os jogos); sem jogos tenta apagar o arquivo; com tudo, grava também o outro ano, as datas dos clubes e os confrontos de ida e volta',
          "saves the year's fixtures into fixtures_<year>.tmp in the temporary folder (the count and the fixtures); with none it tries to remove the file; with everything, it also saves the other year, the club dates and the leg checks")])
CAMPOS = []
INTERESSA = []
