# Mutacoes. No jogo novo, o jogador acima do limit de idade so fica com habilidade (+5) acima de 100, nao de 80 (a
# lista inicial ganha aposentadorias: os regens e os elencos mudam na temporada). Na busca de novas aposentadorias, o
# jogador de clube fora das competicoes do jogo se aposenta com sorteio(2) = 0, nao sorteio(3) (mais aposentadorias;
# os sorteios seguintes tambem mudam). A data da aposentadoria pela nacao com sorteio(31) e nao sorteio(30) (as datas
# mudam no jogo novo e na temporada; a 0x7a59e0 aposenta e cria os regens em outras datas).
MUTACOES = [
    ('cpr8_habilidade_ate_100', 'if hab > 0x50 {\n            return 0; // cpr8: a habilidade alta segura o jogador',
     'if hab > 0x64 {\n            return 0; // mutacao: ate 100',
     'cpr_8 inicio habilidade'),
    ('cpr8_um_em_dois', "if random(3) == 0 {\n                break 'm 1; // cpr8: um em três se aposenta",
     "if random(2) == 0 {\n                break 'm 1; // mutacao: um em dois",
     'cpr_8 fora do jogo'),
    ('cpr8_fim_de_temporada_31', 'let d2 = (random(30) as u16).wrapping_add(cpr8_u(t + 0x15)); // o fim de temporada da nação',
     'let d2 = (random(31) as u16).wrapping_add(cpr8_u(t + 0x15)); // mutacao: sorteio(31)',
     'cpr_8 datas pela nacao'),
]
