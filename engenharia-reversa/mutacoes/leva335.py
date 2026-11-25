# Mutações da 335a leva (itens sx12_3). Uso: python mutacoes.py mutacoes\leva335.py --carreira 2002-06-01
MUTACOES = [
    ('sx12_nacao_do_meio', 'let middle = (count as i8 as i32 + 1) / 2; // sx12_3: the middle nation', 'let middle = (count as i8 as i32 + 3) / 2; // mutation: the nation after the middle one', 'sx12_3 split letter'),
    ('sx12_cabecalho_segunda', 'story_lang!(text, 0x9eb0b0, letter + 1); // sx12_3: "Nations <%c - Character>-Z"', 'story_lang!(text, 0x9eb0b0, letter); // mutation: the split letter itself', 'sx12_3 second part header'),
    ('sx12_lado_da_nacao', '(first as i8 > read_i8(this)) as u8 // sx12_3: after the split letter', '(first as i8 >= read_i8(this)) as u8 // mutation: from the split letter on', 'sx12_3 second part'),
]
