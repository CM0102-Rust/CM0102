# Mutacoes (cqf_1), na oferta posta no vetor (0x8be300; 1743 chamadas na carreira automatica ate 2002-06-01,
# docs/cobertura_simulacao.txt, 17/10, pelos lances da IA 0x84a020 e pela oferta de emprestimo 0x8bed60):
# - sem ligar as ofertas rivais pelo mesmo jogador (a lista +0 da oferta): a resposta da IA (0x8c8090) e a conclusao
#   deixam de ver as outras ofertas pelo jogador (contador "cqf_1 rival offers linked");
# - o prazo da resposta 3 dias maior: as ofertas sao decididas mais tarde (contador "cqf_1 schedule by deadline");
# - sem a vaga da oferta do mesmo comprador pelo mesmo alvo: o novo lance fica numa vaga nova, ao lado do antigo
#   (contador "cqf_1 same slot").
# Os contadores ficam colados no efeito, depois de todas as condicoes do trecho mutado.
MUTACOES = [
    ('cqf1_sem_rivais',
     'if rival != -1 { // cqf1: a rival liga as ofertas',
     'if rival != -1 && false { // mutacao: sem ligar as rivais',
     'cqf_1 rival offers linked'),
    ('cqf1_prazo_mais_tres',
     'agendar(this, cqf1_d(slot(vaga)), dias as u32, fatias as u32); // cqf1: o prazo da oferta',
     'agendar(this, cqf1_d(slot(vaga)), dias as u32 + 3, fatias as u32); // mutacao: 3 dias a mais',
     'cqf_1 schedule by deadline'),
    ('cqf1_sem_mesma_vaga',
     'vaga = i; // cqf1: a vaga da oferta do mesmo comprador',
     '// mutacao: sem a vaga do mesmo comprador',
     'cqf_1 same slot'),
]
