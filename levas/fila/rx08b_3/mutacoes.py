# Mutacoes para a sessao do Lucas (TELAS: a cadeia nao roda); o comparador de cada formatador as acusa na aba de
# estatisticas e na de historico.
MUTACOES = [
    ('rx08b_3_contagens_deslocadas',
     'k @ 5..=12 => count(k as usize + 1), // rx08b_3: the counts +6..+0xd',
     'k @ 5..=12 => count(k as usize), // rx08b_3 mutation',
     'rx08b_3 season stat field'),
    ('rx08b_3_chutes_sem_os_minutos',
     'let enough = shots > 0x18 || (shots > 4 && minutes / shots <= 0x2d) || (minutes > 0x21c && shots > 0xa); // rx08b_3: the shooting threshold',
     'let enough = shots > 0x18 || (minutes > 0x21c && shots > 0xa); // rx08b_3 mutation',
     'rx08b_3 career stat field'),
]
