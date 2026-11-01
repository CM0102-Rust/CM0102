# O calendário (fix_man.cpp): FIXTURE_MANAGER::find_new_fixture_date (0x5ab510, 0x5f8 bytes, 285 chamadas na carreira
# até 2002-06-01, docs/cobertura_simulacao.txt), FIXTURE_MANAGER::add_fixture (0x5aa680, 0x5ed bytes, 7.650 chamadas:
# copas, ligas, amistosos, remarcações) e FIXTURE_MANAGER::set_year (0x5aa2d0, 0x3a6 bytes, 6 chamadas: o bloco do
# outro ano na virada). Os conflitos que elas usam (0x5ad780, 0x5adba0) estão na cqc_3. Quadros SEH só das datas e dos
# blocos novos, sem destrutor no caminho normal. Iguais no cm0102-1080.exe (corpo inteiro das três, com a tabela de
# salto da 0x5ab510).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - As datas (thiscall): 0x54a110 (lea ecx, data), 0x549ef0 (push ano; push dia; ret 8), 0x417c10 (push fonte; push
#   ret; ret 8), 0x54c5b0 / 0x54c8e0 / 0x54c770 (push n; push ret; ret 8), 0x54a2d0 (push dest; push alvo; ret 8),
#   0x54a3f0 (sem pushes). O alvo vai com os bytes de cima sem valor; elas leem a palavra.
# - 0x4b0850 (unidades da rodada): push; add esp, 4 -> cdecl. 0x68d0d0 (acrescenta): mov ecx, lista; push tamanho; push
#   dado -> thiscall, ret 8.
# - 0x5af2f0 (faixa de horário): mov ecx, gerenciador; push dia; push comp -> thiscall, ret 8.
# - 0x5ad780 (conflitos): mov ecx, gerenciador; push marca; push &gravidade; push jogo -> thiscall, ret 0xc.
# - 0x68d040 (destrói): mov ecx, lista -> thiscall. 0x944c94 (free), 0x944cf1 (new): push; add esp, 4 -> cdecl.
# - Na 0x5aa680: 0x5af180 (push jogo; add esp, 4 -> cdecl), 0x5ab510 (mov ecx, this; push jogo -> thiscall, ret 4),
#   0x5ae380 (mov ecx, this; push lista; push jogo -> thiscall, ret 8), método +0x64 da competição (mov ecx, comp; push
#   jogo -> thiscall), 0x7e6f20 (mov ecx, [0xb74434]; push comp -> thiscall, ret 4), 0x5aa680 (mov ecx, this ou o
#   outro bloco; push 0; push jogo -> thiscall, ret 8), 0x5ab230 e 0x5abc30 (mov ecx, this; push jogo -> thiscall, ret 4),
#   0x5c6710 (mov ecx, [0xae2c74]; push comp -> thiscall, ret 4), 0x5c6570 (mov ecx, [0xae2c74]; push 0; push jogo ->
#   thiscall, ret 8), 0x5aac70 (mov ecx, [0xae2a58]; push 0; push jogo; push &cópia -> thiscall, ret 0xc), 0x48dd70
#   (push jogo; push comp; add esp, 8 -> cdecl), 0x5ab3c0 (mov ecx, this; push jogo -> thiscall, ret 4), 0x53b900 (push;
#   add esp, 4 -> cdecl), 0x76d940 (mov ecx, [0xb74318]; push comp -> thiscall, ret 4; a cópia Rust é stdcall e ignora
#   o ecx), 0x7e6720 (mov ecx, [0xb74434]; push jogo -> thiscall, ret 4), 0x76bab0 (mov ecx, [0xb74318]; push jogo ->
#   thiscall, ret 4), 0x5ae9d0 (mov ecx, this; push jogo -> thiscall, ret 4), 0x5aa2d0 (mov ecx, outro bloco; push ano
#   -> thiscall, ret 4), 0x5a9cf0 (mov ecx, bloco novo; push 0; push this; push ano -> thiscall, ret 0xc).
# - Na 0x5aa2d0: 0x5ac060 (mov ecx, this; push 0 -> thiscall, ret 4), 0x5aa2d0 (mov ecx, outro bloco; push ano vizinho
#   -> thiscall, ret 4), 0x5aa0d0 (mov ecx, bloco -> thiscall), 0x54cfe0 (push ano; add esp, 4 -> cdecl), 0x68d020 (mov
#   ecx, bloco -> thiscall), 0x5ac3b0 (mov ecx, this -> thiscall).
# Comparadores: a 0x5ab510 lado a lado no jogo (0x4f bytes; os conflitos de dentro criam e destroem a lista deles), só
# a original quando o jogo entra na conferência das pernas (aloca o nó) ou com a caixa da janela sem data; a 0x5aa680
# e a 0x5aa2d0 só a original (gravam nas listas do calendário, alocam, remarcam, entregam notícias e mexem nos arquivos).
SAVE = False
ROTINAS = [('nova_data_do_jogo', 0x5ab510, [0x6a, 0xff, 0x68, 0x07, 0x97, 0x95, 0x00]),
           ('jogo_no_calendario', 0x5aa680, [0x6a, 0xff, 0x68, 0x24, 0x96, 0x95, 0x00]),
           ('ano_do_calendario', 0x5aa2d0, [0x6a, 0xff, 0x68, 0xea, 0x95, 0x95, 0x00])]
