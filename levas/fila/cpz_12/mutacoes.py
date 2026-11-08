# Mutacoes (cpz_12). Toda transferencia com valor paga a compra (0x599890) e recebe a venda (0x599800); o jogador que
# muda de clube ou aceita a oferta de contrato recebe as luvas (0x5998e0). Sem a venda no saldo, sem a compra tirada
# do saldo ou sem as luvas tiradas, o saldo dos clubes envolvidos muda: a verba de transferencias (0x598800), a band
# das financas (0x5986b0) e os eventos da semana (divida, injecao) mudam, e com eles as ofertas seguintes. Escapam so
# se nenhuma transferencia com valor (ou com luvas) acontecer ate 2002-06-01 (os contadores mostram).
MUTACOES = [
    ('cpz12_venda_fora_do_saldo', '    fin_prm_add_i64(this, v); // cpz12: a venda entra no saldo',
     '    let _ = v; // mutacao: a venda nao entra no saldo',
     'cpz_12 sale received'),
    ('cpz12_compra_fora_do_saldo', '    fin_prm_add_i64(this, v.wrapping_neg()); // cpz12: a compra sai do saldo',
     '    let _ = v; // mutacao: a compra nao sai do saldo',
     'cpz_12 purchase paid'),
    ('cpz12_luvas_no_saldo', '    fin_prm_add_i64(this, (valor as i64).wrapping_neg()); // cpz12: as luvas saem do saldo',
     '    fin_prm_add_i64(this, valor as i64); // mutacao: as luvas entram no saldo',
     'cpz_12 signing-on fees paid'),
]
