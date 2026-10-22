# Mutacoes da ordem da tabela, da grade de turnos e da margem do ultimo dia. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('lt_pontos', "        return pb - pa; // pontos", "        return pa - pb; // pontos"),
    ('lt_grade', "                    *c = k; // vaga preenchida", "                    *c = k + 1; // vaga preenchida"),
    ('lt_ultimo_dia', "        return 0xff; // fora do último dia", "        return 0xfe; // fora do último dia"),
]
