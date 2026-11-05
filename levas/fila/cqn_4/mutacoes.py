# Mutacoes (cqn_4), nas ligas de fundo francesas (a criacao da carreira de 8 ligas): o sorteio da lista dos clubes e
# o da grade das rodadas mudam a contagem dos sorteios logo no inicio.
# - na CFA, os clubes da divisao generica entram sem conferir a competicao de reserva (outros clubes nos grupos);
# - o embaralhamento da lista com um clube a menos (3 trocas sorteadas a menos): acusada no 1o dia na prova do agente;
# - os bits de regra sem a ordem fixa das rodadas (0x43 -> 3: a grade sorteia os saltos): acusada no 1o dia.
# Trocada (05/11): "um clube a menos da divisao generica" derrubava o jogo (a lista de 72/40 tem de encher: um grupo
# com um clube nulo).
MUTACOES = [
    ('cqn4_cfa_sem_conferir_a_reserva',
     'comp != 0 && id == cqn4_d(0x9cf764) && reserva != 0 && cqn4_d(reserva) == cqn4_d(p.competicao)',
     'comp != 0 && id == cqn4_d(0x9cf764) // mutacao: sem conferir a reserva',
     'cqn_4 liga de fundo francesa'),
    ('cqn4_sorteio_com_um_a_menos',
     'embaralhar(lista.as_mut_ptr() as usize, k as u16 as u32); // cqn4: o sorteio da lista',
     'embaralhar(lista.as_mut_ptr() as usize, (k - 1) as u16 as u32); // mutacao: um a menos',
     'cqn_4 liga de fundo francesa'),
    ('cqn4_rodadas_sem_ordem_fixa',
     'core::ptr::write_unaligned((this + 0xd9) as *mut u16, 0x43); // cqn4: os bits de regra das ligas de fundo',
     'core::ptr::write_unaligned((this + 0xd9) as *mut u16, 3); // mutacao: sem a ordem fixa',
     'cqn_4 liga de fundo francesa'),
]
