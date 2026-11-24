# Mutacoes (cx08_15), com a Belgica ate 2002-08-01 (a virada belga do verao de 2002):
# - os rebaixados do grupo B da Terceira descem para a Quarta A: as divisoes mudam (a carreira);
# - a cadeia nao faz o acesso entre a Segunda e o grupo B da Terceira: as divisoes mudam (a carreira);
# - o grupo A fica com 15 clubes (o comparador dos grupos acusa).
MUTACOES = [
    ('cx08_15_grupo_b_para_quarta_a',
     '(group_b.table(), KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_B)]',
     '(group_b.table(), KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_A)] /* mutation */',
     'cx08_15 belgian third division moves'),
    ('cx08_15_sem_grupo_b',
     '    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION), read_usize(third.stages()), 1, 1, -1, -1)\n}',
     '    0 // mutation\n}',
     'cx08_15 belgian chain'),
    ('cx08_15_grupo_a_15',
     'write_usize(club + club::RESERVE_DIVISION, if k < 16 { a } else { b });',
     'write_usize(club + club::RESERVE_DIVISION, if k < 15 { a } else { b }); // mutation',
     'cx08_15 belgian third division groups'),
]
