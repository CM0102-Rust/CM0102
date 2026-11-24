# Mutacoes (sx13_3).
# - A copia comeca no segundo item da origem: as listas copiadas pelo gestor de transferencias (ofertas, clubes
#   interessados, jogadores de troca) perdem o primeiro item e as negociacoes mudam.
MUTACOES = [
    ('sx13_3_sem_o_primeiro',
     '    src.set_cursor(src.head()); // sx13_3: from the source\'s head',
     '    src.set_cursor(if src.head() != 0 { LlistNode(src.head()).next() } else { 0 }); // mutation: from the second item',
     'sx13_3 item copied'),
]
