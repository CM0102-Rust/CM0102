# Mutacoes (mkd_3). O contrato desejado e o pedido de toda negociacao (renovacoes, lances, a folha): o salario (+0xc)
# decide quem o clube consegue pagar e quem aceita, e a data de fim decide quando o contrato vence:
# - o valor pedido 25% maior (todos os jogadores pedem mais: outras recusas e outros contratos);
# - sem os 5000 do inquieto e do agente livre de vontade baixa (pedem menos: outros aceites);
# - um ano a menos em todo contrato (os contratos vencem antes: outras renovacoes e saidas).
MUTACOES = [
    ('mkd3_valor_maior',
     'v = (q as f64 * w as f64) as f32; // mkd3: the value asked',
     'v = (q as f64 * w as f64 * 1.25) as f32; // mutation: the value asked 25% higher',
     'mkd_3 player contract'),
    ('mkd3_sem_5000',
     'w = (w as f64 + f(rdata::F32_5000)) as f32; // mkd3: the restless ask 5000 more',
     'w = w; // mutation: the restless ask nothing more',
     'mkd_3 player contract'),
    ('mkd3_ano_a_menos',
     'year = year.wrapping_add(ftol(l + d(rdata::F64_1)) as u16); // mkd3: the years of the contract',
     'year = year.wrapping_add(ftol(l) as u16); // mutation: one year less',
     'mkd_3 player contract'),
]
