# Mutacoes (sx05_1). O texto e o valor da tela nao voltam para a simulacao: acusadas pelos comparadores (o valor e o
# texto lado a lado), nos trechos que as noticias de contrato e de transferencia da carreira ENG rodam (bandeiras 4..6).
# - o valor da tela truncado ao passo em vez de arredondado;
# - o passo de 25.000 entre 250.000 e 1 milhao virando 50.000;
# - as decimais do valor abreviado sempre com uma casa so.
MUTACOES = [
    ('sx05_1_sem_meio_passo',
     '    (step / 2 + amount) / step * step\n',
     '    amount / step * step // mutation: truncated to the step\n',
     'sx05_1 display amount rounded'),
    ('sx05_1_passo_de_um_milhao',
     '(1000000.0, 25000.0), (5000000.0, 100000.0), (20000000.0, 250000.0), (50000000.0, 500000.0),\n];',
     '(1000000.0, 50000.0), (5000000.0, 100000.0), (20000000.0, 250000.0), (50000000.0, 500000.0),\n]; // mutation: 50,000 step',
     'sx05_1 rounding step ladder'),
    ('sx05_1_uma_decimal',
     "let decimals = if d[whole + 1] != b'0' { 2 }",
     "let decimals = if d[whole + 1] != b'0' { 1 } /* mutation: one decimal */",
     'sx05_1 money abbreviated'),
]
