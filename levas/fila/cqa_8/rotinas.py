# Os contratos depois do jogo (os bonus, a felicidade, a moral e a evolucao de quem ficou de fora), a moral da ficha e
# o texto dela (contract_manager.cpp).
# - 0x4ca6d0 (CONTRACT_MANAGER::update_after_match, 19.414 chamadas ate 2002-06-01 na cobertura de 17/10, pelo
#   fechamento da partida do dia 0x6b5f00, ja em Rust): bonus pagos (0x599960), felicidade e moral dos contratos,
#   evolucao de quem ficou de fora (0x6d26b0) e a troca do principal (0x4cbd20). Sorteia (2 x sorteio(30) por reserva ou
#   titular de pouco jogo, sorteio(21 - +0x57) por jogador do vencedor e do perdedor).
# - 0x4cc2d0 (contract_manager_get_player_morale) e 0x4cc2e0 (contract_manager_write_morale_text): so pelas telas
#   (passageiras; a carreira automatica nao abre telas).
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   0x540a50 parceiro: cdecl (add esp, 0xc), 3 args. 0x53b900 selecao: cdecl (add esp, 4), 1 arg. 0x4cf5f0 contrato com
#   o clube: thiscall (mov ecx, [esp+0x24] = this), 2 args. 0x5c6710 amistoso: thiscall (mov ecx, [0xae2c74]), 1 arg.
#   0x5a0590 financas: thiscall (mov ecx, [0xae2a38]), 1 arg. 0x599960 bonus: thiscall (mov ecx, ebx = financas), 1 arg.
#   0x90cfc0 sorteio: cdecl (add esp, 4; os dois sorteio(30) saem juntos com add esp, 8), 1 arg. 0x600a50 humano: cdecl
#   (add esp, 0x14), 5 args. 0x540980 marca de elenco: cdecl (add esp, 8), 2 args. 0x4cb0a0 moral sobe: a original poe
#   ecx = this e empurra 3 (ret 0xc; a copia Rust e stdcall): declarada thiscall com 3 na pilha, serve aos dois.
#   0x4cb250 moral cai: thiscall (mov ecx, this), 3 args. 0x55d270 suspenso: thiscall (mov ecx, [0xae28f0]), 4 args.
#   0x62c400 lesao: thiscall (mov ecx, [0xb63d28]), 1 arg. 0x7757d0 selecao do jogador: thiscall (mov ecx, [0xb74318]),
#   5 args. 0x6d26b0 evolucao: cdecl (add esp, 0x20), 8 args (o float em fstp dword [esp]). 0x4cbb60 lugar no elenco:
#   thiscall (mov ecx, this), 2 args. 0x4cbd20 troca: thiscall (mov ecx, this), 3 args. 0x66f4e0 formatar: cdecl (add
#   esp, 0xc), 3 args.
# Comparadores: 0x4ca6d0 so a original (a moral, a evolucao e a troca sorteiam, abrem queixa/pedido e mexem no elenco),
# conferindo os bonus previstos antes (o +0x138 das financas de cada clube); 0x4cc2d0 pelo valor; 0x4cc2e0 pelo texto.
# Iguais no cm0102-1080.exe.
SAVE = False
ROTINAS = [('contratos_depois_do_jogo', 0x4ca6d0, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8]),
           ('moral_da_ficha', 0x4cc2d0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x61]),
           ('texto_da_moral', 0x4cc2e0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x61])]
DOCS = ('os contratos depois do jogo (os bônus, a felicidade, a moral e a evolução de quem ficou de fora), a moral da ficha e o texto dela',
        'contracts after the match (bonuses, happiness, morale and the development of those left out), the record morale and its text',
        [('contratos_depois_do_jogo', 'CONTRACT_MANAGER::update_after_match', '0x4ca6d0', 'contract_manager.cpp', 'thiscall',
          'depois de cada jogo: os bônus dos contratos, a felicidade pelos minutos, a moral pelo sorteio e pelo resultado, e a evolução e a felicidade de quem ficou de fora',
          'after each match: contract bonuses, happiness by minutes played, morale by a draw and by the result, and the development and happiness of those left out'),
         ('moral_da_ficha', 'contract_manager_get_player_morale', '0x4cc2d0', 'contract_manager.cpp', 'stdcall',
          'a moral da ficha do jogador (+0x45), pelas telas',
          'the morale of the player record (+0x45), for the screens'),
         ('texto_da_moral', 'contract_manager_write_morale_text', '0x4cc2e0', 'contract_manager.cpp', 'stdcall',
          'a palavra da moral (de "Very Low" a "Superb") num texto, pelas telas',
          'the morale word (from "Very Low" to "Superb") into a text, for the screens')])
CAMPOS = [('jogo +0x40, +0x41 marcas; +0x43/+0x44 e +0x47/+0x48 placar; +0x14 competição; +0x1c/+0x20 clubes; +0x3a reputação conta; valor/ponteiro; lê (contratos_depois_do_jogo)',
           'match +0x40, +0x41 flags; +0x43/+0x44 and +0x47/+0x48 score; +0x14 competition; +0x1c/+0x20 clubs; +0x3a reputation check; value/pointer; reads (contratos_depois_do_jogo)'),
          ('resumo da partida + lado × 0x625: +0x190 vagas de 0x49 bytes (+0/+4 ids, +0x1f, +0x20, +0x21, +0x22, +0x27, +0x30, +0x32 minutos), +0x154 palavras; valor; lê',
           'match summary + side × 0x625: +0x190 0x49-byte slots (+0/+4 ids, +0x1f, +0x20, +0x21, +0x22, +0x27, +0x30, +0x32 minutes), +0x154 words; value; reads'),
          ('contrato +0x10, +0x14, +0x18 bônus; +0x35 & 0x3f tipo; valor; lê', 'contract +0x10, +0x14, +0x18 bonuses; +0x35 & 0x3f type; value; reads'),
          ('contrato +0x3d felicidade, +0x3e e +0x3f contas de jogos, +0x44 moral; valor; lê e grava', 'contract +0x3d happiness, +0x3e and +0x3f match counters, +0x44 morale; value; reads and writes'),
          ('registro de 79 bytes em [0xae2d0c] +9 (seleção) e +4 (segundo contrato); valor/índice; lê e grava (+9)', '79-byte record at [0xae2d0c] +9 (national team) and +4 (second contract); value/index; reads and writes (+9)'),
          ('clube +0xd7 elenco (50), +0x80 reputação; ponteiro/valor; lê', 'club +0xd7 squad (50), +0x80 reputation; pointer/value; reads'),
          ('pessoa +0x57, +0x61 ficha; valor/ponteiro; lê', 'person +0x57, +0x61 player record; value/pointer; reads'),
          ('ficha +0x45 moral; valor; lê (moral_da_ficha, texto_da_moral)', 'player record +0x45 morale; value; reads (moral_da_ficha, texto_da_moral)'),
          ('[0xb74350] +0x10/+0x18 notas do campeonato; valor; lê', '[0xb74350] +0x10/+0x18 league ratings; value; reads')]
INTERESSA = ['CMRTE']
