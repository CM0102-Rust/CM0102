# Mutacoes (sx05_6). A selecao decide quem entra na equipe da semana e da temporada (o premio registrado, as noticias
# e o credito dos titulares); acusadas pelos comparadores lado a lado (as vagas e o retorno).
# - a posicao 8 deixando de valer 0x10;
# - sem a troca do ocupante (o jogador fica de fora quando so cabe numa vaga ocupada);
# - os reservas com a ordem 4 e 0x40 trocada.
MUTACOES = [
    ('sx05_6_oito_sem_dezesseis',
     '    if m & 8 != 0 {\n        m |= 0x10;',
     '    if m & 8 != 0 {\n        m |= 0; // mutation: 8 no longer gives 0x10',
     'sx05_6 candidate'),
    ('sx05_6_sem_troca',
     '            core::ptr::copy(t.slot(k) as *const u8, t.slot(j) as *mut u8, TEAM_AWARD_SLOT_SIZE);\n            return place(k);',
     '            return t.complete() as u32; // mutation: no move',
     'sx05_6 occupant moved'),
    ('sx05_6_reservas_trocados',
     'const TEAM_AWARD_SUBSTITUTES: [u16; 9] = [1, 0x40, 4,',
     'const TEAM_AWARD_SUBSTITUTES: [u16; 9] = [1, 4, 0x40, /* mutation */',
     'sx05_6 candidate'),
]
