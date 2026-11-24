# Mutacoes (sx17_6), item de TELAS: a carreira automatica nao passa por estas rotinas; servem para provar o comparador
# numa sessao do Lucas jogando comparando:
# - o cabecalho gravado a partir de +6 em vez de +5 (o comparador relê o arquivo: acusa);
# - a carga ficando com o clube observado de outro jogo (o comparador nao o confere: escapa; o efeito aparece na lista
#   do olheiro, que observa um clube de outro jogo).
MUTACOES = [
    ('sx17_6_cabecalho_deslocado',
     '    if !write(5, SEARCH_FILE_HEADER_SIZE as u32, 1) {',
     '    if !write(6, SEARCH_FILE_HEADER_SIZE as u32, 1) { // mutation: the header one byte off'),
    ('sx17_6_clube_de_outro_jogo',
     'let assigned = if club < -1 || watched || !same_game { -1 } else { club }; // sx17_6: a club only from the same game',
     'let assigned = if club < -1 || watched { -1 } else { club }; // mutation: any game'),
]
