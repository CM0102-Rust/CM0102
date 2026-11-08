# Mutacoes do alcance da injury_word_0c, da injury_word_0c vista por alguem e de estrangeiro. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('dq1_mesma_nacao', "        return 2; // mesma nação", "        return 3; // mesma nação"),
    ('dq1_reputacao_atual', "        2 => r(0), // escopo 2: a atual", "        2 => r(2), // escopo 2: a atual"),
    ('dq1_estrangeira', "    1 // estrangeira", "    0 // estrangeira"),
]
