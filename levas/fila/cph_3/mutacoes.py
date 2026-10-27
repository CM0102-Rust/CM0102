# Mutacoes da copia do contrato, da moral e das conversas. A da copia deixa o 5o byte (+0x20) sem copiar: deve acusar
# se o campo decidir algo na carreira; senao o comparando acusa na primeira copia. As da moral (teto de 90 sem queixa;
# felicidade + 6 no ganho) mudam a moral dos jogadores: devem acusar (a moral esta no contrato do banco). A das
# conversas ignora o clube dado: tira conversas de mais; deve acusar (o comparando acusa na primeira com clube).
MUTACOES = [
    ('cph_copia_bytes', 'for o in 0x1c..0x21 {\n        b(o); // os cinco bytes',
     'for o in 0x1c..0x20 {\n        b(o); // mutacao: so quatro'),
    ('cph_moral_teto', 'let mut teto: i8 = if queixas & 2 != 0 { 50 } else { 100 };',
     'let mut teto: i8 = if queixas & 2 != 0 { 50 } else { 90 }; // mutacao'),
    ('cph_moral_ganho', 'let felicidade = (cph3_b(contrato + 0x3f) as i8 as i32 + 5) as f64; // +0x3f + 5',
     'let felicidade = (cph3_b(contrato + 0x3f) as i8 as i32 + 6) as f64; // mutacao: + 6'),
    ('cph_conversas_clube', '&& (clube == 0 || cph3_d(r + 4) == cph3_d(clube)) // o clube',
     '&& (clube == 0 || true) // mutacao: sem o clube'),
]
