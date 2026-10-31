# A temporada do clube e a nação principal (key_nation.cpp): se o clube está na temporada da linha da nação dele e as
# datas de início e fim (key_nation_club_in_season 0x669bd0, pelo clube na temporada club_is_in_season 0x547770, pela
# forma física Injury::init_squad_fitness 0x62a920, Injury::update_player_fitness 0x62c680 e
# injury_player_in_close_season 0x62ce30, pelo mês do fim da temporada 0x4d45f0 e pela data da transferência 0x8cee70;
# quadro SEH só das datas locais, sem destrutor), o continente principal (key_nation_get_main_continent 0x6695c0) e a
# nação principal (key_nation_get_main_nation 0x669600), as duas pelo menu do jogo (passageiras). Chamadas originais
# pelo endereço: 0x549ef0 (data com o bissexto, já em Rust). Iguais no cm0102-1080.exe. Comparadores: a temporada pela
# devolução e pelas duas datas (a cópia grava em datas locais); as outras duas pela devolução.
SAVE = False
ROTINAS = [('clube_na_temporada', 0x669bd0, [0x6a, 0xff, 0x68, 0x58, 0xc6, 0x95, 0x00]),
           ('continente_principal', 0x6695c0, [0xa1, 0x08, 0x2d, 0xae, 0x00]),
           ('nacao_principal', 0x669600, [0xa1, 0x08, 0x2d, 0xae, 0x00])]
DOCS = ('a temporada do clube e a nação principal: se o clube está na temporada e as datas dela, o continente principal e a nação principal',
        'the club season and the main nation: whether the club is in season and its dates, the main continent and the main nation',
        [('clube_na_temporada', 'key_nation_club_in_season', '0x669bd0', 'key_nation.cpp', 'cdecl',
          'pelos dias de início e fim da linha da nação do clube e o dia de hoje: se está na temporada e as datas de início e fim com o ano certo',
          'by the start and end days of the club nation row and today: whether it is in season and the start and end dates with the right year'),
         ('continente_principal', 'key_nation_get_main_continent', '0x6695c0', 'key_nation.cpp', 'cdecl',
          'o continente da nação do humano, ou o da primeira nação principal da tabela',
          'the continent of the human\'s nation, or that of the first main nation of the table'),
         ('nacao_principal', 'key_nation_get_main_nation', '0x669600', 'key_nation.cpp', 'cdecl',
          'a nação do humano, ou a primeira nação principal da tabela',
          'the human\'s nation, or the first main nation of the table')])
CAMPOS = [('tabela das nações-chave 0xb63d60 (35 × 72 bytes): nação +0, data +0x2a (início), data +0x39 (fim); lê (clube_na_temporada)',
           'key nation table 0xb63d60 (35 × 72 bytes): nation +0, date +0x2a (start), date +0x39 (end); reads (clube_na_temporada)'),
          ('clube +0x53: nação; ponteiro; lê (clube_na_temporada)',
           'club +0x53: nation; pointer; reads (clube_na_temporada)'),
          ('nação +0x11c bit 2: nação principal; +0x71: continente; lê (continente_principal, nacao_principal)',
           'nation +0x11c bit 2: main nation; +0x71: continent; reads (continente_principal, nacao_principal)'),
          ('[0xae2d08] nação do humano; ponteiro; [0xae2c90]/[0xae2c92] dia e ano de hoje; lê',
           '[0xae2d08] human\'s nation; pointer; [0xae2c90]/[0xae2c92] day and year of today; reads')]
INTERESSA = ['CMRTE']
