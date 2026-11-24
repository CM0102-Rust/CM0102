# Mutacoes (sx13_1).
# - O no copia um byte a menos do item: o ultimo byte de cada item posto numa lista fica com o que havia no bloco
#   novo (as listas de ofertas, faltas, amistosos, partidas do dia...): o que o jogo le dos itens muda.
# - A lista nova nao zera o cursor: o comparador acusa (lado a lado no cabecalho) sempre que a memoria tinha algo ali;
#   a carreira pode nao acusar (quase todos os percursos poem o cursor na cabeca antes de andar).
# O destrutor do no so libera memoria: sem mutacao (o comparador confere o no intacto).
MUTACOES = [
    ('sx13_1_copia_curta',
     '    llist_copy_bytes(data, copy, size as i32 as usize); // sx13_1: the whole item',
     '    llist_copy_bytes(data, copy, (size as i32 as usize).saturating_sub(1)); // mutation: one byte short',
     'sx13_1 node built'),
    ('sx13_1_cursor_sujo',
     '    list.set_cursor(0); // sx13_1: the cursor cleared',
     '    // mutation: the cursor left as it was'),
]
