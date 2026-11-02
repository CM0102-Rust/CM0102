# Mutacoes (cqh_6), nas rotinas da competicao:
# - todas as fases completas sem olhar as ligas de baixo da cadeia (0x48ce10): o fim da temporada das competicoes
#   chega antes;
# - a lista dos clubes da competicao com no maximo 16 (0x48e1c0);
# - a situacao do time na copa lida de +4 em vez de +5 (0x48cc70).
# [inferencia] a lista dos clubes e o progresso do clube alimentam sobretudo telas e textos e podem escapar.
MUTACOES = [
    ('cqh6_sem_as_ligas_de_baixo',
     'i = cqh6_d(c + 0x20) as i32; // cqh6: a liga de baixo',
     'i = -1; // mutacao: sem as ligas de baixo',
     'cqh_6 liga de baixo completa'),
    ('cqh6_lista_de_16',
     '&& n < 0xc8 { // cqh6: até 200 clubes',
     '&& n < 0x10 { // mutacao: ate 16 clubes',
     'cqh_6 clube na lista'),
    ('cqh6_situacao_de_mais_4',
     '*situacao = cqh6_b(t + 5); // cqh6: a situação do time',
     '*situacao = cqh6_b(t + 4); // mutacao: o byte +4',
     'cqh_6 progresso na copa'),
]
