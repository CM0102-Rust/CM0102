# Mutacoes (cpx_9), em tirar o jogador da escalacao (0x896c60: transferencias, emprestimos, aposentadorias, saidas do
# elenco e a escalacao automatica dos jogos): a vaga fica com a pessoa (ela continua escalada no clube que deixou ou
# machucada), so os titulares (vagas 0..10) sao conferidos e so os reservas (11..19). As tres deixam pessoas em vagas que
# a original esvazia; a escalacao seguinte do clube (as vagas marcadas, 0x8972c0) muda e o jogo com ela. Escapam se a
# pessoa tirada nunca estiver numa vaga do tipo mutado (contadores "cpx_9 tirou titular" e "cpx_9 tirou reserva") ou se
# a escalacao automatica refizer as vagas antes de usa-las.
MUTACOES = [
    ('cpx9_vaga_fica', '            cpx9_pd(vaga, u32::MAX); // cpx9: a vaga fica vazia',
     '            let _ = vaga; // mutacao: a vaga fica com a pessoa',
     'cpx_9 tirou titular'),
    ('cpx9_so_titulares', '    for k in 0..20usize { // cpx9: as vagas da escalação',
     '    for k in 0..11usize { // mutacao: so os titulares',
     'cpx_9 tirou reserva'),
    ('cpx9_so_reservas', '    for k in 0..20usize { // cpx9: as vagas da escalação',
     '    for k in 11..20usize { // mutacao: so os reservas',
     'cpx_9 tirou titular'),
]
