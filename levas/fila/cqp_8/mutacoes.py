# Mutacoes (cqp_8). A virada de cada estadual roda uma vez na carreira de 8 ligas ate 2002-06-01 (o estadual de 2002, que
# se joga de janeiro a maio): cada mutacao passa o ano dois adiante em vez de um. As regras e os jogos do estadual
# saem para 2003, e os jogos de 2002 nao acontecem antes de 2002-06-01 (outra contagem de partidas e de sorteios).
# O contador fica logo depois do ano gravado, com o nome de cada estadual.
MUTACOES = [
    ('cqp8_goiano_dois_anos',
     '[0x1c3, 0x1ca, 0x1d0, 0x1d7], 1, "cqp_8 virada do goiano")',
     '[0x1c3, 0x1ca, 0x1d0, 0x1d7], 2, "cqp_8 virada do goiano") /* mutacao: dois anos adiante */',
     'cqp_8 virada do goiano'),
    ('cqp8_norte_dois_anos',
     '[0x1ca, 0x1d1, 0x1d7, 0x1de], 1, "cqp_8 virada do norte")',
     '[0x1ca, 0x1d1, 0x1d7, 0x1de], 2, "cqp_8 virada do norte") /* mutacao: dois anos adiante */',
     'cqp_8 virada do norte'),
    ('cqp8_nordeste_dois_anos',
     '[0x1bc, 0x1c3, 0x1c9, 0x1d0], 1, "cqp_8 virada do nordeste")',
     '[0x1bc, 0x1c3, 0x1c9, 0x1d0], 2, "cqp_8 virada do nordeste") /* mutacao: dois anos adiante */',
     'cqp_8 virada do nordeste'),
]
# Prova do agente (06/11, carreira Rust de 8 ligas ate 2002-06-01, base com os 9 itens cqo/cqp/cqq_7..9): as tres ACUSADAS (a virada roda em 2002-011; separam em 2002-013 a 2002-020).
