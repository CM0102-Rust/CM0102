# Mutacoes (cx16_10), ENG ate 2005-12-31 (a campanha de 2004):
# - os vencedores dos grupos da America Central no lugar dos segundos (quem entra na repescagem muda; acusada pelo
#   comparador da repescagem, cujo plano usa o mesmo ajudante, e pela carreira Rust);
# - a selecao da repescagem contra a Asia: a ultima linha em vez da primeira fora da repescagem (acusada pelo
#   comparador, lado a lado).
# A advance_stage fica sem mutacao propria: trocar a fase que cria os grupos deixa uma fase nula que derruba o jogo; o
# comparador confere que a fase anda uma.
MUTACOES = [
    ('cx16_10_vencedores_no_lugar_dos_segundos',
     'v.push(read_usize(group.table() + league_table_row::SIZE)); // cx16_10: the group runner-up',
     'v.push(read_usize(group.table())); // mutation: the group winner',
     'cx16_10 concacaf play-off'),
    ('cx16_10_asia_ultima',
     'return club; // cx16_10: the first runner-up not in the play-off',
     'if i + 1 == runners.club_count() as usize { return club; } // mutation: the last one',
     'cx16_10 european asian play-off team'),
]
