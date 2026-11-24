# Mutacoes (sx15_2) pela carreira ENG ate 2002-06-01 (motor de partidas). A reclamacao (0x716e10) roda depois de
# muitas faltas; mais cartoes mudam suspensoes e escalacoes. As brigas (0x716630) e a simulacao (0x716c90) sao raras:
# podem escapar da carreira, mas o comparador de cada uma acusa (compare_com_fita).
# - a reclamacao de nivel 1 com gravidade 5 (mais cartoes);
# - a simulacao com gravidade 2 (menos cartoes por simulacao);
# - a gravidade de um incidente novo da briga zerada.
MUTACOES = [
    ('sx15_2_reclamacao_grave',
     'Some(1) // sx15_2: arguing, severity 1',
     'Some(5) // mutation: arguing as if striking the referee',
     'sx15_2 sent off after the card'),
    ('sx15_2_simulacao_leve',
     "engine_card_by_severity(j.engine().addr(), j.addr(), &mut card, 6, 0); // sx15_2: the dive's severity",
     'engine_card_by_severity(j.engine().addr(), j.addr(), &mut card, 2, 0); // mutation: a light dive',
     'sx15_2 suspected dive'),
    ('sx15_2_briga_sem_gravidade',
     "e.set_pending_incident_severity(severity); // sx15_2: the new incident's severity",
     'e.set_pending_incident_severity(0); // mutation: no severity',
     'sx15_2 fight'),
]
