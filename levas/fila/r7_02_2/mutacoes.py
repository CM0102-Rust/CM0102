# Mutações (r7_02_2). A carreira não as acusa (na criação a liberação acha as tabelas vazias): escapam e são
# confirmadas no modo comparando pelo comparador da liberação, quando a carga da original (desviada pelo r7_02_1)
# libera as tabelas que a cópia acabou de carregar.
# - a língua dos nomes fica 0, não −1;
# - a contagem de clubes não é zerada com a tabela;
# - no modo ligado, a contagem de competições de clube não é zerada.
MUTACOES = [
    ('r7022_lingua_zero',
     '    write_i32(G_DATABASE_NAMES_LANGUAGE, -1);',
     '    write_i32(G_DATABASE_NAMES_LANGUAGE, 0); // mutation',
     'r7_02_2 free all tables'),
    ('r7022_clubes_contados',
     '    by_free(&mut *release, G_CLUB_TABLE, &[G_CLUB_COUNT, G_NAT_CLUB_TABLE, 0xae2360]); // with the national teams and their count',
     '    by_free(&mut *release, G_CLUB_TABLE, &[G_NAT_CLUB_TABLE, 0xae2360]); // mutation',
     'r7_02_2 free all tables'),
    ('r7022_competicoes_contadas',
     '        for c in [0xae2398, 0xae239c, G_COMMON_NAME_COUNT, G_CITY_COUNT, 0xae2388, G_CLUB_COMP_COUNT, 0xae2380,',
     '        for c in [0xae2398, 0xae239c, G_COMMON_NAME_COUNT, G_CITY_COUNT, 0xae2388, 0xae2380, // mutation',
     'r7_02_2 linked tables zeroed'),
]
