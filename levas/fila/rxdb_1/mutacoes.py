# Mutações (rxdb_1), na gravação do banco: cada uma muda o .qst gravado (a prova do save compara a gravação com e sem a
# rotina). Provadas também na emulação (check.py do leitor): cada uma acusada.
MUTACOES = [
    ('rxdb_1_nulo_vira_zero',
     '                    write_i32(field, if target == 0 { -1 } else { read_i32(target + id_at) });',
     '                    write_i32(field, if target == 0 { 0 } else { read_i32(target + id_at) }); // mutation',
     'rxdb_1 database pointers to ids'),
    ('rxdb_1_clubes_com_selecoes',
     'unsafe fn database_save_count_clubs() -> i32 { read_i32(G_CLUB_COUNT).wrapping_sub(read_i32(DATABASE_SAVE_NAT_CLUB_COUNT)) }',
     'unsafe fn database_save_count_clubs() -> i32 { read_i32(G_CLUB_COUNT) } // mutation',
     'rxdb_1 save database'),
    ('rxdb_1_sem_competicoes_de_reserva',
     '    if written && t.kind == DatabaseSaveKind::NationComps && reserve >= 1 {',
     '    if written && t.kind == DatabaseSaveKind::NationComps && reserve >= 1 && false { // mutation',
     'rxdb_1 save database'),
    ('rxdb_1_versao_1',
     '    write_i32(entry + db_index_entry::VERSION, t.version);',
     '    write_i32(entry + db_index_entry::VERSION, 1); // mutation',
     'rxdb_1 save database'),
    ('rxdb_1_nomes_pelo_inicio',
     '    (staff::FIRST_NAME, 1, names::ID), (staff::SECOND_NAME, 1, names::ID), (staff::COMMON_NAME, 1, names::ID),',
     '    (staff::FIRST_NAME, 1, 0), (staff::SECOND_NAME, 1, names::ID), (staff::COMMON_NAME, 1, names::ID), // mutation',
     'rxdb_1 database pointers to ids'),
]
