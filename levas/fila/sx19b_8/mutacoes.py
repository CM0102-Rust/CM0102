# Mutacoes (sx19b_8), item de TELAS: a carreira automatica nao passa; servem para provar os comparadores numa sessao do
# Lucas jogando comparando (a tela de busca dos olheiros):
# - a sessao abre sempre na lista 0 (o comparador do construtor acusa ao abrir pedindo outra lista);
# - a lista curta mudada sem a marca de mudanca (o comparador da lista curta acusa).
MUTACOES = [
    ('sx19b_8_lista_zero',
     'write_i8(this, if (0..10).contains(&k) { k } else { get_selected_list(read_usize(G_SCOUT_MANAGER), 0) as i8 }); // sx19b_8: the current search',
     'write_i8(this, 0); // mutation: always the first list'),
    ('sx19b_8_sem_marca',
     'session_mark_changed(s, k); // sx19b_8: the short list changed',
     '// mutation: not marked'),
]
