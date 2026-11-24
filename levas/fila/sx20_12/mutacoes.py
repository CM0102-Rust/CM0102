# Mutacoes (sx20_12), item de TELAS: a carreira automatica nao passa; servem para provar o comparador numa sessao do
# Lucas jogando comparando:
# - o valor do bonus formatado sem a bandeira de valor inteiro (o perfil de um jogador com bonus no contrato);
# - o vencimento "mes ano" com o mes seguinte (a tela de proposta de contrato);
# - a clausula de cargo de tecnico a partir dos 30 anos (a lista de clausulas na proposta a um jogador de 31);
# - o salario + desce (o botao + da proposta de contrato).
MUTACOES = [
    ('sx20_12_bonus_bandeiras',
     'exe::cash_format_money(money.as_mut_ptr() as usize, 0xc8, exe::cash_get_display_amount(&amount, 0xa), 0xa); // sx20_12: the cash flags',
     'exe::cash_format_money(money.as_mut_ptr() as usize, 0xc8, exe::cash_get_display_amount(&amount, 0x8), 0x8); // mutation: other flags'),
    ('sx20_12_mes_seguinte',
     'langlib_text(out, EXPIRY_MONTHS[month as usize], &[year as usize]); // sx20_12: the month of the end',
     'langlib_text(out, EXPIRY_MONTHS[(month as usize + 1) % 12], &[year as usize]); // mutation: the next month'),
    ('sx20_12_tecnico_30',
     '4 => (read_i8(person.addr() + staff::AGE) > 31) as u32, // sx20_12: the manager job clause age',
     '4 => (read_i8(person.addr() + staff::AGE) > 29) as u32, // mutation: from 30'),
    ('sx20_12_mais_desce',
     'contract_wage_step(this, FN_CASH_STEP_UP); // sx20_12: one step up',
     'contract_wage_step(this, FN_CASH_STEP_DOWN); // mutation: down'),
]
