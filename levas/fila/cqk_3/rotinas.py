# As regras de transferência por nação (transfer/*_rules.cpp), parte 3: Brasil, Alemanha e Escócia. 1 chamada cada até
# 2002-06-01. O Brasil é liga da carreira de 8 ligas (as 3 mutações estão nele); a Alemanha e a Escócia não são
# (passageiras: as janelas delas só valem para os clubes de lá).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama os construtores: a fábrica das regras 0x8d280c (Brasil, regra 4, [0x9cf260]) e 0x8d2c7f (Escócia, regra
#   0x15, [0x9cf45c]):
#   push 0; push 1; push continente (edi); push id da nação ([chave]); push a regra; mov ecx, objeto (o new) -> thiscall(this, regra, nação, continente, 1, 0), ret 0x14.
# - quem chama a 0x5dffe0: o construtor das regras da Alemanha (0x5dffbd: mov ecx, esi; sem push) -> thiscall, ret 0.
# - dentro dos construtores: 0x8e84a0 (5 push com os argumentos; mov ecx, esi) -> thiscall, ret 0x14 (original, só grava o objeto: +0 a vtable da base, +4, +9 a regra, +0xa a nação, +0xe o continente, +0x12, +0x13 = 50, +0x14 = 30, +0x15); 0x944e46 push tamanho; add esp, 4 -> cdecl; a caixa (aviso_em).
# Comparadores: lado a lado à mão (a original grava o objeto de quem chama e aloca as janelas; a cópia roda num
# objeto de rascunho com os mesmos 0x19 bytes de antes): iguais os campos fora o ponteiro +4 e as janelas alocadas
# (as do rascunho são soltas). Rodam uma vez, na criação do jogo (a fábrica das regras 0x8d2xxx).
# Determinismo: só constantes; nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('regras_brasil', 4452576, [106, 255, 104, 219, 78, 149, 0]),
           ('janelas_alemanha', 6160352, [129, 236, 0, 2, 0, 0]),
           ('regras_escocia', 8348560, [106, 255, 104, 235, 14, 150, 0])]
DOCS = ('as regras de transferência do Brasil, da Alemanha e da Escócia: as janelas de transferência',
        'the transfer rules of Brazil, Germany and Scotland: the transfer windows',
        [('regras_brasil', 'BRAZIL_RULES::BRAZIL_RULES', '0x43f0e0', 'brazil_rules.cpp', 'thiscall, ret 0x14', 'a regra brasileira (4): uma janela, de 11 de dezembro (andando para a quarta-feira [inferência]) a 8 de novembro (andando para a sexta-feira [inferência]): fechada só de novembro a dezembro; o +0x13 = 3 (o padrão é 50) [inferência: o limite de estrangeiros contratados]', 'the Brazilian rule (4): one window, from 11 December (moved to the Wednesday [inference]) to 8 November (moved to the Friday [inference]): closed only from November to December; +0x13 = 3 (default 50) [inference: the foreign signing limit]'),
         ('janelas_alemanha', 'GERMANY_RULES::init_transfer_windows', '0x5dffe0', 'germany_rules.cpp', 'thiscall, ret 0', 'a regra alemã (10): duas janelas, de 1º de julho a 15 de agosto e de 1º de julho a 15 de janeiro [inferência: a segunda para os jogadores sem contrato]', 'the German rule (10): two windows, from 1 July to 15 August and from 1 July to 15 January [inference: the second for free agents]'),
         ('regras_escocia', 'SCOTLAND_RULES::SCOTLAND_RULES', '0x7f6390', 'scotland_rules.cpp', 'thiscall, ret 0x14', 'a regra escocesa (21): uma janela, de 30 de maio (andando para o sábado [inferência]) a 30 de março (andando para a quinta-feira [inferência]); o dword +0x15 = 1', 'the Scottish rule (21): one window, from 30 May (moved to the Saturday [inference]) to 30 March (moved to the Thursday [inference]); dword +0x15 = 1')])
CAMPOS = [('regras de transferência +0 vtable, +4 janelas (datas de 6 bytes, duas por janela), +8 número de janelas, +9 a regra, +0xa a nação, +0xe o continente, +0x12, +0x13 (padrão 50) [inferência: limite de estrangeiros], +0x14 (padrão 30), +0x15; ponteiro e valor; grava (todas)', 'transfer rules +0 vtable, +4 windows (6-byte dates, two per window), +8 number of windows, +9 the rule, +0xa nation, +0xe continent, +0x12, +0x13 (default 50) [inference: foreign limit], +0x14 (default 30), +0x15; pointer and value; writes (all)')]
INTERESSA = ['CMRTE', 'SGE']
