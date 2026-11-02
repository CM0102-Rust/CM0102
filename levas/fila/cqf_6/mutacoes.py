# Mutacoes (cqf_6), na tabela das rodadas das fases de grupos da Copa do Mundo (0x92c290: a fase -1 na criacao das
# regras da Copa, na carga do jogo, e as fases 0..6 no sorteio de 27/12/2001):
# - a primeira rodada em 12 de maio em vez de 12 de junho: os jogos da Copa caem antes de 2002-06-01;
# - a primeira rodada levada a outro dia da semana: as datas dos jogos (e a escolha dos jogos da TV quatro semanas
#   antes, com os sorteios dela) mudam de dia;
# - duas rodadas em vez de tres: a fase recebe outra contagem de rodadas e os jogos dos grupos mudam.
# O contador fica depois da alocacao da tabela (todas as fases de grupos passam por ele).
MUTACOES = [
    ('cqf6_primeira_rodada_em_maio',
     'data(t, 0, 0xc, 5, 0, 4, 1, ano, 1); // cqf6: a primeira rodada',
     'data(t, 0, 0xc, 4, 0, 4, 1, ano, 1); // mutacao: em maio',
     'cqf_6 tabela das rodadas'),
    ('cqf6_terceira_rodada_outro_dia',
     'data(t, 2, 0x18, 5, 0, 2, 1, ano, 1); // cqf6: a terceira rodada',
     'data(t, 2, 0x18, 5, 0, 6, 1, ano, 1); // mutacao: outro dia da semana',
     'cqf_6 tabela das rodadas'),
    ('cqf6_duas_rodadas',
     '*rodadas = 3; // cqf6: três rodadas',
     '*rodadas = 2; // mutacao: duas rodadas',
     'cqf_6 tabela das rodadas'),
]
