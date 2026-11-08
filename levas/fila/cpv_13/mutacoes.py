# Mutacoes. A 0x86c8c0 roda 120 vezes na carreira automatica ate 2002-06-01 (cobertura de 17/10): os premios do mes
# de jogador e de tecnico das ligas. As duas primeiras tiram o sorteio do agradecimento (0x90cfc0) dos jogadores
# premiados (+0x59 ate 0x7f e +0x5b ate 20): a contagem de sorteios muda no primeiro premio de jogador que sortearia. A
# terceira conta 2 premios do tecnico no registro dele (+0x9e): esse registro nao entra no hash do banco (as mutacoes
# do registro de pontos da cpq_4 escaparam da carreira e so a comparacao as pegou); pode escapar da carreira se nada
# ler o +0x9e ate 2002-06-01, e o comparador lado a lado pelo registro a pega no --confirmar.
MUTACOES = [
    ('cpv13_sem_sorteio_59', 'if v59 <= 0xf { // cpv13: o +0x59',
     'if v59 <= 0x7f { // mutacao: sem sorteio pelo +0x59',
     'cpv_13 thanks draw'),
    ('cpv13_sem_sorteio_5b', 'if cpv13_b(p + 0x5b) as i8 <= 0xa { // cpv13: o +0x5b',
     'if cpv13_b(p + 0x5b) as i8 <= 0x14 { // mutacao: sem sorteio pelo +0x5b',
     'cpv_13 thanks draw'),
    ('cpv13_premio_em_dobro', 'core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(1)); // cpv13: conta o prêmio',
     'core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(2)); // mutacao: conta 2',
     'cpv_13 manager award'),
]
