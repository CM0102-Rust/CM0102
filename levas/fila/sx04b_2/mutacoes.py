# Mutacoes (sx04b_2):
# - o premio do mes vira o ano depois de novembro (as vagas de dezembro em diante ficam com o mes e o ano errados: o
#   vencedor do mes nao acha a vaga, a reputacao muda; o comparador lado a lado acusa);
# - as notas do mes comecam no mes 6 (o comparador lado a lado acusa; na carreira o primeiro dia fecha o mes do mesmo
#   jeito, pelo ano 0);
# - a vtable base trocada pela do premio do mes (o construtor de cada nacao a reescreve logo depois: so o comparador
#   lado a lado acusa).
MUTACOES = [
    ('sx04b_2_dezembro_pulado',
     'if month > 11 { // sx04b_2: December is followed by January',
     'if month > 10 { // mutation: December skipped',
     'sx04b_2 month award built'),
    ('sx04b_2_notas_no_mes_6',
     'write_u8(this + month_ratings_fields::MONTH, 0xff); // sx04b_2: no month yet',
     'write_u8(this + month_ratings_fields::MONTH, 6); // mutation: July',
     'sx04b_2 month ratings built'),
    ('sx04b_2_vtable_trocada',
     'write_u32(this, NATION_AWARDS_BASE_VTABLE); // sx04b_2: the base vtable',
     'write_u32(this, MONTH_AWARD_VTABLE); // mutation',
     'sx04b_2 nation awards built'),
]
