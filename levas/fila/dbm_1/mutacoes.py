# Mutacoes dos nomes em ingles (sigla da nacao, competicao, competicao de pessoal). ESCAPAM da prova do save, da
# carreira e do jogo comparando (o caminho do eng.lng so roda com o jogo em outro idioma): so o autoteste_dbm_1
# (CM_AUTOTESTE=1 ou mutacoes.py --autoteste-banco) as acusa, se estiver ligado em desvios::autotestes.
MUTACOES = [
    ('dbm1_sigla', '.wrapping_add(0x4f) // sigla na linha', '.wrapping_add(0x4e) // sigla na linha'),
    ('dbm1_competicao', 'if id >= de_clube {', 'if id > de_clube {'),
    ('dbm1_pessoal', '.wrapping_mul(79)) // competição de pessoal na tabela', '.wrapping_mul(78)) // competição de pessoal na tabela'),
]
