# Mutacoes (cqx_7), nas posicoes previstas da Copa da Italia (rodam na criacao, no primeiro dia; nada as le ate junho:
# sao para o comparador lado a lado, pela confirmacao no modo comparando ate 2001-08-20): faixas de 4 em vez de 8; o
# campeao da final previsto em 2 (o lugar comeca em 1). Os registros e a noticia sao passageiros (sem mutacao: o
# historico nao chega a carreira ate junho). A noticia: o vinculo do clube com o tipo 0x7d0 (o de competicao) em vez
# de 0x7d5: so o comparador lado a lado do texto (compare_text_writer) pega, se o grupo da copa cair na confirmacao.
MUTACOES = [
    ('cqx7_copa_da_italia_faixas_de_4',
     '7, 8, "cqx_7 coppa italia predicted") // cqx7: the Coppa Italia bands of 8',
     '7, 4, "cqx_7 coppa italia predicted") // mutation: bands of 4',
     'cqx_7 coppa italia predicted'),
    ('cqx7_campeao_previsto_em_2',
     'give(place, &mut idx, &mut next); // cqx7: the final\'s winner',
     'give(place + 1, &mut idx, &mut next); // mutation: the winner predicted 2nd',
     'cqx_7 coppa italia predicted'),
    ('cqx7_vinculo_do_clube_trocado',
     'add_link(text, club + 0x38, 0x7d5, club); // cqx7: the club link',
     'add_link(text, club + 0x38, 0x7d0, club); // mutation: the competition link type',
     'cqx_7 coppa italia through to first round'),
]
