# As regras de transferência por nação (transfer/*_rules.cpp), parte 6: Turquia, Polônia, Finlândia. 1 chamada cada até 2002-06-01, na criação do
# jogo (a fábrica das regras 0x8d2xxx). Nenhuma é liga da carreira de 8 ligas, mas as regras do vendedor também contam:
# a data de uma transferência anda pelas regras da nação do comprador e, com vendedor, da do vendedor (0x8cf0e0,
# TRANSFER_MANAGER::find_next_allowed_transfer_date): um clube das 8 ligas que compra de um clube destas nações espera a
# janela delas. As mutações estão nessas janelas.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - Turquia (0x9020a0): a fábrica das regras (0x8d2xxx): push 0; push 1; push continente (edi); push id da nação; push 24 (a regra);
#   mov ecx, objeto (o new) -> thiscall(this, regra, nação, continente, 1, 0), ret 0x14.
# - Polônia (0x7cd830): a fábrica das regras (0x8d2xxx): push 0; push 1; push continente (edi); push id da nação; push 18 (a regra);
#   mov ecx, objeto (o new) -> thiscall(this, regra, nação, continente, 1, 0), ret 0x14.
# - Finlândia (0x5a9950): a fábrica das regras (0x8d2xxx): push 0; push 1; push continente (edi); push id da nação; push 8 (a regra);
#   mov ecx, objeto (o new) -> thiscall(this, regra, nação, continente, 1, 0), ret 0x14.
# - dentro: 0x8e84a0 (5 push com os argumentos; mov ecx, esi) -> thiscall, ret 0x14 (original); 0x944e46 push tamanho;
#   add esp, 4 -> cdecl; a caixa (aviso_em).
# Comparadores: lado a lado à mão (a original grava o objeto de quem chama e aloca as janelas; a cópia roda num
# objeto de rascunho com os mesmos 0x19 bytes de antes): iguais os campos fora o ponteiro +4 e as janelas alocadas
# (as do rascunho são soltas).
# Determinismo: só constantes; nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('regras_turquia', 0x9020a0, [0x6a, 0xff, 0x68, 0xab, 0x51, 0x96, 0x00]),
           ('regras_polonia', 0x7cd830, [0x6a, 0xff, 0x68, 0xab, 0xf9, 0x95, 0x00]),
           ('regras_finlandia', 0x5a9950, [0x6a, 0xff, 0x68, 0x4b, 0x95, 0x95, 0x00])]
DOCS = ('as regras de transferência da Turquia, da Polônia e da Finlândia: as janelas de transferência',
        'the transfer rules of Turkey, Poland and Finland: the transfer windows',
        [('regras_turquia', 'TURKEY_RULES::TURKEY_RULES', '0x9020a0', 'turkey_rules.cpp', 'thiscall, ret 0x14', 'a regra da Turquia (24): uma janela, de 1º de junho a 31 de janeiro; o +0x13 = 8 (o padrão da base é 50) [inferência: o limite de estrangeiros]; o +0x14 = 8 (o padrão é 30)', 'the Turkey rule (24): one window, from 1 June to 31 January; +0x13 = 8 (base default 50) [inference: the foreign limit]; +0x14 = 8 (default 30)'),
         ('regras_polonia', 'POLAND_RULES::POLAND_RULES', '0x7cd830', 'poland_rules.cpp', 'thiscall, ret 0x14', 'a regra da Polônia (18): uma janela, de 1º de julho a 20 de fevereiro; o +0x13 = 5 (o padrão da base é 50) [inferência: o limite de estrangeiros]; o +0x14 = 5 (o padrão é 30)', 'the Poland rule (18): one window, from 1 July to 20 February; +0x13 = 5 (base default 50) [inference: the foreign limit]; +0x14 = 5 (default 30)'),
         ('regras_finlandia', 'FINLAND_RULES::FINLAND_RULES', '0x5a9950', 'finland_rules.cpp', 'thiscall, ret 0x14', 'a regra da Finlândia (8): uma janela, de 16 de novembro a 15 de agosto; o dword +0x15 = 1', 'the Finland rule (8): one window, from 16 November to 15 August; dword +0x15 = 1')])
CAMPOS = [('regras de transferência +0 vtable, +4 janelas (datas de 6 bytes, duas por janela), +8 número de janelas, +9 a regra, +0xa a nação, +0xe o continente, +0x12, +0x13 (padrão 50) [inferência: limite de estrangeiros], +0x14 (padrão 30), +0x15; ponteiro e valor; grava (todas)', 'transfer rules +0 vtable, +4 windows (6-byte dates, two per window), +8 number of windows, +9 the rule, +0xa nation, +0xe continent, +0x12, +0x13 (default 50) [inference: foreign limit], +0x14 (default 30), +0x15; pointer and value; writes (all)')]
INTERESSA = ['CMRTE', 'SGE']
