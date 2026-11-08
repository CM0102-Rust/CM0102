# Mutacoes (cqo_4), no calendario das ligas francesas de baixo (a criacao da carreira de 8 ligas): a 1a rodada de cada
# liga levada a outro dia da semana (os jogos dela caem em outro dia, e o resto da carreira muda desde agosto).
MUTACOES = [
    ('cqo4_terceira_primeira_rodada_em_outro_dia',
     'const FRENCH_THIRD_DIVISION_ROUNDS: [FraThirdLeagueRound; 38] = [\n    (7, 7, 0, 5,',
     'const FRENCH_THIRD_DIVISION_ROUNDS: [FraThirdLeagueRound; 38] = [\n    (7, 7, 0, 2,',
     'cqo_4 calendario da terceira francesa'),
    ('cqo4_segunda_primeira_rodada_em_outro_dia',
     'const SECOND_DIVISION_ROUNDS: [FraThirdLeagueRound; 38] = [\n    (31, 6, 0, 5,',
     'const SECOND_DIVISION_ROUNDS: [FraThirdLeagueRound; 38] = [\n    (31, 6, 0, 2,',
     'cqo_4 calendario da segunda francesa'),
    ('cqo4_baixo_primeira_rodada_em_outro_dia',
     'const LOWER_DIVISION_ROUNDS: [FraThirdLeagueRound; 38] = [\n    (31, 6, 0, 5,',
     'const LOWER_DIVISION_ROUNDS: [FraThirdLeagueRound; 38] = [\n    (31, 6, 0, 2,',
     'cqo_4 calendario da divisao de baixo francesa'),
]
