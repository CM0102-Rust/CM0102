# Mutacoes (sx04c_23), para o comparador numa sessao do Lucas (a carreira nao abre a tela dos premios): a decisao da
# copia contra a montagem da original.
# - a Espanha sem a equipe da semana da Segunda Divisao;
# - a Escocia sem a da Third Division.
MUTACOES = [
    ('sx04c_23_espanha',
     'const SPAIN_TEAMS_OF_THE_WEEK: [usize; 2] = [2, 5];',
     'const SPAIN_TEAMS_OF_THE_WEEK: [usize; 2] = [2, 2]; // mutation'),
    ('sx04c_23_escocia',
     'const SCOTLAND_TEAMS_OF_THE_WEEK: [usize; 4] = [3, 9, 15, 21];',
     'const SCOTLAND_TEAMS_OF_THE_WEEK: [usize; 4] = [3, 9, 15, 15]; // mutation'),
]
