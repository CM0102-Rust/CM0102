# Mutacoes da historia punivel, do tipo de lesao e da categoria da noticia. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('m_nenhuma', "    0 // nenhuma história de indisciplina", "    1 // nenhuma história de indisciplina"),
    ('l_empate', "            if r < melhor {", "            if r <= melhor {"),
    ('n_5000', "        5000..=5999 => 1,", "        5000..=5999 => 5,"),
]
