# Mutacoes (sbE_8), com a Russia ate 2002-08-01 (os calendarios na criacao do jogo e na virada de novembro de 2001):
# - a primeira rodada da Primeira Divisao 7 dias depois (acusada pelo comparador lado a lado);
# - a sexta rodada da Premier 7 dias depois (acusada pelo comparador lado a lado, sem a Russia nas finais);
# - um confronto a mais no play-off do titulo (acusada pelo comparador lado a lado; so roda com dois empatados no topo).
MUTACOES = [
    ('sbE_8_primeira_divisao_depois',
     '(1, 3, 0, 6, 1, &[(-1, -1, -1)]), // 0: 1 Apr',
     '(8, 3, 0, 6, 1, &[(-1, -1, -1)]), // 0: 8 Apr (mutation)',
     'sbE_8 russian first division schedule'),
    ('sbE_8_premier_depois',
     '(30, 3, 0, 6, 1, &[(-1, -1, -1)]), // 5: 30 Apr',
     '(7, 4, 0, 6, 1, &[(-1, -1, -1)]), // 5: 7 May (mutation)',
     'sbE_8 russian premier league schedule'),
    ('sbE_8_confronto_a_mais',
     'r.teams = ((p << 1) as u16, p as u16, (p << 1) as u16, 0);',
     'r.teams = ((p << 1) as u16, (p + 1) as u16, (p << 1) as u16, 0); // mutation',
     'sbE_8 russian title play-off round'),
]
