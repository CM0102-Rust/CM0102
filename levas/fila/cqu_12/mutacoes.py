# Mutacoes (cqu_12), na eliminatoria da Oceania que o jogo de 2001 monta ja no comeco: so no dia do comeco (nao em
# 2001: a eliminatoria nao se monta); 4 selecoes no primeiro grupo (a quinta fica sem jogos); o segundo grupo de tras
# para a frente (o mando e a ordem dos jogos mudam).
MUTACOES = [
    ('cqu12_sem_a_montagem_de_2001',
     '|| ano_de_hoje == 0x7d1; // cqu12: o jogo de 2001 monta já',
     '|| ano_de_hoje == 0x7d2; // mutacao: so no dia do comeco',
     'cqu_12 montagem da eliminatoria'),
    ('cqu12_quatro_no_primeiro_grupo',
     'core::ptr::write_unaligned((this + 0x3e) as *mut i16, 5); // cqu12: 5 seleções no primeiro grupo',
     'core::ptr::write_unaligned((this + 0x3e) as *mut i16, 4); // mutacao: 4 selecoes',
     'cqu_12 linha do primeiro grupo'),
    ('cqu12_segundo_grupo_de_tras_para_frente',
     'cqu12_pd(times + i * 4, cqu12_d(cqu12_d(this + 0xef) + i * 8 + 4)); // cqu12: a seleção na vaga i do segundo grupo',
     'cqu12_pd(times + i * 4, cqu12_d(cqu12_d(this + 0xef) + (n - 1 - i) * 8 + 4)); // mutacao: de tras para a frente',
     'cqu_12 time do segundo grupo'),
]
