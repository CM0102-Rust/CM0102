# Mutacoes (sx21b_11). Os ordinais e os numeros por extenso entram nos textos das noticias e das partidas da
# carreira ENG; caem pelos comparadores (o buffer do texto lado a lado).
MUTACOES = [
    ('sx21b_11_primeiro_nd',
     '            1 => 0xad4200, // sx21b_11: "st"',
     '            1 => 0xad41f0, // mutation: "nd"',
     'sx21b_11 English ordinal'),
    ('sx21b_11_ate_dezenove',
     '    if n <= 0x14 { // sx21b_11: words up to twenty',
     '    if n <= 0x13 { // mutation: up to nineteen',
     'sx21b_11 number in words'),
    ('sx21b_11_quebra_a_direita',
     '            write_u8(text + r as usize, b\'\\n\'); // sx21b_11: the break to the right',
     '            write_u8(text + r as usize, b\'\\r\'); // mutation: a carriage return',
     'sx21b_11 line broken to the right'),
]
