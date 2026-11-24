# Mutacoes (sx12b_2). Os construtores rodam uma vez na criacao do jogo (a carreira ENG cria o jogo do zero).
# - o registro do tecnico humano sem as definicoes da visao do elenco: o comparador lado a lado do HUMAN_RECORD acusa
#   na primeira construcao (os construtores do HUMAN_MANAGER e do Injury so rodam a original no comparador);
# - nenhum registro humano reposto (fica zerado: sem a tatica 0 nem a velocidade do comentario): a carreira so ve se o
#   tecnico humano usa o registro antes de ser reposto na nomeacao; pode escapar.
MUTACOES = [
    ('sx12b_visao_do_elenco_omitida',
     '    defaults(this + 0xa6d4); // sx12b_2: the squad view settings',
     '    let _ = defaults; // mutation: no squad view settings',
     'sx12b_2 human record'),
    ('sx12b_registros_humanos_sem_repor',
     '        exe::human_manager_reset_human_record(this, read_usize(G_STAFF_TABLE).wrapping_add((row as usize).wrapping_mul(staff::SIZE))); // sx12b_2: each record reset',
     '        let _ = row; // mutation: the record left zeroed',
     'sx12b_2 new human managers'),
]
