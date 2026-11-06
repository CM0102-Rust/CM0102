# Mutacoes (cqr_4). O play-off e a final da MLS de 2001 rodam uma vez cada na carreira de 8 ligas ate 2002-06-01
# (contadores "cqr_4 play-off" e "cqr_4 final"); as tres mudam quem joga com quem:
# - o play-off com o sexto melhor dos outros no lugar do quinto: outro clube no play-off;
# - o play-off com o quarto e o quinto melhores dos outros trocados de vaga: o melhor e o segundo campeao de divisao
#   enfrentam outros adversarios;
# - a final com os finalistas trocados (o segundo vencedor primeiro na lista): outro mandante [inferencia], outro jogo.
MUTACOES = [
    ('cqr4_play_off_com_o_sexto',
     'poe(0, cqr4_clube(&r, 4)); // cqr4: o quinto melhor dos outros',
     'poe(0, cqr4_clube(&r, 5)); // mutacao: o sexto melhor dos outros',
     'cqr_4 play-off'),
    ('cqr4_play_off_pares_trocados',
     '    poe(2, cqr4_clube(&r, 3));\n    poe(0, cqr4_clube(&r, 4));',
     '    poe(2, cqr4_clube(&r, 4)); // mutacao: pares trocados\n    poe(0, cqr4_clube(&r, 3));',
     'cqr_4 play-off'),
    ('cqr4_final_trocada',
     'core::ptr::write_unaligned((lista + 4 * n as usize) as *mut usize, cqr4_d(e)); // cqr4: os finalistas',
     'core::ptr::write_unaligned((lista + 4 * (1 - n) as usize) as *mut usize, cqr4_d(e)); // mutacao: finalistas trocados',
     'cqr_4 final'),
]
