# Mutacoes (sbA_2): o saldo inicial e o teto salarial pesam no mercado da carreira ENG desde o comeco; a obra do
# estadio muda o saldo de quem sobe para uma liga com estadio minimo (e a capacidade, que pesa na bilheteria). Os
# comparadores lado a lado tambem as acusam na comparacao.
MUTACOES = [
    ('sbA_2_saldo_sem_padrao',
     '            b = (standard.wrapping_mul(standard) as f64 * read_f64(rdata::F64_0_05263157895)) * (b * read_f64(rdata::F64_0_05)) / n;',
     '            b = (standard.wrapping_mul(standard) as f64 * read_f64(rdata::F64_0_05263157895)) * (b * read_f64(rdata::F64_0_05)) / n * 3.0; // mutation'),
    ('sbA_2_teto_plc',
     '    let (low, high) = if club.plc() != 0 { (1250, 10000) } else { (750, 6000) };',
     '    let (low, high) = if club.plc() != 0 { (2250, 10000) } else { (1750, 6000) }; // mutation'),
    ('sbA_2_obra_de_graca',
     '    add_i32_to_balance(fin.addr(), cost.wrapping_neg());',
     '    add_i32_to_balance(fin.addr(), 0); // mutation'),
]
