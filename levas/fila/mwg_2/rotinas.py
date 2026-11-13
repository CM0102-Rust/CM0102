# Os jogadores virtuais: contar o elenco disponível de um clube e completá-lo (VIRTUAL_STAFF, virtual_staff.cpp;
# leitor mwg, rodada 2 do mundo inteiro, 12/11/2026). Chamadas por fill_fixture_squads (mwg_1) em toda partida, dos
# dois lados: 243.982 e 243.980 chamadas até 2002-08-01 no mundo inteiro (cobertura_mundo.txt). Rodam também na
# carreira só com a Inglaterra (os clubes ingleses completam com jovens os grupos abaixo do mínimo; os estrangeiros
# das copas europeias, com o elenco inteiro). Por isso LIGAS = 'ENG'.
# Convenções conferidas na desmontagem (0x90d8f0..0x90dad2, 0x90dae0..0x90e00e):
# - count_available_players: thiscall(this, clube, passo (não lido), partida, &tabela, &disponíveis, &reais,
#   &elegíveis), ret 0x1c, devolve 1 sempre (o teste de 30 disponíveis e de goleiros no fim acaba igual nos dois
#   caminhos: libera a tabela só sem ponteiro para ela);
# - top_up_club_squad: thiscall(this, clube, staff, jogador, partida, passo), ret 0x14;
# - cdecl: 0x53b900 database_is_national_team, 0x5409b0 club_is_national_team, 0x600a50 is_human_club (5 args),
#   0x53b3d0 get_national_team, 0x542740 club_has_reserve_squad (0/1 no eax inteiro, repassado como reserva),
#   0x945501 calloc(8, 1), 0x9452ca free, 0x9469fa srand, 0x90cfc0 random(30000);
# - thiscall: 0x7757d0 ([0xb74318]; pessoa, &data, clube, 0, 0), 0x62c400 ([0xb63d28]; pessoa) -> al, 0x55d270
#   ([0xae28f0]; pessoa, competição, 1, 0), 0x896c60 ([0xde1d08]; clube, pessoa, 1|0, 1), 0x7754c0/0x775520
#   ([0xb74318]; clube, 0|1) -> al (máximo/mínimo do elenco da seleção), 0x5fd6b0 ([0xb63c98]; 0, &todas) -> eax 0/1,
#   0x90e020 (pessoa) -> al, 0x90d8f0 (7 args), 0x90e0a0 create_virtual_player (7 args, ret 0x1c).
# Fidelidade: o grupo vai para create_virtual_player no registro do 3º argumento (os bytes de cima são do id do
# jogador); a cópia passa o byte (a criação só lê o byte). As saídas antecipadas da original (seleção humana, todas as
# seleções) não liberam a tabela nem devolvem a semente, como a cópia.
# Comparadores: count roda a original (que tira os virtuais do elenco) e depois a cópia sobre o mesmo elenco (os reais
# não mudam), compara as três contagens e os 8 grupos; top_up lado a lado (sorteio e semente da CRT, as 50 vagas do
# elenco, os 25 registros de staff e de jogador do bloco).
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('count_available_players', 0x90d8f0, [0x83, 0xec, 0x10, 0x53, 0x55, 0x56, 0x57]),
           ('top_up_club_squad', 0x90dae0, [0x83, 0xec, 0x2c, 0x53, 0x55, 0x56, 0x57])]
DOCS = ('os jogadores virtuais: contar o elenco de um clube e completá-lo',
        "virtual players: counting a club's squad and topping it up",
        [('count_available_players', 'VIRTUAL_STAFF::count_available_players', '0x90d8f0', 'virtual_staff.cpp', 'thiscall, ret 0x1c',
          'conta os jogadores de um elenco que podem jogar, por grupo de posição: um jogador virtual (id de staff na contagem de staff ou acima) sai da escalação e do elenco; os outros contam como reais; quem está a serviço da seleção hoje, ou num clube quem não é do quadro dele, para aí; os marcados para o elenco (bit 4 numa seleção B, bit 1 nas outras) são elegíveis; o lesionado grave (estado 2) e, com partida não marcada −2, o suspenso na competição param; os outros são disponíveis e somam um no seu grupo (get_player_position_group); as contagens saem pelos ponteiros não nulos; devolve 1',
          "counts the players of a squad who can play, by position group: a virtual player (staff id at or above the staff count) leaves the selection and the squad; the others count as real; one away on international duty today, or at a club one not on its staff, stops there; those marked for the squad (bit 4 in a B national squad, bit 1 elsewhere) are eligible; the badly injured (state 2) and, with a fixture not marked −2, the banned in its competition stop; the rest are available and add one to their group (get_player_position_group); the counts go out through the non-null pointers; returns 1"),
         ('top_up_club_squad', 'VIRTUAL_STAFF::top_up_club_squad', '0x90dae0', 'virtual_staff.cpp', 'thiscall, ret 0x14',
          'cria jogadores virtuais nos grupos de posição em falta: guarda a semente (sorteio(30000) + 1); clube sem nação e os MLS All Stars saem; conta o elenco; os criados são jovens (17 anos, mais fracos) numa seleção principal, num clube de liga selecionada com competição e num clube japonês fora das ligas nomeadas; o alvo de uma seleção é a soma dos máximos do elenco principal e do B (seleção humana sai; seleção B sem mínimo fica só com o principal; seleção principal de nação humana sai se o humano não dirige todas, e nação com mais de 500 pessoas e sem mínimo, ou com mais disponíveis que ele, não ganha virtuais); o de um clube é 50, e o clube do computador precisa de 20 elegíveis antes dos mínimos menores; enquanto os reais estiverem abaixo do alvo e menos de 25 tiverem sido criados, cada volta pelos 8 grupos cria um no grupo abaixo do mínimo (4, 4, 4, 8, 4, 8, 4, 8; seleção B 6, 4, 4, 12, 4, 8, 4, 8), com a semente (criados no grupo + 10000) × (id do clube + 1) × (grupo + 1000), de modo que o clube sempre tem os mesmos virtuais: numa seleção, do elenco principal enquanto o máximo passar os elegíveis, senão da seleção B como reserva; num clube, normal, ou reserva quando a liga é selecionada ou o grupo já tem o mínimo menor com 20 elegíveis e o clube tem a marca de reserva; números de camisa a partir de 51; no fim libera a tabela e devolve a semente',
          "creates virtual players in the position groups that are short: keeps the seed (random(30000) + 1); a club without a nation and the MLS All Stars leave; counts the squad; the created players are youngsters (17 years old, weaker) at a senior national team, at a club of a selected league with a competition and at a Japanese club outside the named leagues; a national squad's target is the sum of the main and B squad maxima (a human national team leaves; a B squad without a minimum keeps only the main one; a senior team of a human nation leaves unless the human manages all of them, and a nation with more than 500 people and no minimum, or more available than it, gets no virtual players); a club's is 50, and a computer club needs 20 eligible before the lower minimums; while the real players are below the target and fewer than 25 were created, each round over the 8 groups creates one in each group below its minimum (4, 4, 4, 8, 4, 8, 4, 8; B squad 6, 4, 4, 12, 4, 8, 4, 8), seeded with (created in the group + 10000) × (club id + 1) × (group + 1000), so a club always gets the same virtual players: at a national team, a main squad one while the maximum is above the eligible, otherwise a B squad reserve; at a club a normal one, or a reserve when the league is selected or the group already has the lower minimum with 20 eligible and the club has the reserve mark; squad numbers from 51; at the end it frees the table and puts the seed back")])
INTERESSA = ['Tweaks']
