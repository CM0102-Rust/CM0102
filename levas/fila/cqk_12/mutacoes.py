# Mutacoes (cqk_12), na Copa Africana de Nacoes de janeiro e fevereiro de 2002 (jogadores das ligas da Inglaterra,
# Franca, Espanha e Italia convocados): o mata-mata criado uma fase antes (0x401e10, 1 chamada ate 2002-06-01 com as 8
# ligas), o grupo A com tres times e as linhas do grupo A trocadas aos pares (0x4019c0, 1 chamada: a ordem da tabela da
# a ordem dos jogos) mudam os jogos das selecoes e o tempo de convocacao. A carreira pega pelos jogos e escalacoes.
MUTACOES = [
    ('cqk12_mata_mata_uma_fase_antes',
     'if f == 3 { // cqk12: o mata-mata da Copa Africana depois dos grupos',
     'if f == 2 { // mutacao: o mata-mata uma fase antes',
     'cqk_12 african cup knockout'),
    ('cqk12_grupo_a_com_tres',
     'let times = 4i16; // cqk12: os quatro do grupo A',
     'let times = 3i16; // mutacao: tres no grupo A',
     'cqk_12 african cup group a'),
    ('cqk12_linhas_trocadas',
     'let selecao = cqk12_d(cqk12_d(this + 0xee).wrapping_add((i as isize * 4) as usize)); // cqk12: a seleção da linha',
     'let selecao = cqk12_d(cqk12_d(this + 0xee).wrapping_add((((i as isize) ^ 1) * 4) as usize)); // mutacao: linhas trocadas aos pares',
     'cqk_12 african cup group a'),
]
