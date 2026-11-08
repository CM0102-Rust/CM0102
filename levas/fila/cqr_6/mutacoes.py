# Mutacoes (cqr_6): a Supercopa japonesa de 2002 (marco) passa para 2003 (o ano sobe 2, na virada de 2002-024); o
# sorteio do estadio do Jogo das Estrelas some (um sorteio a menos, no construtor de 2001-000 e na virada de 2002-050);
# o mandante do Jogo das Estrelas de 2001 trocado (o ano impar vira o caso par). Os contadores ficam colados no efeito.
MUTACOES = [
    ('cqr6_supercopa_japonesa_ano_mais_2',
     '0x9eaf4c, [0x14b, 0x152, 0x159], 1, true, "cqr_6 virada da supercopa japonesa")',
     '0x9eaf4c, [0x14b, 0x152, 0x159], 2, true, "cqr_6 virada da supercopa japonesa")',
     'cqr_6 virada da supercopa japonesa'),
    ('cqr6_estadio_sem_sorteio',
     'let mut k = random(6) as u8; // cqr6: o estádio sorteado entre 6',
     'let mut k = { let _ = sortear; 0u8 }; // mutacao: sem sorteio',
     'cqr_6 estadio do jogo das estrelas'),
    ('cqr6_mandante_trocado',
     'let conferencia = if cqr6_w(this + 0x40) % 2 != 0 {',
     'let conferencia = if cqr6_w(this + 0x40) % 2 == 0 { // mutacao',
     'cqr_6 estadio do jogo das estrelas'),
]
