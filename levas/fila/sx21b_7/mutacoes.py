# Mutacoes (sx21b_7). TELAS: a cadeia nao as roda; servem para provar os comparadores numa sessao do Lucas jogando
# comparando (o caminho no rotinas.py).
MUTACOES = [
    ('sx21b_7_taxa_6',
     '([5, 7, 0xa, 0x14, 0x1e, 0x28], 0x50) // sx21b_7: the fastest learner gets 5',
     '([6, 7, 0xa, 0x14, 0x1e, 0x28], 0x50) // mutation: 6',
     'sx21b_7 position training rate'),
    ('sx21b_7_vaga_errada',
     'write_u8(e + 4, read_u8(training_entry(this, p))); // sx21b_7: his schedule slot',
     'write_u8(e + 4, read_u8(training_entry(this, p)).wrapping_add(1)); // mutation: the next slot',
     'sx21b_7 squad player listed'),
    ('sx21b_7_salva',
     'write_u32(this + 0x93, 0); // sx21b_7: no longer as saved',
     'write_u32(this + 0x93, 1); // mutation: still as saved',
     'sx21b_7 category level set'),
]
