# Mutacao para a sessao do Lucas (TELAS: a cadeia nao roda; o comparador so roda a original, entao a mutacao so e
# vista no texto da noticia jogando em modo Rust): o estadio sem a metade do semiprofissional.
MUTACOES = [
    ('rx08_10_semiprofissional_inteiro',
     'expected = (expected as f64 * read_f64(rdata::F64_0_5)) as i32 as i16; // rx08_10: half for a part-time club',
     'expected = expected; // rx08_10 mutation',
     'rx08_10 ultimatum news'),
]
