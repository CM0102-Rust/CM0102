# Roda uma vez na criação do jogo: create_game 0x828110 chama a 0x535a80 (0x828894) com a data do jogo 0xae2c90, depois
# dos cortes e antes dos jogadores falsos (0x532e00). A gravação do .qst da prova do save cria o jogo pelo início rápido
# e grava o banco já inicializado: pessoas (+4/+8/+0xc nomes, datas, +0x18 idade, +0x1a nação, +0x3d cargo, +0x56..+0x5d),
# jogadores (habilidades, potencial, posições, atributos) e não jogadores (reputações, atributos, +0x43) vão para o save.
# Quadro de 0xe40 sem local lida sem valor (atribuição definida por byte, da entrada e da volta do laço das pessoas): as
# únicas leituras sem escrita são as metades altas de curtos lidos como dword (só o curto conta). Do laço para a pessoa
# seguinte passam, como na original: os vetores 0x54 e 0x290 (limpos só na entrada), o melhor escore [0x40], a barra.
# Sorteios só pela 0x90cfc0 (e pelas 0x539c60/0x53a1b0 já em Rust), na mesma ordem; caixas: sem pessoas (a original),
# os testes de pessoa nula (inalcançáveis) e o bloco morto de 0x536952 (clube ≠ 0 e = 0) ficam de fora.
SAVE = True
ROTINAS = [('inicializar_jogadores', 0x535a80, [0x81, 0xec, 0x40, 0x0e, 0x00, 0x00])]
DOCS = ('a criação do jogo: inicializar os jogadores e as pessoas',
        'game creation: initialising the players and the staff',
        [('inicializar_jogadores', 'database_player_initialisation', '0x535a80', 'Database.cpp', 'cdecl',
          'prepara cada pessoa do jogo novo: nomes que faltam, datas, atributos de pessoa, nação; do jogador as habilidades, o potencial, a atual, a idade, as posições e os atributos que faltam (pelo jogador mais parecido entre 250 sorteados); do não jogador as reputações, os atributos (o molde entre 250), o cargo e os nomes famosos; depois os vetores relativos à habilidade e as normalizações; barra "Player Initialisation"',
          'prepares every person of the new game: missing names, dates, staff attributes, nation; for the player the abilities, potential, current ability, age, positions and missing attributes (from the most similar of 250 random players); for the non-player the reputations, attributes (template among 250), job and the famous names; then the ability-relative vectors and the normalisations; "Player Initialisation" bar')])
CAMPOS = [('TStaff +4/+8/+0xc nomes, +0x10 nascimento, +0x18 idade, +0x1a nação, +0x31/+0x3e/+0x46 datas, +0x39 clube, +0x3d cargo, +0x56..+0x5d atributos; grava (inicializar_jogadores)', 'TStaff +4/+8/+0xc names, +0x10 birth date, +0x18 age, +0x1a nation, +0x31/+0x3e/+0x46 dates, +0x39 club, +0x3d job, +0x56..+0x5d attributes; writes (inicializar_jogadores)'),
          ('TPlayer +5 atual, +7 potencial, +9/+0xb/+0xd habilidades (×50), +0x45, posições, lados e atributos; grava (inicializar_jogadores)', 'TPlayer +5 current, +7 potential, +9/+0xb/+0xd abilities (×50), +0x45, positions, sides and attributes; writes (inicializar_jogadores)'),
          ('TNonPlayer +4/+6 habilidades, +8/+0xa/+0xc reputações (×50), atributos, +0x23..+0x42 zerados, +0x43; grava (inicializar_jogadores)', 'TNonPlayer +4/+6 abilities, +8/+0xa/+0xc reputations (×50), attributes, +0x23..+0x42 cleared, +0x43; writes (inicializar_jogadores)'),
          ('tabela de 79 bytes por pessoa [0xae2d0c]: +9 = 0x9c (jogador), +0xb &= 0xf0; grava (inicializar_jogadores)', '79-byte per-person table [0xae2d0c]: +9 = 0x9c (player), +0xb &= 0xf0; writes (inicializar_jogadores)'),
          ('vetores dos moldes 0xae2044/0xae2114 (bandeira 0x9b59b8) e 0xae18bc/0xae2238 (0x9b59b9); grava (inicializar_jogadores)', 'template vectors 0xae2044/0xae2114 (flag 0x9b59b8) and 0xae18bc/0xae2238 (0x9b59b9); writes (inicializar_jogadores)')]
INTERESSA = ['SGE', 'CMRTE']
