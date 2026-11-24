# Mutacoes (sx21b_8). TELAS: a cadeia nao as roda; servem para provar os comparadores numa sessao do Lucas jogando
# comparando (o caminho no rotinas.py).
MUTACOES = [
    ('sx21b_8_bits_de_posicao',
     'set_training_bits(this, 0xdf80, bits) // sx21b_8: the position bits',
     'set_training_bits(this, 0xdf81, bits) // mutation: keeps the goalkeeper bit',
     'sx21b_8 position training bits'),
    ('sx21b_8_dez_letras',
     'exe::at_9466d0(this + 9, name.as_ptr(), 0xb); // sx21b_8: eleven characters',
     'exe::at_9466d0(this + 9, name.as_ptr(), 0xa); // mutation: ten',
     'sx21b_8 default name'),
    ('sx21b_8_sem_treino_todos',
     '0 => exe::injury_get_player_injury_status(read_usize(0xb63d28), p) == 2, // sx21b_8: No Training lists the injured',
     '0 => true, // mutation: everybody',
     'sx21b_8 player counted'),
]
