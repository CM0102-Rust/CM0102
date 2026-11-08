# Mutacoes (com o ramo de cada uma). A oferta de prazo imediato (tipo 0) so pode se concretizar amanha (a verificacao
# das restricoes 0x8cfc90 segura as transferencias imediatas da IA um dia: as datas das transferencias mudam). O
# primeiro dia permitido pelas regras comeca um dia depois (as ofertas de tipo 1 e 4 concluem um dia mais tarde). A
# janela de julho comeca em 2 de julho (as ofertas de tipo 5 feitas entre fevereiro e junho concluem um dia depois).
# Os ramos dizem se a carreira passou por eles.
MUTACOES = [
    ('cpq5_amanha', 'copia(saida, &mut t, hoje); // tipo 0: hoje',
     'mais(hoje, saida, 1); // mutacao: amanha',
     'cpq_5 immediate completion'),
    ('cpq5_dia_depois', 'copia(&mut dia, &mut t, &inicio); // o primeiro dia testado é o início',
     'mais(&inicio, &mut dia, 1); // mutacao: um dia depois',
     'cpq_5 allowed day'),
    ('cpq5_julho', 'dma(&mut a, 1, 6, ano(), 0xffff_ffff); // 1º de julho',
     'dma(&mut a, 2, 6, ano(), 0xffff_ffff); // mutacao: 2 de julho',
     'cpq_5 July window'),
]
