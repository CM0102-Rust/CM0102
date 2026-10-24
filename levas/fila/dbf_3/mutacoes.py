# Mutacoes da leitura dos .dat: cada uma muda um campo de todos os registros lidos (sem tocar ids,
# nomes-chave nem ponteiros, para o jogo nao parar numa caixa); a tabela vai para o .qst da prova do save.
MUTACOES = [
    ('dbf_historicos_de_competicao', '    (lidos == n) as i32 // históricos de competição', '    for i in 0..n as usize { let r = registros + i * 26; let p = (r + 0x8) as *mut i16; p.write_unaligned(p.read_unaligned().wrapping_add(1)); } // mutacao: o ano (+0x8) + 1\n    (lidos == n) as i32 // históricos de competição'),
    ('dbf_competicoes_de_pessoal', '    (lidos == n) as i32 // competições de pessoal', '    for i in 0..n as usize { let r = registros + i * 101; let p = (r + 0x63) as *mut i16; p.write_unaligned(p.read_unaligned().wrapping_add(1)); } // mutacao: a reputação (+0x63) + 1\n    (lidos == n) as i32 // competições de pessoal'),
    ('dbf_historico_de_competicoes_de_pessoal', '    (lidos == n) as i32 // histórico de competições de pessoal', '    for i in 0..n as usize { let r = registros + i * 58; let p = (r + 0x8) as *mut i16; p.write_unaligned(p.read_unaligned().wrapping_add(1)); } // mutacao: o ano (+0x8) + 1\n    (lidos == n) as i32 // histórico de competições de pessoal'),
]
