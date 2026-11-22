# Mutacoes (tel39_7). Acusada pelo comparador do construtor (lado a lado): o construtor nao copia o texto inicial
# (todo Text construido com texto). O destruidor e o esvaziar nao tem mutacao segura: um ponteiro esquecido usaria
# memoria liberada (o esvaziar e conferido pelo comparador depois da original).
MUTACOES = [
    ('tel39_7_sem_texto',
     '    if source != 0 {\n        text_set_string(this, source);',
     '    if source != 0 && false {\n        text_set_string(this, source);',
     'tel39_7 construct a text'),
]
