# Mutacoes (cqd_8), todas na 0x8b1750 (a iniciacao do jogador em toda partida a chama; as semanas andam no 7o dia
# pela 0x8b1730, entao todo jogador que entra em campo depois de uma virada de semana sorteia o progresso). O progresso
# vai para os atributos (0x8b2be0), que decidem as partidas, as notas e as transferencias da carreira.
# - A subida vale 2 em vez de 1 (programas de nivel acima de 20: peso acima de 100).
# - O sorteio da subida passa com menos de 30 em vez de 20.
# - O peso do nivel soma 70 em vez de 60 (muda a descida dos programas abaixo de 110 e a subida dos de 100 a 110).
MUTACOES = [
    ('cqd8_subida_de_2', '                        d[k] = d[k].wrapping_add(1); // cqd8: o progresso sobe',
     '                        d[k] = d[k].wrapping_add(2); // mutacao: o progresso sobe', 'cqd_8 progresso sobe'),
    ('cqd8_sorteio_da_subida_30', '                if cqd8_sortear(0xd2 - peso as i32) < 0x14 && v < teto { // cqd8: o sorteio da subida',
     '                if cqd8_sortear(0xd2 - peso as i32) < 0x1e && v < teto { // mutacao: o sorteio da subida', 'cqd_8 progresso sobe'),
    ('cqd8_peso_mais_10', '        let base = (lv as i16).wrapping_mul(2).wrapping_add(0x3c); // cqd8: o peso do nível',
     '        let base = (lv as i16).wrapping_mul(2).wrapping_add(0x46); // mutacao: o peso do nível', 'cqd_8 progresso desce'),
]
