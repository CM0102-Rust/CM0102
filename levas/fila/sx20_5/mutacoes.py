# Mutacoes (sx20_5). As tres rodam uma vez na carreira ENG (a criacao do jogo e uma lista refeita); os valores e
# salarios entram no banco, as listas de posicao e os orcamentos nas compras da IA:
# - o gerente sem as listas de posicao montadas (so as refaz o ciclo de 30 dias);
# - o salario padrao pelo status 0 (outros salarios em todo contrato novo);
# - nenhum orcamento de transferencias dividido nas vagas no comeco;
# - os valores de um clube da maquina refeitos como se fosse humano.
MUTACOES = [
    ('sx20_5_sem_listas',
     'build(this, 1); // sx20_5: the position lists',
     'let _ = build; // mutation: no position lists',
     'sx20_5 shortlist manager built'),
    ('sx20_5_status_zero',
     'let status = (contract.squad_status() >> 4) as i32; // sx20_5: the squad status',
     'let status = 0i32; // mutation: status 0',
     'sx20_5 default wage'),
    ('sx20_5_sem_orcamentos',
     'exe::shortlist_manager_allocate_transfer_budget(this, by_reputation(i)); // sx20_5: the transfer budgets',
     'let _ = by_reputation(i); // mutation: no transfer budgets',
     'sx20_5 default wage'),
    ('sx20_5_valores_humanos',
     'let mut flag = 0u32; // sx20_5: a computer club',
     'let mut flag = 1u32; // mutation: a computer club as human',
     'sx20_5 club shortlist rebuilt'),
]
