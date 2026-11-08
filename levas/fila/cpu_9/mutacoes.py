# Mutacoes, todas nas fichas de treino iniciais (uma vez no comeco de cada carreira nova). A primeira atualizacao do
# treino de cada jogador (0x8b1750, na primeira escalacao dele) poe em dia as semanas desde o comeco pela vaga da ficha,
# e so depois refaz a vaga dos jogadores do computador; o clube humano fica com ela. Goleiros no programa geral, os
# outros no programa 2 (50 na primeira categoria) ou so os goleiros de nota 20 no programa de goleiros mudam o progresso
# dessa primeira atualizacao, e os atributos vao para o motor das partidas. A 0x7a8350 (~1 chamada) e a 0x7a6e20 vao de
# passageiras, sem mutacao.
MUTACOES = [
    ('cpu9_goleiros_no_geral', '*(ficha() as *mut u8) = 6; // cpu9: o programa dos goleiros',
     '*(ficha() as *mut u8) = 1; // mutacao: goleiros no programa geral',
     'cpu_9 goalkeeper'),
    ('cpu9_linha_no_programa_2', '*(ficha() as *mut u8) = 1; // cpu9: o programa geral',
     '*(ficha() as *mut u8) = 2; // mutacao: os outros no programa 2',
     'cpu_9 outfield'),
    ('cpu9_goleiro_so_com_20', 'if gol >= 0xf { // cpu9: goleiro de 15 para cima',
     'if gol >= 0x14 { // mutacao: so a nota 20 vai para o programa dos goleiros',
     'cpu_9 goalkeeper'),
]
