# Mutacoes (sx12b_7). A carreira automatica nao abre a tela do hall da fama: a prova e o Lucas jogando comparando; os
# comparadores lado a lado acusam ao abrir a tela.
# - a entrada seguinte a pedida (as linhas da tabela mostrada deslocadas de uma);
# - o nome longo no lugar da forma do menu (o menu das ligas).
MUTACOES = [
    ('sx12b_entrada_seguinte',
     '    assign(out, this + (table as usize * hf::ENTRIES + rank as usize) * hf::ENTRY_SIZE); // sx12b_7: the chosen entry',
     '    assign(out, this + (table as usize * hf::ENTRIES + rank as usize + 1) * hf::ENTRY_SIZE); // mutation: the next entry'),
    ('sx12b_nome_longo_no_menu',
     '        text(out, menu, 0xad9c64usize); // sx12b_7: the menu form',
     '        text(out, long); // mutation: the long name in the menu'),
]
