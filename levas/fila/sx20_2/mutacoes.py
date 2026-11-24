# Mutacoes (sx20_2). O construtor roda na criacao do jogo e quando a passada das temporadas refaz o objeto; os grupos
# decidem quem joga contra quem nos jogos simulados (sx20_1 / 0x84be20), cujas estatisticas entram no banco:
# - a ordem pela reputacao invertida (menor primeiro: outros grupos);
# - metade das trocas do embaralhamento.
MUTACOES = [
    ('sx20_2_reputacao_invertida',
     'if club_at(a as isize).reputation() < club_at(b as isize).reputation() { // sx20_2: highest reputation first',
     'if club_at(a as isize).reputation() > club_at(b as isize).reputation() { // mutation: lowest first',
     'sx20_2 simulated stats groups'),
    ('sx20_2_menos_trocas',
     'let swaps = n / 2; // sx20_2: how many swaps',
     'let swaps = n / 4; // mutation: half the swaps',
     'sx20_2 simulated stats groups'),
]
