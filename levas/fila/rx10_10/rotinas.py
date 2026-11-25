# As vagas de voto e as listas de historico do gerente de premios (award_manager.cpp; leitor rx10, rodada 6,
# 25/11/2026). Telas: o voto pendente na tela de indicados (tel27_1: AWARD_SHORTLIST, a tela de indicados) e no perfil
# (tel07_3); as listas de historico na tela de premios (YEAR_AWARD/TEAM_AWARD::get_history_list, sx05b_25). Nenhuma
# roda na carreira automatica (0 chamadas).
# - 0x4148e0 AWARD_MANAGER::set_pending_vote thiscall(tecnico, jogador, a, b) ret 0x10: a vaga = id do tecnico -
#   [0xae2364] (o numero de pessoas) + 16, de 0 a 16; a vaga de 12 bytes em +0x1c recebe jogador, a, b; fora: a caixa
#   0x3bc. Prologo mov eax, [esp+4]; sub esp, 0x200 (10 bytes).
# - 0x4149a0 AWARD_MANAGER::get_pending_vote thiscall(tecnico, jogador) ret 8: sem tecnico 0; fora das vagas a caixa
#   0x3e1 e 0; senao o endereco da vaga se o jogador dela for o pedido, senao 0 (setne/dec/and). Mesmo prologo.
# - 0x4142e0 AWARD_MANAGER::get_year_award_records thiscall(primeiro, ultimo) ret 8 e 0x414320
#   AWARD_MANAGER::get_team_award_records (o mesmo com +4): FILE_LLIST::get_nodes 0x58f120 thiscall([this] ou
#   [this+4], primeiro, ultimo) ret 8. Prologo mov eax, [esp+8]; mov edx, [esp+4] (8 bytes).
# Iguais no cm0102-1080.exe. Comparadores: set so a original com o plano (a vaga com os tres argumentos); get lado a
# lado (fora das vagas so a original, para a caixa nao sair duas vezes); as listas so a original (cada chamada le o
# arquivo e aloca) com o plano minimo.
TELAS = True
ROTINAS = [('award_manager_set_pending_vote', 0x4148e0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('award_manager_get_pending_vote', 0x4149a0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('award_manager_get_year_award_records', 0x4142e0, [0x8b, 0x44, 0x24, 0x08, 0x8b, 0x54, 0x24, 0x04]),
           ('award_manager_get_team_award_records', 0x414320, [0x8b, 0x44, 0x24, 0x08, 0x8b, 0x54, 0x24, 0x04])]
DOCS = ('as vagas de voto e as listas de histórico do gerente de prêmios',
        'the award manager\'s vote slots and history lists',
        [('award_manager_set_pending_vote', 'AWARD_MANAGER::set_pending_vote', '0x4148e0', 'award_manager.cpp', 'thiscall, ret 0x10',
          'regra: o voto pendente de um técnico humano é o jogador que ele escolheu na tela de indicados e os índices do prêmio, guardados na vaga dele até ele votar',
          'rule: a human manager\'s pending vote is the player he picked on the nominations screen and the award\'s indices, kept in his own slot until he casts it'),
         ('award_manager_get_pending_vote', 'AWARD_MANAGER::get_pending_vote', '0x4149a0', 'award_manager.cpp', 'thiscall, ret 8',
          'regra: a tela de indicados pergunta se o humano já escolheu um jogador: a vaga dele quando é esse jogador, senão nada',
          'rule: the nominations screen asks whether the human already picked a given player: his slot when it holds that player, else nothing'),
         ('award_manager_get_year_award_records', 'AWARD_MANAGER::get_year_award_records', '0x4142e0', 'award_manager.cpp', 'thiscall, ret 8',
          'regra: os vencedores passados dos prêmios do ano, na tela de prêmios, são os registros de histórico entre dois deslocamentos do primeiro arquivo do gerente',
          'rule: the year awards\' past winners on the award screen are the history records between two offsets of the manager\'s first history file'),
         ('award_manager_get_team_award_records', 'AWARD_MANAGER::get_team_award_records', '0x414320', 'award_manager.cpp', 'thiscall, ret 8',
          'regra: as seleções passadas dos prêmios de equipe, na tela de prêmios, são os registros de histórico entre dois deslocamentos do segundo arquivo do gerente',
          'rule: the team awards\' past teams on the award screen are the history records between two offsets of the manager\'s second history file')])
CAMPOS = []
INTERESSA = []
