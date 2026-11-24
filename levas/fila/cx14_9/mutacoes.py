# Mutacoes (cx14_9), com a Suecia ate 2002-08-01:
# - a cadeia sem a troca entre a Allsvenskan e a Superettan (ninguem sobe nem desce na virada do fim de 2001: outra
#   Allsvenskan e outra Superettan em 2002);
# - os rebaixados da Superettan para a secao Sul de Gotaland em vez da Norte (acusada pelo comparador da subida);
# - o pior codigo do mata-mata 12 em vez de 13 (acusada pelo comparador do progresso, lado a lado).
MUTACOES = [
    ('cx14_9_cadeia_sem_troca',
     'exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION), first, 1, 1, -1, -1); // cx14_9: the Allsvenskan and the Superettan',
     'let _ = first; // mutation: nobody moves',
     'cx14_9 swedish chain without the second division'),
    ('cx14_9_rebaixados_ao_sul',
     'comp_record_of(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_NORTH), 1); // cx14_9: down to the North section',
     'comp_record_of(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_SOUTH_GOTALAND), 1); // mutation: the South',
     'cx14_9 swedish regional clubs up'),
    ('cx14_9_pior_12',
     'play_off(worst as u8, 0xd)); // cx14_9: 10 and 13',
     'play_off(worst as u8, 0xc)); // mutation: 12',
     'cx14_9 allsvenskan progress'),
]
