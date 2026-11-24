# Mutacoes (cx16_3), ENG ate 2002-08-01:
# - Cuba no lugar de Jamaica na lista fixa de 2000 (quem entra muda; acusada pelo comparador das inscritas, lado a lado,
#   e pelas marcas de ocupada da rank_teams na carreira);
# - a parte da America Central e do Norte sem ordenar na virada (acusada pelo comparador das inscritas);
# - os grupos da America Central trocados (as vagas 25/27/29 e 26/28/30): quem joga em cada grupo muda na virada; o
#   plano do comparador usa o mesmo ajudante (concacaf_group_slots) e acusa contra a original;
# - as selecoes ocupadas todo dia de atualizacao completa, nao so em 10 de janeiro (as norte-americanas deixam de estar
#   livres para amistosos).
MUTACOES = [
    ('cx16_3_cuba_no_lugar_da_jamaica',
     'KEY_NATION_HAITI, KEY_NATION_ANGUILLA, KEY_NATION_THE_BAHAMAS, KEY_NATION_JAMAICA, KEY_NATION_HONDURAS',
     'KEY_NATION_HAITI, KEY_NATION_ANGUILLA, KEY_NATION_THE_BAHAMAS, KEY_NATION_CUBA, KEY_NATION_HONDURAS',
     'cx16_3 concacaf 2000 entrants'),
    ('cx16_3_sem_ordenar',
     'exe::crt_qsort(list + 0x64, others as u32, 4, 0x4b0810); // cx16_3: the Central and North American part sorted',
     '// mutation: the Central and North American part unsorted',
     'cx16_3 concacaf entrants'),
    ('cx16_3_grupos_trocados',
     '([25, 27, 29], n1, [26, 28, 30], n2) // cx16_3: the alternate slots of the two groups',
     '([26, 28, 30], n1, [25, 27, 29], n2) // mutation: groups swapped',
     'cx16_3 central american groups'),
    ('cx16_3_ocupadas_todo_dia',
     'if concacaf_busy_day(this, today) && full != 0 { // cx16_3: the busy day',
     'if full != 0 { // mutation: every full day',
     'cx16_3 concacaf busy day'),
]
