# Mutacoes (cqc_3). Os conflitos de calendario rodam a cada jogo marcado ou remarcado (8.129 buscas e 24.387 dias ate
# 2002-06-01) e decidem a data dos jogos (remarcacao, nova data, adiamento), que a carreira joga: o jogo do mesmo
# clube na mesma fatia que nao entra na lista (o conflito some), o mesmo clube noutra fatia com gravidade 1 no lugar
# de 0x14 (outro caminho no calendario) e a busca sem o dia seguinte. Contadores colados no efeito, depois das
# condicoes do trecho.
MUTACOES = [
    ('cqc3_mesma_fatia_nao_entra',
     'add_clash_by_type(grav, lista, x, true); // cqc3: o mesmo clube na mesma fatia',
     '{} // mutacao: o jogo nao entra',
     'cqc_3 same club in the same pot'),
    ('cqc3_outra_fatia_um',
     'default_clash_severity(grav, true); // cqc3: o mesmo clube noutra fatia',
     'default_clash_severity(grav, false); // mutacao: 1 no lugar de 0x14',
     'cqc_3 same club in another pot'),
    ('cqc3_sem_dia_seguinte',
     'do_dia(this, jogo, &mut g, lista, pa, n as u32, pd, 1); // cqc3: o dia seguinte',
     '// mutacao: sem o dia seguinte',
     'cqc_3 next day'),
]
