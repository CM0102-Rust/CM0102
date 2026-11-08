# Mutações (cqo_1): todo jogo comum passa pelo estádio do mandante (milhares na carreira de 8 ligas); a busca do estádio
# neutro pela nota roda nos jogos de seleção e nas finais (centenas); o rodízio das sedes nos torneios com sede (dezenas).
# O estádio entra no público e na renda (0x85cba0) e nas reservas que as buscas seguintes conferem.
# - o jogo comum no estádio do mandante marcado como neutro (+0x3c = 1: o público usa a metade da lotação e o motor
#   tira o fator casa) (contador "cqo_1 estadio do mandante"). A versão anterior (o estádio do visitante) derrubava o
#   jogo logo no começo (EXCECAO): trocada por esta, que muda o resultado sem travar;
# - o neutro: o primeiro livre com nota em vez do de maior nota (contador "cqo_1 neutro pela nota");
# - o rodízio sem a reserva do estádio (o mesmo estádio fica livre para o jogo seguinte) (contador "cqo_1 rodizio").
MUTACOES = [
    ('cqo1_mandante_neutro', 'fixar(cqo1_d(cqo1_d(fx + 0x1c) + 0x69)); // cqo1: o estádio do mandante', '*neutro = 1; // mutacao: o jogo comum marcado como neutro\n                fixar(cqo1_d(cqo1_d(fx + 0x1c) + 0x69));', 'cqo_1 estadio do mandante'),
    ('cqo1_primeiro_neutro', 'melhor = n; // cqo1: a melhor nota\n                        escolhida = e;', 'melhor = n; // mutacao: o primeiro com nota\n                        if escolhida == 0 {\n                            escolhida = e;\n                        }', 'cqo_1 neutro pela nota'),
    ('cqo1_rodizio_sem_reserva', 'crate::comparison::branch("cqo_1 rodizio");\n                        add_stadium_booking(this, cqo1_d(e + 0xc), data);', 'crate::comparison::branch("cqo_1 rodizio"); // mutacao: sem a reserva', 'cqo_1 rodizio'),
]
