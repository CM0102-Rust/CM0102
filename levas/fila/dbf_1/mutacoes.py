# Mutacoes da leitura dos .dat: cada uma muda um campo de todos os registros lidos (sem tocar ids,
# nomes-chave nem ponteiros, para o jogo nao parar numa caixa); a tabela vai para o .qst da prova do save.
MUTACOES = [
    ('dbf_continentes', '    (lidos == n) as i32 // continentes', '    for i in 0..n as usize { let r = registros + i * 198; let p = (r + 0xbe) as *mut f64; p.write_unaligned(p.read_unaligned() + 1.0); } // mutacao: a força regional (+0xbe, double) + 1 (o nome da federação é refeito pela tradução)\n    (lidos == n) as i32 // continentes'),
    ('dbf_nacoes', '    (lidos == n) as i32 // nações', '    for i in 0..n as usize { let r = registros + i * 290; let p = (r + 0xa8) as *mut f64; p.write_unaligned(p.read_unaligned() + 1.0); } // mutacao: o coeficiente FIFA (+0xa8, double) + 1\n    (lidos == n) as i32 // nações'),
    ('dbf_cidades', '    (lidos == n) as i32 // cidades', '    for i in 0..n as usize { let r = registros + i * 56; *((r + 0x33) as *mut u8) = (*((r + 0x33) as *const u8)).wrapping_add(1); } // mutacao: a atração (+0x33) + 1\n    (lidos == n) as i32 // cidades'),
]
