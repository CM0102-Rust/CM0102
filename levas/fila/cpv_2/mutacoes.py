# Mutacoes (com o ramo de cada uma). Todas na recusa dos termos (0x8c28c0, 544 chamadas ate 2002-06-01 na cobertura
# de 17/10); as noticias so vao as caixas dos humanos. A oferta recusada de volta em 5 dias (e nao 10) muda o dia em
# que ela e retomada (outras respostas, outros dias). Sem a soma na reputacao, o jogador que recusa um clube mais
# famoso fica com a reputacao de antes (+0xb da ficha, no banco do dia). Com o tipo da anotacao trocado, o gerente de
# listas grava a recusa como a do outro tipo de oferta (as listas dos clubes do computador mudam as proximas ofertas).
MUTACOES = [
    ('cpv2_volta_em_5', 'agendar(this, o, 0xa, 0); // cpv2: de volta em 10 dias',
     'agendar(this, o, 5, 0); // mutacao: 5 dias',
     'cpv_2 recusa do jogador'),
    ('cpv2_sem_reputacao', 'let soma = ftol(dif * k) as i16; // cpv2: a reputação sobe',
     'let soma = (ftol(dif * k) as i16) & 0; // mutacao: nao sobe',
     'cpv_2 reputacao do jogador'),
    ('cpv2_tipo_trocado', 'anotar(cpv2_d(this + 0x856), pessoa, comprador, 1, (cpv2_b(o + 0x11) != 0xb) as u32); // cpv2: a recusa na lista',
     'anotar(cpv2_d(this + 0x856), pessoa, comprador, 1, (cpv2_b(o + 0x11) == 0xb) as u32); // mutacao: o outro tipo',
     'cpv_2 recusa anotada na lista'),
]
