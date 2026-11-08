# Mutacoes (cqs_3). Os grupos, a segunda e a terceira fases da Serie C de 2001 rodam na carreira de 8 ligas ate
# 2002-06-01 (os grupos tambem na virada para 2002); as tres mudam quem joga com quem:
# - os grupos com a vitoria valendo o empate (+0xc3 no lugar de +0xc2): outras tabelas, outros classificados;
# - a segunda fase com as vagas em ordem (o melhor contra o segundo, e nao contra o 16o): outros confrontos;
# - a terceira fase embaralhando 7 em vez de 8 (outra contagem de sorteios e outros grupos).
MUTACOES = [
    ('cqs3_grupos_vitoria_vale_empate',
     'bra_nat_third_create_group_stage_cqs3_grupo(this, g, 0xc2) // cqs3: os pontos por vitória nos grupos',
     'bra_nat_third_create_group_stage_cqs3_grupo(this, g, 0xc3) // mutacao: a vitoria vale o empate',
     'cqs_3 grupo'),
    ('cqs3_segunda_fase_em_ordem',
     'const SECOND_ROUND_SEEDS: [u8; 16] = [0, 2, 4, 6, 8, 10, 12, 14, 15, 13, 11, 9, 7, 5, 3, 1]; // cqs3: as vagas da segunda fase',
     'const SECOND_ROUND_SEEDS: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]; // mutacao: as vagas em ordem',
     'cqs_3 segunda fase'),
    ('cqs3_terceira_embaralha_sete',
     'embaralhar(v.as_mut_ptr(), 8); // cqs3: os oito vencedores embaralhados',
     'embaralhar(v.as_mut_ptr(), 7); // mutacao: embaralha sete',
     'cqs_3 terceira fase'),
]
# Prova do agente (06/11, carreira Rust de 8 ligas ate 2002-06-01; base com os 12 itens cqr/cqs/cqt_1..4, impressao b690295afd222f6a): as tres ACUSADAS (2001-246, 2001-286, 2001-294).
