# Mutacoes, todas no fisico do dia (0x62c680: toda pessoa que joga, uma vez por dia em que a escalacao, o motor ou o
# estado da lesao perguntam). A forma (+8) e a match_condition de jogo (+0xa) entram na escalacao e no motor das partidas; o
# sorteio(2) da pre-temporada muda a contagem de sorteios. A recuperacao por dia livre roda em quase todo jogador todo
# dia; a perda por dia parado em todo lesionado (e nas ferias); o sorteio da forma no clube fora da temporada (julho de
# 2001 e o fim de maio de 2002), com forma x match_condition abaixo de 80%.
MUTACOES = [
    ('cpw8_recuperacao_13',
     'let s = (1000 - 12 * v as i32) as i16 as i32; // cpw8: a recuperação por dia livre',
     'let s = (1000 - 13 * v as i32) as i16 as i32; // mutacao: 13 por ponto',
     'cpw_8 recovery'),
    ('cpw8_perda_60',
     'let mut x = f.wrapping_sub(66i32.wrapping_mul(l34)); // cpw8: a perda por dia parado',
     'let mut x = f.wrapping_sub(60i32.wrapping_mul(l34)); // mutacao: perda de 60',
     'cpw_8 loss'),
    ('cpw8_sorteio_da_forma_70',
     'if mc.wrapping_mul(f) / 10000 < 8000 { // cpw8: a forma abaixo de 80% na pré-temporada',
     'if mc.wrapping_mul(f) / 10000 < 7000 { // mutacao: abaixo de 70%',
     'cpw_8 form draw'),
]
