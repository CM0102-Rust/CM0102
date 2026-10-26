# Mutacoes do nome da posicao, da descricao e do melhor candidato. A do nome ("player" no lugar de "utility player",
# jogadores de 4 ou mais posicoes) e a da descricao (veterano 3 anos antes) so mudam texto e o byte do genero: PODEM
# ESCAPAR da carreira ate 2002-06-01 (as duas rodam nas transferencias 0x8dc370 e na midia, mas o texto pode nao
# chegar a impressao); o comparando acusa (o comparador confere o texto). A do candidato troca o desempate % 500 por
# % 50: a 0x548810 so roda na midia 0x758200, que nao aparece na cobertura da carreira (lacuna provavel); o comparando
# acusa se a midia a chamar.
MUTACOES = [
    ('dbs_nome_da_posicao', '        escrever(0x9b7c9c, 2, 1); // utility player',
     '        escrever(0x9b7c94, 2, 1); // mutacao: "player" no lugar de "utility player"'),
    ('dbs_descricao', '    if q & 1 == 0 && idade > pessoa % 3 + 0x21 {',
     '    if q & 1 == 0 && idade > pessoa % 3 + 0x1e { // mutacao: veterano 3 anos antes'),
    ('dbs_candidato', '    n += id.wrapping_add(oid) % 500; // desempate pelos ids (dbs)',
     '    n += id.wrapping_add(oid) % 50; // mutacao: % 50 no lugar de % 500'),
]
