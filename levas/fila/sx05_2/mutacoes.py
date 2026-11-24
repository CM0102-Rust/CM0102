# Mutacoes (sx05_2). A ordem decide quem ganha os premios de jogador do ano pelas notas (fim da temporada, ate
# 2002-08-01); acusadas pelo comparador de cada ordem (lado a lado em cada chamada do qsort) e, mudando o vencedor,
# pela reputacao do premiado na carreira.
# - o limite de 3 partidas virando 5 (vale para os seis modos);
# - sem o bonus de 1,2 do atacante;
# - o termo das partidas sem o dobro.
MUTACOES = [
    ('sx05_2_cinco_partidas',
     '    if a.matches() < 3 {\n        return (b.matches() >= 3) as i32;',
     '    if a.matches() < 5 { // mutation: 5 matches\n        return (b.matches() >= 3) as i32;',
     'sx05_2 year order by score'),
    ('sx05_2_sem_bonus_do_atacante',
     '            score *= 1.2f32 as f64;',
     '            score *= 1.0; // mutation: no attacker bonus',
     'sx05_2 attacker bonus'),
    ('sx05_2_partidas_sem_dobro',
     '(n * 0.02 + n * 0.02)',
     '(n * 0.02) /* mutation: not doubled */',
     'sx05_2 year order by score'),
]
