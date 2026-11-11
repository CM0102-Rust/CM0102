# Mutacoes (cqv_4). O construtor de League_Stage roda ~120 vezes na carreira de 8 ligas ate 2002-06-01 (os turnos dos
# estaduais e da J1, os grupos da Serie C, as fases das copas e das eliminatorias); o destrutor, ~40 (fica sem mutacao:
# so libera memoria, nada que a carreira leia depende dele):
# - a fase com um ponto a mais por vitoria (outras tabelas dos turnos e dos grupos: outros campeoes de turno);
# - a fase com um ponto a mais por empate;
MUTACOES = [
    ('cqv4_fase_ponto_a_mais_por_vitoria',
     "b(league::WIN_POINTS, win_points); // cqv4: the stage's points for a win",
     'b(league::WIN_POINTS, win_points + 1); // mutation: one more point for a win',
     'cqv_4 league stage'),
    ('cqv4_fase_ponto_a_mais_por_empate',
     "b(league::DRAW_POINTS, draw_points); // cqv4: the stage's points for a draw",
     'b(league::DRAW_POINTS, draw_points + 1); // mutation: one more point for a draw',
     'cqv_4 league stage'),
]
# Prova do agente (07/11, carreira Rust de 8 ligas ate 2002-06-01 com motor de lances; base com os 10 itens finais): as duas ACUSADAS (2001-083, 2001-091). A da minitabela (sem os jogos entre os clubes) escapou e a SubLeague saiu do item.
