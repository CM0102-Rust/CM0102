# Mutacoes das noticias nao lidas, da marca na noticia e do clube de tecnico humano. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('nt_total', "        return *total; // total da caixa", "        return *total + 1; // total da caixa"),
    ('nt_marca', "valor); // marca na caixa", "valor ^ 1); // marca na caixa"),
    ('nt_humano', "    1 // humano\n", "    0 // humano\n"),
]
