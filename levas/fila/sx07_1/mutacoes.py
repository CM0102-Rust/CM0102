# Mutacoes (sx07_1). A noticia nao volta para a simulacao; o sorteio sim (o gerador anda uma vez por chamada).
# - sem o sorteio da vaga 19: a carreira acusa (a sequencia do gerador muda) e o comparador tambem;
# - o clube nao recebe a noticia: o comparador acusa quando o clube e do tecnico humano;
# - o codigo do fim do contrato com a data de +365 no lugar da de +91: muda a vaga 17 e quem recebe (comparador).
MUTACOES = [
    ('sx07_sem_sorteio',
     'set(0x13, exe::utils_random(3) as u32); // sx07_1: the headline\'s draw',
     'set(0x13, 0); // mutation: no draw',
     'sx07_1 offer response to the club'),
    ('sx07_clube_sem_noticia',
     'offer_response_send(n, club_at(o.club_id() as i32 as isize)); // sx07_1: the club gets it',
     '// mutation: the club does not get it',
     'sx07_1 offer response to the club'),
    ('sx07_codigo_365',
     '    if !contract_manager_date_is_before(G_CONTRACT_DATE_PLUS_91_DAYS, end) {',
     '    if !contract_manager_date_is_before(G_CONTRACT_DATE_PLUS_365_DAYS, end) { // mutation',
     'sx07_1 offer response to the transfer recipients'),
]
