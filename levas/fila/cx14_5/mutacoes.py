# Mutacoes (cx14_5), com a Turquia ate 2002-08-01:
# - a tabela com as linhas de tras para frente (acusada pelo comparador da tabela, que confere a ordem das linhas);
# - a regra de disciplina 0x24 em vez da turca (muda as suspensoes da categoria B; acusada pelo comparador do
#   construtor, que confere a regra).
# As honras da temporada ficam sem mutacao (a historia nao volta a simulacao; comparador so a original).
MUTACOES = [
    ('cx14_5_tabela_invertida',
     'exe::league_init_table_row(l, k as u32, club, 0, 0); // cx14_5: the row of the k-th club',
     'exe::league_init_table_row(l, 9u32.wrapping_sub(k as u32), club, 0, 0); // mutation: backwards',
     'cx14_5 category b table'),
    ('cx14_5_disciplina_0x24',
     'l.set_discipline_rule(0x25); // cx14_5: the Turkish discipline rule',
     'l.set_discipline_rule(0x24); // mutation: another rule',
     'cx14_5 new category b'),
]
