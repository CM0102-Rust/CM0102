# Mutacoes (cqf_5), no sorteio da Copa do Mundo de 2002 (27/12/2001):
# - o grupo 1 (o da propria Copa, com o jogo de abertura de 31/05) recebe os times percorrendo a lista do fim para o
#   comeco: a ordem das linhas muda e, com ela, a ordem sorteada dos clubes e os jogos do grupo;
# - os times de cada grupo 2..8 entram nas vagas na ordem inversa: os jogos das fases mudam (e os da TV escolhidos
#   quatro semanas antes, em maio);
# - so metade das nacoes passa a ter a Copa como competicao atual (0x775220): o gerenciador dos tecnicos e as
#   convocacoes das selecoes mudam.
# Os contadores ficam colados no efeito: o time do grupo um e o time do grupo depois da condicao do grupo; a competicao
# da nacao dentro do laco.
MUTACOES = [
    ('cqf5_grupo_um_do_fim',
     'let e = i as usize; // cqf5: a entrada da lista',
     'let e = 31 - i as usize; // mutacao: a lista do fim para o comeco',
     'cqf_5 time do grupo um'),
    ('cqf5_vagas_do_grupo_invertidas',
     'cqf5_pd(times + n as usize * 4, cqf5_clube(lista, i as usize)); // cqf5: o time na vaga do grupo',
     'cqf5_pd(times + (3 - n as usize) * 4, cqf5_clube(lista, i as usize)); // mutacao: as vagas invertidas',
     'cqf_5 time do grupo'),
    ('cqf5_metade_das_nacoes',
     'i += 1; // cqf5: a próxima nação',
     'i += 2; // mutacao: uma sim, uma nao',
     'cqf_5 competição da nação'),
]
