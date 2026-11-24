# Mutacao (cx01_9), carreira com a Turquia ate 2002-08-01: a divisao de menor reputacao primeiro: quem entra na copa
# turca (as 26 primeiras da lista ordenada) muda e o comparador acusa. Nao abre caixa.
MUTACOES = [
    ('cx01_9_reputacao_menor_primeiro',
     '            let by_reputation = db.reputation() as i32 - da.reputation() as i32;',
     '            let by_reputation = da.reputation() as i32 - db.reputation() as i32; // mutation'),
]
