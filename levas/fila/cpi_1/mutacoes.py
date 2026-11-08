# Mutacoes dos construtores dos recordes do clube: a data um byte antes (pisa o campo vizinho). Os recordes do clube
# ficam fora da impressao da carreira: PODEM ESCAPAR da carreira; o comparando acusa na primeira construcao.
MUTACOES = [
    ('cpi_valor_pago_data', 'init_default_date(this + 0x14); // a data do valor pago',
     'init_default_date(this + 0x13); // mutacao: a data um byte antes'),
    ('cpi_valor_recebido_data', 'init_default_date(this + 0x20); // a 2ª data do valor recebido',
     'init_default_date(this + 0x1f); // mutacao: a data um byte antes'),
    ('cpi_sequencia_data', 'init_default_date(this + 4); // o começo da sequência',
     'init_default_date(this + 3); // mutacao: a data um byte antes'),
]
