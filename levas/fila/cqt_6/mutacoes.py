# Mutacoes (cqt_6), na Copa dos Campeoes de 2001 (a preliminar em 2001-167, os grupos em 2001-183, as semifinais em
# 2001-200; o campeao vai a Libertadores de 2002): os 3 da preliminar ficam na lista (entram de novo nos grupos); as
# linhas da preliminar em ordem inversa (outros mandos e datas); a tabela da preliminar sem a ordem inicial.
MUTACOES = [
    ('cqt6_time_fica_na_lista', 'core::ptr::write_unaligned((cqt6_d(this + 0x14) + e) as *mut u32, 0); // cqt6: o time sai da lista', '// mutacao: o time fica na lista', 'cqt_6 tabela da preliminar'),
    ('cqt6_linhas_em_ordem_inversa', 'iniciar(this, i as u8 as u32, cqt6_d(cqt6_d(this + 0x14) + e), 0, 0);', 'iniciar(this, (2 - i) as u8 as u32, cqt6_d(cqt6_d(this + 0x14) + e), 0, 0); // mutacao', 'cqt_6 tabela da preliminar'),
    ('cqt6_preliminar_sem_ordem', 'if ordenar(this, 0) == 0 { // cqt6: a ordem da tabela da preliminar', 'if { let _ = ordenar; 1 } == 0 { // mutacao', 'cqt_6 preliminar dos campeoes'),
]
