# Mutacoes (cqo_6), na criacao da carreira de 8 ligas: a 1a rodada da Serie A, da Serie B e da primeira divisao
# francesa levada a outro dia da semana (os jogos dela caem em outro dia, e o resto da carreira muda desde agosto).
MUTACOES = [
    ('cqo6_serie_a_primeira_rodada_em_outro_dia',
     'const SERIE_A_ROUNDS: [ItaSerALeagueRound; 34] = [\n    (28, 7, 0, 6,',
     'const SERIE_A_ROUNDS: [ItaSerALeagueRound; 34] = [\n    (28, 7, 0, 2,',
     'cqo_6 calendario da serie a'),
    ('cqo6_serie_b_primeira_rodada_em_outro_dia',
     'const SERIE_B_ROUNDS: [ItaSerALeagueRound; 38] = [\n    (29, 7, 0, 6,',
     'const SERIE_B_ROUNDS: [ItaSerALeagueRound; 38] = [\n    (29, 7, 0, 2,',
     'cqo_6 calendario da serie b'),
    ('cqo6_francesa_primeira_rodada_em_outro_dia',
     'const FRENCH_FIRST_DIVISION_ROUNDS: [(bool, ItaSerALeagueRound); 38] = [\n    (false, (31, 6, 0, 5,',
     'const FRENCH_FIRST_DIVISION_ROUNDS: [(bool, ItaSerALeagueRound); 38] = [\n    (false, (31, 6, 0, 2,',
     'cqo_6 calendario da primeira francesa'),
]
