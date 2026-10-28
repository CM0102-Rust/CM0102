# Mutacoes dos construtores: a data um byte antes (ou a idade sem montar). Fora da impressao da carreira: PODEM ESCAPAR
# da carreira; o comparando acusa na primeira construcao.
MUTACOES = [
    ('cpi_sem_vencer_data', 'cpf2_data_padrao(this + 0xc); // o fim dos jogos sem vencer',
     'cpf2_data_padrao(this + 0xb); // mutacao: a data um byte antes'),
    ('cpi_gols_data', 'cpf2_data_padrao(this + 0x19); // a data da partida',
     'cpf2_data_padrao(this + 0x18); // mutacao: a data um byte antes'),
    ('cpi_mais_novo_idade', 'cpf2_data_padrao(this); // a idade do mais novo',
     '// mutacao: a idade sem montar'),
]
