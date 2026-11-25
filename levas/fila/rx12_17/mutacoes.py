# Mutacoes (rx12_17), ENG ate 2005-12-31 (a campanha de 2006).
# - o texto: o vencedor da final com "qualify for World Cup" no lugar de "qualify for playoff"; o texto nao volta a
#   simulacao: acusada pelo comparador lado a lado (compare_text_writer), que conta como acusada;
# - a repescagem com o vencedor da final dos dois lados (a America do Sul fora): muda a Copa de 2006 e o plano do
#   comparador acusa.
MUTACOES = [
    ('rx12_17_texto_trocado',
     '        (true, true) => t::QUALIFY_FOR_PLAYOFF,',
     '        (true, true) => t::QUALIFY_FOR_WORLD_CUP, // mutation',
     'rx12_17 oceania tie fate'),
    ('rx12_17_sem_america',
     '        write_usize(v + 4 * k, n); // rx12_17: the play-off list',
     '        write_usize(v + 4 * k, nations[0]); // mutation',
     'rx12_17 oceania play-off'),
]
