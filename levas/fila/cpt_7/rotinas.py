# O estádio neutro e o clima inicial (stadium.cpp, weather.cpp): a nota do estádio para o jogo Stadium::
# score_venue_for_fixture 0x85da00 (pela busca do estádio neutro 0x85c800, que a 255ª mostrou rodar nos jogos da seleção
# inglesa; ali só com as marcas 0x20: o público esperado, os lugares e o sorteio(100) final), o fator da distância no
# público Stadium::get_travel_distance_factor 0x85d8a0 (no público do jogo 0x85cba0, pelo motor 0x6b7f20; pura, devolve
# double no st0) e a leitura do weather.cfg Weather::load_config 0x922810 (na criação do jogo pelo Setup.cpp, 0x9226c0:
# regiões pela 0x922c40 e 7 sorteios por cidade). Fora de fechar_partida_do_dia e de eventos_do_pos_jogo. Iguais no
# cm0102-1080.exe. Sem SEH e sem local lida antes de gravada (a 0x85da00 reaproveita as vagas dos argumentos como a
# original, aqui variáveis). x87 nas três (somas, produtos, raiz e comparações na ordem da original, iguais em f64; as
# FDIVP conferidas pelos bytes: DE F9/DE FA dividem st(i) por st(0)). Comparadores: a 0x85da00 e a 0x922810 só a
# original (sorteios; arquivo e memória), a cópia leva os ramos; a 0x85d8a0 pelo valor (bits do double).
SAVE = False
ROTINAS = [('nota_do_estadio_neutro', 0x85da00, [0x83, 0xec, 0x0c, 0xa1, 0xe4, 0xf2, 0x9c, 0x00]),
           ('fator_da_distancia', 0x85d8a0, [0x83, 0xec, 0x10, 0x8b, 0x4c, 0x24, 0x14]),
           ('ler_o_clima', 0x922810, [0x81, 0xec, 0xf0, 0x05, 0x00, 0x00])]
DOCS = ('o estádio neutro e o clima inicial: a nota do estádio para o jogo, o fator da distância no público e a leitura do weather.cfg',
        'the neutral venue and the initial weather: the venue score for the fixture, the distance factor in the crowd and the weather.cfg reading',
        [('nota_do_estadio_neutro', 'Stadium::score_venue_for_fixture', '0x85da00', 'stadium.cpp', 'thiscall, ret 0x14',
          'o estádio nacional inglês antes de 2004 vale 0; a nota parte de 100000, muda pelas distâncias entre as cidades (marca 8) ou pelos lugares contra o público esperado (marca 0x20: seleção 60000, senão a soma dos públicos dos clubes) ou vira sorteio(lugares) (marca 4), e sai × (sorteio(100) × 0,004 + 0,8)',
          'the English national stadium before 2004 scores 0; the score starts at 100000, changes with the city distances (flag 8) or with the places against the expected crowd (flag 0x20: national team 60000, otherwise the clubs crowd sum) or becomes random(places) (flag 4), and ends × (random(100) × 0.004 + 0.8)'),
         ('fator_da_distancia', 'Stadium::get_travel_distance_factor', '0x85d8a0', 'stadium.cpp', 'thiscall, ret 0x14',
          '1 − distância × k (ao quadrado sem +0x3c), dividido por (reputação + 1)² longe e com reputação; com mínimos pela base, pelo tipo 10 (0,2), pelos clubes da mesma nação (0,05) e um geral',
          '1 − distance × k (squared without +0x3c), divided by (reputation + 1)² when far and with reputation; with minimums by the base, by type 10 (0.2), by both clubs of the same nation (0.05) and a general one'),
         ('ler_o_clima', 'Weather::load_config', '0x922810', 'weather.cpp', 'thiscall',
          'lê o CM3_DATA\\weather.cfg: quantas regiões, as regiões (0x922c40) e a tabela das cidades, com as três medidas ((sorteio(100) + sorteio(100)) × 0,5) e o vento (sorteio(3)) sorteados; erro de arquivo ou memória para o jogo',
          'reads CM3_DATA\\weather.cfg: the region count, the regions (0x922c40) and the city table, with the three readings ((random(100) + random(100)) × 0.5) and the wind (random(3)) drawn; a file or memory error stops the game')])
CAMPOS = [('estádio (0x4e bytes em [0xae23b0]) +0x38 cidade, +0x3c lugares; ponteiro/valor; lê (nota_do_estadio_neutro)',
           'stadium (0x4e bytes at [0xae23b0]) +0x38 city, +0x3c places; pointer/value; reads (nota_do_estadio_neutro)'),
          ('clube +0x69 estádio, +0x73 público médio; ponteiro/valor; lê (nota_do_estadio_neutro)',
           'club +0x69 stadium, +0x73 average crowd; pointer/value; reads (nota_do_estadio_neutro)'),
          ('cidade +0x23/+0x2b coordenadas; doubles; lê (nota_do_estadio_neutro, fator_da_distancia)',
           'city +0x23/+0x2b coordinates; doubles; reads (nota_do_estadio_neutro, fator_da_distancia)'),
          ('nação +0x80 estádio nacional; ponteiro; lê na Inglaterra [0x9cf2e4] (nota_do_estadio_neutro)',
           'nation +0x80 national stadium; pointer; reads for England [0x9cf2e4] (nota_do_estadio_neutro)'),
          ('jogo +0x1c/+0x20 clubes, +0x30 marcas (0xa0), +0x3a tipo, +0x3c marca; ponteiros/valores; lê (nota_do_estadio_neutro, fator_da_distancia)',
           'fixture +0x1c/+0x20 clubs, +0x30 flags (0xa0), +0x3a type, +0x3c flag; pointers/values; reads (nota_do_estadio_neutro, fator_da_distancia)'),
          ('nação do clube (+0x53) +0x7e reputação; valor; lê (fator_da_distancia)',
           'club nation (+0x53) +0x7e reputation; value; reads (fator_da_distancia)'),
          ('Weather [0xde2f74] +0 regiões (0x6e bytes), +4 tabela das cidades, +8 quantas regiões; ponteiros/valor; grava (ler_o_clima)',
           'Weather [0xde2f74] +0 regions (0x6e bytes), +4 city table, +8 region count; pointers/value; writes (ler_o_clima)')]
INTERESSA = ['GPF2']
