# As regras de transferência por nação (transfer/*_rules.cpp), parte 7: Argentina, Portugal, Coreia do Sul. 1 chamada cada até 2002-06-01, na criação do
# jogo (a fábrica das regras 0x8d2xxx). Nenhuma é liga da carreira de 8 ligas, mas as regras do vendedor também contam:
# a data de uma transferência anda pelas regras da nação do comprador e, com vendedor, da do vendedor (0x8cf0e0,
# TRANSFER_MANAGER::find_next_allowed_transfer_date): um clube das 8 ligas que compra de um clube destas nações espera a
# janela delas. As mutações estão nessas janelas.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - Argentina (0x40a620): o construtor das regras dela, depois da base (0x8e84a0) e da vtable: mov ecx, esi; call (sem
#   push) -> thiscall(this), ret 0.
# - Portugal (0x7d3df0): o construtor das regras dela, depois da base (0x8e84a0) e da vtable: mov ecx, esi; call (sem
#   push) -> thiscall(this), ret 0.
# - Coreia do Sul (0x66e630): a fábrica das regras (0x8d2xxx): push 0; push 1; push continente (edi); push id da nação; push 27 (a regra);
#   mov ecx, objeto (o new) -> thiscall(this, regra, nação, continente, 1, 0), ret 0x14.
# - dentro: 0x8e84a0 (5 push com os argumentos; mov ecx, esi) -> thiscall, ret 0x14 (original); 0x944e46 push tamanho;
#   add esp, 4 -> cdecl; a caixa (aviso_em).
# Comparadores: lado a lado à mão (a original grava o objeto de quem chama e aloca as janelas; a cópia roda num
# objeto de rascunho com os mesmos 0x19 bytes de antes): iguais os campos fora o ponteiro +4 e as janelas alocadas
# (as do rascunho são soltas).
# Determinismo: só constantes; nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('janelas_argentina', 0x40a620, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('janelas_portugal', 0x7d3df0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('regras_coreia', 0x66e630, [0x6a, 0xff, 0x68, 0x9b, 0xc8, 0x95, 0x00])]
DOCS = ('as regras de transferência da Argentina, de Portugal e da Coreia do Sul: as janelas de transferência',
        'the transfer rules of Argentina, Portugal and South Korea: the transfer windows',
        [('janelas_argentina', 'ARGENTINA_RULES::init_transfer_windows', '0x40a620', 'argentina_rules.cpp', 'thiscall, ret 0', 'a regra da Argentina (1): duas janelas, de 24 de julho (andando para o domingo [inferência]) a 6 de agosto (andando para a quarta-feira [inferência]); de 14 de dezembro (andando para o domingo [inferência]) a 4 de março (andando para a quarta-feira [inferência])', 'the Argentina rule (1): two windows, from 24 July (moved to the Sunday [inference]) to 6 August (moved to the Wednesday [inference]); from 14 December (moved to the Sunday [inference]) to 4 March (moved to the Wednesday [inference])'),
         ('janelas_portugal', 'PORTUGAL_RULES::init_transfer_windows', '0x7d3df0', 'portugal_rules.cpp', 'thiscall, ret 0', 'a regra de Portugal (19): duas janelas, de 1º de junho a 1º de setembro; de 15 de dezembro a 15 de janeiro', 'the Portugal rule (19): two windows, from 1 June to 1 September; from 15 December to 15 January'),
         ('regras_coreia', 'KOREA_RULES::KOREA_RULES', '0x66e630', 'korea_rules.cpp', 'thiscall, ret 0x14', 'a regra da Coreia do Sul (27): uma janela, de 25 de novembro (andando para o domingo [inferência]) a 25 de julho; o +0x13 = 7 (o padrão da base é 50) [inferência: o limite de estrangeiros]', 'the South Korea rule (27): one window, from 25 November (moved to the Sunday [inference]) to 25 July; +0x13 = 7 (base default 50) [inference: the foreign limit]')])
CAMPOS = [('regras de transferência +0 vtable, +4 janelas (datas de 6 bytes, duas por janela), +8 número de janelas, +9 a regra, +0xa a nação, +0xe o continente, +0x12, +0x13 (padrão 50) [inferência: limite de estrangeiros], +0x14 (padrão 30), +0x15; ponteiro e valor; grava (todas)', 'transfer rules +0 vtable, +4 windows (6-byte dates, two per window), +8 number of windows, +9 the rule, +0xa nation, +0xe continent, +0x12, +0x13 (default 50) [inference: foreign limit], +0x14 (default 30), +0x15; pointer and value; writes (all)')]
INTERESSA = ['CMRTE', 'SGE']
