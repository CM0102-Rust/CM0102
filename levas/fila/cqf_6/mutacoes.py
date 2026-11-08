# Mutacoes (cqf_6), na tabela das rodadas das fases de grupos da Copa do Mundo (0x92c290: a fase -1 na criacao das
# regras da Copa, na carga do jogo, e as fases 0..6 no sorteio de 27/12/2001; o contador passou de 1024):
# - a primeira rodada em 12 de maio em vez de 12 de junho: os jogos da Copa caem antes de 2002-06-01;
# - a segunda rodada em 19 de maio;
# - a terceira rodada em 24 de maio.
# (02/11: as primeiras versoes da segunda e da terceira, outro dia da semana na terceira rodada e duas rodadas em vez
# de tres, escaparam: mudam so jogos de depois de 2002-06-01.)
# O contador fica depois da alocacao da tabela (todas as fases de grupos passam por ele).
MUTACOES = [
    ('cqf6_primeira_rodada_em_maio',
     'data(t, 0, 0xc, 5, 0, 4, 1, ano, 1); // cqf6: a primeira rodada',
     'data(t, 0, 0xc, 4, 0, 4, 1, ano, 1); // mutacao: em maio',
     'cqf_6 round fixtures'),
    ('cqf6_segunda_rodada_em_maio',
     'data(t, 1, 0x13, 5, 0, 4, 1, ano, 1); // cqf6: a segunda rodada',
     'data(t, 1, 0x13, 4, 0, 4, 1, ano, 1); // mutacao: em maio',
     'cqf_6 round fixtures'),
    ('cqf6_terceira_rodada_em_maio',
     'data(t, 2, 0x18, 5, 0, 2, 1, ano, 1); // cqf6: a terceira rodada',
     'data(t, 2, 0x18, 4, 0, 2, 1, ano, 1); // mutacao: em maio',
     'cqf_6 round fixtures'),
]
