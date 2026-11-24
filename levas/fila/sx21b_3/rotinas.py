# Os jogadores virtuais de um clube visto: tira-los, o bloco de ids e muda-los de bloco (virtual_staff.cpp; leitor
# sx21b, rodada 5, 24/11/2026). remove_virtual_players roda na carreira (17.286 chamadas no mundo ate 2002-08-01: os
# contratos 0x4cf87d/0x4d03e5, o gestor de transferencias 0x8ca117 e fill_viewed_club_squad); get_id_block em toda
# partida (fill_fixture_squads e release_fixture, mwg_1, ja Rust, a chamam pelo endereco); relocate_virtual_players
# quando um clube de partida e o visto por um tecnico humano (release_fixture) e nas telas.
# Convencoes conferidas: 0x90d860 thiscall(clube, nao_usado) ret 8; 0x90e760 thiscall(vaga (palavra com sinal),
# e_partida, mandante, &staff, &jogador) ret 0x14 (vaga de clube visto negativa: a caixa da linha 0x5e6 de
# virtual_staff.cpp); 0x90e9a0 thiscall(clube, staff, jogador) ret 0xc. 0x896c60 Tactics::remove_player_from_selection
# ([0xde1d08]; clube, pessoa, 1|0, 1); 0x5335f0 Staff::operator= (ecx = destino; origem).
# A vaga do tecnico humano da tela: [0xb74c26 + 0x300 * [0xb77c7a]]; sem tecnico, 16; senao a palavra baixa de
# 0xae2364 menos o id dele menos 1.
# Comparadores: lado a lado nas 17 vagas de clube visto e nas 50 do elenco; nos dois ids; no elenco e nos registros de
# pessoa e de jogador do bloco (tantos quantos os virtuais do clube). A selecao do time fica como a original deixa.
SAVE = False
ROTINAS = [
    ('remove_virtual_players', 0x90d860, [0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00]),
    ('get_id_block', 0x90e760, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
    ('relocate_virtual_players', 0x90e9a0, [0x83, 0xec, 0x08, 0x8b, 0x44, 0x24, 0x0c]),
]
DOCS = ('os jogadores virtuais de um clube visto: tirá-los, o bloco de ids e mudá-los de bloco',
        "a viewed club's virtual players: removing them, their id block and moving them",
        [('remove_virtual_players', 'remove_virtual_players', '0x90d860', 'virtual_staff.cpp', 'thiscall, ret 8',
          'regra: a vaga de clube visto do técnico humano da tela fica vazia e cada jogador virtual (id de staff a partir da contagem) sai da seleção do time e do elenco',
          "rule: the viewed club slot of the screen's human manager is freed and each virtual player (staff id from the count up) leaves the team selection and the squad"),
         ('get_id_block', 'get_id_block', '0x90e760', 'virtual_staff.cpp', 'thiscall, ret 0x14',
          'regra: blocos de 25 registros depois das contagens de staff e de jogadores: partida, vaga x 50 (mais 25 para o visitante; -2: 800/825, -3: 850/875); clube visto, (vaga + 36) x 25',
          'rule: blocks of 25 records past the staff and player counts: a fixture, slot x 50 (plus 25 for the away side; -2: 800/825, -3: 850/875); a viewed club, (slot + 36) x 25'),
         ('relocate_virtual_players', 'relocate_virtual_players', '0x90e9a0', 'virtual_staff.cpp', 'thiscall, ret 0xc',
          'regra: os virtuais do clube vão, em ordem, para os registros do bloco (pessoa copiada, ficha de jogador copiada, ids novos); para logo se um deles já está no bloco',
          "rule: the club's virtual players move, in order, into the block's records (person copied, player sheet copied, new ids); it stops at once if one of them is already in the block")])
CAMPOS = []
INTERESSA = []
