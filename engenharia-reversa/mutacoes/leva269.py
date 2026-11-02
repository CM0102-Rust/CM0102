# Mutações da 269a leva (itens cqc_6). Uso: python mutacoes.py mutacoes\leva269.py --carreira 2002-06-01
MUTACOES = [
    ('cqc6_bonus_ao_mandante', 's = sorteio(cqc6_w(cqc6_d(f + 0x20) + 0x80) as i32 + 0x5dc); // cqc6: o visitante com o bônus', 's = sorteio(cqc6_w(cqc6_d(f + 0x1c) + 0x80) as i32 + 0x5dc); // mutacao: o mandante com o bonus', 'cqc_6 jogo para a TV'),
    ('cqc6_tv_no_dia_seguinte', 'cqc6_pw(r + 0x2a, cqc6_w(pa) as u16); // cqc6: o dia da TV', 'cqc6_pw(r + 0x2a, cqc6_w(pa).wrapping_add(1) as u16); // mutacao: o dia seguinte ao da TV', 'cqc_6 jogo para a TV'),
    ('cqc6_conflito_nao_zera', 's = 0; // cqc6: conflito na janela', '// mutacao: o conflito na janela nao zera a nota', 'cqc_6 conflito na janela'),
]
