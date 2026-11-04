# As regras de transferência por nação (transfer/*_rules.cpp), parte 2: Espanha, Itália e França (as que montam as
# janelas numa rotina própria, `init_transfer_windows`, chamada pelo construtor depois da base e da vtable). 1 chamada
# cada até 2002-06-01 (docs/cobertura_competicoes.txt).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama: os construtores das regras (Espanha 0x85944d, Itália 0x6613cd, França 0x5c15bd: mov ecx, esi; sem push
#   logo antes) -> thiscall(this), ret 0.
# - dentro das inits: 0x944e46 push tamanho; add esp, 4 -> cdecl; a caixa (aviso_em).
# Comparadores: lado a lado à mão (a original grava o objeto de quem chama e aloca as janelas; a cópia roda num
# objeto de rascunho com os mesmos 0x19 bytes de antes): iguais os campos fora o ponteiro +4 e as janelas alocadas
# (as do rascunho são soltas). Rodam uma vez, na criação do jogo (a fábrica das regras 0x8d2xxx).
# Determinismo: só constantes; nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('janelas_espanha', 8754288, [129, 236, 0, 2, 0, 0]),
           ('janelas_italia', 6689776, [129, 236, 0, 2, 0, 0]),
           ('janelas_franca', 6034912, [129, 236, 0, 2, 0, 0])]
DOCS = ('as regras de transferência da Espanha, da Itália e da França: as janelas de transferência',
        'the transfer rules of Spain, Italy and France: the transfer windows',
        [('janelas_espanha', 'SPAIN_RULES::init_transfer_windows', '0x859470', 'spain_rules.cpp', 'thiscall, ret 0', 'a regra espanhola (22): duas janelas, de 17 de junho (andando para a quarta-feira [inferência]) a 20 de agosto (andando para a quinta-feira [inferência]) e de 15 de dezembro a 31 de janeiro', 'the Spanish rule (22): two windows, from 17 June (moved to the Wednesday [inference]) to 20 August (moved to the Thursday [inference]) and from 15 December to 31 January'),
         ('janelas_italia', 'ITALY_RULES::init_transfer_windows', '0x6613f0', 'italy_rules.cpp', 'thiscall, ret 0', 'a regra italiana (14): três janelas, de 1º de julho a 27 de setembro (andando para a quinta-feira [inferência]), de 2 a 31 de janeiro, e de 1º de julho a 30 de abril [inferência: a dos jogadores sem contrato]', 'the Italian rule (14): three windows, from 1 July to 27 September (moved to the Thursday [inference]), from 2 to 31 January, and from 1 July to 30 April [inference: the free-agent one]'),
         ('janelas_franca', 'FRANCE_RULES::init_transfer_windows', '0x5c15e0', 'france_rules.cpp', 'thiscall, ret 0', 'a regra francesa (9): três janelas, de 30 de maio a 31 de agosto, de 1º de setembro a 20 de dezembro e de 21 de dezembro a 10 de janeiro (as datas andam para dias da semana [inferência])', 'the French rule (9): three windows, from 30 May to 31 August, from 1 September to 20 December and from 21 December to 10 January (the dates move to weekdays [inference])')])
CAMPOS = [('regras de transferência +0 vtable, +4 janelas (datas de 6 bytes, duas por janela), +8 número de janelas, +9 a regra, +0xa a nação, +0xe o continente, +0x12, +0x13 (padrão 50) [inferência: limite de estrangeiros], +0x14 (padrão 30), +0x15; ponteiro e valor; grava (todas)', 'transfer rules +0 vtable, +4 windows (6-byte dates, two per window), +8 number of windows, +9 the rule, +0xa nation, +0xe continent, +0x12, +0x13 (default 50) [inference: foreign limit], +0x14 (default 30), +0x15; pointer and value; writes (all)')]
INTERESSA = ['CMRTE', 'SGE']
