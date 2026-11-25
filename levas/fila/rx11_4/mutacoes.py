# Mutacoes (rx11_4), ENG ate 2005-12-31 (a campanha de 2006); acusadas pelos comparadores lado a lado:
# - os grupos de cinco com o segundo 7 no lugar de 6 (a faixa do segundo muda);
# - a situacao "classificado" so com o codigo 2 (todas as selecoes mudam de situacao);
# - a mudanca de situacao: quem deixou o codigo 1 conta como estavel (0).
MUTACOES = [
    ('rx11_4_segundo_7',
     'all(&b, &b, &b) // rx11_4: five groups of five',
     'all(&[1, 7, 0xb, 0x10, 0x15], &b, &b) // mutation: runner-up 7',
     'rx11_4 african progress'),
    ('rx11_4_classificado_codigo_2',
     'if qualifying_stat_byte(this, club, 0x12) == 1 { -2 } else { -3 } // rx11_4: qualified at code 1',
     'if qualifying_stat_byte(this, club, 0x12) == 2 { -2 } else { -3 } // mutation: code 2',
     'rx11_4 qualification status'),
    ('rx11_4_saida_do_1_estavel',
     '(before == 1) as i8 // rx11_4: left code 1',
     '0 // mutation: never up',
     'rx11_4 african status change'),
]
