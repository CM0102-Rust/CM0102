# Mutacoes da set_key_ids. A prova e o .qst da prova do save e a carga do FeriasTesteSave (CARGA): as chaves fixas
# guiam a criacao (cores, continentes, competicoes, clubes). As tres do eng.lng so contam se [0x9b5984] != 0 e o
# CM3_TEMP\eng.lng abrir (no jogo em ingles ficam pela base: podem escapar; ver o LEIA da leva).
MUTACOES = [
    ('dbw_nacoes_eng', '            key_ids_loop(|| c[3], |i| { nacao(em(b, i, 0x6d), i as u32); }); // nações do eng.lng',
     '            key_ids_loop(|| c[3], |i| { nacao(em(b, i, 0x6d), i as u32 + 1); }); // mutacao: nacoes do eng.lng com i+1'),
    ('dbw_competicoes_c5', '|i| { competicao(em(b, i, 0x53), c[5].wrapping_add(i) as u32); }); // competições do eng.lng depois das c5',
     '|i| { competicao(em(b, i, 0x53), i as u32); }); // mutacao: sem o deslocamento c5'),
    ('dbw_lista_de_clubes', '                let lista = rd(0xae23fc); // a lista de clubes da base',
     '                let lista = 0u32; // mutacao: a lista de clubes ignorada'),
    ('dbw_clubes_base', '|i| { clube(em(rd(0xae23b4), i, 0x245) + 4, i as u32); }); // clubes da base',
     '|i| { clube(em(rd(0xae23b4), i, 0x245) + 4, i as u32 + 1); }); // mutacao: clubes da base com i+1'),
    ('dbw_continentes', '            match_key_name_chain(nome, &CONTINENTES_DAS_CHAVES, i as u32); // continentes da base',
     '            let mut t = CONTINENTES_DAS_CHAVES; t.swap(1, 2); match_key_name_chain(nome, &t, i as u32); // mutacao: Asia e Europa trocadas'),
]
