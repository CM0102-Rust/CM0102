# As historias do jogador com o tecnico: a gratidao pela fase, a euforia com a virada e a chance no time principal
# (european_cup.cpp pelo mapa; leitor cx04, rodada 5, 24/11/2026). Simulacao, carreira ENG: chamadas pelo dia do jogo
# (0x5d0c02) e pelo dia de jogos (0x6b7ca8, 0x6b7cbd, ja em Rust no match_day.rs, pelo exe::).
# Convencoes conferidas: as tres cdecl (add esp, 8 / 0x10 / 8); as do exe como no cx04_7. Condicoes comuns: jogador
# com ficha (+0x61) e nacao (+0x1a); clube com nacao (+0x53), estadio (+0x69) com cidade (+0x38) e tecnico (+0xcf). As
# vagas: o jogador (0..3), o clube (4), reputacao atual da ficha (+0xb, palavra com sinal) > 7250 ? 2 : 1 (5), o
# tecnico (6..9). Quadro SEH so para a noticia local. Prologo mov eax, fs:[0] (6 bytes). Iguais no cm0102-1080.exe.
# - 0x589300 news_post_player_thanks_manager(jogador, clube): noticia 0xbde.
# - 0x5896d0 news_post_player_ecstatic_at_turnaround(jogador, clube, adversario, sequencia): noticia 0xbe1; o
#   adversario e obrigatorio; vagas 10 (o adversario) e 11 (a sequencia).
# - 0x589bc0 news_post_player_pleased_with_chance(jogador, clube): noticia 0xbe2; vaga 10 = sorteio de 2 nos clubes de
#   reputacao (+0x80) acima de 6250, senao 0.
# Comparador: compare_story_post do cx04_7 (integrar depois dele). Os auxiliares club_player_story_ready e
# club_player_story servem tambem ao cx04_10.
SAVE = False
ROTINAS = [('news_post_player_thanks_manager', 0x589300, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('news_post_player_ecstatic_at_turnaround', 0x5896d0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('news_post_player_pleased_with_chance', 0x589bc0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as histórias do jogador com o técnico: gratidão, virada e chance no time',
        'the player-and-manager stories: thanks, turnaround and first-team chance',
        [('news_post_player_thanks_manager', 'news_post_player_thanks_manager', '0x589300', 'european_cup.cpp', 'cdecl',
          'manda ao técnico do clube a história de que um jogador credita ao técnico a boa fase, com o jogador, o clube, se ele é famoso (reputação acima de 7250) e o técnico',
          'sends the club\'s manager the story that a player credits the manager for his good form, with the player, the club, whether he is famous (reputation above 7250) and the manager'),
         ('news_post_player_ecstatic_at_turnaround', 'news_post_player_ecstatic_at_turnaround', '0x5896d0', 'european_cup.cpp', 'cdecl',
          'manda ao técnico do clube a história de que um jogador está eufórico com a vitória depois de uma sequência sem vencer, com o adversário e o tamanho da sequência',
          'sends the club\'s manager the story that a player is ecstatic at the win after a winless run, with the opponent and the length of the run'),
         ('news_post_player_pleased_with_chance', 'news_post_player_pleased_with_chance', '0x589bc0', 'european_cup.cpp', 'cdecl',
          'manda ao técnico do clube a história de que um jogador está contente com a chance no time principal; nos clubes de reputação acima de 6250 a redação é sorteada entre 2',
          'sends the club\'s manager the story that a player is pleased with his first-team chance; at clubs of reputation above 6250 the wording is drawn among 2')])
CAMPOS = []
INTERESSA = []
