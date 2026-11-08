# Mutacoes (cpy_4), nos amistosos internacionais do dia (0x5cbb00, 515 chamadas ate 2002-06-01 na cobertura de 17/10,
# pelo dia dos amistosos 0x5c2460). Sem a 0x5cbdf0 as datas internacionais nao viram jogos: as selecoes jogam menos, os
# convocados dos clubes descansam e os sorteios das partidas que faltam deslocam a carreira inteira. Com hoje + 11
# semanas em vez de 12, os mesmos amistosos sao marcados uma semana antes: os sorteios da 0x5cbdf0 caem em outros dias.
# Sem ordenar as nacoes pela injury_word_0c na virada do ano (1/1/2002, dentro da carreira), a 0x5cbdf0 pareia outras
# selecoes nos amistosos de 2002.
MUTACOES = [
    ('cpy4_sem_marcar_amistosos', '    marcar(this, primeira, quantas); // cpy4: marcar os amistosos da data',
     '    let _ = marcar; // mutacao: os amistosos nao sao marcados',
     'cpy_4 friendlies scheduled'),
    ('cpy4_onze_semanas', '    semanas(0xae2c90, &mut lim, 0xc); // cpy4: hoje + 12 semanas',
     '    semanas(0xae2c90, &mut lim, 0xb); // mutacao: hoje + 11 semanas',
     'cpy_4 friendlies scheduled'),
    ('cpy4_nacoes_sem_ordem', '        ordenar(cpy4_d(this + 0x34fa), cpy4_d(this + 0x34fe) as u32, 4, 0x5cc5f0); // cpy4: as nações pela reputação',
     '        // mutacao: as nacoes ficam fora da ordem de injury_word_0c',
     'cpy_4 year rollover'),
]
