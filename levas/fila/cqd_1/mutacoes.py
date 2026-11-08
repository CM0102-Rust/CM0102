# Mutacoes (cqd_1), uma por rotina, no dia das transferencias (0x8c5880, chamado pelo laco do dia 0x5cd3ee em cada
# fatia; 966 chamadas na carreira automatica ate 2002-06-01, docs/cobertura_simulacao.txt, 17/10), na oferta que vence
# hoje (0x8c63a0; 4053) e nas restricoes da oferta (0x8cfc90; 2897):
# - a oferta do balde processada noutra fatia: as ofertas agendadas para esta fatia nunca sao processadas (o balde e
#   esvaziado depois), as negociacoes param e os elencos, os saldos e as noticias mudam;
# - o lance de transferencia (status 0, vendedor da IA, fora do emprestimo) fica sem a resposta da IA (0x8c8090): os
#   lances nao sao aceitos, recusados nem contrapropostos e as transferencias entre clubes da IA param;
# - o orcamento pela metade: valor acima de metade do orcamento do comprador ja barra a oferta (antes, 1,1 vez), e as
#   ofertas da IA que cabiam no orcamento caem.
# Os contadores ficam colados no efeito: o da oferta depois do indice valido, da data e da fatia; o da resposta depois
# do vendedor da IA e da marca de emprestimo; o do orcamento depois de decidir o comprador, logo antes da comparacao.
MUTACOES = [
    ('cqd1_oferta_noutra_fatia',
     '&& cqd1_b(o + 0x1a) == f { // cqd1: a oferta desta fatia',
     '&& cqd1_b(o + 0x1a) != f { // mutacao: a oferta noutra fatia',
     'cqd_1 oferta do dia'),
    ('cqd1_lance_sem_resposta',
     'respond(this, vaga, 0); // cqd1: a resposta da IA ao lance',
     'let _ = respond; // mutacao: o lance sem a resposta da IA',
     'cqd_1 resposta da IA'),
    ('cqd1_orcamento_pela_metade',
     'if t as f64 > v as f64 * 1.1 { // cqd1: acima do orçamento',
     'if t as f64 > v as f64 * 0.5 { // mutacao: o orcamento pela metade',
     'cqd_1 orçamento'),
]
