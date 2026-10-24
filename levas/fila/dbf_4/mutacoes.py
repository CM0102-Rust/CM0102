# Mutacoes da leitura dos .dat: cada uma muda um campo de todos os registros lidos (sem tocar ids,
# nomes-chave nem ponteiros, para o jogo nao parar numa caixa); a tabela vai para o .qst da prova do save.
MUTACOES = [
    ('dbf_arbitros', '    (lidos == n) as i32 // árbitros', '    for i in 0..n as usize { let r = registros + i * 43; let p = (r + 0x22) as *mut i16; p.write_unaligned(p.read_unaligned().wrapping_add(1)); } // mutacao: a reputação (+0x22) + 1\n    (lidos == n) as i32 // árbitros'),
    ('dbf_cores', '    (lidos == n) as i32 // cores', '    for i in 0..n as usize { let r = registros + i * 58; *((r + 0x37) as *mut u8) = (*((r + 0x37) as *const u8)).wrapping_add(1); } // mutacao: o vermelho (+0x37) + 1\n    (lidos == n) as i32 // cores'),
    ('dbf_nomes', '    (lidos == n) as i32 // nomes', '    for i in 0..n as usize { let r = registros + i * 60; *((r + 0x3b) as *mut u8) = (*((r + 0x3b) as *const u8)).wrapping_add(1); } // mutacao: a contagem (+0x3b) + 1\n    (lidos == n) as i32 // nomes'),
]
