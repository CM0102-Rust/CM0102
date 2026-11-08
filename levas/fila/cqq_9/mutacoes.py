# Mutacoes (cqq_9). A final da J1 de 2001 e a virada do Japao para 2002 rodam uma vez na carreira de 8 ligas ate
# 2002-06-01:
# - o vice do primeiro turno no lugar do campeao: outro clube na final (outro campeao);
# - os finalistas na ordem trocada: o campeao do segundo turno primeiro (outro mandante [inferencia]);
# - a virada com o ano dois adiante: a J1 de 2002 (marco a novembro) sai para 2003 (outros jogos ate 2002-06-01).
MUTACOES = [
    ('cqq9_vice_do_primeiro_turno_na_final',
     '    core::ptr::write_unaligned(lista as *mut usize, cqq9_campeao_do_turno(this, 0)); // cqq9: o campeão do primeiro turno\n    core::ptr::write_unaligned((lista + 4) as *mut usize, cqq9_campeao_do_turno(this, 1)); // cqq9: o campeão do segundo turno\n',
     '    core::ptr::write_unaligned(lista as *mut usize, cqq9_d(cqq9_d(cqq9_d(cqq9_d(this + 0xc)) + 0xb1) + 0x3b)); // mutacao: o vice do primeiro turno\n    core::ptr::write_unaligned((lista + 4) as *mut usize, cqq9_campeao_do_turno(this, 1)); // cqq9: o campeão do segundo turno\n',
     'cqq_9 final'),
    ('cqq9_finalistas_trocados',
     '    core::ptr::write_unaligned(lista as *mut usize, cqq9_campeao_do_turno(this, 0)); // cqq9: o campeão do primeiro turno\n    core::ptr::write_unaligned((lista + 4) as *mut usize, cqq9_campeao_do_turno(this, 1)); // cqq9: o campeão do segundo turno\n',
     '    core::ptr::write_unaligned(lista as *mut usize, cqq9_campeao_do_turno(this, 1)); // mutacao: o segundo turno primeiro\n    core::ptr::write_unaligned((lista + 4) as *mut usize, cqq9_campeao_do_turno(this, 0));\n',
     'cqq_9 final'),
    ('cqq9_virada_dois_anos',
     'a.wrapping_add(1)); // cqq9: o ano seguinte da J1',
     'a.wrapping_add(2)); // mutacao: dois anos adiante',
     'cqq_9 j1 rollover'),
]
# Prova do agente (06/11, carreira Rust de 8 ligas ate 2002-06-01, base com os 9 itens): as tres ACUSADAS (a final em 2001-328, separam em 2001-335; a virada em 2002-024, separa em 2002-054).
