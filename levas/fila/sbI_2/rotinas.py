# Os construtores dos amistosos, da tabela de sedes e do tempo (friendly.cpp, host_country.cpp, weather.cpp; leitor
# sbI, rodada 5, 24/11/2026). Simulacao, carreira ENG (sem LIGAS). Os tres rodam na criacao do jogo (setup, passos 7,
# 0xf e 0x2b, so this) e na carga (passos 0xb, 0x13 e 0x2c, so this: leem o Friendly.tmp, o HostCountry.tmp e o
# weather.cfg). O CD pelo find_game_cd (sx02b_1, ja no HEAD desde a leva 330): o patch sem CD do cm0102-1080 e do
# portatil troca 3 bytes de cada (0x5c2026/0x5c2068, 0x5f97fa/0x5f983d, 0x9226fe/0x922741); as rotinas sao desviadas
# nos dois exes pelos dois hashes. Fora esses bytes, iguais no cm0102-1080.exe.
# - 0x5c1f80 Friendly::Friendly thiscall() ret 0 -> this (quadro SEH so para os destrutores): Comp::Comp 0x48c580,
#   Date::Date 0x54a110, vector constructor iterator 0x9463af (stdcall: +0xb1, 0x69, 0x7f, 0x5c22c0, 0x5a53c0), a
#   vtable 0x96ae7c, Date::Date 0x549f70 thiscall(d, m, a, -1) ret 0x10 (24/5/1998 e 3/0/1998), as tres rotinas ja em
#   Rust (0x5ca4f0, 0x5ca3e0, 0x5cc2d0), malloc 0x944e46, a carga 0x5c8a00 thiscall(this, 1) ret 4 (sbH_4, chamada pelo
#   endereco: sem dependencia), operator new 0x944cf1 + LLIST::LLIST 0x68d020. Caixa friendly.cpp 0x69.
# - 0x5f97c0 HostCountry::HostCountry thiscall() ret 0 -> this: load_temp_file 0x5fb160 (Rust), malloc(0x792), a caixa
#   host_country.cpp 0x50, init_default_table 0x5f9970 (Rust, sorteia os dias), qsort 0x945333 (tabela, conta short,
#   0x22, 0x5fb3f0; o comparador e do cx05_18, chamado pelo endereco).
# - 0x9226c0 Weather::Weather thiscall() ret 0 -> this: zera +8, +0, +4 e chama o load_config 0x922810 (Rust).
# Prologos: push -1; push 0x959d6b (7); sub esp, 0x4e0 (6); sub esp, 0x474 (6). Sem relativas.
# Comparadores: so a original (alocam, sorteiam e esperam o CD), com o que cada uma deixa: a vtable, a competicao, o
# primeiro torneio, os dois dias e o vetor das listas; a tabela de sedes ordenada; as tabelas do tempo.
SAVE = False
ROTINAS = [('new_friendly', 0x5c1f80, [0x6a, 0xff, 0x68, 0x6b, 0x9d, 0x95, 0x00]),
           ('new_host_country', 0x5f97c0, [0x81, 0xec, 0xe0, 0x04, 0x00, 0x00]),
           ('new_weather', 0x9226c0, [0x81, 0xec, 0x74, 0x04, 0x00, 0x00])]
DOCS = ('os construtores dos amistosos, da tabela de sedes e do tempo',
        'the constructors of the friendlies, the host table and the weather',
        [('new_friendly', 'Friendly::Friendly', '0x5c1f80', 'friendly.cpp', 'thiscall, ret 0',
          'monta a competição dos amistosos: a base de competição, as 127 vagas de torneio, o CD, o registro da competição, o primeiro torneio (o número de competições menos 127), os dois dias de planejar a pré-temporada (24 de junho e 3 de janeiro), as listas vazias, a tabela das datas internacionais, as nações ordenadas e os pares proibidos; carrega o arquivo temporário ou, no jogo novo, esvazia as vagas e dá a cada nação uma lista vazia',
          'builds the friendlies competition: the competition base, the 127 tournament slots, the CD, the competition record, the first tournament (the competition count less 127), the two pre-season planning days (24 June and 3 January), the empty lists, the international date table, the sorted nations and the banned pairs; loads the temporary file or, in a new game, empties the slots and gives each nation an empty list'),
         ('new_host_country', 'HostCountry::HostCountry', '0x5f97c0', 'host_country.cpp', 'thiscall, ret 0',
          'monta a tabela das sedes: confere o CD; carrega o arquivo temporário ou, no jogo novo, monta as 57 edições padrão (com o sorteio dos dias de decisão) e as ordena pelo ano e pelo dia da decisão',
          'builds the host table: checks the CD; loads the temporary file or, in a new game, builds the 57 default editions (with the draw of the decision days) and sorts them by year and decision day'),
         ('new_weather', 'Weather::Weather', '0x9226c0', 'weather.cpp', 'thiscall, ret 0',
          'monta o tempo: confere o CD, zera as tabelas e lê as regiões e o tempo das cidades do weather.cfg',
          "builds the weather: checks the CD, clears the tables and reads the regions and the cities' weather from weather.cfg")])
CAMPOS = []
INTERESSA = []
