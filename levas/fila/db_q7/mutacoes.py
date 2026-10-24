# Mutacoes do sorteio de nomes e do nome formatado. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('dq7_primeiro', "for _ in 0..20 { // 20 sorteios do primeiro nome", "for _ in 0..19 { // 20 sorteios do primeiro nome"),
    ('dq7_segundo', "for _ in 0..20 { // 20 sorteios do segundo nome", "for _ in 0..19 { // 20 sorteios do segundo nome"),
    ('dq7_copia', "        *((destino + i) as *mut u8) = b;", "        *((destino + i) as *mut u8) = b ^ 0x20;"),
]
