# Mutacoes dos conversores da versao 1. ESCAPAM da prova do save e da carreira (base de fabrica e save sao
# versao 2): so o autoteste_dbj_1 (CM_AUTOTESTE=1) as acusa, se estiver ligado em desvios::autotestes.
# dbj1_clubes so muda a injury_word_0c 0 (os casos so de zeros do autoteste); as outras deslocam o arquivo.
MUTACOES = [
    ('dbj1_clubes', '|b| (b.max(1) as u16).wrapping_mul(500)', '|b| (b as u16).wrapping_mul(500)'),
    ('dbj1_nao_jogadores', '(0x23, 4, 8)', '(0x23, 4, 7)'),
    ('dbj1_jogadores', '(0xf, 1, 55)', '(0xf, 1, 54)'),
]
