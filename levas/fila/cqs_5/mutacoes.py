# Mutacoes (cqs_5), no Trofeu da Liga montado no comeco (2001-000; jogado de outubro de 2001 a abril de 2002): o
# rebaixado para a Conference nao entra (outro clube completa os 56); as chaves sem a ordem pela latitude (Norte e Sul
# trocam clubes); a fase das chaves com a lista em ordem inversa (outros confrontos).
MUTACOES = [
    ('cqs5_rebaixado_nao_entra', 'cqs5_por(this, &mut k, c); // cqs5: o rebaixado entra', '// mutacao: o rebaixado nao entra', 'cqs_5 rebaixado para a conference'),
    ('cqs5_chaves_sem_latitude', 'ordenar(cqs5_d(this + 0xb2), cqs5_d(this + 0xb6) as u32, 4, 0x53fd40); // cqs5: Norte e Sul pela latitude', '// mutacao: sem a latitude', 'cqs_5 chaves do trofeu da liga'),
    ('cqs5_chaves_em_ordem_inversa', 'cqs5_pd(times + i as usize * 4, cqs5_d(cqs5_d(this + 0xb2) + i as usize * 4)); // cqs5: a ordem das chaves', 'cqs5_pd(times + i as usize * 4, cqs5_d(cqs5_d(this + 0xb2) + (cqs5_w(this + 0x3e) as i32 - 1 - i) as usize * 4)); // mutacao', 'cqs_5 fase das chaves do trofeu'),
]
