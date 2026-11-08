# Mutacoes (com o ramo de cada uma). Desfazer a oferta sem tirar a oferta de contrato (o jogador ainda pode assinar com
# o comprador: contratos e elencos mudam). O contrato do emprestimo com 10 pontos a mais na porcentagem do salario (os
# salarios pagos pelo clube que pega emprestado mudam). A transferencia futura do dia nao acontece (o jogador fica no
# clube: elencos, dinheiro e escalacoes mudam). Os ramos dizem se a carreira passou por eles.
MUTACOES = [
    ('cpq8_contrato_fica', 'tirar_contrato(cpq8_d(0xae1050), cpq8_pessoa(cpq8_d(oferta + 0xc)), cpq8_clube(cpq8_d(oferta + 8))); // a oferta de contrato sai',
     'let _ = tirar_contrato; // mutacao: a oferta de contrato fica',
     'cpq_8 undo offer'),
    ('cpq8_salario', 'pelo_atual(c.as_mut_ptr(), jogador, clube, fim, write_percent_pair, a5, a6); // o salário pela porcentagem',
     'pelo_atual(c.as_mut_ptr(), jogador, clube, fim, write_percent_pair.wrapping_add(10), a5, a6); // mutacao',
     'cpq_8 loan offer'),
    ('cpq8_sem_conclusao', 'concluir(this, oferta); // a transferência acontece',
     'let _ = concluir; // mutacao: nao acontece',
     'cpq_8 future transfer'),
]
