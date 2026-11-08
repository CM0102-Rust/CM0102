# Mutacoes (cpw_2): a conclusao agendada para daqui a 2 dias (e nao hoje) na aprovacao final (0x8c3310, 1799 chamadas
# ate 2002-06-01 na cobertura de 17/10; "cpn_5 agendar oferta" passa de 4096) adia as transferencias acertadas: o
# jogador muda de clube em outro dia. A transferencia futura trocada pela conclusao agendada (as ofertas com prazo cuja
# data ainda vem; "cpr_3 acrescentar futura" passa de 128) faz o jogador mudar antes do prazo. Sem a conclusao
# (0x8c9390) no match_wrap_up da oferta aprovada (0x8c34e0, 739 chamadas; "cpp_7 soltar jogadores" passa de 512), a
# transferencia nao sai. As tres mudam os elencos.
MUTACOES = [
    ('cpw2_agendar_2_dias', '    cpw2_agendar(this, oferta, 0, 1); // cpw2: a conclusão agendada para hoje',
     '    cpw2_agendar(this, oferta, 2, 1); // mutacao: daqui a 2 dias',
     'cpw_2 agendar conclusao'),
    ('cpw2_sem_futura', '        if !antes { // cpw2: a data de conclusão ainda por vir',
     '        if antes || !antes { // mutacao: sem a futura',
     'cpw_2 transferencia futura'),
    ('cpw2_sem_concluir', '    concluir(this, oferta); // cpw2: a transferência se conclui',
     '    let _ = concluir; // mutacao: sem a conclusao',
     'cpw_2 concluir agora'),
]
