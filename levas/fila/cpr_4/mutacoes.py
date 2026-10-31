# Mutacoes (com o ramo de cada uma). As ofertas desfeitas (estado 0xf com +0x2e = 3) passam a contar entre as
# ofertas abertas pelos jogadores do clube (a lista das transferencias futuras do clube 0x8ccb80 cresce e a verba de
# transferencias 0x598800, que a desconta, muda: as ofertas seguintes mudam). Os jogadores emprestados tambem passam
# a contar para o clube de onde sairam. A oferta desfeita pelo jogador da troca nao e agendada (nao volta a ser
# tratada e fica no vetor). Os ramos dizem se a carreira passou por eles.
MUTACOES = [
    ('cpr4_desfeita_conta', 'cpr4_b(o + 0x2e) < 3 // desfeita (+0x2e = 3) não conta',
     'true // mutacao: a desfeita conta',
     'cpr_4 oferta desfeita'),
    ('cpr4_emprestimo_conta', 'let dono = if c != 0 && cpr4_b(c + 0x35) & 0x3f != 5 { cpr4_clube(cpr4_d(c + 4)) } else { 0 }; // o empréstimo não conta',
     'let dono = if c != 0 { cpr4_clube(cpr4_d(c + 4)) } else { 0 }; // mutacao: o emprestimo conta',
     'cpr_4 jogador emprestado'),
    ('cpr4_sem_agenda', 'agendar(this, o, 0xa, 0); // a oferta volta daqui a 10 dias',
     'let _ = agendar; // mutacao: sem agenda',
     'cpr_4 desfazer oferta'),
]
