# Mutacoes da gravacao dos .dat: cada uma muda um campo (nao id, chave nem ponteiro) de todos os registros
# antes de gravar; a tabela gravada vai para o .qst da prova do save, que acusa pelos bytes.
MUTACOES = [
    ('dbg_estadios', '    crate::comparison::branch("banco gravar estádios");', '    crate::comparison::branch("banco gravar estádios");\n    for i in 0..n as usize { let r = registros + i * 78; let p = (r + 0x3c) as *mut i32; p.write_unaligned(p.read_unaligned().wrapping_add(1)); } // mutacao: a capacidade (+0x3c) + 1 (fica na memória também; não é id, chave nem ponteiro)'),
    ('dbg_clubes', '    crate::comparison::branch("banco gravar clubes");', '    crate::comparison::branch("banco gravar clubes");\n    for i in 0..n as usize { let r = registros + i * 581; let p = (r + 0x73) as *mut i32; p.write_unaligned(p.read_unaligned().wrapping_add(1)); } // mutacao: o público (+0x73) + 1 (fica na memória também; não é id, chave nem ponteiro)'),
    ('dbg_pessoas_110', '    crate::comparison::branch("banco gravar pessoas 110");', '    crate::comparison::branch("banco gravar pessoas 110");\n    for i in 0..n as usize { let r = registros + i * 110; let p = (r + 0x52) as *mut i32; p.write_unaligned(p.read_unaligned().wrapping_add(1)); } // mutacao: o valor (+0x52) + 1 (fica na memória também; não é id, chave nem ponteiro)'),
]
