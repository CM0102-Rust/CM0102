# Mutacoes (sx20_4). A limpeza do contrato roda na aposentadoria do jogador que tambem e tecnico (process_retirements
# 0x7a59e0, em Rust, pelo endereco); a ordem das listas de posicao, no qsort da criacao das listas curtas
# (create_initial_shortlists 0x83a1b0, na criacao do jogo):
# - o bonus de gol fica no contrato (o comparador lado a lado acusa; o contrato entra no banco);
# - as listas de posicao pelo maior valor primeiro (outras listas curtas iniciais, outras compras da IA).
MUTACOES = [
    ('sx20_4_bonus_fica',
     'this.set_goal_bonus(-1); // sx20_4: no goal bonus',
     '(); // mutation: the goal bonus stays',
     'sx20_4 contract bonuses cleared'),
    ('sx20_4_maior_valor',
     'let by_value = read_i32(a + 8).wrapping_sub(read_i32(b + 8)); // sx20_4: lowest value first',
     'let by_value = read_i32(b + 8).wrapping_sub(read_i32(a + 8)); // mutation: highest first',
     'sx20_4 position list order'),
]
