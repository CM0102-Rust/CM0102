# Mutacoes da gravacao dos .dat: cada uma muda um campo de valor (nao id, nao chave, nao ponteiro) de todos
# os registros antes de gravar; a tabela gravada vai para o .qst da prova do save.
# A do historico de pessoas ESCAPA: no save a tabela ja virou .tmp ([0xae23cc] = 0) e o save_database nao chama a
# gravadora; ela so roda is_recording a base na pasta Data. Fica como lacuna (PENDENCIAS).
MUTACOES = [
    ('dbh_gravar_historico_de_pessoas', '    crate::comparison::branch("banco gravar histórico de pessoas");', '    crate::comparison::branch("banco gravar histórico de pessoas");\n    for i in 0..n as usize { let r = registros + i * 17; *((r + 0xf) as *mut u8) = (*((r + 0xf) as *const u8)).wrapping_add(1); } // mutacao: os jogos (+0xf) + 1 antes de gravar'),
    ('dbh_gravar_competicoes', '    crate::comparison::branch("banco gravar competições");', '    crate::comparison::branch("banco gravar competições");\n    for i in 0..n as usize { let r = registros + i * 107; let p = (r + 0x69) as *mut i16; p.write_unaligned(p.read_unaligned().wrapping_add(1)); } // mutacao: a reputação (+0x69) + 1 antes de gravar'),
    ('dbh_gravar_historicos_de_competicao', '    crate::comparison::branch("banco gravar históricos de competição");', '    crate::comparison::branch("banco gravar históricos de competição");\n    for i in 0..n as usize { let r = registros + i * 26; let p = (r + 0x8) as *mut i16; p.write_unaligned(p.read_unaligned().wrapping_add(1)); } // mutacao: o ano (+0x8) + 1 antes de gravar'),
]
