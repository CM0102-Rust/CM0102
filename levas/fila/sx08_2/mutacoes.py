# Mutacoes (sx08_2). As duas rotinas ficam com a porta do pos-jogo na comparacao (ver rotinas.py): as mutacoes sao
# provadas pela carreira Rust. As duas mudam o consumo do sorteador (o sorteio de 3 da vaga forcada), alem das
# preferencias, e a carreira diverge a partir da primeira lista cheia.
# - a antipatia sem forcar: com as 3 vagas ocupadas a pessoa nunca entra (e o sorteio some);
# - o clube favorito nunca entra (como se o clube ja gostasse da pessoa): nem vaga livre nem sorteio.
MUTACOES = [
    ('sx08_2_antipatia_sem_forcar',
     'database_add_to_preference_list(disliked, pref + preferences::STAFF_FAVOURITE_STAFF1, other, list3_contains(disliked, other), force)',
     'database_add_to_preference_list(disliked, pref + preferences::STAFF_FAVOURITE_STAFF1, other, list3_contains(disliked, other), 0) // mutation: never forced'),
    ('sx08_2_clube_favorito_nunca',
     '    let club_favours_person = list3_contains(club.addr() + club::FAV_STAFF1, person.addr());',
     '    let club_favours_person = true; // mutation: never added'),
]
