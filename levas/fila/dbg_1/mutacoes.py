# Mutacoes da gravacao dos .dat: cada uma muda um campo (nao id, chave nem ponteiro) de todos os registros
# antes de gravar; a tabela gravada vai para o .qst da prova do save, que acusa pelos bytes.
MUTACOES = [
    ('dbg_continentes', '    crate::comparison::branch("banco gravar continentes");', '    crate::comparison::branch("banco gravar continentes");\n    for i in 0..n as usize { let r = registros + i * 198; let p = (r + 0xbe) as *mut f64; p.write_unaligned(p.read_unaligned() + 1.0); } // mutacao: a força regional (+0xbe, double) + 1 (fica na memória também; não é id, chave nem ponteiro)'),
    ('dbg_nacoes', '    crate::comparison::branch("banco gravar nações");', '    crate::comparison::branch("banco gravar nações");\n    for i in 0..n as usize { let r = registros + i * 290; let p = (r + 0xa8) as *mut f64; p.write_unaligned(p.read_unaligned() + 1.0); } // mutacao: o coeficiente FIFA (+0xa8, double) + 1 (fica na memória também; não é id, chave nem ponteiro)'),
    ('dbg_cidades', '    crate::comparison::branch("banco gravar cidades");', '    crate::comparison::branch("banco gravar cidades");\n    for i in 0..n as usize { let r = registros + i * 56; *((r + 0x33) as *mut u8) = (*((r + 0x33) as *const u8)).wrapping_add(1); } // mutacao: a atração (+0x33) + 1 (fica na memória também; não é id, chave nem ponteiro)'),
]
