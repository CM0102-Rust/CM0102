# Mutacoes (com o ramo de cada uma). Duas na negociacao guardada (0x8be9d0, 14 chamadas ate 2002-06-01 na cobertura de
# 17/10) e uma no boato do tecnico (0x69fd50, 7 chamadas). A oferta do computador agendada em 3 dias (e nao 2) volta em
# outro dia (outra resposta, outro dia da transferencia). Com o status 2 (e nao 1) na vaga nova, a oferta segue pelo
# outro caminho da fila. Sem o segundo sorteio(4) do boato, a contagem de sorteios muda (pega mesmo sem humano para
# receber a noticia; pode escapar se nenhum boato pass_ball do primeiro sorteio e da data das noticias).
MUTACOES = [
    ('cpv4_agendada_em_3', 'agendar(this, oferta, 2, 0); // cpv4: a oferta em 2 dias',
     'agendar(this, oferta, 3, 0); // mutacao: 3 dias',
     'cpv_4 negociacao agendada'),
    ('cpv4_status_2', 'cpv4_pb(oferta + 0x2c, 1); // cpv4: a oferta em negociação',
     'cpv4_pb(oferta + 0x2c, 2); // mutacao: status 2',
     'cpv_4 vaga nova'),
    ('cpv4_boato_sem_sorteio', 'let s = sorteio(4) as i16 as i32; // cpv4: a variante do boato',
     'let s = 0i32; // mutacao: sem o sorteio',
     'cpv_4 boato do tecnico'),
]
