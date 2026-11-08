# Mutacoes das lesoes (o comparando acusa a recorrente pelo plano; a carreira pela contagem dos sorteios). A recorrente
# em 1 de 10 em vez de 1 de 20: mais registros marcados, outra escolha e outro tratamento (0x62d250), sorteios a mais
# em cada lesao seguinte. A escolha sem parar no primeiro forte: um sorteio a mais por registro marcado depois dele. O
# regen comecando com desgaste -2000 em vez de 0: o desgaste do dia (0x62c2f0) desse jogador deixa de sortear (abaixo
# de -1000 so desce 1); os regens da carreira (0x7abac0) passam por ela.
MUTACOES = [
    ('cpm5_recorrente_um_em_dez', 'if s != 0 { // 1 em 20',
     'if s > 1 { // mutacao: 1 em 10', 'cpm_5 recurring injury'),
    ('cpm5_recorrente_sem_parar', 'if melhor >= 0x5f { // forte o bastante: para',
     'if melhor >= 0x5f && false { // mutacao: nao para', 'cpm_5 strong recurring'),
    ('cpm5_desgaste_do_regen', 'core::ptr::write_unaligned((r + 0xc) as *mut i16, 0); // sem desgaste',
     'core::ptr::write_unaligned((r + 0xc) as *mut i16, -2000); // mutacao: desgaste -2000', 'cpm_5 reset player'),
]
