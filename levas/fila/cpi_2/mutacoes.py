# Mutacoes dos construtores das sequencias: a data um byte antes. Fora da impressao da carreira: PODEM ESCAPAR da
# carreira; o comparando acusa na primeira construcao.
MUTACOES = [
    ('cpi_vitorias_data', 'cpf2_data_padrao(this + 4); // o começo das vitórias',
     'cpf2_data_padrao(this + 3); // mutacao: a data um byte antes'),
    ('cpi_derrotas_data', 'cpf2_data_padrao(this + 0xc); // o fim das derrotas',
     'cpf2_data_padrao(this + 0xb); // mutacao: a data um byte antes'),
    ('cpi_invencibilidade_data', 'cpf2_data_padrao(this + 4); // o começo da invencibilidade',
     'cpf2_data_padrao(this + 3); // mutacao: a data um byte antes'),
]
