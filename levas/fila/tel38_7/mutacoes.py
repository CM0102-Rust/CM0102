# A base da interface (7): a carreira automatica nao abre telas, entao a cadeia nao roda mutacoes nesta leva. Estas
# provam o comparador numa sessao do Lucas com o JOGAR_COMPARANDO.bat:
# - as linhas pares de uma lista viram as escurecidas (Guio::init lado a lado; qualquer lista com area, ex. o elenco);
# - o escurecimento da linha impar nao se apaga (Guio::draw confere a marca; a mesma lista).
MUTACOES = [
    ('tel38_7_linhas_pares',
     "if added & 1 != 0 { // tel38_7: the odd rows",
     "if added & 1 == 0 { // mutation: the even rows"),
    ('tel38_7_escurece_sempre',
     "write_u32(item + m::ODD_ROW, 0); // tel38_7: the darkening only once",
     "write_u32(item + m::ODD_ROW, 1); // mutation: the flag stays"),
]
