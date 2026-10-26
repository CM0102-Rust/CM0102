# Mutações da 238a leva (itens dbw_1). Uso: python mutacoes.py mutacoes\leva238.py --prova-save 238
MUTACOES = [
    ('dbw_nacoes_eng', '            laco_das_chaves_dbw(|| c[3], |i| { nacao(em(b, i, 0x6d), i as u32); }); // nações do eng.lng', '            laco_das_chaves_dbw(|| c[3], |i| { nacao(em(b, i, 0x6d), i as u32 + 1); }); // mutacao: nacoes do eng.lng com i+1'),
    ('dbw_competicoes_c5', '|i| { competicao(em(b, i, 0x53), c[5].wrapping_add(i) as u32); }); // competições do eng.lng depois das c5', '|i| { competicao(em(b, i, 0x53), i as u32); }); // mutacao: sem o deslocamento c5'),
    ('dbw_lista_de_clubes', '                let lista = rd(0xae23fc); // a lista de clubes da base', '                let lista = 0u32; // mutacao: a lista de clubes ignorada'),
    ('dbw_clubes_base', '|i| { clube(em(rd(0xae23b4), i, 0x245) + 4, i as u32); }); // clubes da base', '|i| { clube(em(rd(0xae23b4), i, 0x245) + 4, i as u32 + 1); }); // mutacao: clubes da base com i+1'),
    ('dbw_continentes', '            cadeia_das_chaves_dbw(nome, &CONTINENTES_DAS_CHAVES, i as u32); // continentes da base', '            let mut t = CONTINENTES_DAS_CHAVES; t.swap(1, 2); cadeia_das_chaves_dbw(nome, &t, i as u32); // mutacao: Asia e Europa trocadas'),
]
