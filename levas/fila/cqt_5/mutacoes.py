# Mutacoes (cqt_5): a Copa do Rei so refeita no ano seguinte (a de 2001, refeita em 2001-000, fica com as tabelas
# antigas); a Copa do Rei sem a marca +0x45 (refeita todo dia enquanto o ano vale: nao anda); o historico da Copa da
# Serie C pela fase 14 (um grupo) em vez do mata-mata.
MUTACOES = [
    ('cqt5_copa_do_rei_no_ano_seguinte', 'cqt5_w(this + 0x40) as i32 && fase != 0 { // cqt5: o ano da Copa do Rei', '(cqt5_w(this + 0x40) as i32 + 1) && fase != 0 { // mutacao', 'cqt_5 copa del rey redone'),
    ('cqt5_copa_do_rei_sem_marca', 'core::ptr::write_unaligned((this + 0x45) as *mut u32, 1); // cqt5: a Copa do Rei refeita uma vez', '// mutacao: refeita todo dia', 'cqt_5 copa del rey redone'),
    ('cqt5_historico_da_serie_c_da_fase_14', 'record_season_honours(this, 0x3c, "cqt_5 historico da copa da serie c")', 'record_season_honours(this, 0x38, "cqt_5 historico da copa da serie c") // mutacao', 'cqt_5 historico da copa da serie c'),
]
