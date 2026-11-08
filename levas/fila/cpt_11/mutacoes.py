# Mutacoes. A pre-temporada da IA marca os torneios, as excursoes e os amistosos de todos os clubes (datas, publico,
# desgaste, financas): o dia da pre-temporada sem planejar (2 vezes ate 2002-06-01: os amistosos da IA somem), a band
# dos candidatos pela injury_word_0c / 250 (2: as ordens e a busca dos adversarios pela band mudam) e a janela comecando 11
# dias antes do fim em vez de 18 (as datas sorteadas na janela mudam). Os ramos ficam depois de todas as condicoes,
# colados no efeito.
MUTACOES = [
    ('cpt11_sem_plano', 'planejar(this); // cpt11: planejar',
     '// mutacao: nao planeja',
     'cpt_11 planned the pre-season'),
    ('cpt11_faixa_250', 'let band = (rep / 500) as u8; // cpt11: a band',
     'let band = (rep / 250) as u8; // mutacao: band pela metade',
     'cpt_11 pre-season candidate'),
    ('cpt11_janela_curta', 'let mut ini = fim.wrapping_sub(18); // cpt11: o início da janela',
     'let mut ini = fim.wrapping_sub(11); // mutacao: janela curta',
     'cpt_11 window recorded'),
]
