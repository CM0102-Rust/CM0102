# Mutacoes (cx03_23), carreira com as ligas da Suecia ate 2002-08-01. Nenhuma abre caixa.
# - o premio da Copa da Suecia 900.000: o dinheiro das rodadas muda (a carreira; o comparador confere o plano da
#   original);
# - a virada sem as posicoes previstas: as expectativas dos clubes suecos ficam as velhas (a carreira);
# - as posicoes previstas a partir do primeiro clube da reserva: o comparador acusa.
MUTACOES = [
    ('cx03_23_premio_900_mil',
     '    cup.set_prize_fund(800_000);',
     '    cup.set_prize_fund(900_000); // mutation'),
    ('cx03_23_virada_sem_posicoes_previstas',
     '    swe_cup_stages_and_ranking(cup, || {\n        call_cup_virtual(cup, comp_vtable::SET_PREDICTED_POSITIONS);\n    });',
     '    swe_cup_stages_and_ranking(cup, || {}); // mutation'),
    ('cx03_23_lugares_desde_o_primeiro',
     '    let mut k: i16 = 1;\n    let mut next: u8 = 1;',
     '    let mut k: i16 = 0; // mutation\n    let mut next: u8 = 1;'),
]
