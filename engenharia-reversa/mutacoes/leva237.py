# Mutações da 237a leva (itens dbu_1, dbu_2, dbu_3). Uso: python mutacoes.py mutacoes\leva237.py --carreira 2002-06-01
MUTACOES = [
    ('dbu_nome_textos', '                    1 => { sprintf(texto, 0x98f0b8, a, b); } // "%s %s" (dbu)', '                    1 => { sprintf(texto, 0x98f0b8, b, a); } // mutacao: sobrenome primeiro'),
    ('dbu_cargo', '    (0x9b6964, 0x9b6988, 0x9b695c, 0x9b6440, 4), // manager (dbu)', '    (0x9b6964, 0x9b6988, 0x9b68f8, 0x9b642c, 4), // mutacao: manager vira assistant manager'),
    ('dbu_idioma', '{ DBU1_IDIOMAS[x as usize - 1] } else { 0x9b6b60 }; // Unknown (dbu)', '{ DBU1_IDIOMAS[x as usize % 64] } else { 0x9b6b60 }; // mutacao: o idioma seguinte'),
    ('dbu_selecao', '            (2, false) => 0x9b776c, // Under 21 (dbu)', '            (2, false) => 0x9b778c, // mutacao: Under 23 no lugar de Under 21'),
    ('dbu_atributo', 'let (s, vazio) = if abreviado != 0 { (c, cv) } else { (l, lv) }; // (dbu)', 'let (s, vazio) = if abreviado == 0 { (c, cv) } else { (l, lv) }; // mutacao: abreviado e inteiro trocados'),
    ('dbu_posicoes', 'sprintf(texto, 0x9a3d8c, buf, lado); // "%s (%s)" (dbu)', 'sprintf(texto, 0x98f0b8, buf, lado); // mutacao: sem os parenteses'),
    ('dbu_regiao', 'let usa_adjetivo = if grupo { fb == 0xff } else { fb != 0xff }; // (dbu)', 'let usa_adjetivo = if grupo { fb == 0xff } else { fb == 0xff }; // mutacao: adjetivo e substantivo trocados'),
    ('dbu_continente', '    let proprio = continente + 0x23; // o adjetivo do registro (dbu)', '    let proprio = continente + 0x24; // mutacao: sem a primeira letra'),
]
