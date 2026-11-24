# Mutacoes (sx05b_22). O passo decide os valores das ofertas de contrato e de transferencia da IA (a carreira acusa)
# e o comparador lado a lado acusa o valor.
# - o passo acima pelo passo do proprio valor (sem o + 1);
# - o passo abaixo pelo dobro do passo.
MUTACOES = [
    ('sx05b_22_sem_mais_um',
     '    let v = cash_from_display(amount.wrapping_add(cash_get_rounding_step(amount.wrapping_add(1), flags)));',
     '    let v = cash_from_display(amount.wrapping_add(cash_get_rounding_step(amount, flags))); // mutation',
     'sx05b_22 cash stepped up'),
    ('sx05b_22_passo_dobrado',
     '    let v = cash_from_display(amount.wrapping_sub(cash_get_rounding_step(amount, flags)));',
     '    let v = cash_from_display(amount.wrapping_sub(2 * cash_get_rounding_step(amount, flags))); // mutation',
     'sx05b_22 cash stepped down'),
]
