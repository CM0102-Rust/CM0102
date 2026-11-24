# Mutacoes (sx21_33). Rodam nas ofertas da carreira ENG; caem pelos comparadores (e pela carreira quando mudam uma
# oferta).
MUTACOES = [
    ('sx21_33_sem_data',
     'exe::date_date_54a110(this + offer::DUE_DATE); // sx21_33: the due date',
     '// mutation: the due date is not built',
     'sx21_33 empty offer'),
    ('sx21_33_clausulas_negativas',
     '(if a > 0 { a } else { 0 }) + if b > 0 { b } else { 0 } // sx21_33: only positive clauses',
     'a + if b > 0 { b } else { 0 } // mutation: a negative clause counts',
     'sx21_33 clause total'),
    ('sx21_33_sem_fracasso',
     'o.set_terms_state(4); // sx21_33: the talks collapse',
     '// mutation: the talks stay',
     'sx21_33 withdrawn offer talks collapse'),
]
