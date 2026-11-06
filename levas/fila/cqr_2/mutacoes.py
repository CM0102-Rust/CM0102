# Mutacoes (cqr_2). A virada de cada estadual roda uma vez na carreira de 8 ligas ate 2002-06-01 (o estadual de 2002,
# jogado de janeiro a maio): cada mutacao passa o ano dois adiante em vez de um (como na cqp_7). As regras e os jogos
# do estadual saem para 2003, e os jogos de 2002 nao acontecem antes de 2002-06-01 (outra contagem de partidas e de
# sorteios). O contador fica logo depois do ano gravado, com o nome de cada estadual.
MUTACOES = [
    ('cqr2_carioca_dois_anos',
     '0x98b014, 1, "cqr_2 virada do carioca")',
     '0x98b014, 2, "cqr_2 virada do carioca") /* mutacao: dois anos adiante */',
     'cqr_2 virada do carioca'),
    ('cqr2_mineiro_dois_anos',
     '0x98aed0, 1, "cqr_2 virada do mineiro")',
     '0x98aed0, 2, "cqr_2 virada do mineiro") /* mutacao: dois anos adiante */',
     'cqr_2 virada do mineiro'),
    ('cqr2_paranaense_dois_anos',
     '0x98af98, 1, "cqr_2 virada do paranaense")',
     '0x98af98, 2, "cqr_2 virada do paranaense") /* mutacao: dois anos adiante */',
     'cqr_2 virada do paranaense'),
]
# Prova do agente (06/11, carreira Rust de 8 ligas ate 2002-06-01; base com os 12 itens cqr/cqs/cqt_1..4, impressao b690295afd222f6a): as tres ACUSADAS (a virada roda em 2002-011; divergem em 2002-013).
