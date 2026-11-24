# Mutacoes (sx04_1). Os premios do mes mudam a reputacao dos vencedores (award_raise_winner_reputation), lida pela
# carreira:
# - sem os premios do mes da nacao na virada (a carreira perde todos os jogadores, jovens e tecnicos do mes);
# - as notas do mes nao se esvaziam (o mes seguinte soma as notas dos anteriores: outros vencedores);
# - a ordem das notas com o corte em 2 jogos em vez de 3 (o comparador lado a lado acusa; outro vencedor);
# - a nova temporada anda dois anos (nenhum vencedor do mes acha a vaga) -- so roda se o dia de inicio da temporada
#   inglesa cair depois do inicio da carreira com as vagas noutro ano; senao o ramo nao roda e a mutacao e pulada.
MUTACOES = [
    ('sx04_1_sem_premios_do_mes',
     'on_month_change(this); // sx04_1: the month awards',
     '{} // mutation: no month awards',
     'sx04_1 month change'),
    ('sx04_1_notas_sem_esvaziar',
     "reset(read_usize(this + nation_awards_obj::MONTH_RATINGS), month, year); // sx04_1: the new month's ratings",
     'let _ = (reset, month, year); // mutation: the ratings are not emptied',
     'sx04_1 month change'),
    ('sx04_1_corte_em_dois_jogos',
     'if ma < 3 { // sx04_1: fewer than three matches go last',
     'if ma < 2 { // mutation: the cut at two matches',
     'sx04_1 averages compared'),
    ('sx04_1_temporada_dois_anos',
     'read_u16(slot + month_award_fields::SLOT_YEAR).wrapping_add(1)); // sx04_1: one year on',
     'read_u16(slot + month_award_fields::SLOT_YEAR).wrapping_add(2)); // mutation: two years on',
     'sx04_1 month slots moved on'),
]
