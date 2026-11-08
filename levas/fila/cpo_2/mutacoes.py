# Mutacoes. A pessoa fica com o indice do contrato depois de tira-lo (o jogo le a vaga em cada contrato, salario e
# transferencia dela: muda a carreira; comparando, acusa). As ofertas guardadas tiradas sem reenviar (so o modo Rust e
# a carreira: o comparador roda a original; se nao houver oferta guardada ate 2002-06-01 PODE ESCAPAR, ver o ramo). O
# mes do fim da temporada 40 dias depois em vez de 10 (o mes sobe para quase todos os clubes; sem ramo: a 0x4d45f0
# tem 10 chamadores e pode ser quente).
MUTACOES = [
    ('cpo_tirar_contrato', 'cpo2_por(main_contract_slot(pessoa), usize::MAX); // a pessoa fica sem contrato',
     '// mutacao: a pessoa fica com o indice do contrato',
     'cpo_2 remove contract'),
    ('cpo_reenviar_oferta', 'reenviar(this, item, t, cpo2_d(item + 0x5d), (q as u32 & 0xffff_ff00) | byte, 1, 1, 0); // com a transferência',
     '// mutacao: sem reenviar',
     'cpo_2 resend offer'),
    ('cpo_mes_do_fim', 'let p = somar(d1.as_mut_ptr(), d2.as_mut_ptr(), 0xa); // dez dias depois',
     'let p = somar(d1.as_mut_ptr(), d2.as_mut_ptr(), 0x28); // mutacao: quarenta dias depois'),
]
