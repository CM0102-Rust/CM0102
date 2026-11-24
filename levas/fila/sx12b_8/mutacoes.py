# Mutacoes (sx12b_8). As telas nao rodam na carreira automatica: a prova e o Lucas jogando comparando; os comparadores
# lado a lado acusam ao usar a tela.
# - a pasta de pesquisa trocada pela de impressao (dialogo de arquivo da pesquisa de jogadores);
# - o caminho repetido nao sobe do lugar dele: sempre sai o ultimo (salvar uma tatica de novo);
# - a contagem dos controlados pelo computador nao sobe (passar o time ao computador).
MUTACOES = [
    ('sx12b_pasta_de_pesquisa',
     '        2 => record + 0x9ba8, // sx12b_8: the search folder',
     '        2 => record + 0xa5d0, // mutation: the print folder'),
    ('sx12b_recente_sempre_o_ultimo',
     '    let found = (0..4).find(|&i| exe::crt_stricmp(entry(i), path) == 0).unwrap_or(3); // sx12b_8: the place it leaves',
     '    let found = 3; // mutation: always the last place',
     'sx12b_8 recent file added'),
    ('sx12b_contagem_do_computador',
     '        write_u32(this + 8, read_u32(this + 8).wrapping_add(1)); // sx12b_8: one more under computer control',
     '        write_u32(this + 8, read_u32(this + 8)); // mutation: the count kept'),
]
