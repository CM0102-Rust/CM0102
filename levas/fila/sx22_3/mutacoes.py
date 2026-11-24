# Mutacoes (sx22_3, TELAS: a cadeia nao as roda; para provar o comparador numa sessao do Lucas).
# - Com vento e chuva, o texto sai sem a chuva: o comparador acusa a primeira partida com os dois.
# - O gramado com neve sai como congelado: o comparador acusa a primeira tela do clube com neve prevista.
MUTACOES = [
    ('sx22_3_sem_a_chuva',
     '        crt(out, 0xad4e88, wp, rp, tp); // sx22_3: "%s, %s, %s"',
     '        crt(out, 0x98f6ac, wp, tp); // mutation: without the rain'),
    ('sx22_3_neve_congelada',
     '        0x20 => fmt(out, 0xad4f84), // "Snowy" (sx22_3)',
     '        0x20 => fmt(out, 0xad4f8c), // mutation: "Frozen"'),
]
