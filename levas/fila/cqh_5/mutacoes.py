# Mutacoes (cqh_5), nas rotinas da liga: as expectativas da temporada (0x68aa80) e a cadeia de promocoes (0x689c20):
# - as expectativas sem a noticia (0x6a3770 com 0): [inferencia] a noticia vai so ao tecnico humano e pode escapar;
# - so um clube em cada dois recebe as expectativas: a diretoria dos outros fica com as de antes;
# - a troca entre ligas sempre aos pares: so roda no fim da temporada, depois de 2002-06-01 (escapa na carreira
#   automatica; fica pela documentacao).
MUTACOES = [
    ('cqh5_expectativas_sem_noticia',
     'expectativas(cqh5_d(0xb67a44), clube, 1); // cqh5: com a notícia',
     'expectativas(cqh5_d(0xb67a44), clube, 0); // mutacao: sem a noticia',
     'cqh_5 expectativa do clube'),
    ('cqh5_um_clube_em_dois',
     'i += 1; // cqh5: o próximo clube',
     'i += 2; // mutacao: um em dois',
     'cqh_5 expectativa do clube'),
    ('cqh5_troca_sempre_aos_pares',
     'trocar(this, acima, abaixo, 1, aos_pares, -1, -1);',
     'trocar(this, acima, abaixo, 1, 1, -1, -1); // mutacao: sempre aos pares',
     'cqh_5 troca entre ligas'),
]
