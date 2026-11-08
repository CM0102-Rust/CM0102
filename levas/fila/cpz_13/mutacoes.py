# Mutacoes (cpz_13). O premio das copas (0x5999a0, a cada fase das copas inglesas; "cpx_11 premio pago" 64 na base)
# fora do saldo, a TV da liga (0x5987c0, ja na criacao do jogo para todo clube das ligas inglesas) fora do saldo e a
# compensacao da liberacao sem custo (0x599920, "cps_3 compensacao" 8; a cps3_compensacao_em_dobro foi acusada em
# 2001-282) somada ao saldo em vez de tirada: o saldo dos clubes muda, e com ele a verba de transferencias (0x598800),
# a faixa das financas (0x5986b0) e os sorteios dos eventos da semana. A terceira escapa se nenhuma compensacao for
# paga ate 2002-06-01 (o contador mostra).
MUTACOES = [
    ('cpz13_premio_fora_do_saldo', '    add_to_balance(this, valor as i64); // cpz13: o prêmio entra no saldo',
     '    let _ = valor; // mutacao: o premio nao entra no saldo',
     'cpz_13 premio recebido'),
    ('cpz13_tv_fora_do_saldo', '    add_to_balance(this, valor as i64); // cpz13: a TV entra no saldo',
     '    let _ = valor; // mutacao: a TV nao entra no saldo',
     'cpz_13 tv recebida'),
    ('cpz13_salario_no_saldo', '    add_to_balance(this, (valor as i64).wrapping_neg()); // cpz13: o salário sai do saldo',
     '    add_to_balance(this, valor as i64); // mutacao: o salario entra no saldo',
     'cpz_13 salario pago'),
]
