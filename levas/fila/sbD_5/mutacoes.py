# Mutacoes (sbD_5), pela carreira com a Belgica ate 2002-08-01 e pelo comparador do calendario (lado a lado):
# - a ultima rodada dos grupos 7 dias depois (13 de maio);
# - o extra dos grupos trocado (a fase -1 com o id da fase 0);
# - o jogo unico da repescagem da fase 8 sete dias depois (7 de junho).
MUTACOES = [
    ('sbD_5_ultima_rodada_7_dias', '    (6, 4, 1, 6, 1, &[(-1, -1, -1)]),   // 29: 6 May +1',
     '    (13, 4, 1, 6, 1, &[(-1, -1, -1)]),   // 29: mutation', 'sbD_5 belgian third division league schedule'),
    ('sbD_5_extra_dos_grupos', '        *extra = 0x41f + (stage != -1) as i16;',
     '        *extra = 0x41f + (stage == -1) as i16; // mutation', 'sbD_5 belgian third division league schedule'),
    ('sbD_5_repescagem_7_dias', '(0x1f, 4, 1, 2, 2, 0x28)', '(7, 5, 1, 2, 2, 0x28) /* mutation */',
     'sbD_5 belgian third division play-off schedule'),
]
