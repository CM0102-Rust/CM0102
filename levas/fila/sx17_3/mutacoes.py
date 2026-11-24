# Mutacoes (sx17_3), item de TELAS: a carreira automatica nao passa por estas rotinas; servem para provar o comparador
# numa sessao do Lucas jogando comparando:
# - o campo do nome com 0x64 bytes em vez de 0xc9 (nome longo cortado: o comparador lado a lado acusa com nomes longos);
# - o arquivo de outro tecnico tratado como proprio (9 em vez de 5: o comparador do filtro acusa ao abrir a busca de
#   outro tecnico; com o proprio arquivo escapa);
# - o tipo de lista ignorado no cabecalho (todo arquivo de busca serve para qualquer dialogo: o comparador acusa ao abrir
#   uma lista de outro tipo).
MUTACOES = [
    ('sx17_3_nome_curto',
     'exe::database_format_staff_name(person, this + 0x70, 0xc9, 1); // sx17_3: the name field, 201 bytes',
     'exe::database_format_staff_name(person, this + 0x70, 0x64, 1); // mutation: a shorter field'),
    ('sx17_3_arquivo_alheio',
     'if guio_same_c_string(password, h + PLAYER_SEARCH_FILE_PASSWORD) { 9 } else { 5 } // sx17_3: own file or someone else\'s',
     '9 // mutation: every file is the own'),
    ('sx17_3_tipo_ignorado',
     '(read_u8(out + 5) == kind as u8) as u32 // sx17_3: the list type must match',
     '1 // mutation: any list type'),
]