DOCS = ('o calendário: a nova data de um jogo, a entrada do jogo no calendário e a troca do ano do calendário',
        'the fixture calendar: a fixture\'s new date, adding a fixture to the calendar and switching the calendar year',
        [('nova_data_do_jogo', 'FIXTURE_MANAGER::find_new_fixture_date', '0x5ab510', 'fix_man.cpp', 'thiscall, ret 4',
          'a janela pelo tipo do jogo; em duas passadas, os dias da semana na ordem 2, 1, 5, 6, 0, 3, 4, de 4 em 4 dias, a partir de 2 dias depois de hoje: a primeira data sem conflito (ou de gravidade 1 ou abaixo do tipo) fica, com a faixa de horário; sem achar, 2 dias depois; nas rodadas de casa 3 e 4, o registro na conferência das pernas',
          'the window by fixture type; in two passes, weekdays in the order 2, 1, 5, 6, 0, 3, 4, every 4 days, from 2 days after today: the first date with no clash (or severity 1 or below the type) stays, with the time slot; otherwise 2 days later; for rounds ending in 3 and 4, the leg check record'),
         ('jogo_no_calendario', 'FIXTURE_MANAGER::add_fixture', '0x5aa680', 'fix_man.cpp', 'thiscall, ret 8',
          'acerta a fase, dá nova data ao jogo atrasado, leva o jogo de liga do fim de semana ao dia preferido do mandante, resolve os conflitos (o do mesmo dia, a recusa da competição, a nova data ou a saída dos jogos do conflito com a notícia), marca os jogos de seleções e os vizinhos e põe o jogo na lista do dia, ou no bloco do outro ano',
          'fixes the stage, gives a late fixture a new date, moves a weekend league fixture to the home club\'s preferred day, resolves the clashes (same day, competition refusal, new date or removing the clashing fixtures with the news), handles national team fixtures and neighbours and puts the fixture in the day list, or in the other year\'s block'),
         ('ano_do_calendario', 'FIXTURE_MANAGER::set_year', '0x5aa2d0', 'fix_man.cpp', 'thiscall, ret 4',
          'grava os jogos do ano, leva o outro bloco ao ano vizinho se preciso, destrói as listas dos 366 dias, troca o ano e o bissexto, cria as listas novas e carrega os jogos do ano novo',
          'saves the year\'s fixtures, moves the other block to the neighbouring year if needed, destroys the 366 days\' lists, switches the year and leap flag, creates the new lists and loads the new year\'s fixtures')])
CAMPOS = [('jogo (0x4f bytes) +0x14 a competição, +0x1c / +0x20 os clubes, +0x24 / +0x26 a data limite, +0x28 / +0x2a a data, +0x2e grava −1, +0x30 / +0x32 a rodada, +0x34, +0x3a o tipo, +0x3b a fase, +0x3f a faixa de horário, +0x42, +0x43 jogado, +0x4d o bit 0x800 (dia preferido já visto), +0x4e; ponteiro e valor; lê e grava (nova_data_do_jogo, jogo_no_calendario)',
           'fixture (0x4f bytes) +0x14 the competition, +0x1c / +0x20 the clubs, +0x24 / +0x26 the limit date, +0x28 / +0x2a the date, +0x2e written −1, +0x30 / +0x32 the round, +0x34, +0x3a the type, +0x3b the stage, +0x3f the time slot, +0x42, +0x43 played, +0x4d bit 0x800 (preferred day done), +0x4e; pointer and value; reads and writes (nova_data_do_jogo, jogo_no_calendario)'),
          ('clube +0x57 / +0x60 as ligas, +0x72 o dia preferido (4, 5, 6); ponteiro e valor; lê (jogo_no_calendario)',
           'club +0x57 / +0x60 the leagues, +0x72 the preferred day (4, 5, 6); pointer and value; reads (jogo_no_calendario)'),
          ('FIXTURE_MANAGER (0x1184 bytes) +0 as listas por dia (366 × 3 fatias × nações-chave), +0x112c o bloco do outro ano, +0x1130 o ano, +0x1132 bissexto, +0x1183 o principal; ponteiro e valor; lê e grava (jogo_no_calendario, ano_do_calendario)',
           'FIXTURE_MANAGER (0x1184 bytes) +0 the per-day lists (366 × 3 slices × key nations), +0x112c the other year\'s block, +0x1130 the year, +0x1132 leap year, +0x1183 the main one; pointer and value; reads and writes (jogo_no_calendario, ano_do_calendario)'),
          ('[0xae2a40] as remarcações em cadeia (palavra, até 10); [0xae2a48] a lista da conferência das pernas; [0xb64738] as nações-chave (byte); valor; lê e grava (jogo_no_calendario, nova_data_do_jogo, ano_do_calendario)',
           '[0xae2a40] chained reschedules (word, up to 10); [0xae2a48] the leg check list; [0xb64738] the key nations (byte); value; reads and writes (jogo_no_calendario, nova_data_do_jogo, ano_do_calendario)')]
INTERESSA = ['SGE']
