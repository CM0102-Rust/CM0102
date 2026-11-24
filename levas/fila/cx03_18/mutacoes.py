# Mutacoes (cx03_18), carreira de 8 ligas ate a data do rotinas.py (a medir). Nenhuma abre caixa nem deixa a copa sem
# tabela (as que tiram as regras, a arvore ou a primeira fase poderiam derrubar o jogo e ficaram de fora).
# - a virada sem as posicoes previstas: as expectativas dos clubes asiaticos ficam as da edicao velha (a carreira).
MUTACOES = [
    ('cx03_18_virada_sem_posicoes_previstas',
     '    call(t.first_stage);\n    if read_usize(club_list) != 0 {\n        exe::crt_free(read_usize(club_list));\n        write_usize(club_list, 0);\n    }\n    call_cup_virtual(this, comp_vtable::SET_PREDICTED_POSITIONS) as u32',
     '    call(t.first_stage);\n    if read_usize(club_list) != 0 {\n        exe::crt_free(read_usize(club_list));\n        write_usize(club_list, 0);\n    }\n    0 // mutation: no predicted positions',
     'cx03_18 asian club championship turnover'),
]
