# Os jogadores falsos (Database.cpp; leitor rxs1, rodada 6, 25/11/2026): a troca das pessoas reais por falsas, a troca
# dos registros dos jogadores e a troca de atributos entre colegas de elenco. As três inteiras (0x532e00..0x53307d,
# 0x533080..0x5335e1, 0x533710..0x533996), iguais no cm0102-1080.exe. Só rodam na criação do jogo com "Use Real
# Players: No" (ou a marca alemã): o setup (Rust, setup.rs) chama a 0x532e00 quando G_USE_REAL_PLAYERS = 0 ou
# G_PLAYER_SETUP_GERMAN_FLAG ≠ 0; a 0x532e00 chama as outras duas. Contagem 0 na carreira de teste (jogadores reais).
# Convenções conferidas na desmontagem: as três cdecl() sem argumentos, ret 0. A 0x532e00 começa com push ebx; push ebp;
# push esi; push edi; call 0x533080: o prólogo de 9 bytes leva o call rel32 na posição 4, (0x532e00, 4) nas RELATIVAS
# (INTEGRAR_ANTES.txt). A busca do nome comum vazio da 0x532e00 é a free_record (0x53a5b0, já em Rust) inline: a cópia
# a chama. 0x53ab90 database_get_staff_age(pessoa, &data atual) devolve char; 0x90cfc0 utils_random; 0x53d740
# database_is_staff_eu_national(pessoa, 0); 0x53d980 database_is_staff_foreign(pessoa, nação, 1); 0x5335f0
# Staff::operator= thiscall(destino, origem) ret 4 (rxs1_10); 0x538ec0 e 0x538bc0 normalizam os atributos. As
# tentativas da 0x533080 são o número de pessoas × 16.0 (rdata::F64_16) truncado; os Date::Date (0x54a110) dos locais
# e o destrutor vazio 0x5a53c0 não têm efeito e a cópia não os chama.
# Comparadores: só a original (sorteiam e reescrevem a tabela de pessoas inteira uma vez); a 0x532e00 confere depois o
# que garante (sem salário, sem valor, sem gols sem jogos pela seleção), a 0x533080 que cada registro guardou o id e o
# vínculo de clube.
# A prova: só uma criação de jogo com "Use Real Players: No" (opção do jogo novo; na carreira automática, o
# G_USE_REAL_PLAYERS = 0) exercita as rotinas; a cadeia normal não as roda e as mutações escapam.
SAVE = False
ROTINAS = [('database_apply_fake_players', 0x532e00, [0x53, 0x55, 0x56, 0x57, 0xe8, 0x77, 0x02, 0x00, 0x00]),
           ('database_shuffle_player_records', 0x533080, [0x6a, 0xff, 0x68, 0xff, 0x78, 0x95, 0x00]),
           ('database_shuffle_squad_player_attributes', 0x533710, [0x83, 0xec, 0x0c, 0xa1, 0x5c, 0x23, 0xae, 0x00])]
DOCS = ('os jogadores falsos: a troca das pessoas reais, dos registros e dos atributos',
        'the fake players: the swap of the real people, of the records and of the attributes',
        [('database_apply_fake_players', 'database_apply_fake_players', '0x532e00', 'Database.cpp', 'cdecl',
          'troca as pessoas reais por falsas: embaralha os registros dos jogadores, dá a cada pessoa com nação um nome e um sobrenome sorteados dela e o nome comum vazio, embaralha os atributos entre colegas de elenco e, a cada jogador, jogos e gols pela seleção plausíveis para a idade, a habilidade e a reputação (sem jogos, sem gols) e zera o salário e o valor',
          'replaces the real people with fake ones: shuffles the player records, gives every person with a nation a random first name and surname of it and the empty common name, shuffles the attributes between squad mates and gives every player caps and goals plausible for his age, ability and reputation (no caps, no goals), zeroing his wage and value'),
         ('database_shuffle_player_records', 'database_shuffle_player_records', '0x533080', 'Database.cpp', 'cdecl',
          'número de pessoas × 16 tentativas de trocar os registros de dois jogadores sorteados (ambos com ficha de jogador, ambos ou nenhum com ficha de não jogador, ambos com clube; de nações ou divisões diferentes só com a mesma condição de comunitário e ambos aceitos como estrangeiros na nação do seu clube), mantendo no lugar o id, o vínculo de clube, a habilidade atual e potencial e, por cara ou coroa, a data de nascimento',
          'staff count × 16 tries to swap the records of two random players (both with a player sheet, both or neither with a non-player sheet, both with a club; of different nations or divisions only with the same EU status and both accepted as foreigners in their club\'s nation), keeping in place the id, the club job, the current and potential ability and, by a coin, the date of birth'),
         ('database_shuffle_squad_player_attributes', 'database_shuffle_squad_player_attributes', '0x533710', 'Database.cpp', 'cdecl',
          'em cada clube, 100 tentativas de trocar atributos entre dois colegas de elenco sorteados de idades próximas (até 4 anos) e, com jogos pela seleção, de habilidade próxima (até 10): por cara ou coroa a velocidade e a aceleração, a resistência, a impulsão e o cabeceio (não para um zagueiro nota 20), e sempre as habilidades atual e potencial; depois normaliza os atributos de cada jogador',
          'in every club, 100 tries to swap attributes between two random squad mates of close ages (up to 4 years) and, with caps, close ability (up to 10): by a coin pace and acceleration, stamina, jumping and heading (not for a defender rated 20), and always the current and potential abilities; then rebalances every player\'s attributes')])
