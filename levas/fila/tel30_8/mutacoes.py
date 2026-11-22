# Mutations (tel30_8). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat): the club screen > Squad (the opener's best players) and the club screen's
# squad statistics view (Squad > View > Statistics, the routine passed to 0x460590).
MUTACOES = [
    ('tel30_8_star_score', 'let s = reputation.wrapping_add(read_i16(sheet + player_sheet::CURRENT_ABILITY).wrapping_mul(10));',
     'let s = reputation.wrapping_add(read_i16(sheet + player_sheet::CURRENT_ABILITY));'),
    ('tel30_8_oldest_order', 'if read_usize(out + s::OLDEST) == 0 || oyear > year || (oyear == year && oday > day) {',
     'if read_usize(out + s::OLDEST) == 0 || oyear < year || (oyear == year && oday > day) {'),
]
