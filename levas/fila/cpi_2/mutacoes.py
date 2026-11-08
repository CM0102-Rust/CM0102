# Mutacoes dos construtores das sequencias: a data um byte antes. Fora da impressao da carreira: PODEM ESCAPAR da
# carreira; o comparando acusa na primeira construcao.
MUTACOES = [
    ('cpi_vitorias_data', 'init_default_date(this + 4); // o começo das vitórias',
     'init_default_date(this + 3); // mutacao: a data um byte antes'),
    ('cpi_derrotas_data', 'init_default_date(this + 0xc); // o fim das derrotas',
     'init_default_date(this + 0xb); // mutacao: a data um byte antes'),
    ('cpi_invencibilidade_data', 'init_default_date(this + 4); // o começo da invencibilidade',
     'init_default_date(this + 3); // mutacao: a data um byte antes'),
]
