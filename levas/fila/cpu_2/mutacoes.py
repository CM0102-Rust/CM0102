# Mutacoes. Todas na temporada do clube (0x669bd0) com a > c, o caso da linha da Europa (a = 23 de maio, c = 20 de
# abril na montagem 0x667150) e, pela mesma montagem, o das ligas europeias. De janeiro ao fim (t < c) fora da temporada:
# o clube na temporada 0x547770 e a forma fisica 0x62a920/0x62c680/0x62ce30 tratam a primavera como ferias (a forma e as
# lesoes de todos os jogadores mudam). Entre o fim e o inicio na temporada: o jogador fora da temporada 0x62ce30 e a
# forma da pre-temporada mudam nessa janela (julho de 2001 e maio de 2002, se a linha da nacao tiver a janela nessas
# datas). O fim no mesmo ano depois do inicio: a data da transferencia 0x8cee70 (o fim da temporada do vendedor mais
# 7 dias) cai um ano antes nas transferencias de agosto a dezembro que passam por ela (pode escapar se nenhuma pass_ball).
# Chamada a cada jogador: sem contador.
MUTACOES = [
    ('cpu2_fim_no_mesmo_ano', 'write_date(fim, c, y.wrapping_add(1)); // cpu2: o fim no ano seguinte',
     'write_date(fim, c, y); // mutacao: o fim no mesmo ano'),
    ('cpu2_primavera_fora', 'return 1; // cpu2: antes do fim, na temporada',
     'return 0; // mutacao: antes do fim, fora'),
    ('cpu2_janela_na_temporada', 'return 0; // cpu2: entre o fim e o inicio, fora',
     'return 1; // mutacao: entre o fim e o inicio, na temporada'),
]
