# Mutacoes da familia da cor, das cores que se confundem e do atributo calculado do nao jogador. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('dq12_familia', "        Some(f) => f,", "        Some(f) => f % 10 + 1,"),
    ('dq12_confundem', "    if fa == fb || par(8, 9)", "    if fa != fb || par(8, 9)"),
    ('dq12_atributo', "(b + b)) * 0.05 + 10.0;", "(b + b)) * 0.05 + 11.0;"),
]
