# Mutacoes (cqf_2), nos termos aceitos (0x8c2180; 1123 chamadas na carreira automatica ate 2002-06-01,
# docs/cobertura_simulacao.txt, 17/10) e na noticia do jogador listado (0x4d71b0; 340 chamadas):
# - fora do emprestimo, a oferta nao segue depois do acordo do jogador (0x8c2f50): as transferencias aceitas nao andam
#   (contador "cqf_2 seguir a oferta");
# - o emprestimo concluido na hora fica para 9 dias (antes, 5): o jogador chega mais tarde ao clube (contador "cqf_2
#   empréstimo na hora");
# - a noticia do jogador listado sem o sorteio do dado 12 (0x90cfc0 com 2): um sorteio a menos a cada noticia muda a
#   sequencia do gerador (contador "cqf_2 notícia do listado").
MUTACOES = [
    ('cqf2_sem_seguir',
     'seguir(this, oferta); // cqf2: a oferta segue',
     '// mutacao: a oferta nao segue',
     'cqf_2 seguir a oferta'),
    ('cqf2_emprestimo_em_9_dias',
     'agendar(this, oferta, 5, 0); // cqf2: o empréstimo concluído em 5 dias',
     'agendar(this, oferta, 9, 0); // mutacao: 9 dias',
     'cqf_2 empréstimo na hora'),
    ('cqf2_sem_sorteio_do_dado_12',
     'let s = sorteio(2); // cqf2: o sorteio do dado 12',
     'let s = 0; // mutacao: sem o sorteio',
     'cqf_2 notícia do listado'),
]
