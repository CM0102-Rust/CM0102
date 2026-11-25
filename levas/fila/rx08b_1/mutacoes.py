# Mutacoes para a sessao do Lucas (TELAS: a cadeia nao roda). Os comparadores da fase e dos vereditos acusam as duas
# primeiras; a terceira (a adaptacao na noticia) so se ve no texto do relatorio jogando em modo Rust.
MUTACOES = [
    ('rx08b_1_dias_base_trocada',
     'let base = if coach_ability > 0x8c { 5 } else if coach_ability <= 0x50 { 0xf } else { 10 }; // rx08b_1: the base by the coach\'s ability',
     'let base = if coach_ability > 0x8c { 5 } else if coach_ability <= 0x50 { 10 } else { 0xf }; // rx08b_1 mutation',
     'rx08b_1 assessment days'),
    ('rx08b_1_veredito_sem_quase',
     'let current = coach_band(now, t, 0x200).unwrap_or(if now as f64 > t * read_f64(rdata::F64_0_8) { 2 } else { 1 }); // rx08b_1: the current verdict',
     'let current = coach_band(now, t, 2).unwrap_or(if now as f64 > t * read_f64(rdata::F64_0_8) { 2 } else { 1 }); // rx08b_1 mutation',
     'rx08b_1 player assessed'),
    ('rx08b_1_adaptado_invertido',
     'let settled: i8 = if days > 0x3e8 && read_i8(player.addr() + staff::LOYALTY) > 10 { 1 } else if days >= settle_days { 4 } else { 2 }; // rx08b_1: how settled',
     'let settled: i8 = if days > 0x3e8 && read_i8(player.addr() + staff::LOYALTY) > 10 { 1 } else if days >= settle_days { 2 } else { 4 }; // rx08b_1 mutation',
     'rx08b_1 coach report posted'),
]
