# Mutacoes (sbI_4), prova do save (carga): o construtor sem ler o ano do save; o ano das inscricoes fica 0 depois da
# carga (a impressao da carga e a gravacao seguinte mudam). Nao abre caixa.
MUTACOES = [
    ('sbI_4_sem_ler_o_ano',
     '        load(this, save); // sbI_4: the year from the saved game',
     '        let _ = load; // mutation',
     'sbI_4 squad manager from the saved game'),
]
