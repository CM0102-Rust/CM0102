# Mutacoes (cqu_12), na eliminatoria da Oceania que o jogo de 2001 monta ja no comeco: so no dia do comeco (nao em
# 2001: a eliminatoria nao se monta); 4 selecoes no primeiro grupo (a quinta fica sem jogos); o segundo grupo de tras
# para a frente (o mando e a ordem dos jogos mudam).
MUTACOES = [
    ('cqu12_sem_a_montagem_de_2001',
     '|| today_year == 0x7d1; // cqu12: the 2001 game sets it up at once',
     '|| today_year == 0x7d2; // mutation: only on the start day',
     'cqu_12 qualifying set up'),
    ('cqu12_quatro_no_primeiro_grupo',
     'write_i16(this + league::CLUB_COUNT, 5); // cqu12: 5 national teams in the first group',
     'write_i16(this + league::CLUB_COUNT, 4); // mutation: 4 national teams',
     'cqu_12 first group row'),
    ('cqu12_segundo_grupo_de_tras_para_frente',
     'write_usize(teams + i * 4, read_usize(read_usize(this + 0xef) + i * 8 + 4)); // cqu12: the national team in place i of the second group',
     'write_usize(teams + i * 4, read_usize(read_usize(this + 0xef) + (n - 1 - i) * 8 + 4)); // mutation: backwards',
     'cqu_12 second group team'),
]
