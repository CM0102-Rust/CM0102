# Mutacoes (r7_07_2). TELAS: a cadeia nao roda; o comparador as acusa quando a busca de jogadores e usada jogando
# comparando (set_option_flag lado a lado sobre o filtro; player_matches_filter pela resposta).
# - a faixa de idade 0x20000 (60 ou mais) deixa de ser desligada quando se escolhe outra faixa;
# - o filtro de cargo 2 deixa de aceitar o cargo 4;
# - a fluencia minima no idioma cai de 4 para 3.
MUTACOES = [
    ('r7_07_2_faixa_de_idade',
     '        (0x80 | 0x100 | 0x200 | 0x400 | 0x800 | 0x1000 | 0x2000 | 0x4000 | 0x8000 | 0x10000 | 0x20000, 0) => (0xfffc_007f, !0), // age bands',
     '        (0x80 | 0x100 | 0x200 | 0x400 | 0x800 | 0x1000 | 0x2000 | 0x4000 | 0x8000 | 0x10000 | 0x20000, 0) => (0xfffe_007f, !0), // r7_07_2 mutation',
     'r7_07_2 option group'),
    ('r7_07_2_cargo_2',
     '        2 => matches!(job, 2 | 3 | 4), // r7_07_2: coaches',
     '        2 => matches!(job, 2 | 3), // r7_07_2 mutation',
     'r7_07_2 candidate'),
    ('r7_07_2_fluencia',
     '    let fluent = |language: i8| exe::playerstats_get_language_fluency(read_usize(G_PLAYER_STATS), p.addr(), 0, language as u8 as u32, 1) as i8 >= 4;',
     '    let fluent = |language: i8| exe::playerstats_get_language_fluency(read_usize(G_PLAYER_STATS), p.addr(), 0, language as u8 as u32, 1) as i8 >= 3; // r7_07_2 mutation',
     'r7_07_2 candidate'),
]
