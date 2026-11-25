# Mutacoes (rx15_2). A vaga roda 80 vezes na criacao do jogo (a carreira ENG cria o jogo do zero).
# - a vaga sem a formacao: o comparador lado a lado acusa na primeira construcao (a original escreve o 4-4-2 padrao, a
#   copia deixa a vaga como estava).
MUTACOES = [
    ('rx15_vaga_sem_formacao',
     "    formation_new(this); // rx15_2: the slot's formation",
     '    let _ = formation_new; // mutation: the slot left without its formation',
     'rx15_2 human tactic slot'),
]
