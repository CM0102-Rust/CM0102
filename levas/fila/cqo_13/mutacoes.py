# Mutacoes (cqo_13), nas copas sul-americanas de clubes (clubes brasileiros jogaveis na carreira de 8 ligas; a Mercosul
# de 2001 vai de julho a dezembro e a Libertadores de 2002 de fevereiro a junho, dentro da janela): a 1a rodada dos
# grupos da Libertadores em 18/2 em vez de 25/2; a 1a rodada dos grupos da Mercosul em 22/7 em vez de 29/7 (uma semana antes: as datas sao levadas ao dia da semana); os
# convidados da Mercosul embaralhados como 19 (o ultimo fica no lugar e a contagem dos sorteios muda). Os contadores
# ficam colados no efeito.
MUTACOES = [
    ('cqo13_libertadores_semana_antes',
     'const LIBERTADORES_GROUP_ROUNDS: [ConmebolMercGroupRound; 6] = [(0x19, 1, 2, 2), (4, 2, 2, 2), (0xb, 2, 2, 2), (0x12, 2, 2, 2), (0x19, 2, 2, 2), (1, 3, 2, 2)]; // cqo13: os grupos da Libertadores',
     'const LIBERTADORES_GROUP_ROUNDS: [ConmebolMercGroupRound; 6] = [(0x12, 1, 2, 2), (4, 2, 2, 2), (0xb, 2, 2, 2), (0x12, 2, 2, 2), (0x19, 2, 2, 2), (1, 3, 2, 2)]; // mutacao: 18/2',
     'cqo_13 grupos da libertadores'),
    ('cqo13_mercosul_semana_antes',
     'const MERCOSUR_GROUP_ROUNDS: [ConmebolMercGroupRound; 6] = [(0x1d, 6, 2, 2), (0x13, 7, 2, 2), (2, 8, 2, 2), (0x10, 8, 2, 2), (0x1e, 8, 2, 2), (0xe, 9, 2, 2)]; // cqo13: os grupos da Mercosul',
     'const MERCOSUR_GROUP_ROUNDS: [ConmebolMercGroupRound; 6] = [(0x16, 6, 2, 2), (0x13, 7, 2, 2), (2, 8, 2, 2), (0x10, 8, 2, 2), (0x1e, 8, 2, 2), (0xe, 9, 2, 2)]; // mutacao: 22/7',
     'cqo_13 grupos da mercosul'),
    ('cqo13_convidados_embaralhados_como_19',
     'embaralhar(cqo13_d(this + 0xee), core::ptr::read_unaligned((this + 0xf2) as *const u16) as u32); // cqo13: embaralha os convidados',
     'embaralhar(cqo13_d(this + 0xee), 19); // mutacao: 19',
     'cqo_13 convidados da mercosul'),
]
