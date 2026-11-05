# A tabela das ligas do Japão e da Austrália (o slot create_table de jap_j1.cpp, jap_j2.cpp e aus_nsl.cpp; agente C da
# frente das competições, 04/11/2026). Cada uma roda duas vezes na carreira de 8 ligas até 2002-06-01
# (docs/cobertura_competicoes.txt): o construtor e a virada da temporada as chamam direto (0x662bf0/0x663816,
# 0x66456e/0x665272, 0x410f50/0x411e62: mov ecx, esi; call -> thiscall, ret 0).
# - 0x6634e0 JapJ1::create_table (312 bytes): 16 clubes (+0x57 = [0x9cf65c]); os que sobram perdem a divisão; o fundo
#   de prêmios (cota 100.000) e a TV 3.000.000 a cada clube.
# - 0x665030 JapJ2::create_table (262 bytes): 12 clubes ([0x9cf768]); os que sobram perdem a divisão.
# - 0x411b70 AusNsl::create_table (356 bytes): 13 clubes ([0x9cf89c]); sem encher, a caixa de erro.
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - 0x944e46 (malloc): push n; call; add esp, 4 -> cdecl;
# - 0x687430 (League::init_table_row): push 0; push 0; push esi (o clube); push eax (al = a linha, bits de cima de
#   sobra); mov ecx, edi -> thiscall, ret 0x10 (a cópia passa a linha limpa: a 0x687430 só lê o char);
# - 0x682f70 (League::calc_prize_fund): push 0x186a0; mov ecx, edi -> thiscall, ret 4; devolve o fundo (+0x28);
# - 0x683010 (League::pay_tv_rights): push 0; push 0x2dc6c0; mov ecx, edi -> thiscall, ret 8;
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparador: só a original (aloca a tabela; a J1 paga a TV com notícia), conferindo o plano feito antes: os clubes da
# divisão na ordem do banco nas linhas, +0x3e, a devolução, no Japão os que sobram sem divisão e na J1 cheia +0xe1 = 1
# e +0x28 = o fundo da cota.
# Determinismo: percorre o banco na ordem; nada de pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqm9_tabela_da_j1', 0x6634e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqm9_tabela_da_j2', 0x665030, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqm9_tabela_da_nsl', 0x411b70, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a tabela das ligas do Japão e da Austrália: os clubes da J1, da J2 e da NSL, o fundo de prêmios e a TV da J1',
        'the table of the Japanese and Australian leagues: the J1, J2 and NSL clubs, the J1 prize fund and TV money',
        [('cqm9_tabela_da_j1', 'JapJ1::create_table', '0x6634e0', 'jap_j1.cpp', 'thiscall, ret 0',
          'a J1 tem 16 clubes, os da divisão na ordem do banco; os que passam de 16 perdem a divisão (ficam sem liga); '
          'cheia, o fundo de prêmios parte de 100.000 por posição e a TV paga 3.000.000 a cada clube',
          'J1 has 16 clubs, those of the division in database order; those beyond 16 lose their division (left with no '
          'league); when full, the prize fund starts at 100,000 per place and TV pays 3,000,000 to each club'),
         ('cqm9_tabela_da_j2', 'JapJ2::create_table', '0x665030', 'jap_j2.cpp', 'thiscall, ret 0',
          'a J2 tem 12 clubes, os da divisão na ordem do banco; os que passam de 12 perdem a divisão; sem fundo de '
          'prêmios nem TV',
          'J2 has 12 clubs, those of the division in database order; those beyond 12 lose their division; no prize fund '
          'nor TV money'),
         ('cqm9_tabela_da_nsl', 'AusNsl::create_table', '0x411b70', 'aus_nsl.cpp', 'thiscall, ret 0',
          'a NSL australiana tem 13 clubes, os da divisão na ordem do banco; com menos, o jogo mostra a caixa de erro; '
          'sem fundo de prêmios nem TV',
          'the Australian NSL has 13 clubs, those of the division in database order; with fewer, the game shows the '
          'error box; no prize fund nor TV money')])
CAMPOS = [('clube +0x57: divisão (registro de competição); ponteiro; lê (as três) e grava 0 nos que sobram (J1, J2)',
           'club +0x57: division (competition record); pointer; reads (all three) and writes 0 to the surplus ones (J1, J2)'),
          ('[0x9cf65c] / [0x9cf768] / [0x9cf89c]: ids da J1, da J2 e da NSL; id; lê', '[0x9cf65c] / [0x9cf768] / [0x9cf89c]: J1, J2 and NSL ids; id; reads'),
          ('liga +0x3e: número de times da tabela; valor; grava (16, 12, 13)', 'league +0x3e: number of teams in the table; value; writes (16, 12, 13)'),
          ('liga +0xb1: a tabela (linhas de 0x3b bytes, +0 o clube); ponteiro; grava', 'league +0xb1: the table (0x3b-byte rows, +0 the club); pointer; writes'),
          ('liga +0x28: fundo de prêmios; valor; grava (J1)', 'league +0x28: prize fund; value; writes (J1)'),
          ('liga +0xe1: marca da tabela pronta; valor; grava 1 (J1)', 'league +0xe1: table ready flag; value; writes 1 (J1)')]
INTERESSA = ['CMRTE']
