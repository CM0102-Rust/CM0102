# Mutacoes (cqv_4). O construtor de League_Stage roda ~120 vezes na carreira de 8 ligas ate 2002-06-01 (os turnos dos
# estaduais e da J1, os grupos da Serie C, as fases das copas e das eliminatorias); a minitabela, 9 (as tabelas dos
# segundos colocados das eliminatorias europeias da Euro e da Copa); o destrutor, ~40 (fica sem mutacao: so libera
# memoria, nada que a carreira leia depende dele):
# - a fase com um ponto a mais por vitoria (outras tabelas dos turnos e dos grupos: outros campeoes de turno);
# - a fase com um ponto a mais por empate;
# - a minitabela sem os jogos entre os clubes (a tabela dos segundos colocados vazia: outra ordem, outro melhor segundo
#   colocado e outros classificados para a repescagem).
MUTACOES = [
    ('cqv4_fase_ponto_a_mais_por_vitoria',
     'b(0xc2, pv); // cqv4: os pontos por vitória da fase',
     'b(0xc2, pv + 1); // mutacao: um ponto a mais por vitoria',
     'cqv_4 fase'),
    ('cqv4_fase_ponto_a_mais_por_empate',
     'b(0xc3, pe); // cqv4: os pontos por empate da fase',
     'b(0xc3, pe + 1); // mutacao: um ponto a mais por empate',
     'cqv_4 fase'),
    ('cqv4_minitabela_sem_jogos',
     'if somar(this, filtro, arg, de, ate, comp, varredura, lista) == 0 { // cqv4: os jogos entre os clubes da minitabela',
     'if { let _ = (somar, filtro, arg, de, ate, varredura, lista); 1 } == 0 { // mutacao: sem os jogos',
     'cqv_4 minitabela'),
]
