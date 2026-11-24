# Mutacoes (sx08b_2). A prova e o .qst da prova do save: a criacao do jogo (que roda os cortes uma vez) vai inteira
# para o save, com as tabelas de pessoas, jogadores, nao jogadores e preferencias ja cortadas e compactadas.
# - clube grande a partir de 6000 de reputacao (e nao 7500): mais pessoas ficam (nos tamanhos que nao sao 1 e 2) ou
#   a ordem dos sorteios muda (1 e 2); a tabela de pessoas do save muda;
# - as pessoas virtuais nascem em 1983: as datas de nascimento das ultimas linhas mudam;
# - so 9 copias de nao jogadores no fim: a ultima linha da tabela de nao jogadores fica outra.
# Evitadas: as caixas fatais (realloc que muda de lugar, calloc sem memoria) e a falta da tabela de nacoes.
MUTACOES = [
    ('sx08b_2_clube_grande_6000',
     '    let big_club = club != 0 && read_i16(club + club::REPUTATION) >= 0x1d4c;',
     '    let big_club = club != 0 && read_i16(club + club::REPUTATION) >= 0x1770; // mutation: 6000'),
    ('sx08b_2_virtuais_em_1983',
     '        let d = exe::new_date(birth.as_mut_ptr() as usize, day, 0x7be);',
     '        let d = exe::new_date(birth.as_mut_ptr() as usize, day, 0x7bf); // mutation: 1983'),
    ('sx08b_2_nove_copias',
     '    for i in 0..10 {',
     '    for i in 0..9 { // mutation: one copy less'),
]
