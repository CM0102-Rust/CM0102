# Mutacoes (cx07_2), carreira de 8 ligas ate 2002-06-01 (a Copa America de julho de 2001).
# - o mata-mata ordena as quartas colocadas no lugar das terceiras: entram duas quartas, saem as duas melhores
#   terceiras (os jogos do mata-mata mudam; acusada pela carreira: as selecoes da America do Sul jogam outros jogos);
# - o 4o do grupo vira 7 e nao 10 nas estatisticas (acusada pelo comparador lado a lado).
MUTACOES = [
    ('cx07_2_quartas_no_lugar_das_terceiras',
     '+ 2 * league_table_row::SIZE; // cx07_2: the third of each group',
     '+ 3 * league_table_row::SIZE; // mutation'),
    ('cx07_2_quarto_do_grupo_7',
     '[[5, 5, 9, 0xa], [1, 1, 1, 0xa], [5, 5, 9, 0xa]]); // cx07_2: the Copa América codes',
     '[[5, 5, 9, 7], [1, 1, 1, 0xa], [5, 5, 9, 0xa]]); // mutation'),
]
