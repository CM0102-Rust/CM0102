# As regras de transferência por nação (transfer/*_rules.cpp), parte 4: Holanda, Croácia, Irlanda. 1 chamada cada até 2002-06-01, na criação do
# jogo (a fábrica das regras 0x8d2xxx). Nenhuma é liga da carreira de 8 ligas, mas as regras do vendedor também contam:
# a data de uma transferência anda pelas regras da nação do comprador e, com vendedor, da do vendedor (0x8cf0e0,
# TRANSFER_MANAGER::find_next_allowed_transfer_date): um clube das 8 ligas que compra de um clube destas nações espera a
# janela delas. As mutações estão nessas janelas.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - Holanda (0x5f9690): a fábrica das regras (0x8d2xxx): push 0; push 1; push continente (edi); push id da nação; push 12 (a regra);
#   mov ecx, objeto (o new) -> thiscall(this, regra, nação, continente, 1, 0), ret 0x14.
# - Croácia (0x518500): a fábrica das regras (0x8d2xxx): push 0; push 1; push continente (edi); push id da nação; push 5 (a regra);
#   mov ecx, objeto (o new) -> thiscall(this, regra, nação, continente, 1, 0), ret 0x14.
# - Irlanda (0x63c210): a fábrica das regras (0x8d2xxx): push 0; push 1; push continente (edi); push id da nação; push 13 (a regra);
#   mov ecx, objeto (o new) -> thiscall(this, regra, nação, continente, 1, 0), ret 0x14.
# - dentro: 0x8e84a0 (5 push com os argumentos; mov ecx, esi) -> thiscall, ret 0x14 (original); 0x944e46 push tamanho;
#   add esp, 4 -> cdecl; a caixa (aviso_em).
# Comparadores: lado a lado à mão (a original grava o objeto de quem chama e aloca as janelas; a cópia roda num
# objeto de rascunho com os mesmos 0x19 bytes de antes): iguais os campos fora o ponteiro +4 e as janelas alocadas
# (as do rascunho são soltas).
# Determinismo: só constantes; nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('regras_holanda', 0x5f9690, [0x6a, 0xff, 0x68, 0x3b, 0xaf, 0x95, 0x00]),
           ('regras_croacia', 0x518500, [0x6a, 0xff, 0x68, 0xbb, 0x75, 0x95, 0x00]),
           ('regras_irlanda', 0x63c210, [0x6a, 0xff, 0x68, 0xab, 0xb7, 0x95, 0x00])]
DOCS = ('as regras de transferência da Holanda, da Croácia e da Irlanda: as janelas de transferência',
        'the transfer rules of Holland, Croatia and Ireland: the transfer windows',
        [('regras_holanda', 'HOLLAND_RULES::HOLLAND_RULES', '0x5f9690', 'holland_rules.cpp', 'thiscall, ret 0x14', 'a regra da Holanda (12): uma janela, de 8 de junho (andando para o sábado [inferência]) a 4 de abril (andando para a sexta-feira [inferência]); o +0x13 = 50 (o padrão da base é 50) [inferência: o limite de estrangeiros]; o +0x14 = 50 (o padrão é 30)', 'the Holland rule (12): one window, from 8 June (moved to the Saturday [inference]) to 4 April (moved to the Friday [inference]); +0x13 = 50 (base default 50) [inference: the foreign limit]; +0x14 = 50 (default 30)'),
         ('regras_croacia', 'CROATIA_RULES::CROATIA_RULES', '0x518500', 'croatia_rules.cpp', 'thiscall, ret 0x14', 'a regra da Croácia (5): uma janela, de 1º de junho a 30 de março', 'the Croatia rule (5): one window, from 1 June to 30 March'),
         ('regras_irlanda', 'IRELAND_RULES::IRELAND_RULES', '0x63c210', 'ireland_rules.cpp', 'thiscall, ret 0x14', 'a regra da Irlanda (13): uma janela, de 10 de maio a 31 de janeiro', 'the Ireland rule (13): one window, from 10 May to 31 January')])
CAMPOS = [('regras de transferência +0 vtable, +4 janelas (datas de 6 bytes, duas por janela), +8 número de janelas, +9 a regra, +0xa a nação, +0xe o continente, +0x12, +0x13 (padrão 50) [inferência: limite de estrangeiros], +0x14 (padrão 30), +0x15; ponteiro e valor; grava (todas)', 'transfer rules +0 vtable, +4 windows (6-byte dates, two per window), +8 number of windows, +9 the rule, +0xa nation, +0xe continent, +0x12, +0x13 (default 50) [inference: foreign limit], +0x14 (default 30), +0x15; pointer and value; writes (all)')]
INTERESSA = ['CMRTE', 'SGE']
