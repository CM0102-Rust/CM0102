# Mutacoes. A 0x775cd0 roda 1.315 vezes na carreira automatica ate 2002-06-01 (cobertura de 17/10), 140 delas com a
# retirada inteira (a 0x76f7d0 roda 140 vezes, so por ela). As tres mexem no jogador convocado ou no substituto (a
# tabela de pessoas entra no hash diario do banco): o limit de dias subindo para 28 deixa na selecao os lesionados de
# 14 a 27 dias; o tipo trocado pelo sub-21 manda a saida para o time errado (a 0x7762f0 nao acha o jogador nas vagas e
# o +0x24 dele fica); sem a 0x775f90 ninguem e convocado no lugar (os tecnicos das selecoes da carreira automatica sao
# da IA, entao o caminho do pedido ao humano, 0x7701f0, nao roda). Os ramos ficam colados no efeito.
MUTACOES = [
    ('cpv11_limite_28', 'if dias < 0xe { // cpv11: o limit de dias',
     'if dias < 0x1c { // mutacao: limit de 28 dias',
     'cpv_11 injury of 14 to 27 days'),
    ('cpv11_tipo_sub21', '(cpv11_b(p + 0x5e) >> 3) & 1 // cpv11: o tipo pelas marcas',
     '2 // mutacao: o sub-21',
     'cpv_11 type by flags'),
    ('cpv11_sem_substituto', 'substituto(this, nacao, tipo as u32, nivel, p); // cpv11: o substituto',
     'let _ = (substituto, nivel); // mutacao: sem substituto',
     'cpv_11 replacement'),
]
