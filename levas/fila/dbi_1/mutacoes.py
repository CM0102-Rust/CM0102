# Mutacoes de liberar e crescer: as liberacoes passam a manter a tabela (o save_database grava a tabela
# presente, que na base ja saiu); o crescer grava o indice + 1 nas linhas novas. A do crescer ESCAPA da prova
# do save (so roda no fim de competicao): lacuna (PENDENCIAS).
MUTACOES = [
    ('dbi_liberar_historico_de_pessoas', '    crate::comparacao::ramo("banco liberar histórico de pessoas");', '    crate::comparacao::ramo("banco liberar histórico de pessoas");\n    if true { return; } // mutacao: não libera (a tabela fica e vai para o save)'),
    ('dbi_liberar_historico_de_premios', '    crate::comparacao::ramo("banco liberar histórico de prêmios");', '    crate::comparacao::ramo("banco liberar histórico de prêmios");\n    if true { return; } // mutacao: não libera (a tabela fica e vai para o save)'),
    ('dbi_crescer_historico_de_competicoes', '        core::ptr::write_unaligned(linha as *mut i32, i); // a linha nova guarda o próprio índice', '        core::ptr::write_unaligned(linha as *mut i32, i + 1); // mutacao: o índice + 1'),
]
