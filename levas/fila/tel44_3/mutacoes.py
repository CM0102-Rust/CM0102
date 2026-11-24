# Mutação (tel44_3), acusada pelo comparador (lado a lado sobre a contagem e os votos) a cada chamada; pela carreira
# só se um voto 0 de um treinador mudar o vencedor (a contagem não muda).
# - os votos zerados em vez de −1.
MUTACOES = [
    ('tel44_3_votes_cleared',
     "write_i16(this + AWARD_SHORTLIST_VOTES + 2 * k, -1); // tel44_3: no manager has voted",
     "write_i16(this + AWARD_SHORTLIST_VOTES + 2 * k, 0); // mutation: the votes at 0"),
]
