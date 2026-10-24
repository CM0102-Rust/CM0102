# Mutacoes da leitura dos .dat (caminho da versao 2, o da base de fabrica): cada uma muda um campo de todos
# os registros lidos (sem tocar ids, nomes-chave nem ponteiros, para o jogo nao parar numa caixa); a tabela
# vai para o .qst da prova do save. O caminho da versao 1 (conversores, pelo endereco) a base nao exercita.
MUTACOES = [
    ('dbf_clubes', '    1 // clubes', '    for i in 0..n as usize { let p = (registros + i * 581 + 0x73) as *mut i32; p.write_unaligned(p.read_unaligned().wrapping_add(1)); } // mutacao: o público (+0x73) + 1\n    1 // clubes'),
    ('dbf_competicoes', '    (lidos == n) as i32 // competições de clube', '    for i in 0..n as usize { let r = registros + i * 107; let p = (r + 0x69) as *mut i16; p.write_unaligned(p.read_unaligned().wrapping_add(1)); } // mutacao: a reputação (+0x69) + 1\n    (lidos == n) as i32 // competições de clube'),
    ('dbf_nao_jogadores', '    (lidos == n) as i32 // não jogadores', '    for i in 0..n as usize { let r = registros + i * 68; *((r + 0x10) as *mut u8) = (*((r + 0x10) as *const u8)).wrapping_add(1); } // mutacao: o treino (coaching, +0x10) + 1\n    (lidos == n) as i32 // não jogadores'),
]
