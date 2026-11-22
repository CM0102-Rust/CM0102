# Mutations (tel24_3). The automatic career opens no screen: the chain runs none; these prove the comparers in a
# session of Lucas's (JOGAR_COMPARANDO.bat), side by side over the human's terms:
# - the release fee popup step halved (Clauses tab, pick a fee in the popup);
# - the bonus reset to a fifth of the wage instead of a tenth when the person asked none (Bonuses tab, last button).
MUTACOES = [
    ('tel24_3_release_fee_step', 'write_u32(fee, tel26_item_value(item16));',
     'write_u32(fee, tel26_item_value(item16) / 2);'),
    ('tel24_3_bonus_reset', 'let v = if wanted < 0 { read_i32(edited + contract::WAGE) / 10 } else { wanted };',
     'let v = if wanted < 0 { read_i32(edited + contract::WAGE) / 5 } else { wanted };'),
]
