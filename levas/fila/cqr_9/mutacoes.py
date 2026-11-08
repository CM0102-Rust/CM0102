# Mutacoes (cqr_9), nos mata-matas das copas italianas (2001-247): os segundos dos grupos no lugar dos campeoes (Copa
# da Italia); os 6 primeiros grupos dao os melhores segundos (sem a classificacao, Serie C); todas as sementes iguais
# (o sorteio da Copa da Italia sem potes).
MUTACOES = [
    ('cqr9_segundos_no_lugar_dos_campeoes',
     'cqr9_pd(times + i * 4, cqr9_d(group_table(this, i))); // cqr9: o campeão do grupo i',
     'cqr9_pd(times + i * 4, cqr9_d(group_table(this, i) + 0x3b)); // mutacao',
     'cqr_9 mata-mata da copa da italia'),
    ('cqr9_segundos_sem_classificacao',
     'ordenar(liga, 0); // cqr9: a classificação dos segundos',
     'let _ = ordenar; // mutacao',
     'cqr_9 mata-mata da copa da serie c'),
    ('cqr9_sementes_iguais',
     '        *s = 3 - (k / 8) as u8;',
     '        *s = 1 + 0 * k as u8; // mutacao',
     'cqr_9 mata-mata da copa da italia'),
]
