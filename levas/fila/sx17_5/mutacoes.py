# Mutacoes (sx17_5), item de TELAS: a carreira automatica nao passa por estas rotinas; servem para provar o comparador
# numa sessao do Lucas jogando comparando:
# - a reputacao minima dos jogadores famosos 3000 em vez de 4000 (o comparador do collect_known_people compara as duas
#   listas: acusa na busca rapida);
# - a marca de "alguem fora do proprio elenco" nunca dada (o comparador nao a ve: so a tela muda; escapa do comparador);
# - o proprio elenco com a bandeira 6 (o comparador do add_own_squad compara as bandeiras: acusa).
MUTACOES = [
    ('sx17_5_fama_menor',
     'if exe::database_get_staff_reputation_for_viewer(person, owner, 0) >= 4000 { // sx17_5: the famous players',
     'if exe::database_get_staff_reputation_for_viewer(person, owner, 0) >= 3000 { // mutation: less famous'),
    ('sx17_5_sem_marca',
     '*outside_own_squad = 1; // sx17_5: someone outside the own squad',
     '// mutation: never flagged'),
    ('sx17_5_bandeira_do_elenco',
     'player_search_fill_result(this, e, player, 2, 0); // the own squad flag',
     'player_search_fill_result(this, e, player, 6, 0); // mutation: another flag'),
]
