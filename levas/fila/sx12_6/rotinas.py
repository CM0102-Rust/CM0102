# O registro do tecnico humano zerado e as competicoes das nacoes-chave criadas (human_manager.cpp, key_nation.cpp;
# leitor sx12, rodada 5, 23/11/2026). Rodam na criacao do jogo, que a carreira de teste faz do zero a cada execucao:
# HUMAN_MANAGER::reset_human_record 0x5fb810 (17 chamadas: os 16 registros do construtor 0x5fb430 e o tecnico da
# carreira) e key_nation_create_all_comps 0x669180 (passo de carga 0x1f, setup.rs).
# Convencoes conferidas na desmontagem:
# - reset_human_record: thiscall(tecnico), ret 4, eax 1 ou 0; quadro com SEH so pelos locais (STAFF_CONTRACT 0x85e410 e
#   o bloco da vista do elenco 0x487810, ambos thiscall, construidos antes de conferir o tecnico); 0x90bea0
#   cdecl(nome, saida) monta o caminho da pasta (cria a pasta: roda nos dois lados do comparador, e idempotente);
#   0x896570 Tactics::get_tactic_block thiscall([0xde1d08], 0, 0) ret 8; 0x9466d0 strncpy cdecl(destino, origem,
#   0x104); 0x8e5710 ~TRANSFER_OFFER thiscall; 0x944c94 operator delete; 0x417c10 Date::operator= thiscall(tmp, origem)
#   ret 8. Os campos de bits de +0x35 e +0x4f do contrato sao copiados por mascaras que, somadas, copiam o byte
#   inteiro. Ordem das pastas pelos deslocamentos de pilha: "tactics" em cada vaga (+0x18f1), "search" em +0x9ba8,
#   "training" em +0xa0bc, "print" em +0xa5d0.
# - key_nation_create_all_comps: cdecl, eax 1 ou 0 (caixa linha 0x1dc); a funcao de criacao de cada linha cdecl(linha);
#   0x549f70 Date::Date thiscall(dia, mes, ano, dia da semana) ret 0x10: a original empilha as palavras com a metade
#   alta do registro que sobrou; o Date::Date le dia, ano e dia da semana em 16 bits e o mes em 8 (conferido ate o
#   0x54a0f2; o 0x54a2d0 recebe o eax inteiro do dia da semana e nao foi conferido).
SAVE = False
ROTINAS = [('reset_human_record', 0x5fb810, [0x6a, 0xff, 0x68, 0x16, 0xb0, 0x95, 0x00]),
           ('key_nation_create_all_comps', 0x669180, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00])]
DOCS = ('o registro do tecnico humano zerado e as competicoes das nacoes-chave criadas',
        "a human manager's record reset and the key nation competitions created",
        [('reset_human_record', 'HUMAN_MANAGER::reset_human_record', '0x5fb810', 'human_manager.cpp', 'thiscall, ret 4',
          'zera o registro do tecnico humano (o atual, se nulo): as cinco vagas de tatica com a tatica pre-definida 0, a tela de taticas zerada, a pasta de taticas e sem arquivos recentes; desconta o tecnico dos contadores de controle pelo computador e de ferias; destroi as duas propostas pendentes; um contrato novo no rascunho, sem ferias, sem opcoes, senha vazia, narracao lenta (0x7d), sem partida assistida, as pastas de busca, treino e impressao sem arquivos recentes, a vista padrao do elenco e sem apelidos (o vetor de apelidos e largado sem ser liberado); devolve 1, ou 0 depois da caixa "Error" (sem tecnico ou fora da tabela)',
          "resets the human manager's record (the current one when null): the five tactic slots with the preset tactic 0, the tactics screen cleared, the tactics folder and no recent files; takes the manager off the computer control and holiday counts; destroys the two pending offers; a new draft contract, no holiday, no options, empty password, slow commentary (0x7d), no watched fixture, the search, training and print folders with no recent files, the default squad view and no nicknames (the nickname array is dropped without being freed); returns 1, or 0 after the \"Error\" box (no manager or out of range)"),
         ('key_nation_create_all_comps', 'key_nation_create_all_comps', '0x669180', 'key_nation.cpp', 'cdecl',
          'cria as competicoes de cada linha ativa da tabela das nacoes-chave (sem nacao ou com a nacao escolhida, e com funcao de criacao) e poe na linha as datas de inicio e fim da temporada, montadas do dia, mes, ano e dia da semana guardados nela; uma criacao que falha da a caixa "Error" e 0; senao 1',
          'creates the competitions of each active key nation table row (no nation or a chosen one, and with a creation function) and sets the row\'s season start and end dates, built from the day, month, year and weekday stored in it; a failed creation gives the "Error" box and 0; otherwise 1')])
