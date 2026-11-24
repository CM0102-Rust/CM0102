# Mutacoes (sx21b_4). Rodam na criacao do jogo; caem pelos comparadores (o objeto de rascunho, o plano das vagas de
# treinador e a programacao lado a lado) e pela carreira (os minimos mudam quantos virtuais cada partida cria).
MUTACOES = [
    ('sx21b_4_minimo_3',
     '    4, 4, 4, 8, 4, 8, 4, 8, // sx21b_4: four of each line and eight centre players',
     '    3, 4, 4, 8, 4, 8, 4, 8, // mutation: three goalkeepers',
     'sx21b_4 virtual staff built'),
    ('sx21b_4_treinador_0',
     'write_i32(slot, -1); // sx21b_4: no coach',
     'write_i32(slot, 0); // mutation: coach 0',
     'sx21b_4 new game training'),
    ('sx21b_4_assinatura',
     'write_u32(this + 0x29, 0x200cb); // sx21b_4: the schedule file magic',
     'write_u32(this + 0x29, 0x200cc); // mutation: another magic',
     'sx21b_4 new game training'),
]
