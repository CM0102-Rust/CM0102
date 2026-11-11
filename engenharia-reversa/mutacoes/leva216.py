# Mutações da 216a leva (itens sim_lote_c1, sim_lote_c2, sim_lote_c3). Uso: python mutacoes.py mutacoes\leva216.py --carreira 2002-06-01
MUTACOES = [
    ('lc_rebaixamento', '    (league.relegated_count() != 0) as u32 // tem rebaixamento', '    (league.relegated_count() == 0) as u32 // tem rebaixamento'),
    ('lc_ano_relativo', 'd[1].wrapping_sub(base as u16)); // ano relativo', 'd[1].wrapping_sub(base as u16).wrapping_add(1)); // ano relativo'),
    ('lc_importancia', '    imp = imp.wrapping_add(10); // base importance', '    imp = imp.wrapping_add(11); // base importance'),
    ('cr_recordista', '    d(0); // identification', '    write_u32(this, read_u32(source) ^ 1); // identification'),
    ('cr_partida', '    d(8); // opponent', '    write_u32(this + 8, read_u32(source + 8) ^ 1); // opponent'),
    ('cr_sequencia', '    write_u32(this, read_u32(source)); // jogos', '    write_u32(this, read_u32(source) ^ 1); // jogos'),
    ('ha_ninguem_assiste', '    0 // nobody watches', '    1 // nobody watches'),
    ('ha_apelido', '    found // apelido', '    found ^ 1 // apelido'),
    ('ha_motivo', 'this.set_reason(source.reason()); // motivo', 'this.set_reason(source.reason() ^ 1); // motivo'),
]
