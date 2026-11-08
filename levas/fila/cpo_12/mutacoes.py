# Mutacoes (com o ramo de cada uma). A fluencia do idioma e lida na escalacao (0x7c6830 de 0x6e4e76/0x6e72ea, a
# barreira do idioma do estrangeiro): o jogador no clube estrangeiro sem o aumento do idioma (0x5d3044/0x5d340a); o
# idioma novo (o do clube estrangeiro, 0x5d3020/0x5d33e9, e os dos regens) com metade da fluencia; a tabela sem
# ordenar depois do acrescimo (o bsearch 0x7c6830/0x7c6ca0 deixa de achar idiomas: fluencias erradas e idiomas
# repetidos). O comparador da 0x7c6ca0 com registro e lado a lado (pega a primeira tambem comparando); as outras duas so
# pelo modo Rust e a carreira (o comparador da 0x7c6a30 roda so a original e confere o plano).
MUTACOES = [
    ('cpo12_idioma_sem_aumento', 'let f = (*((r + 5) as *const u8)).wrapping_add(aumento as u8);',
     'let f = *((r + 5) as *const u8); // mutacao: sem o aumento',
     'cpo_12 language improved'),
    ('cpo12_idioma_novo_pela_metade', '*((r + 5) as *mut u8) = if (f as i8) > 10 { 10 } else { f };',
     '*((r + 5) as *mut u8) = (if (f as i8) > 10 { 10 } else { f }) / 2; // mutacao: metade da fluencia',
     'cpo_12 new language'),
    ('cpo12_idiomas_sem_ordem', 'qsort(cpo12_d(this + 0x3c), cpo12_d(this + 0x44) as u32, 6, 0x7c6d40);',
     'let _ = &qsort; // mutacao: sem ordenar',
     'cpo_12 languages sorted'),
]
