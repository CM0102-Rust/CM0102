# As regras de transferência por nação (transfer/*_rules.cpp), parte 1: Inglaterra, EUA e Austrália (as três ligas
# da carreira de 8 ligas que constroem as regras num construtor próprio). Cada construtor chama a base (0x8e84a0), põe a
# vtable da nação e aloca as janelas de transferência (datas de 6 bytes: a regra, o número da janela, o dia da semana
# para o qual a data anda [inferência], o dia, o mês e 1 no início / 0 no fim). 1 chamada cada até 2002-06-01.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama: a fábrica das regras 0x8d28d5 (Inglaterra, regra 7, [0x9cf2e4]), 0x8d2d8b (EUA, regra 0x19,
#   [0x9cf4f8]) e 0x8d2786 (Austrália, regra 2, [0x9cf224]):
#   push 0; push 1; push continente (edi); push id da nação ([chave]); push a regra; mov ecx, objeto (o new) -> thiscall(this, regra, nação, continente, 1, 0), ret 0x14.
# - dentro dos construtores: 0x8e84a0 (5 push com os argumentos; mov ecx, esi) -> thiscall, ret 0x14 (original, só grava o objeto: +0 a vtable da base, +4, +9 a regra, +0xa a nação, +0xe o continente, +0x12, +0x13 = 50, +0x14 = 30, +0x15); 0x944e46 push tamanho; add esp, 4 -> cdecl; a caixa (aviso_em).
# Comparadores: lado a lado à mão (a original grava o objeto de quem chama e aloca as janelas; a cópia roda num
# objeto de rascunho com os mesmos 0x19 bytes de antes): iguais os campos fora o ponteiro +4 e as janelas alocadas
# (as do rascunho são soltas). Rodam uma vez, na criação do jogo (a fábrica das regras 0x8d2xxx).
# Determinismo: só constantes; nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('regras_inglaterra', 5741360, [106, 255, 104, 235, 134, 149, 0]),
           ('regras_eua', 9484608, [106, 255, 104, 59, 85, 150, 0]),
           ('regras_australia', 4271232, [106, 255, 104, 27, 59, 149, 0])]
DOCS = ('as regras de transferência da Inglaterra, dos EUA e da Austrália: as janelas de transferência',
        'the transfer rules of England, the USA and Australia: the transfer windows',
        [('regras_inglaterra', 'ENGLAND_RULES::ENGLAND_RULES', '0x579b30', 'england_rules.cpp', 'thiscall, ret 0x14', 'a regra inglesa (7): uma janela de transferência, de 2 de junho (andando para a segunda-feira [inferência]) a 26 de março (andando para a quinta-feira [inferência]); as outras regras da Inglaterra (estrangeiros fora da UE, empréstimos: 5 por temporada e 2 de uma vez, a mesma liga) ficam nos métodos da vtable 0x969f4c', "the English rule (7): one transfer window, from 2 June (moved to the Monday [inference]) to 26 March (moved to the Thursday [inference]); England's other rules (non-EU players, loans: 5 a season and 2 at once, same league) live in the vtable 0x969f4c methods"),
         ('regras_eua', 'USA_RULES::USA_RULES', '0x90b940', 'usa_rules.cpp', 'thiscall, ret 0x14', 'a regra dos EUA (25): uma janela, de 7 de janeiro (andando para a quarta-feira [inferência]) a 15 de agosto; o +0x13 = 4 (o padrão é 50) [inferência: o limite de estrangeiros no elenco]', 'the USA rule (25): one window, from 7 January (moved to the Wednesday [inference]) to 15 August; +0x13 = 4 (default 50) [inference: the foreign squad limit]'),
         ('regras_australia', 'AUSTRALIA_RULES::Australia_rules', '0x412c80', 'australia_rules.cpp', 'thiscall, ret 0x14', 'a regra da Austrália (2): uma janela, de 11 de junho (andando para o sábado [inferência]) a 17 de abril; o +0x13 = 6 [inferência: o limite de estrangeiros no elenco]', 'the Australian rule (2): one window, from 11 June (moved to the Saturday [inference]) to 17 April; +0x13 = 6 [inference: the foreign squad limit]')])
CAMPOS = [('regras de transferência +0 vtable, +4 janelas (datas de 6 bytes, duas por janela), +8 número de janelas, +9 a regra, +0xa a nação, +0xe o continente, +0x12, +0x13 (padrão 50) [inferência: limite de estrangeiros], +0x14 (padrão 30), +0x15; ponteiro e valor; grava (todas)', 'transfer rules +0 vtable, +4 windows (6-byte dates, two per window), +8 number of windows, +9 the rule, +0xa nation, +0xe continent, +0x12, +0x13 (default 50) [inference: foreign limit], +0x14 (default 30), +0x15; pointer and value; writes (all)')]
INTERESSA = ['CMRTE', 'SGE']
