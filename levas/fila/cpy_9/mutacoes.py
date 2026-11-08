# Mutacoes (cpy_9), nos arbitros sorteados da criacao do jogo (0x79c5c0, chamada pela montagem dos arbitros em cada
# nacao com poucos arbitros; contador "cpy_9 new referee"). A tabela dos arbitros ([0xae23e8]) entra no hash do banco
# (0xae2300..0xae2400), e a ordem dela decide as faixas de arbitros de cada nacao e liga (0x79bf60) que a escolha de
# cada jogo (0x79c7b0) usa: as tres mudam a tabela desde a criacao.
# 1) a nota +0x22 com +2 em vez de +1 (muda a chave da ordem, +0x28 x 50 + +0x22);
# 2) o atributo +0x28 com +8 em vez de +7 (a outra parte da chave);
# 3) sem ordenar a tabela depois de acrescentar (contador "cpy_9 sort"): as fichas novas ficam no fim e a montagem
#    calcula o primeiro de cada nacao pelo id.
MUTACOES = [
    ('cpy9_nota_mais_dois', '    cpy9_pw(r + 0x22, random_byte_plus(0x50, 1) as i8 as i16); // cpy9: a nota +0x22',
     '    cpy9_pw(r + 0x22, random_byte_plus(0x50, 2) as i8 as i16); // mutacao: +2',
     'cpy_9 new referee'),
    ('cpy9_atributo_28_mais_oito', '    cpy9_pb(r + 0x28, random_byte_plus(10, 7)); // cpy9: o atributo +0x28',
     '    cpy9_pb(r + 0x28, random_byte_plus(10, 8)); // mutacao: +8',
     'cpy_9 new referee'),
    ('cpy9_sem_ordenar', '    ordenar(cpy9_d(0xae23e8), n, 0x2b, 0x79d480); // cpy9: ordenar a tabela',
     '    let _ = (ordenar, n); // mutacao: sem ordenar',
     'cpy_9 sort'),
]
