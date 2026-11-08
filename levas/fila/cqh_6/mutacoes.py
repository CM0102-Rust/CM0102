# Mutacoes (cqh_6), na lista dos clubes da competicao (0x48e1c0: o contador "cqh_6 club in the list" passou de 128; o
# hash das partidas a pega) e nas outras duas:
# - a lista com no maximo 16 clubes;
# - a lista sem tirar os repetidos;
# - todas as fases completas sem olhar as ligas de baixo da cadeia (0x48ce10): [inferencia] escapa ate 2002-06-01 (o
#   contador da liga de baixo completa nao rodou: a cadeia so fecha no fim da temporada).
# (02/11: a situacao do time na copa lida de +4 (0x48cc70) escapou: a 0x48cc70 nao rodou ate 2002-06-01.)
# Resultado (03/11): a lista de 16 pega (so pelo hash das partidas); a lista com repetidos escapa (sem clube repetido nas
# fases da carreira) e a das ligas de baixo escapa (a cadeia so fecha no fim da temporada).
MUTACOES = [
    ('cqh6_lista_de_16',
     '&& n < 0xc8 { // cqh6: até 200 clubes',
     '&& n < 0x10 { // mutacao: ate 16 clubes',
     'cqh_6 club in the list'),
    ('cqh6_lista_com_repetidos',
     'if clube != 0 && !(0..n).any(|k| cqh6_d(v + k as usize * 4) == clube) && n < 0xc8 {',
     'if clube != 0 && n < 0xc8 { // mutacao: com os repetidos',
     'cqh_6 club in the list'),
    ('cqh6_sem_as_ligas_de_baixo',
     'i = cqh6_d(c + 0x20) as i32; // cqh6: a liga de baixo',
     'i = -1; // mutacao: sem as ligas de baixo',
     'cqh_6 lower league complete'),
]
