# Mutacoes da dbh_3. db_files_write_or_read_records_0x3c: a contagem de todos os nomes antes de gravar (vai para o .qst).
# abrir: na criacao do jogo (load_database pela pasta Data), first_names.dat abre o second_names.dat (mais
# registros, os mesmos 60 bytes e ids 0..N): a tabela de primeiros nomes muda e vai para o .qst. (Fazer o
# fopen falhar nao serve: com use_zip = 0 o load_database chama fclose(0) e o jogo cai.)
# criar: "ab" no lugar de "wb"; ESCAPA da prova do save: criar so roda em save_database(..., 0, ...), a
# base gravada na pasta, e o .qst vai por zopen. Fica como lacuna (PENDENCIAS).
MUTACOES = [
    ('dbh_gravar_nomes', '    crate::comparacao::ramo("banco gravar nomes");', '    crate::comparacao::ramo("banco gravar nomes");\n    for i in 0..n as usize { let r = registros + i * 60; *((r + 0x3b) as *mut u8) = (*((r + 0x3b) as *const u8)).wrapping_add(1); } // mutacao: a contagem (+0x3b) + 1 antes de gravar'),
    ('dbh_abrir', '    caminho_do_banco(0xde1f64, pasta, nome); // abrir (rb)', '    let nome = if core::slice::from_raw_parts(nome as *const u8, 16) == b"first_names.dat\\0" { b"second_names.dat\\0".as_ptr() as usize } else { nome }; // mutacao: first_names.dat abre o second_names.dat\n    caminho_do_banco(0xde1f64, pasta, nome); // abrir (rb)'),
    ('dbh_criar', '    fopen(0xde1f64, 0x9a7f28) // criar (wb)', '    fopen(0xde1f64, b"ab\\0".as_ptr() as usize) // mutacao: "ab" (acrescenta sem truncar) no lugar de "wb"'),
]