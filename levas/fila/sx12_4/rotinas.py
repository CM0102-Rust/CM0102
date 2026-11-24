# O estado de tela do tecnico humano: a tela de taticas, a velocidade da narracao, as propostas, a senha e a vista do
# elenco (human_manager.cpp; leitor sx12, rodada 5, 23/11/2026). Treze acessores do mesmo molde (o registro do tecnico,
# ou do humano atual quando nulo; sem tecnico ou fora da tabela, a caixa "Error" com a linha propria e 0). Nenhum roda na
# carreira de ferias (0 chamadas na cobertura do mundo): so nas telas.
# Caminho no jogo para o Lucas provar (JOGAR_COMPARANDO.bat):
# - Taticas do clube: trocar a vista (Geral / Com a bola / Sem a bola), selecionar zonas, ligar "Show Runs", sair e
#   voltar (as cinco 0x5fc010..0x5fc490);
# - Opcoes do jogo: a velocidade da narracao (0x5fcff0 le, 0x5fd100 grava), e assistir a uma partida;
# - Oferecer contrato a um jogador (0x5fd930), fazer uma proposta por um jogador (0x5fda30) e confirmar uma
#   transferencia (0x5fdb30);
# - Senha: dar senha ao tecnico na criacao do jogo e carregar o jogo com senha (0x5fde40, 0x5fdf40);
# - Elenco: mudar a vista e voltar a tela (0x5fe950).
# Convencoes conferidas na desmontagem: thiscall(tecnico), ret 4, exceto set_commentary_speed thiscall(velocidade (so a
# palavra baixa), tecnico), ret 8; get_commentary_speed devolve ax (o resto do eax e a metade alta da base da tabela).
# Os ponteiros das taticas: registro + vaga ativa (byte com sinal +0x9a69) x 0x1ecd + 0x18e3/0x18e7/0x18eb/0x18ec/0x18ed.
SAVE = False
TELAS = True
ROTINAS = [('get_tactic_zone_selection_ptr', 0x5fc010, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('get_tactic_prev_zone_selection_ptr', 0x5fc130, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('get_tactic_view_mode_ptr', 0x5fc250, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('get_tactic_prev_view_mode_ptr', 0x5fc370, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('get_tactic_show_runs_ptr', 0x5fc490, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('get_commentary_speed', 0x5fcff0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('set_commentary_speed', 0x5fd100, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('get_contract_offer', 0x5fd930, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('get_bid_offer_slot', 0x5fda30, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('get_confirm_offer_slot', 0x5fdb30, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('get_password', 0x5fde40, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('get_password_entry', 0x5fdf40, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('get_squad_view_settings', 0x5fe950, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
_E = ('; sem tecnico (nem humano atual) ou fora da tabela, a caixa "Error" e 0',
      '; with no manager (nor current human) or out of range, the "Error" box and 0')
DOCS = ('o estado de tela do tecnico humano',
        "the human manager's screen state",
        [('get_tactic_zone_selection_ptr', 'HUMAN_MANAGER::get_tactic_zone_selection_ptr', '0x5fc010', 'human_manager.cpp', 'thiscall, ret 4',
          'devolve o endereco da selecao de zonas guardada da vaga de tatica ativa do tecnico, que a tela de taticas guarda e restaura' + _E[0],
          "returns the address of the stored zone selection of the manager's active tactic slot, which the tactics screen saves and restores" + _E[1]),
         ('get_tactic_prev_zone_selection_ptr', 'HUMAN_MANAGER::get_tactic_prev_zone_selection_ptr', '0x5fc130', 'human_manager.cpp', 'thiscall, ret 4',
          'devolve o endereco da selecao de zonas anterior da vaga de tatica ativa' + _E[0],
          'returns the address of the previous zone selection of the active tactic slot' + _E[1]),
         ('get_tactic_view_mode_ptr', 'HUMAN_MANAGER::get_tactic_view_mode_ptr', '0x5fc250', 'human_manager.cpp', 'thiscall, ret 4',
          'devolve o endereco da vista da tela de taticas da vaga ativa (geral, com a bola ou sem a bola)' + _E[0],
          'returns the address of the tactics screen view mode of the active slot (overview, with the ball or without it)' + _E[1]),
         ('get_tactic_prev_view_mode_ptr', 'HUMAN_MANAGER::get_tactic_prev_view_mode_ptr', '0x5fc370', 'human_manager.cpp', 'thiscall, ret 4',
          'devolve o endereco da vista anterior da tela de taticas da vaga ativa' + _E[0],
          'returns the address of the previous tactics screen view mode of the active slot' + _E[1]),
         ('get_tactic_show_runs_ptr', 'HUMAN_MANAGER::get_tactic_show_runs_ptr', '0x5fc490', 'human_manager.cpp', 'thiscall, ret 4',
          'devolve o endereco do indicador "mostrar as corridas" da vaga de tatica ativa' + _E[0],
          'returns the address of the Show Runs flag of the active tactic slot' + _E[1]),
         ('get_commentary_speed', 'HUMAN_MANAGER::get_commentary_speed', '0x5fcff0', 'human_manager.cpp', 'thiscall, ret 4',
          'devolve a velocidade da narracao das partidas do tecnico' + _E[0],
          "returns the manager's match commentary speed" + _E[1]),
         ('set_commentary_speed', 'HUMAN_MANAGER::set_commentary_speed', '0x5fd100', 'human_manager.cpp', 'thiscall, ret 8',
          'grava a velocidade da narracao das partidas do tecnico; sem tecnico ou fora da tabela, a caixa "Error" e nada',
          "stores the manager's match commentary speed; with no manager or out of range, the \"Error\" box and nothing"),
         ('get_contract_offer', 'HUMAN_MANAGER::get_contract_offer', '0x5fd930', 'human_manager.cpp', 'thiscall, ret 4',
          'devolve o endereco do rascunho de proposta de contrato do tecnico, usado pelas telas de oferecer contrato' + _E[0],
          "returns the address of the manager's draft contract offer, used by the offer contract screens" + _E[1]),
         ('get_bid_offer_slot', 'HUMAN_MANAGER::get_bid_offer_slot', '0x5fda30', 'human_manager.cpp', 'thiscall, ret 4',
          'devolve o endereco do ponteiro da proposta de transferencia pendente, usado pela tela de proposta' + _E[0],
          "returns the address of the pending transfer bid pointer, used by the bid screen" + _E[1]),
         ('get_confirm_offer_slot', 'HUMAN_MANAGER::get_confirm_offer_slot', '0x5fdb30', 'human_manager.cpp', 'thiscall, ret 4',
          'devolve o endereco do ponteiro da proposta usado pela tela de confirmar a transferencia' + _E[0],
          'returns the address of the offer pointer used by the confirm transfer screen' + _E[1]),
         ('get_password', 'HUMAN_MANAGER::get_password', '0x5fde40', 'human_manager.cpp', 'thiscall, ret 4',
          'devolve o endereco da senha do tecnico, usada nas telas de nome da criacao do jogo e na carga com senha' + _E[0],
          "returns the address of the manager's password, used by the setup name screens and the password checks" + _E[1]),
         ('get_password_entry', 'HUMAN_MANAGER::get_password_entry', '0x5fdf40', 'human_manager.cpp', 'thiscall, ret 4',
          'devolve o endereco dos 50 bytes da senha digitada, que as caixas de senha comparam com a guardada' + _E[0],
          'returns the address of the 50-byte typed password, which the password boxes compare with the stored one' + _E[1]),
         ('get_squad_view_settings', 'HUMAN_MANAGER::get_squad_view_settings', '0x5fe950', 'human_manager.cpp', 'thiscall, ret 4',
          'devolve o endereco dos 15 bytes onde o tecnico guarda a vista da tela do elenco entre as visitas' + _E[0],
          'returns the address of the 15 bytes where the manager keeps the squad screen view between visits' + _E[1])])
