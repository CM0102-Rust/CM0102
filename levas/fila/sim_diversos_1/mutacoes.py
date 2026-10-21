# Mutacoes do jogador que sai da selecao, da faixa de horario e do teto da lista de observados. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('s_faixa', "                *tamanho = (*tamanho).wrapping_sub(1);", "                *tamanho = (*tamanho).wrapping_sub(0);"),
    ('h_dia', "    if dia == 6 || dia == 5 { 1 } else { 2 }", "    if dia == 6 { 1 } else { 2 }"),
    ('o_fator', "        5 => rep.wrapping_mul(550),", "        5 => rep.wrapping_mul(551),"),
]
