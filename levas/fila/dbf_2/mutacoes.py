# Mutacoes da leitura dos .dat: cada uma muda um campo de todos os registros lidos (sem tocar ids,
# nomes-chave nem ponteiros, para o jogo nao parar numa caixa); a tabela vai para o .qst da prova do save.
MUTACOES = [
    ('dbf_estadios', '    (lidos == n) as i32 // estádios', '    for i in 0..n as usize { let r = registros + i * 78; let p = (r + 0x3c) as *mut i32; p.write_unaligned(p.read_unaligned().wrapping_add(1)); } // mutacao: a capacidade (+0x3c) + 1\n    (lidos == n) as i32 // estádios'),
    ('dbf_pessoas_157', '    (lidos == n) as i32 // pessoas 157', '    for i in 0..n as usize { let r = registros + i * 157; let p = (r + 0x18) as *mut i16; p.write_unaligned(p.read_unaligned().wrapping_add(1)); } // mutacao: o ano de nascimento (+0x18) + 1\n    (lidos == n) as i32 // pessoas 157'),
    ('dbf_historico_de_pessoas', '    (lidos == n) as i32 // histórico de pessoas', '    for i in 0..n as usize { let r = registros + i * 17; *((r + 0xf) as *mut u8) = (*((r + 0xf) as *const u8)).wrapping_add(1); } // mutacao: os jogos (+0xf) + 1\n    (lidos == n) as i32 // histórico de pessoas'),
]
