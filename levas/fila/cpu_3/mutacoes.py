# Mutacoes. Todas na estatistica de uma categoria (0x7c1e30). A escolha da escalacao 0x6e2f20 pede a nota media da
# categoria 5 (qual 0x12, padrao 6,5) de cada candidato: sem registro (quem ainda nao jogou na temporada: os recem-
# chegados e os jovens, e todos antes da primeira rodada) zero no lugar do padrao muda a escalacao. Meio ponto a mais
# para quem tem registro muda a ordem contra quem nao tem (6,5) e as contas da injury_word_0c do clube 0x545e90 e da forca
# do jogador 0x546ab0. O padrao no lugar de zero sem registro (qual 1, os jogos da categoria 6 no dia da comissao
# 0x5ce2d0 e no MATCH_PLAYER::setup 0x6eaea0) conta 6,5 jogos para quem nao jogou. Quente: sem contador.
MUTACOES = [
    ('cpu3_sem_registro_zero', 'return padrao as f64; // cpu3: sem registro, o padrao',
     'return 0.0; // mutacao: sem registro, zero'),
    ('cpu3_meio_ponto_a_mais', 'return valor(r, qual, padrao.to_bits()); // cpu3: o valor do registro',
     'return valor(r, qual, padrao.to_bits()) + 0.5; // mutacao: meio ponto a mais'),
    ('cpu3_sem_registro_o_padrao', 'return 0.0; // cpu3: sem registro, zero',
     'return padrao as f64; // mutacao: o padrao para todos'),
]
