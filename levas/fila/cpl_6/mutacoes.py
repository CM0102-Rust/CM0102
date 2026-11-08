# Mutacoes (com o ramo de cada uma). O registro de historias sem a conta subir (o comparando acusa pela conta; a carreira
# pelas historias que a 0x757da0 deixa de achar e voltam a sair, com os sorteios delas). A historia do time expirada
# sem o sorteio (a carreira acusa pela contagem de sorteios no primeiro dia com uma historia 0xbdf no registro; o
# comparando roda a original e a copia sem contar: acusa pelo estado do sorteio). A historia do time nunca dada como
# contada (o comparando acusa pelo valor; a carreira pelas historias do time repetidas, com sorteios e registros a mais).
MUTACOES = [
    ('cpl_registrar_conta', 'cpl6_por(this + 8, n); // mais uma história',
     'cpl6_por(this + 8, n.wrapping_sub(1)); // mutacao: a conta nao sobe',
     'cpl_6 record story'),
    ('cpl_expirar_sorteio', 'let s = sorteio(0x28); // o sorteio da história do time',
     'let s = 0; let _ = sorteio; // mutacao: sem o sorteio',
     'cpl_6 expire team story'),
    ('cpl_historia_do_time', 'return 1; // a história do time já contada',
     'return 0; // mutacao: nunca contada',
     'cpl_6 team story already told'),
]
