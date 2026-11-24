# Mutacoes (sx05_3). Acusadas pelo comparador de cada ordem (lado a lado em cada chamada do qsort) quando o modo roda;
# o modo que cada premio usa vem do chamador (NationAwards, sx04), entao uma por modo.
# - modo 3: as assistencias valendo 1 em vez de 3 (o year_goals_score e do 3 e do 5);
# - modo 4: os cartoes por partida em ordem inversa (o menor primeiro);
# - modo 5: sem o corte de carater.
MUTACOES = [
    ('sx05_3_assistencias_valem_um',
     '(5 * e.goals() + 3 * e.assists() + e.rating_sum(2))',
     '(5 * e.goals() + e.assists() + e.rating_sum(2)) /* mutation: assists count 1 */',
     'sx05_3 mode 3 by club reputation'),
    ('sx05_3_cartoes_ao_contrario',
     '        e.cards() as f64 / e.matches() as f64\n',
     '        -(e.cards() as f64) / e.matches() as f64 // mutation: fewest cards first\n',
     'sx05_3 mode 4 cards'),
    ('sx05_3_sem_corte_de_carater',
     '            score *= 0.5;\n',
     '            score *= 1.0; // mutation: no character cut\n',
     'sx05_3 mode 5 poor character'),
]
