# Mutacoes (sx14_4), de TELAS: a carreira automatica nao abre noticia nenhuma destas; servem para provar o comparador
# numa sessao do Lucas (compare_text_writer acusa texto ou vinculo diferente).
# - a resposta a candidatura troca recusada por recebida (manchete);
# - a reacao a reclamacao do arbitro cita o sobrenome novo no lugar do antigo.
MUTACOES = [
    ('sx14_4_candidatura_trocada',
     '        (false, true) => 0xa04f48,  // "<club> dismiss your job application"',
     '        (false, true) => 0xa04f04,  // mutation',
     'sx14_4 application reply'),
    ('sx14_4_sobrenome_trocado',
     '    let (old, new) = (name(old), name(new));',
     '    let (old, new) = (name(new), name(old)); // mutation',
     'sx14_4 referee complaint body'),
]
