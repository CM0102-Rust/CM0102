# Mutacoes da gravacao dos .dat: cada uma muda um campo de valor (nao id, nao chave, nao ponteiro) de todos
# os registros antes de gravar; a tabela gravada vai para o .qst da prova do save.
MUTACOES = [
    ('dbh_gravar_competicoes_de_pessoal', '    crate::comparison::branch("database write staff competitions");', '    crate::comparison::branch("database write staff competitions");\n    for i in 0..n as usize { let r = registros + i * 101; let p = (r + 0x63) as *mut i16; p.write_unaligned(p.read_unaligned().wrapping_add(1)); } // mutacao: a reputação (+0x63) + 1 antes de gravar'),
    ('dbh_gravar_arbitros', '    crate::comparison::branch("database write referees");', '    crate::comparison::branch("database write referees");\n    for i in 0..n as usize { let r = registros + i * 43; let p = (r + 0x22) as *mut i16; p.write_unaligned(p.read_unaligned().wrapping_add(1)); } // mutacao: a reputação (+0x22) + 1 antes de gravar'),
    ('dbh_gravar_registros_58', '    crate::comparison::branch("database write 58-byte records");', '    crate::comparison::branch("database write 58-byte records");\n    for i in 0..n as usize { let r = registros + i * 58; *((r + 0x37) as *mut u8) = (*((r + 0x37) as *const u8)).wrapping_add(1); } // mutacao: o byte +0x37 (o vermelho nas cores) (+0x37) + 1 antes de gravar'),
]
