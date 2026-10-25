# Mutacoes do conversor das competicoes v1 e dos nomes em ingles. ESCAPAM da prova do save, da carreira e do jogo
# comparando (base e save versao 2; o caminho do eng.lng so roda com o jogo em outro idioma): so o
# autoteste_dbj_2 (CM_AUTOTESTE=1) as acusa, se estiver ligado em desvios::autotestes.
MUTACOES = [
    ('dbj2_competicoes', '(0x59, 4, 4)', '(0x59, 4, 3)'),
    ('dbj2_nacao', '(id as usize).wrapping_mul(109))', '(id as usize).wrapping_mul(108))'),
    ('dbj2_clube_curto', '.wrapping_add(0x34) // nome curto na linha', '.wrapping_add(0x33) // nome curto na linha'),
]
