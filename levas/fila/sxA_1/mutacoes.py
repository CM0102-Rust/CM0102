# Mutações (sxA_1). A resposta automática roda na carreira de férias (463 entregas até 2002-06-01): os amistosos pedidos
# ao humano aceitos, os pedidos de empréstimo dos jogadores dele e o pedido de lista de transferência.
MUTACOES = [
    ('sxA_1_friendly_refused',
     "write_u8(m + friendly_match::STATUS, 1); // sxA_1: the friendly accepted",
     "write_u8(m + friendly_match::STATUS, 4); // mutation: the friendly refused",
     'sxA_1 automatic reply'),
    ('sxA_1_loan_rejected',
     "accept(tm, slot, 0); // sxA_1: the loan accepted",
     "exe::transfer_manager_reject_loan_offer(tm, slot, 0, 0); let _ = accept; // mutation: the loan rejected",
     'sxA_1 loan accepted'),
    ('sxA_1_request_refused',
     "if value as f64 * read_f64(rdata::F64_1_5) > asking as f64 {",
     "if value as f64 * read_f64(rdata::F64_1_5) > asking as f64 && false { // mutation: every request refused",
     'sxA_1 transfer request accepted'),
]
