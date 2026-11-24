# Mutacoes (sx08_1).
# - a ordem por idade invertida no ano: o mais novo primeiro (o comparador acusa em cada qsort dos numeros de camisa;
#   a carreira so se o numero pesar adiante);
# - a data das novas regras 7 dias antes (24/08/2001): a noticia do novo sistema e os contratos assinados entre 24 e 31
#   de agosto mudam de regra (a carreira passa pela data; o comparador acusa na partida do jogo).
MUTACOES = [
    ('sx08_1_idade_invertida',
     '        return if year_b > year_a { -1 } else { 1 };',
     '        return if year_b > year_a { 1 } else { -1 }; // mutation: youngest first'),
    ('sx08_1_regras_antes',
     '    exe::date_date_549f70(date as *mut [u16; 4], 0x1f, 7, 0x7d1, u32::MAX); // 31 August 2001',
     '    exe::date_date_549f70(date as *mut [u16; 4], 0x18, 7, 0x7d1, u32::MAX); // mutation: 24 August 2001'),
]
