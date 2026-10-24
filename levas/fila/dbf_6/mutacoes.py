# Mutacoes da leitura dos .dat: cada uma muda um campo de todos os registros lidos (sem tocar ids,
# nomes-chave nem ponteiros, para o jogo nao parar numa caixa). Jogadores: a tabela vai para o .qst da prova
# do save. Pessoas 110 e preferencias so rodam ao carregar um save (a base de fabrica e a versao 1): a prova e
# a carga do FeriasTesteSave (CARGA), pela impressao do banco do dia. Nas preferencias todos os campos sao ids:
# a mutacao troca dois deles entre si (os valores continuam validos).
MUTACOES = [
    ('dbf_jogadores', '    (lidos == n) as i32 // jogadores', '    for i in 0..n as usize { let r = registros + i * 70; *((r + 0x11) as *mut u8) = (*((r + 0x11) as *const u8)).wrapping_add(1); } // mutacao: a posição de zagueiro (+0x11) + 1\n    (lidos == n) as i32 // jogadores'),
    ('dbf_pessoas_110', '    (lidos == n) as i32 // pessoas 110', '    for i in 0..n as usize { let r = registros + i * 110; let p = (r + 0x52) as *mut i32; p.write_unaligned(p.read_unaligned().wrapping_add(1)); } // mutacao: o valor (+0x52) + 1\n    (lidos == n) as i32 // pessoas 110'),
    ('dbf_preferencias', '    (lidos == n) as i32 // preferências', '    for i in 0..n as usize { let r = registros + i * 52; let a = (r + 0x4) as *mut i32; let b = (r + 0x8) as *mut i32; let t = a.read_unaligned(); a.write_unaligned(b.read_unaligned()); b.write_unaligned(t); } // mutacao: o 1º e o 2º clube favorito (+0x4, +0x8) trocados\n    (lidos == n) as i32 // preferências'),
]
