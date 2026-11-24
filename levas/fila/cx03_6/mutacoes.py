# Mutacoes (cx03_6), carreira com as ligas da Polonia ate 2002-08-01. Nenhuma abre caixa.
# - o premio da Copa da Polonia 700.000: o comparador das rodadas acusa na criacao (e o dinheiro das rodadas muda);
# - a Copa da Liga polonesa com 5 reservas: o comparador das rodadas acusa (o banco faz parte da comparacao);
# - a final da Copa da Liga sem o premio do vencedor: o comparador da tabela acusa.
MUTACOES = [
    ('cx03_6_premio_copa_da_polonia',
     '    set_up_prize_rounds(Cup(this), 600_000)',
     '    set_up_prize_rounds(Cup(this), 700_000) // mutation'),
    ('cx03_6_copa_da_liga_cinco_reservas',
     '    cup.set_substitutes_named(7);\n    cup.set_substitutions_allowed(3);\n    r',
     '    cup.set_substitutes_named(5); // mutation\n    cup.set_substitutions_allowed(3);\n    r'),
    ('cx03_6_final_sem_premio',
     "    cql12r((5, 3, 1, 3), (9, 4, 1, 2, 2, 0), 0x96, (0, 0, 1), 6, (2, 1, 0, 0), (0, 2, 8), (0, 0x1d4c0, 0xea60)),\n];\n\n/// The Polish League Cup of the other years",
     "    cql12r((5, 3, 1, 3), (9, 4, 1, 2, 2, 0), 0x96, (0, 0, 1), 6, (2, 1, 0, 0), (0, 2, 8), (0, 0, 0xea60)), // mutation\n];\n\n/// The Polish League Cup of the other years"),
]
