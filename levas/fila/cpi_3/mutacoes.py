# Mutacoes dos construtores: a data um byte antes (ou a idade sem montar). Fora da impressao da carreira: PODEM ESCAPAR
# da carreira; o comparando acusa na primeira construcao.
MUTACOES = [
    ('cpi_sem_vencer_data', 'init_default_date(this + 0xc); // o fim dos jogos sem vencer',
     'init_default_date(this + 0xb); // mutacao: a data um byte antes'),
    ('cpi_gols_data', 'init_default_date(this + 0x19); // a data da partida',
     'init_default_date(this + 0x18); // mutacao: a data um byte antes'),
    ('cpi_mais_novo_idade', 'init_default_date(this); // a idade do mais novo',
     '// mutacao: a idade sem montar'),
]
