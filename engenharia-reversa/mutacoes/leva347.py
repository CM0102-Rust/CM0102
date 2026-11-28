# Mutações da 347a leva (itens sx15_1, sx15_2). Uso: python mutacoes.py mutacoes\leva347.py --carreira 2002-06-01
MUTACOES = [
    ('sx15_1_confianca_inicial', 'j.set_confidence(((25 - person.temperament() as i32) * 2) as f32);', 'j.set_confidence(((25 - person.temperament() as i32) * 3) as f32); // mutation: a higher confidence'),
    ('sx15_1_classico_forte', 'j.set_own_club_feeling(j.own_club_feeling() + 5); // sx15_1: the derby feeling', 'j.set_own_club_feeling(j.own_club_feeling() + 25); // mutation: a stronger derby feeling'),
    ('sx15_1_frustracao_calma', 'j.set_frustration(random(-10) as i8); // sx15_1: the calm draw', 'j.set_frustration(random(10) as i8); // mutation: the calm draw positive'),
    ('sx15_2_reclamacao_grave', 'Some(1) // sx15_2: arguing, severity 1', 'Some(5) // mutation: arguing as if striking the referee', 'sx15_2 sent off after the card'),
    ('sx15_2_simulacao_leve', "engine_card_by_severity(j.engine().addr(), j.addr(), &mut card, 6, 0); // sx15_2: the dive's severity", 'engine_card_by_severity(j.engine().addr(), j.addr(), &mut card, 2, 0); // mutation: a light dive', 'sx15_2 suspected dive'),
    ('sx15_2_briga_sem_gravidade', "e.set_pending_incident_severity(severity); // sx15_2: the new incident's severity", 'e.set_pending_incident_severity(0); // mutation: no severity', 'sx15_2 fight'),
]
