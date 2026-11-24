# Mutacoes (cx02_10), carreira com a Irlanda e a Irlanda do Norte ate 2002-08-01, com o modo de textos. Nenhuma abre
# caixa.
# - a linha de quem passa do grupo sem o link do clube: o comparador de texto acusa (as duas copas usam o auxiliar).
MUTACOES = [
    ('cx02_10_sem_link_do_clube',
     '            exe::text_add_link(text, (club.addr() + 0x38) as *const u8, 0x7d5, club.addr());\n        } else if outcome_byte == 0xfe {',
     '            // mutation: no club link\n        } else if outcome_byte == 0xfe {'),
]
