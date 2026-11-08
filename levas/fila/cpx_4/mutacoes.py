# Mutacoes (cpx_4), todas no emprestimo do arquivo de configuracao (0x8bf400, 49 chamadas ate 2002-06-01 na
# cobertura de 17/10, no comeco do jogo): sem assinar o contrato de emprestimo (0x4d01e0), o jogador fica no clube de
# origem e os elencos da carreira inteira mudam. Sem a permissao de trabalho (0x8c7070) do emprestado que precisa dela,
# ele nao pode jogar no clube que o recebeu. Sem estender a data da pessoa (+0x46) e o fim do contrato ate o fim do
# emprestimo, o contrato acaba antes da volta e o jogador sai livre. As tres mudam elencos e contratos.
MUTACOES = [
    ('cpx4_sem_contrato_de_emprestimo', '    assinar(cpx4_d(0xae1050), l); // cpx4: o contrato de empréstimo',
     '    let _ = assinar; // mutacao: sem o contrato de emprestimo',
     'cpx_4 loan from the settings'),
    ('cpx4_sem_permissao', '        permissao(this, p, cpx4_d(clube + 0x53), cpx4_w(inicio + 2) as u16 as u32, 1, 1); // cpx4: a permissão de trabalho',
     '        let _ = permissao; // mutacao: sem a permissao de trabalho',
     'cpx_4 work permit'),
    ('cpx4_sem_estender_contrato', '        transfer_manager_assign_date(contrato(cpx4_d(0xae1050), p) + 0x2d, fim); // cpx4: o fim do contrato até o fim do empréstimo',
     '        let _ = contrato; // mutacao: o fim do contrato fica antes do fim do emprestimo',
     'cpx_4 contract extended'),
]
