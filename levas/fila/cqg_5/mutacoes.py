# Mutacoes (cqg_5), nas posicoes previstas da Liga dos Campeoes (0x586900, na carga do jogo) e no grupo de uma fase
# (0x4b01d0):
# - o campeao previsto com a posicao 2: a estatistica da competicao muda (a expectativa sobre o clube);
# - os clubes das preliminares uma posicao abaixo do previsto;
# - as fases 0x3fd..0x406 (os grupos da Liga dos Campeoes) no grupo 0xb em vez de 0xa: a fase atual e o comeco do grupo
#   das competicoes mudam.
# [inferencia] podem escapar: as posicoes previstas so alimentam a estatistica (expectativas, textos), e o grupo das fases
# so e lido pelas rotinas das telas e da fase atual.
MUTACOES = [
    ('cqg5_campeao_em_segundo',
     'v.push((i, 1u8)); // cqg5: o campeão',
     'v.push((i, 2u8)); // mutacao: o campeao em segundo',
     'cqg_5 campeão'),
    ('cqg5_preliminar_uma_abaixo',
     'v.push((i, p)); // cqg5: a posição na preliminar',
     'v.push((i, p.wrapping_add(1))); // mutacao: uma posicao abaixo',
     'cqg_5 posição na preliminar'),
    ('cqg5_grupo_0xb',
     'return 0xa; // cqg5: o grupo das fases 0x3fd..0x406',
     'return 0xb; // mutacao: o grupo 0xb',
     'cqg_5 grupo 0xa'),
]
