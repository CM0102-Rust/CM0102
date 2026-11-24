# Mutacoes (sx20_10), item de TELAS: a carreira automatica nao abre o treino; servem para provar o comparador numa
# sessao do Lucas jogando comparando (Treino, editar os horarios):
# - a sessao nasce com o horario 1 selecionado (o comparador lado a lado do construtor acusa);
# - o jogador movido vai para o grupo seguinte ao pedido (o comparador lado a lado de set_player_group acusa).
MUTACOES = [
    ('sx20_10_horario_inicial',
     'write_i8(this + TRAINING_SESSION_SELECTED, 0); // sx20_10: schedule 0 selected',
     'write_i8(this + TRAINING_SESSION_SELECTED, 1); // mutation: schedule 1 selected'),
    ('sx20_10_grupo_seguinte',
     'write_u8(entry + 4, slot); // sx20_10: the new group',
     'write_u8(entry + 4, slot.wrapping_add(1)); // mutation: the next group'),
]
