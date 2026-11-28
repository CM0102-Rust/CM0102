# Mutacoes (r7_08_2). TELAS: a cadeia nao roda; o comparador (lado a lado sobre a contagem e as linhas) as acusa quando
# a tela do clube ou o orcamento de transferencias de um clube humano com propostas e usado jogando comparando.
# Acusadas tambem na emulacao (scratchpad\r7_08\check8.py list).
# - a etapa "esperando o visto" (3) vira 4;
# - uma proposta desfeita continua aberta so ate a rodada 2;
# - o jogador da troca sai com o mesmo sentido do alvo.
MUTACOES = [
    ('r7_08_2_etapa',
     '        0xa => 3, // r7_08_2: awaiting the work permit',
     '        0xa => 4, // r7_08_2 mutation',
     'r7_08_2 exchange player row'),
    ('r7_08_2_desfeita',
     '                0xf => o.response_round() < 3, // r7_08_2: a collapsed offer still open',
     '                0xf => o.response_round() < 2, // r7_08_2 mutation',
     'r7_08_2 exchange player row'),
    ('r7_08_2_troca',
     '                    write_u32(row + budget_list_rows::FUTURE_OUTGOING, buying as u32); // r7_08_2: the exchange player moves the other way',
     '                    write_u32(row + budget_list_rows::FUTURE_OUTGOING, !buying as u32); // r7_08_2 mutation',
     'r7_08_2 exchange player row'),
]
