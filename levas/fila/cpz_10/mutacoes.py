# Mutacoes (cpz_10), todas no sorteio do comentarista (0x7dab90, a cada historia da imprensa: 0x72c1f0, 0x733a60, e
# pelos textos das noticias, que o jogo escreve na carreira automatica: 0x77bc80 com 4568 chamadas ate 2002-06-01 na
# cobertura de 17/10). Sem o sorteio(n): um sorteio a menos por comentarista escolhido. Sem os comentaristas locais (o
# clube do pundit.cfg nao bate): os tipos 2 e 4 devolvem -1 sem sortear. Sem o -1 como qualquer nacao (a historia da
# explosao do jogador 0x733a60 sempre passa -1): ninguem aceito, -1 sem sortear. As tres mudam a contagem dos sorteios.
# Risco: o pundit.cfg so tem comentaristas de clubes ingleses e escoceses (os outros de nacao padrao [0x9cf2e4]);
# historias de clubes de fora nao sorteiam, e os contadores mostram se os ramos rodam.
MUTACOES = [
    ('cpz10_sem_sorteio', '    let r = sortear(n as i32); // cpz10: o sorteio do comentarista',
     '    let r = { let _ = sortear; 0i32 }; // mutacao: sem o sorteio',
     'cpz_10 commentator draw'),
    ('cpz10_sem_locais', '            if cpz10_u(p + 0xcb) == cpz10_u(cpz10_d(cpz10_d(clube + 0x69) + 0x38)) { // cpz10: o comentarista local',
     '            if cpz10_u(p + 0xcb) == cpz10_u(cpz10_d(cpz10_d(clube + 0x69) + 0x38)) && false { // mutacao: nenhum local',
     'cpz_10 local commentator'),
    ('cpz10_sem_qualquer_nacao', '                    true // cpz10: qualquer nação',
     '                    cpz10_u(p + 0xcf) as i32 == nacao // mutacao: -1 nao e qualquer nacao',
     'cpz_10 commentator of any nation'),
]
