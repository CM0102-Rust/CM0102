# Mutacoes (mwf_14), na virada espanhola de meados de 2002: sem a troca entre a Primeira e a Segunda (os rebaixados
# ficam na Primeira e os promovidos na Segunda); os rebaixados da Segunda descem para o grupo B2 em vez do B1 (o
# comparador da troca a pega, e a divisao dos clubes muda); o ano da Primeira sobe 2 (as datas de 2002/03 mudam de
# ano). Acusadas pelo estado no fim da carreira, 2002-08-01.
MUTACOES = [
    ('mwf14_sem_troca_primeira_segunda',
     '    exe::league_promote_and_relegate(this, comp_by_global(first), comp_by_global(second), 1, 1, -1, -1); // mwf14: the First and the Second',
     '    // mutation: no swap between the First and the Second',
     'mwf_14 spanish chain'),
    ('mwf14_rebaixados_para_o_b2',
     'relegate(this.addr(), read_usize(exe::league_get_table_row(second, i)), spanish_b1_record(), 1); // mwf14: down to group B1',
     'relegate(this.addr(), read_usize(exe::league_get_table_row(second, i)), comp_record_of_key(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B2), 1); // mutation: down to B2',
     'mwf_14 relegated to b1'),
    ('mwf14_primeira_ano_mais_2',
     'this.set_year(a.wrapping_add(1)); // mwf14: the year of the First Division',
     'this.set_year(a.wrapping_add(2)); // mutation: the year plus 2',
     'mwf_14 primera rollover'),
]
