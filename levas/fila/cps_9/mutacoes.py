# Mutacoes. Os gols na carreira com um a mais e sem o primeiro grupo da tabela: a MATCH_PLAYER::setup grava o valor em
# +0x43 do jogador da partida, dentro do motor (a impressao das partidas pega; quentes, sem contador). Na fila das
# voltas aos treinos, a data igual passa a ser "antes": o bsearch nao acha as voltas do dia, que nao sao tiradas da fila
# nem dadas (a 0x630ee0 acerta tratamento, ferias e moral; so pega se a fila tiver voltas na carreira automatica). O
# registro do emprestimo so grava o historico em arquivo (fora da impressao): sem mutacao.
MUTACOES = [
    ('cps9_um_gol_a_mais', 's // cps9: os gols na carreira',
     's.wrapping_add(1) // mutacao: um gol a mais'),
    ('cps9_sem_o_primeiro_grupo', 'let mut i = 0i32; // cps9: do primeiro grupo',
     'let mut i = 6i32; // mutacao: sem o primeiro grupo'),
    ('cps9_volta_nao_achada', '0 // cps9: a volta de hoje achada',
     '-1 // mutacao: a volta de hoje nao e achada',
     'cps_9 same date'),
]
