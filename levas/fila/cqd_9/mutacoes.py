# Mutacoes (cqd_9), todas na 0x8b3640: as normalizacoes dos atributos do jogador (0x538bc0/0x538ec0, no envelhecimento
# do diario das pessoas 0x5ce2d0 e no regen jovem 0x7abac0) tiram o progresso do treino antes e o devolvem depois. Com
# a volta errada, o meio byte guardado na entrada do treino (e, pelo passo da 0x8b2be0, os atributos) fica diferente:
# muda o progresso das semanas seguintes (0x8b1750) e as partidas.
# - A volta da categoria 0 soma 1.
# - O guardado da categoria 1 soma 1 (a volta passa do que foi tirado).
# - A volta da categoria 4 tira 1.
MUTACOES = [
    ('cqd9_volta_cat0_mais_1', '        d[0] = g(0xde1f3f); // cqd9: devolve a categoria 0',
     '        d[0] = g(0xde1f3f).wrapping_add(1); // mutacao: devolve a categoria 0', 'cqd_9 devolver'),
    ('cqd9_guardado_cat1_mais_1', '        *(0xde1f40 as *mut i8) = cqd9_meio(e, 1); // cqd9: guarda a categoria 1',
     '        *(0xde1f40 as *mut i8) = cqd9_meio(e, 1).wrapping_add(1); // mutacao: guarda a categoria 1', 'cqd_9 tirar'),
    ('cqd9_volta_cat4_menos_1', '        d[4] = g(0xde1f3e); // cqd9: devolve a categoria 4',
     '        d[4] = g(0xde1f3e).wrapping_sub(1); // mutacao: devolve a categoria 4', 'cqd_9 devolver'),
]
