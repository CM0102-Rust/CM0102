# Mutacoes (sx19b_10), item de TELAS: a carreira automatica nao passa; servem para provar os comparadores numa sessao
# do Lucas jogando comparando (Procurar):
# - o clube comparado so pelo nome longo (o comparador acusa ao procurar pelo nome curto de um clube);
# - o texto inteiro precisa caber deslocado de uma posicao (o comparador acusa quando o texto e o comeco do campo).
MUTACOES = [
    ('sx19b_10_so_nome_longo',
     'record_names_contain(club, text, &CLUB_NAMES) // sx19b_10: the club\'s names',
     'record_names_contain(club, text, &CLUB_NAMES[..1]) // mutation: the long name only'),
    ('sx19b_10_sem_comeco',
     'return found_at(text, text_len) as u32; // sx19b_10: the whole text',
     'return (text_len < field_len && (1..=field_len - text_len).any(|i| exe::utils_string_compare((field + i) as *const u8, text as *const u8, 0, text_len as u32) == 0)) as u32; // mutation: not at the start'),
]
