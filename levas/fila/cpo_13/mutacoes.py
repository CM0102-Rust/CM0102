# Mutacoes (com o ramo de cada uma). A competicao atual da nacao nao muda (a selecao fica com a competicao velha: as
# convocacoes e os jogos das selecoes da 0x76b100 mudam). Os clubes com jogo na data sem a marca do mandante, ou do
# visitante: a remarcacao dos jogos dos clubes com mais de dois convocados (0x7777e0, que le os codigos 1 e 2) deixa
# de remarcar esses jogos (as datas dos jogos mudam). O comparador da 0x775220 roda so a original (a primeira so pela
# carreira); o da 0x7775c0 e lado a lado (as duas ultimas acusam tambem comparando). A 0x556830 (o regen) fica sem
# mutacao: o registro do regen e todo zerado no fim e a entrada liberada nao e lida de novo pela pessoa (os efeitos de
# uma mutacao ali nao chegam ao jogo).
MUTACOES = [
    ('cpo13_competicao_atual_parada', 'cpo13_por(p.vaga, nova);',
     'let _ = nova; // mutacao: a competicao atual nao muda',
     'cpo_13 competicao atual'),
    ('cpo13_mandante_sem_marca', 'cpo13_marcar(this, casa, n, codigo); // o mandante',
     '// mutacao: o mandante sem marca',
     'cpo_13 clubes com jogo'),
    ('cpo13_visitante_sem_marca', 'cpo13_marcar(this, fora, n, codigo); // o visitante',
     '// mutacao: o visitante sem marca',
     'cpo_13 clubes com jogo'),
]
