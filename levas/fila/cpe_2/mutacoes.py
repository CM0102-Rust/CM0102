# Mutacoes da tabela da liga. A do resultado dobra os pontos da vitoria do mandante na tabela (todo resultado de liga,
# 8.322 chamadas): a carreira ate 2002-06-01 deve acusar. A dos pontos do resultado da um ponto a mais por vitoria
# (a virtual +0x90, em toda vitoria de liga): deve acusar.
MUTACOES = [
    ('cpe_resultado_pontos_casa', 'cpe2_somar(r + 0x27, pontos(cpe2_d(p + 0x1c))); // os pontos do mandante',
     'cpe2_somar(r + 0x27, pontos(cpe2_d(p + 0x1c)).wrapping_mul(2)); // mutacao: o dobro'),
    ('cpe_pontos_vitoria', 'if venceu(pb(0x43), pb(0x44)) { ganho(0xc2) } else { 0 } // os pontos da vitória',
     'if venceu(pb(0x43), pb(0x44)) { ganho(0xc2).wrapping_add(1) } else { 0 } // mutacao: um a mais'),
]
