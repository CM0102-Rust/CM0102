# Mutacoes (cqs_7), nas fases de mata-mata das copas (53 construidas ate 2002-06-01): as sementes do vetor zeradas (o
# sorteio sem potes: a Copa da Italia em 2001-247); os times na ordem inversa (outros confrontos); a fase no ano
# seguinte (as rodadas dela fora da carreira).
MUTACOES = [
    ('cqs7_sementes_zeradas', '*((sementes + i as usize) as *const u8) // cqs7: a semente do vetor', '0 // mutacao', 'cqs_7 fase de copa'),
    ('cqs7_times_em_ordem_inversa', 'cqs7_d(times + i as usize * 4)); // cqs7: o time i', 'cqs7_d(times + (m - 1 - i) as usize * 4)); // mutacao', 'cqs_7 fase de copa'),
    ('cqs7_fase_no_ano_seguinte', 'cqs7_pw(this + 0x40, ano as u16); // cqs7: o ano da fase', 'cqs7_pw(this + 0x40, (ano as u16).wrapping_add(1)); // mutacao', 'cqs_7 fase de copa'),
]
