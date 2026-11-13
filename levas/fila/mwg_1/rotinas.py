# Os jogadores virtuais de uma partida: completar os elencos dos dois lados, liberar a partida e o grupo de posição
# de um jogador (VIRTUAL_STAFF, virtual_staff.cpp; leitor mwg, rodada 2 do mundo inteiro, 12/11/2026). Chamadas pelo
# dia de jogo (match_day.rs, Rust): fill_fixture_squads e release_fixture uma vez por partida (121.991 até 2002-08-01
# no mundo inteiro, cobertura_mundo.txt), get_player_position_group 4,1 milhões de vezes (a contagem dos elencos).
# Rodam em toda partida, inclusive na carreira só com a Inglaterra: os clubes ingleses completam os grupos de posição
# abaixo do mínimo (4, 4, 4, 8, 4, 8, 4, 8, do construtor 0x90d140) com jovens virtuais, e os estrangeiros das copas
# europeias e amistosos, com o elenco inteiro. Por isso LIGAS = 'ENG'.
# Convenções conferidas na desmontagem (0x90d2f0..0x90d4eb, 0x90d6b0..0x90d858, 0x90e020..0x90e08e):
# - fill_fixture_squads: thiscall(this, partida, vaga (short; o registro inteiro segue adiante), sem_conferir (char)),
#   ret 0xc; release_fixture: thiscall(this, partida), ret 4; get_player_position_group: thiscall(this, pessoa),
#   ret 4, devolve char (só o al: os bytes de cima do eax são o ponteiro do jogador; os chamadores usam o al);
# - 0x68d1b0 LLIST::get_next (ecx = lista), 0x68d0d0 add_tail (push 6; push &nó; ecx = lista), 0x68d1f0 remove
#   (push nó; ecx = lista); o cursor (+0xa) volta para a cabeça (+2) antes de cada busca;
# - 0x600a50 is_human_club cdecl (clube, 0, 1, 0, 0); 0x53ca90 database_get_club_manager cdecl (clube);
# - 0x90e760 get_id_block thiscall(vaga short, é_partida, mandante, &staff, &jogador), ret 0x14 (lê a palavra);
#   0x90e9a0 relocate_virtual_players thiscall(clube, staff, jogador), ret 0xc; 0x90dae0 top_up_club_squad
#   thiscall(clube, staff, jogador, partida, passo), ret 0x14;
# - 0x896c60 Tactics::remove_player_from_selection thiscall([0xde1d08]; clube, pessoa, 1|0, 1), ret 0x10;
# - a caixa de erro (linha 0xc3 de virtual_staff.cpp, 0xad439c): show_error_at (com o 0 em 0xb67a34, como a
#   original).
# Fidelidade: a vaga de um lado humano reescreve só a palavra baixa (contagem de staff − id do técnico) e depois
# decrementa o registro inteiro (dec edi), como a cópia faz; o nó da lista guarda a partida e a palavra baixa.
# Comparadores: fill e release só a original (nó novo/liberado da lista, sorteios): confere a partida na lista depois
# do fill e fora dela depois do release; get_player_position_group pelo valor (compare_value!).
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('fill_fixture_squads', 0x90d2f0, [0x81, 0xec, 0x14, 0x02, 0x00, 0x00]),
           ('release_fixture', 0x90d6b0, [0x83, 0xec, 0x0c, 0x53, 0x8b, 0x5c, 0x24, 0x14]),
           ('get_player_position_group', 0x90e020, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x40, 0x61])]
DOCS = ('os jogadores virtuais de uma partida: completar e liberar os elencos',
        'virtual players for a match: topping up and releasing the squads',
        [('fill_fixture_squads', 'VIRTUAL_STAFF::fill_fixture_squads', '0x90d2f0', 'virtual_staff.cpp', 'thiscall, ret 0xc',
          'põe a partida na lista dos jogadores virtuais e completa os dois elencos: partida já na lista sai; a vaga do bloco de ids é, com −1, a do técnico humano do mandante (ou do visitante): contagem de staff menos o id do técnico menos 1, senão −2; cada lado pega o seu bloco de 25 registros (get_id_block) e é completado (top_up_club_squad, com 1 quando a marca do lado na partida é 0); a partida entra na lista com a vaga; sem sem_conferir, um lado com menos de 15 jogadores no elenco mostra a caixa de erro (linha 0xc3), e a partida é liberada e completada de novo com a mesma vaga e sem conferir',
          "puts the fixture on the virtual players' list and tops up both squads: a fixture already on the list returns; the id block's slot is, with −1, that of the home (or away) side's human manager: the staff count less the manager's id less 1, otherwise −2; each side takes its block of 25 records (get_id_block) and is topped up (top_up_club_squad, with 1 when the side's mark on the fixture is 0); the fixture enters the list with the slot; unless no_check, a side with fewer than 15 players in its squad shows the error box (line 0xc3), and the fixture is released and filled again with the same slot and no check"),
         ('release_fixture', 'VIRTUAL_STAFF::release_fixture', '0x90d6b0', 'virtual_staff.cpp', 'thiscall, ret 4',
          'libera os jogadores virtuais de uma partida terminada: nas 17 vagas de clube visto, a primeira com o mandante (e depois, noutra, o visitante) guarda os virtuais do clube, que vão para o bloco da vaga (get_id_block com 0, 0; relocate_virtual_players); o lado sem vaga perde os virtuais (id de staff na contagem de staff ou acima): a vaga do elenco fica vazia e a pessoa sai da escalação (duas vezes, com 1 e com 0); por fim a partida sai da lista',
          "releases a finished fixture's virtual players: over the 17 viewed club slots, the first holding the home club (and then, in another, the away club) keeps the club's virtual players, which move to the slot's block (get_id_block with 0, 0; relocate_virtual_players); a side without a slot loses them (staff id at or above the staff count): its squad slot is emptied and the person leaves the team selection (twice, with 1 and with 0); last, the fixture leaves the list"),
         ('get_player_position_group', 'VIRTUAL_STAFF::get_player_position_group', '0x90e020', 'virtual_staff.cpp', 'thiscall, ret 4',
          'o grupo de posição de um jogador pelas notas de posição (15 ou mais vale): sem ficha de jogador 0xff; goleiro 0; defensor (zagueiro ou líbero) 1 na esquerda, senão 2 na direita, senão 3 (centro); meio-campista (volante, meia ou meia-atacante) 4 na esquerda, senão 6 na direita, senão 5; atacante 7; os outros 0xff',
          "a player's position group from his position ratings (15 or more counts): no player data 0xff; goalkeeper 0; defender (defender or sweeper) 1 on the left, else 2 on the right, else 3 (centre); midfielder (defensive, central or attacking) 4 on the left, else 6 on the right, else 5; attacker 7; anyone else 0xff")])
INTERESSA = ['Tweaks']
