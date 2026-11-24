# Mutacoes (sx14b_1), pela carreira ENG ate 2002-06-01 (os comparadores so rodam a original). Na carreira so roda o
# pedido 1 dos clubes da maquina (7 vezes): a contagem sobe e, no limite + 1, a punicao. As respostas (0x6aaa90) e a
# remarcacao (0x6aa7d0) vao a clube da maquina sem entrega (ou nem rodam): sem mutacao propria (nada que a carreira
# leia); a reclamacao do arbitro e o adiamento so rodam para clube humano.
# - a contagem de pedidos nao sobe: o registro da diretoria do clube fica diferente;
# - limite 0: a primeira reclamacao ja e punida (historia na midia, multa de 25000, confianca, exame da demissao).
MUTACOES = [
    ('sx14b_1_contagem_parada',
     '    write_i8(row.addr(), fa_request_count(row).wrapping_add(1));',
     '    let _ = row; // mutation: the request count stays',
     'sx14b_1 request at the limit'),
    ('sx14b_1_limite_zero',
     '    let limit: i8 = if club.reputation() > 0xcb2 { 3 } else { 2 };',
     '    let limit: i8 = if club.reputation() > 0xcb2 { 0 } else { 0 }; // mutation',
     'sx14b_1 FA sanction'),
]
