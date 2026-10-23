# Mutacoes pela prova do save (o .qst gravado pelo inicio rapido): o id unico trocado por outro
# global no general.dat, o resumo com 4 bytes a menos (o tamanho do item muda) e as regras FIFA
# (o ultimo item) puladas: todas mudam o conteudo.
MUTACOES = [
    ('sv3_gravar_id_unico', "|| !campo(0xdd7120, 4, 1, 0x29bd, 0xa81aa4)", "|| !campo(0xdd7124, 4, 1, 0x29bd, 0xa81aa4)"),
    ('sv3_gravar_resumo', "    if !campo(resumo.as_ptr() as usize, 0xf74, 1, 0x29b5, 0xa81ae0)", "    if !campo(resumo.as_ptr() as usize, 0xf70, 1, 0x29b5, 0xa81ae0)"),
    ('sv3_gravar_fifa', "    regras_fifa(l);", "    let _ = regras_fifa;"),
]
