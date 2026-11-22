# Mutations (tel24_4). The automatic career opens no screen: the chain runs none; these prove the comparers in a
# session of Lucas's (JOGAR_COMPARANDO.bat):
# - the wage popup steps a ninth instead of an eighth of the limit (Basic tab, the wage popup; the limit / 8 must be
#   above 100), caught side by side over the display list;
# - the fee "=" goes back to half the requested fee (Basic tab, "=" of the Signing On Fee row), caught side by side
#   over the human's terms.
MUTACOES = [
    ('tel24_4_wage_popup_step', 'wage_steps[k - 1] = ftol(k as f64 * limit * read_f64(rdata::F64_0_125));',
     'wage_steps[k - 1] = ftol(k as f64 * limit * read_f64(rdata::F64_0_125) * 8.0 / 9.0);'),
    ('tel24_4_fee_reset', 'let wanted = read_i32(edited + CS_TERMS_FEE);',
     'let wanted = read_i32(edited + CS_TERMS_FEE) / 2;'),
]
