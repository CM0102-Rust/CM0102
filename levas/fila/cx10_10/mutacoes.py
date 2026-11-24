# Mutacoes (cx10_10), carreira com a Holanda ate 2002-06-01. Nenhuma abre caixa. O comparador acusa (toda rodada de liga
# grava o progresso dos clubes); a da Regionalliga so com ela carregada (escapa no teste, ver rotinas.py).
MUTACOES = [
    ('cx10_10_holanda_pior_igual_melhor',
     '    if (0x10..=0x11).contains(&worst) {\n        worst = 0x12;\n    }',
     '    if (0x10..=0x11).contains(&worst) {\n        worst = 0xf; // mutation\n    }'),
    ('cx10_10_holanda_atual_no_melhor',
     '    store_club_progress(this, club, current as u8, best, worst);',
     '    store_club_progress(this, club, best, best, worst); // mutation'),
    ('cx10_10_regional_sem_impar',
     '    let odd = |c: u32| (c as u8).wrapping_mul(2).wrapping_sub(1);',
     '    let odd = |c: u32| (c as u8).wrapping_mul(2); // mutation'),
]
