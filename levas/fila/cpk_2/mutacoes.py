# Mutacoes das listas do regen (o comparando roda a original: so o modo Rust e a carreira acusam). As tres sem a conta
# subir: so o ultimo item de cada lista fica. Aposentadorias perdidas (jogadores que nao se aposentam) e vagas do regen
# perdidas (regens que nao nascem) mudam o banco de dados; a reserva de tecnico tem poucas chamadas (~10 na carreira):
# PODE ESCAPAR.
MUTACOES = [
    ('cpk_reserva_tecnico_conta', 'cpk2_por(this + 0x20, n.wrapping_add(1)); // mais uma reserva',
     'cpk2_por(this + 0x20, n); // mutacao: a conta nao sobe'),
    ('cpk_aposentadoria_conta', 'cpk2_por(this + 8, cpk2_d(this + 8).wrapping_add(1)); // mais uma aposentadoria',
     '// mutacao: a conta nao sobe'),
    ('cpk_fila_regen_conta', 'cpk2_por(this + 0x14, cpk2_d(this + 0x14).wrapping_add(1)); // mais uma na fila do regen',
     '// mutacao: a conta nao sobe'),
]
