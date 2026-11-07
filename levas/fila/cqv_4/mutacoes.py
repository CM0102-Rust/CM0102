# Mutacoes (cqv_4). O construtor de League_Stage roda ~120 vezes na carreira de 8 ligas ate 2002-06-01 (os turnos dos
# estaduais e da J1, os grupos da Serie C, as fases das copas e das eliminatorias); o destrutor, ~40 (fica sem mutacao:
# so libera memoria, nada que a carreira leia depende dele):
# - a fase com um ponto a mais por vitoria (outras tabelas dos turnos e dos grupos: outros campeoes de turno);
# - a fase com um ponto a mais por empate;
MUTACOES = [
    ('cqv4_fase_ponto_a_mais_por_vitoria',
     'b(0xc2, pv); // cqv4: os pontos por vitória da fase',
     'b(0xc2, pv + 1); // mutacao: um ponto a mais por vitoria',
     'cqv_4 fase'),
    ('cqv4_fase_ponto_a_mais_por_empate',
     'b(0xc3, pe); // cqv4: os pontos por empate da fase',
     'b(0xc3, pe + 1); // mutacao: um ponto a mais por empate',
     'cqv_4 fase'),
]
# Prova do agente (07/11, carreira Rust de 8 ligas ate 2002-06-01 com motor de lances; base com os 10 itens finais): as duas ACUSADAS (2001-083, 2001-091). A da minitabela (sem os jogos entre os clubes) escapou e a SubLeague saiu do item.
