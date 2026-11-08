# Mutacoes (cpy_1), na conta das vagas com lance (0x844540), que a avaliacao das listas (0x84a020, 210 mil chamadas
# ate 2002-06-01) faz com a posicao 0xb antes de decidir os lances do clube do computador. Contar tambem o alvo sem
# lance, ou o lance de outra posicao, ou contar cada vaga em dobro muda a conta que a 0x84a020 guarda e compara com o
# limit de lances do clube: os lances e as transferencias da carreira mudam.
MUTACOES = [
    ('cpy1_conta_alvo_sem_lance', '            continue; // cpy1: alvo sem lance não conta',
     '            // mutacao: o alvo sem lance conta',
     'cpy_1 alvo sem lance'),
    ('cpy1_conta_outra_posicao', '            continue; // cpy1: lance de outra posição não conta',
     '            // mutacao: o lance de outra posicao conta',
     'cpy_1 outra posicao'),
    ('cpy1_conta_em_dobro', '        n = n.wrapping_add(1); // cpy1: conta a vaga com lance',
     '        n = n.wrapping_add(2); // mutacao: cada vaga conta duas',
     'cpy_1 vaga com lance'),
]
