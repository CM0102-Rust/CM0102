# As regras de transferência por nação (transfer/*_rules.cpp), parte 8: Grécia, Rússia, Bélgica. 1 chamada cada até 2002-06-01, na criação do
# jogo (a fábrica das regras 0x8d2xxx). Nenhuma é liga da carreira de 8 ligas, mas as regras do vendedor também contam:
# a data de uma transferência anda pelas regras da nação do comprador e, com vendedor, da do vendedor (0x8cf0e0,
# TRANSFER_MANAGER::find_next_allowed_transfer_date): um clube das 8 ligas que compra de um clube destas nações espera a
# janela delas. As mutações estão nessas janelas.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - Grécia (0x5ecf20): o construtor das regras dela, depois da base (0x8e84a0) e da vtable: mov ecx, esi; call (sem
#   push) -> thiscall(this), ret 0.
# - Rússia (0x7ec7e0): o construtor das regras dela, depois da base (0x8e84a0) e da vtable: mov ecx, esi; call (sem
#   push) -> thiscall(this), ret 0.
# - Bélgica (0x424c00): o construtor das regras dela, depois da base (0x8e84a0) e da vtable: mov ecx, esi; call (sem
#   push) -> thiscall(this), ret 0.
# - dentro: 0x8e84a0 (5 push com os argumentos; mov ecx, esi) -> thiscall, ret 0x14 (original); 0x944e46 push tamanho;
#   add esp, 4 -> cdecl; a caixa (aviso_em).
# Comparadores: lado a lado à mão (a original grava o objeto de quem chama e aloca as janelas; a cópia roda num
# objeto de rascunho com os mesmos 0x19 bytes de antes): iguais os campos fora o ponteiro +4 e as janelas alocadas
# (as do rascunho são soltas).
# Determinismo: só constantes; nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('janelas_grecia', 0x5ecf20, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('janelas_russia', 0x7ec7e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('janelas_belgica', 0x424c00, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as regras de transferência da Grécia, da Rússia e da Bélgica: as janelas de transferência',
        'the transfer rules of Greece, Russia and Belgium: the transfer windows',
        [('janelas_grecia', 'GREECE_RULES::init_transfer_windows', '0x5ecf20', 'greece_rules.cpp', 'thiscall, ret 0', 'a regra da Grécia (11): duas janelas, de 22 de maio (andando para o domingo [inferência]) a 29 de agosto (andando para o sábado [inferência]); de 1º de janeiro a 20 de janeiro', 'the Greece rule (11): two windows, from 22 May (moved to the Sunday [inference]) to 29 August (moved to the Saturday [inference]); from 1 January to 20 January'),
         ('janelas_russia', 'RUSSIA_RULES::init_transfer_windows', '0x7ec7e0', 'russia_rules.cpp', 'thiscall, ret 0', 'a regra da Rússia (20): duas janelas, de 15 de dezembro a 24 de março (andando para a quinta-feira [inferência]); de 28 de junho (andando para a terça-feira [inferência]) a 4 de agosto', 'the Russia rule (20): two windows, from 15 December to 24 March (moved to the Thursday [inference]); from 28 June (moved to the Tuesday [inference]) to 4 August'),
         ('janelas_belgica', 'BELGIUM_RULES::init_transfer_windows', '0x424c00', 'belgium_rules.cpp', 'thiscall, ret 0', 'a regra da Bélgica (3): três janelas, de 1º de junho a 30 de junho; de 1º de junho a 31 de dezembro; de 1º de junho a 31 de março', 'the Belgium rule (3): three windows, from 1 June to 30 June; from 1 June to 31 December; from 1 June to 31 March')])
CAMPOS = [('regras de transferência +0 vtable, +4 janelas (datas de 6 bytes, duas por janela), +8 número de janelas, +9 a regra, +0xa a nação, +0xe o continente, +0x12, +0x13 (padrão 50) [inferência: limite de estrangeiros], +0x14 (padrão 30), +0x15; ponteiro e valor; grava (todas)', 'transfer rules +0 vtable, +4 windows (6-byte dates, two per window), +8 number of windows, +9 the rule, +0xa nation, +0xe continent, +0x12, +0x13 (default 50) [inference: foreign limit], +0x14 (default 30), +0x15; pointer and value; writes (all)')]
INTERESSA = ['CMRTE', 'SGE']
