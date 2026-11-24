# Mutacoes (sx15_1) pela carreira ENG ate 2002-06-01: a preparacao roda para cada jogador de cada partida; a confianca,
# o temperamento, a frustracao e a raiva entram no motor (faltas, cartoes, reacoes, desempenho), entao o resultado das
# partidas muda.
# - a confianca inicial 2 x (25 - temperamento) vira 3 x (todo jogador de toda partida);
# - o sentimento do classico +5 vira +25 (so nos classicos);
# - o sorteio calmo da frustracao random(-10) vira random(10).
MUTACOES = [
    ('sx15_1_confianca_inicial',
     'j.set_confidence(((25 - person.temperament() as i32) * 2) as f32);',
     'j.set_confidence(((25 - person.temperament() as i32) * 3) as f32); // mutation: a higher confidence'),
    ('sx15_1_classico_forte',
     'j.set_own_club_feeling(j.own_club_feeling() + 5); // sx15_1: the derby feeling',
     'j.set_own_club_feeling(j.own_club_feeling() + 25); // mutation: a stronger derby feeling'),
    ('sx15_1_frustracao_calma',
     'j.set_frustration(random(-10) as i8); // sx15_1: the calm draw',
     'j.set_frustration(random(10) as i8); // mutation: the calm draw positive'),
]
