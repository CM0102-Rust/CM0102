# Mutacoes (cqo_5): a 1a rodada da CFA e da C1 levada a outro dia da semana (na criacao da carreira de 8 ligas: os
# jogos dela caem em outro dia) e o jogo de ida das semifinais da C1 uma semana antes (maio de 2002, antes do fim da
# carreira).
MUTACOES = [
    ('cqo5_cfa_primeira_rodada_em_outro_dia',
     'const CFA_ROUNDS: [FraCfaScheduleRound; 34] = [\n    (31, 6, 0, 5,',
     'const CFA_ROUNDS: [FraCfaScheduleRound; 34] = [\n    (31, 6, 0, 2,',
     'cqo_5 calendario da cfa'),
    ('cqo5_c1_primeira_rodada_em_outro_dia',
     'const SERIE_C1_ROUNDS: [FraCfaScheduleRound; 34] = [\n    (5, 8, 0, 6,',
     'const SERIE_C1_ROUNDS: [FraCfaScheduleRound; 34] = [\n    (5, 8, 0, 2,',
     'cqo_5 c1 calendar'),
    ('cqo5_semifinais_uma_semana_antes',
     'const CFA_SEMIFINAL_ROUND: EngCcCupCupRound = cql10r((8, 4, 1, 0), (0x15,',
     'const CFA_SEMIFINAL_ROUND: EngCcCupCupRound = cql10r((8, 4, 1, 0), (0xe,',
     'cqo_5 c1 play-offs'),
]
