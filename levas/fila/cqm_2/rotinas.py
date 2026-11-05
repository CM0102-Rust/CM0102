# As regras de transferência por nação (transfer/*_rules.cpp), parte 5: Noruega, Irlanda do Norte, País de Gales. 1 chamada cada até 2002-06-01, na criação do
# jogo (a fábrica das regras 0x8d2xxx). Nenhuma é liga da carreira de 8 ligas, mas as regras do vendedor também contam:
# a data de uma transferência anda pelas regras da nação do comprador e, com vendedor, da do vendedor (0x8cf0e0,
# TRANSFER_MANAGER::find_next_allowed_transfer_date): um clube das 8 ligas que compra de um clube destas nações espera a
# janela delas. As mutações estão nessas janelas.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - Noruega (0x7944f0): a fábrica das regras (0x8d2xxx): push 0; push 1; push continente (edi); push id da nação; push 17 (a regra);
#   mov ecx, objeto (o new) -> thiscall(this, regra, nação, continente, 1, 0), ret 0x14.
# - Irlanda do Norte (0x7936d0): a fábrica das regras (0x8d2xxx): push 0; push 1; push continente (edi); push id da nação; push 16 (a regra);
#   mov ecx, objeto (o new) -> thiscall(this, regra, nação, continente, 1, 0), ret 0x14.
# - País de Gales (0x90f080): a fábrica das regras (0x8d2xxx): push 0; push 1; push continente (edi); push id da nação; push 26 (a regra);
#   mov ecx, objeto (o new) -> thiscall(this, regra, nação, continente, 1, 0), ret 0x14.
# - dentro: 0x8e84a0 (5 push com os argumentos; mov ecx, esi) -> thiscall, ret 0x14 (original); 0x944e46 push tamanho;
#   add esp, 4 -> cdecl; a caixa (aviso_em).
# Comparadores: lado a lado à mão (a original grava o objeto de quem chama e aloca as janelas; a cópia roda num
# objeto de rascunho com os mesmos 0x19 bytes de antes): iguais os campos fora o ponteiro +4 e as janelas alocadas
# (as do rascunho são soltas).
# Determinismo: só constantes; nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('regras_noruega', 0x7944f0, [0x6a, 0xff, 0x68, 0x2b, 0xe9, 0x95, 0x00]),
           ('regras_irlanda_do_norte', 0x7936d0, [0x6a, 0xff, 0x68, 0xbb, 0xe7, 0x95, 0x00]),
           ('regras_pais_de_gales', 0x90f080, [0x6a, 0xff, 0x68, 0x4b, 0x56, 0x96, 0x00])]
DOCS = ('as regras de transferência da Noruega, da Irlanda do Norte e do País de Gales: as janelas de transferência',
        'the transfer rules of Norway, Northern Ireland and Wales: the transfer windows',
        [('regras_noruega', 'NORWAY_RULES::NORWAY_RULES', '0x7944f0', 'norway_rules.cpp', 'thiscall, ret 0x14', 'a regra da Noruega (17): uma janela, de 1º de setembro a 31 de agosto', 'the Norway rule (17): one window, from 1 September to 31 August'),
         ('regras_irlanda_do_norte', 'NORTHERN_IRELAND_RULES::NORTHERN_IRELAND_RULES', '0x7936d0', 'northern_ireland_rules.cpp', 'thiscall, ret 0x14', 'a regra da Irlanda do Norte (16): uma janela, de 6 de maio (andando para o sábado [inferência]) a 20 de março', 'the Northern Ireland rule (16): one window, from 6 May (moved to the Saturday [inference]) to 20 March'),
         ('regras_pais_de_gales', 'WALES_RULES::WALES_RULES', '0x90f080', 'wales_rules.cpp', 'thiscall, ret 0x14', 'a regra do País de Gales (26): uma janela, de 7 de maio (andando para o domingo [inferência]) a 20 de março', 'the Wales rule (26): one window, from 7 May (moved to the Sunday [inference]) to 20 March')])
CAMPOS = [('regras de transferência +0 vtable, +4 janelas (datas de 6 bytes, duas por janela), +8 número de janelas, +9 a regra, +0xa a nação, +0xe o continente, +0x12, +0x13 (padrão 50) [inferência: limite de estrangeiros], +0x14 (padrão 30), +0x15; ponteiro e valor; grava (todas)', 'transfer rules +0 vtable, +4 windows (6-byte dates, two per window), +8 number of windows, +9 the rule, +0xa nation, +0xe continent, +0x12, +0x13 (default 50) [inference: foreign limit], +0x14 (default 30), +0x15; pointer and value; writes (all)')]
INTERESSA = ['CMRTE', 'SGE']
