# Mutação (rxs1_8): as pessoas trocadas no desempate pelo nome das transferências futuras. Acusada pelo comparador
# (lado a lado do resultado) na primeira tela que ordena duas linhas do mesmo tipo e data; a carreira não abre telas.
MUTACOES = [
    ('rxs1_8_nome',
     'let mut pa = staff_at(read_i32(a + 8) as isize).addr(); // rxs1_8: the people by name',
     'let mut pa = staff_at(read_i32(b + 8) as isize).addr(); // mutation',
     'rxs1_8 future transfer rows by name'),
]
