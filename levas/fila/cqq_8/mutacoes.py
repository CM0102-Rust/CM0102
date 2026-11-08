# Mutacoes (cqq_8). A virada do Brasil (Serie A), a cadeia e a virada da Serie B rodam uma vez na carreira de 8 ligas
# ate 2002-06-01 (a virada do Brasil para 2002, antes dos estaduais):
# - a virada da Serie A sem a do Paulista: o Paulista de 2002 nao e refeito (outros jogos de janeiro a maio);
# - a cadeia sem a troca entre A e B: os clubes marcados para subir e descer ficam (outros clubes nas divisoes; o banco
#   da carreira acusa);
# - a virada da Serie B sem as expectativas das diretorias e sem a renovacao dos arbitros (sorteia arbitros novos:
#   outra contagem de sorteios).
MUTACOES = [
    ('cqq8_sem_a_virada_do_paulista',
     '        bra_nat_first_call_virtual_no_args(comp_by_global(*g), 8); // cqq8: a virada de cada uma',
     '        if k != 13 { bra_nat_first_call_virtual_no_args(comp_by_global(*g), 8); } // mutacao: sem a virada do Paulista',
     'cqq_8 virada do paulista'),
    ('cqq8_sem_troca_entre_a_e_b',
     '    trocar(this, comp_by_global(a), comp_by_global(b), 1, 1, -1, -1); // cqq8: A e B',
     '    // mutacao: sem a troca entre A e B',
     'cqq_8 acesso e rebaixamento'),
    ('cqq8_serie_b_sem_arbitros',
     '    crate::comparacao::ramo("cqq_8 virada da serie b");\n    set_expectations_and_renew_officials(this)\n',
     '    crate::comparacao::ramo("cqq_8 virada da serie b");\n    0 // mutacao: sem expectativas nem arbitros\n',
     'cqq_8 virada da serie b'),
]
# Prova do agente (06/11, carreira Rust de 8 ligas ate 2002-06-01, base com os 9 itens): as tres ACUSADAS (a virada do Brasil em 2002-011; separam em 2002-012).
