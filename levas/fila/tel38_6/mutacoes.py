# A base da interface (6): a carreira automatica nao abre telas, entao a cadeia nao roda mutacoes nesta leva. Estas
# provam o comparador numa sessao do Lucas com o JOGAR_COMPARANDO.bat (imprimir uma tela com lista, ex. o elenco):
# - o trabalho de impressao com uma coluna a mais (o pacote 0x17 conferido);
# - toda linha impressa vazia (o pacote 0x19 conferido).
MUTACOES = [
    ('tel38_6_coluna_a_mais',
     "scrman_net_put(sm, columns, 2, 0x86); // tel38_6: the column count",
     "scrman_net_put(sm, columns + 1, 2, 0x86); // mutation: one column too many"),
    ('tel38_6_linha_vazia',
     "let text = if text == 0 { STR_EMPTY } else { text }; // tel38_6: a null line prints empty",
     "let text = STR_EMPTY; // mutation: every line empty"),
]
