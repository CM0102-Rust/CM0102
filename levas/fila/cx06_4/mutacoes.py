# Mutacoes (cx06_4), com as 8 ligas ate 2002-06-01:
# - a quinta rodada dos grupos (24/03) vai para 31/03: a tabela de rodadas muda (o comparador lado a lado acusa na
#   comparacao; na carreira so depois de mar/2003);
# - a noticia do sorteio do grupo 2 nao sai (a fase do grupo 2 sem League::send_group_draw_news): as noticias do dia do
#   sorteio mudam [duvida: se a carreira acusa; o comparador da fase so roda a original];
# - o sorteio acontece uma semana depois (o dia do sorteio + 7): o gerador de numeros do dia 10/03 muda de lugar e a
#   carreira muda dali em diante.
MUTACOES = [
    ('cx06_4_rodada_cinco_31_marco',
     'Cqp12D(4, 0x18, 2, 1, 5, 1, 0)',
     'Cqp12D(4, 0x1f, 2, 1, 5, 1, 0) /* mutation */',
     'cx06_4 qualifiers group rounds'),
    ('cx06_4_sem_noticia_do_grupo',
     '    exe::league_send_group_draw_news(read_usize(l.stages() + 4 * g as usize));\n    1\n}',
     '    if g != 0 { exe::league_send_group_draw_news(read_usize(l.stages() + 4 * g as usize)); } // mutation\n    1\n}',
     'cx06_4 group drawn teams'),
    ('cx06_4_sorteio_atrasado',
     '    if year == draw_year && day as u16 == draw_day && l.active() == 0 && full != 0 {',
     '    if year == draw_year && day as u16 == draw_day.wrapping_add(7) && l.active() == 0 && full != 0 { // mutation',
     'cx06_4 qualifiers draw day'),
]
