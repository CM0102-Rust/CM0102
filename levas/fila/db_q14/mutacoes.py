# Mutacoes do primeiro nome pelo peso, das cores da competicao e da vaga no cargo. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('dq14_peso', "for _ in 0..3 { // 3 sorteios pelo peso", "for _ in 0..2 { // 3 sorteios pelo peso"),
    ('dq14_cores', "    let (a, b) = if dono != 0 {", "    let (b, a) = if dono != 0 {"),
    ('dq14_vaga', "        v == 0 || v == pessoa\n", "        v == pessoa\n"),
]
