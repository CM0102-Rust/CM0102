# Mutacoes (cx07_8), carreira de 8 ligas ate 2004-11-01 (o sorteio olimpico em 20 de marco de 2004).
# - a Europa com 4 vagas e a Africa com 4: outra selecao entra no torneio (acusada pelo comparador lado a lado das 16 e
#   pela carreira: outros jogos em setembro);
# - os grupos com as vagas 0 e 1 trocadas (a primeira europeia no grupo B e a segunda no A; acusada pelo comparador).
MUTACOES = [
    ('cx07_8_europa_4_vagas',
     '    (KEY_CONTINENT_EUROPE, 5), (KEY_CONTINENT_AFRICA, 8), (KEY_CONTINENT_NORTH_AMERICA, 0xa),',
     '    (KEY_CONTINENT_EUROPE, 4), (KEY_CONTINENT_AFRICA, 8), (KEY_CONTINENT_NORTH_AMERICA, 0xa), // mutation'),
    ('cx07_8_grupos_trocados',
     'const OLYMPICS_GROUP_SEEDING: [usize; 16] = [0, 5, 9, 13, 1, 6, 10, 14, 2, 7, 11, 15, 3, 8, 12, 4];',
     'const OLYMPICS_GROUP_SEEDING: [usize; 16] = [1, 5, 9, 13, 0, 6, 10, 14, 2, 7, 11, 15, 3, 8, 12, 4]; // mutation'),
]
