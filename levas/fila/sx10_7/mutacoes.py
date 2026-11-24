# Mutacoes (sx10_7): telas de taticas; a carreira automatica nao as roda, entao a cadeia nao muta nesta leva. Provam
# os comparadores numa sessao do Lucas com o JOGAR_COMPARANDO.bat:
# - swap_slots: a nao toma a posicao de b (Taticas, arrastar um jogador sobre outro de outra posicao);
# - get_slots_in_position_order: o maior primeiro (tela do clube que lista a formacao por posicao);
# - mirror_zone_coordinates: as pontas de cada linha nao trocam (Taticas, editar a formacao, espelhar uma zona);
# - is_slot_coordinate_customised: a fase sem bola nunca acusa (Taticas, editar a formacao, a marca de posicao mexida).
MUTACOES = [
    ('sx10_7_troca_sem_posicao',
     'formation_take_position(this, a, formation_general(this, b)); // sx10_7: a takes b\'s position',
     '{} // mutation: a keeps its position',
     'sx10_7 swap slots'),
    ('sx10_7_ordem_invertida',
     'if key < best { // sx10_7: the smallest key first',
     'if key > best || best == 1000 { // mutation: the largest key first'),
    ('sx10_7_espelho_sem_troca',
     'points.swap(left, right); // sx10_7: the two ends of the line swap',
     '// mutation: the ends do not swap',
     'sx10_7 mirror zone'),
    ('sx10_7_fase_sem_bola',
     'if stage == 0 && stored(0) != default { // sx10_7: the phase without the ball moved',
     'if stage == 2 && stored(0) != default { // mutation: never'),
]
