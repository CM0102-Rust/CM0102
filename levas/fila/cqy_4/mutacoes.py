# Mutacoes (cqy_4). A arvore de uma fase de copa com arvore (o mata-mata da Liga dos Campeoes depois dos grupos, em
# marco de 2002, dentro da janela; contadores "cqy_4 tie linked to the next round" e "cqy_4 entrant at home"):
# - os classificados entram nos confrontos deslocados de um (o 1o contra o 2o vira o 2o contra o 3o...): outros
#   adversarios nas quartas;
# - o vencedor de cada confronto vai para o lado espelhado da rodada seguinte (outra chave nas semifinais).
MUTACOES = [
    ('cqy4_adversarios_deslocados',
     'let start = q / 2; // cqy4: the entrant\'s tie by the pair',
     'let start = ((q + 1) % read_i16(comp_cup_round(this, r) + cup_round::NEW_TEAMS) as i32) / 2; // mutation: the opponents shifted',
     'cqy_4 entrant at home'),
    ('cqy4_chave_espelhada',
     'if link_tie_to_round(this, r, p, r + 1, s) {',
     'if link_tie_to_round(this, r, p, r + 1, read_i16(next + cup_round::TIES) as i32 - 1 - s) { // mutation: the slots mirrored',
     'cqy_4 tie linked to the next round'),
]
