# Mutations (tel36_10). The automatic career opens no screen; these prove the comparers in a session of Lucas's (the
# screen: Search > Player Search > the attribute filters, "+" / "-" on "attributes to match" and ticking filters).
# - the outfield counter may reach one above the maximum;
# - the index 0x40 gives the single bit 31.
MUTACOES = [
    ('tel36_10_increase_limit', 'if n >= read_u8(filters + max) as i8 {', 'if n > read_u8(filters + max) as i8 {'),
    ('tel36_10_index_0x40', '0x40 => SEARCH_FILTER_INDEX_0X40,', '0x40 => 1u64 << 31,'),
]
