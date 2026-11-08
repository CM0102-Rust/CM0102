# Mutacoes (cqo_7). O Mineiro de 2001 terminou com final (campeoes diferentes nos dois turnos) na carreira de 8 ligas
# (contadores "cqo_7 final" e "cqo_7 final history" = 1 na carreira Rust ate 2001-10-01): as tres mexem nesse caminho.
# - o vice do primeiro turno no lugar do campeao: outro clube na final (ou nenhuma final, se ele venceu o segundo turno);
# - os finalistas na ordem trocada: o campeao do segundo turno vira o primeiro da lista (outro mandante [inferencia]);
# - o fim da temporada como se nao houvesse final: o historico do campeonato e os premios aos dois primeiros da tabela
#   da liga (o saldo muda: o banco da carreira acusa).
MUTACOES = [
    ('cqo7_vice_do_primeiro_turno_na_final',
     '    let a = half_champion(this, 0);\n',
     '    let a = cqo7_d(cqo7_d(cqo7_d(cqo7_d(this + 0xc)) + 0xb1) + 0x3b); // mutacao: o vice do primeiro turno\n',
     'cqo_7 final'),
    ('cqo7_finalistas_trocados',
     '    core::ptr::write_unaligned(lista as *mut usize, a);\n    let b = half_champion(this, 1);\n    core::ptr::write_unaligned((lista + 4) as *mut usize, b);\n',
     '    let b = half_champion(this, 1);\n    core::ptr::write_unaligned(lista as *mut usize, b); // mutacao: o campeao do segundo turno primeiro\n    core::ptr::write_unaligned((lista + 4) as *mut usize, a);\n',
     'cqo_7 final'),
    ('cqo7_fim_sem_final',
     '    if cqo7_d(this + 0x30) == 2 {\n',
     '    if cqo7_d(this + 0x30) == 99 { // mutacao: como se nao houvesse final\n',
     'cqo_7 final history'),
]
# Prova do agente (06/11, carreira Rust de 8 ligas ate 2002-06-01, base com os 9 itens cqo/cqp/cqq_7..9): as tres ACUSADAS (2001-153 a 2001-168).
