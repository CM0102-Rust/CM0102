# Mutations (tel36_11). The automatic career opens no screen; these prove the comparers in a session of Lucas's (the
# screen: Tactics > a player's position / the formation options).
# - the index goes up to 48;
# - the mask of index 0 is bit 1.
MUTACOES = [
    ('tel36_11_index_limit', 'mask.trailing_zeros() < 0x30 {', 'mask.trailing_zeros() < 0x31 {'),
    ('tel36_11_mask_shift', 'b @ 0..=0x2f => 1u64 << b,', 'b @ 0..=0x2f => 2u64 << b,'),
]
