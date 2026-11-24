# Mutacoes (sx12b_9). Os apelidos so existem quando o Lucas da um (perfil de uma pessoa -> Apelido): a carreira
# automatica nao os usa; os comparadores acusam na tela.
# - get_nickname devolve o comeco da entrada (o id) no lugar do texto: o nome aparece errado e o comparador acusa;
# - a chave pela soma no lugar da diferenca (qsort/bsearch da lista).
MUTACOES = [
    ('sx12b_apelido_sem_texto',
     '    if found == 0 { 0 } else { found + 4 } // sx12b_9: the nickname text',
     '    if found == 0 { 0 } else { found } // mutation: the entry, not its text'),
    ('sx12b_chave_somada',
     '    read_i32(a).wrapping_sub(read_i32(b)) // sx12b_9: the key difference',
     '    read_i32(a).wrapping_add(read_i32(b)) // mutation: the sum'),
]
