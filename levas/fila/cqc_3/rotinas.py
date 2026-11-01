# Os conflitos de calendário (fix_man.cpp): FIXTURE_MANAGER::find_fixture_clashes (0x5ad780, 0x41b bytes, 8.129
# chamadas na carreira até 2002-06-01, docs/cobertura_simulacao.txt, pelo calendário: 0x5aa680, 0x5ab510, 0x5ae380 e
# 0x5ae9d0), FIXTURE_MANAGER::collect_day_clashes (0x5adba0, 0x7da bytes, 24.387 chamadas, só pela 0x5ad780) e,
# passageira, fix_man_load_fixture_leg_checks (0x5af390, 0x104 bytes, uma vez pela construção do calendário principal
# 0x5a9cf0). Quadros SEH da 0x5ad780 (a lista nova) e da 0x5adba0 (as datas), sem destrutor no caminho normal. Iguais
# no cm0102-1080.exe (corpo inteiro das três).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x5adba0 (pela 0x5ad780): mov ecx, ebx (this); 7 pushes (deslocamento, &data, n, &nações, lista, &gravidade, jogo);
#   sem add esp -> thiscall, ret 0x1c. O n vai como dword com a palavra de cima sem valor; a 0x5adba0 lê o byte.
# - 0x48dd70 (nação-chave do jogo): push jogo; push comp (ou push 0; push liga); add esp, 8 -> cdecl.
# - 0x48df40 (do estádio), 0x5af5b0 (parceira), 0x48dee0 (da seleção), 0x53b900, 0x53b3d0: push; add esp, 4 -> cdecl.
# - 0x944cf1 (new): push 0xe; add esp, 4 -> cdecl. 0x68d020 (LLIST::LLIST): mov ecx, bloco; sem pushes -> thiscall.
# - 0x549ef0 (data nova): push +0x28; push +0x2a; ecx = local -> thiscall, ret 8 (lê as palavras).
# - 0x68d040 (destrói a lista): mov ecx, lista -> thiscall. 0x944c94 (free): push; add esp, 4 -> cdecl.
# - 0x776fc0 (a seleção deixa marcar): mov ecx, [0xb74318]; push jogo -> thiscall, ret 4.
# - 0x54c5b0 (data + n): ecx = data (arg); push n (movsx ax, lixo em cima); push &local -> thiscall, ret 8.
# - 0x549f70 (data por dia e mês): lea ecx, local; push −1; push ano; push mês; push dia -> thiscall, ret 0x10.
# - 0x54a3f0 (dia da semana): lea ecx, data -> thiscall, sem pushes.
# - 0x5abf10 (lista do dia): mov ecx, this; push nação; push fatia; push dia; push ano -> thiscall, ret 0x10 (a fatia
#   e a nação com os bytes de cima sem valor; ela lê o byte).
# - 0x68d1b0 (próximo da lista): mov ecx, lista -> thiscall. 0x68d0d0 (acrescenta): mov ecx, lista; push 0x4f; push
#   jogo -> thiscall, ret 8.
# - 0x76d940 (competição especial): mov ecx, [0xb74318]; push comp -> thiscall, ret 4 (a cópia Rust é stdcall e ignora
#   o ecx: a chamada thiscall com o mesmo argumento serve às duas).
# - Método +0x60 da competição: mov ecx, comp; push b; push a; call [vtable + 0x60] -> thiscall(a, b).
# - Na 0x5af390: 0x68d080 (mov ecx, 0xae2a48 -> thiscall), 0x90bde0 (2 pushes; add esp, 8 -> cdecl), 0x945b00 (fopen,
#   o "rb" que ficou na pilha e o caminho; add esp, 8 -> cdecl), 0x945bfe (fread, 4 pushes; add esp, 0x10 -> cdecl),
#   0x945a09 (fclose, push; add esp, 4 -> cdecl), 0x68d0d0 (mov ecx, 0xae2a48; push 0xf; push &registro -> thiscall).
# Comparadores: a 0x5adba0 com a cópia em ensaio (guarda os jogos que acrescentaria, gravidade própria) e a original a
# partir do mesmo estado, conferindo a gravidade e os nós novos da lista (0x4f bytes de cada jogo); a 0x5ad780 com as
# duas rodando, cada uma com a sua lista nova (a da cópia destruída e liberada), conferindo a devolução, a saída e os
# jogos das listas; a 0x5af390 só a original (lê o arquivo e aloca os nós).
SAVE = False
ROTINAS = [('conflitos_do_dia', 0x5adba0, [0x6a, 0xff, 0x68, 0xeb, 0x97, 0x95, 0x00]),
           ('conflitos_do_jogo', 0x5ad780, [0x6a, 0xff, 0x68, 0xc9, 0x97, 0x95, 0x00]),
           ('carregar_conferencia_das_pernas', 0x5af390, [0x81, 0xec, 0x14, 0x02, 0x00, 0x00])]
DOCS = ('os conflitos de calendário: os conflitos de um jogo nos dias em volta, os conflitos de um dia e a carga da conferência das pernas',
        'the fixture clashes: a fixture\'s clashes over the surrounding days, one day\'s clashes and loading the fixture leg checks',
        [('conflitos_do_dia', 'FIXTURE_MANAGER::collect_day_clashes', '0x5adba0', 'fix_man.cpp', 'thiscall, ret 0x1c',
          'no dia do jogo mais o deslocamento: as datas proibidas (25 de dezembro, o domingo e 10 a 20 de abril em certas nações, 17 de maio) dão 19; nas listas do dia de cada nação e fatia, os jogos do mesmo clube (mesma fatia pelo tipo; outra fatia 0x14 ou 1), do mesmo estádio, das seleções e dos clubes grandes pela regra da competição entram na lista; jogo já jogado dá 0x16',
          'on the fixture day plus the offset: forbidden dates (25 December, Sunday and 10 to 20 April in some nations, 17 May) give 19; in each nation and slice day list, fixtures of the same club (same slice by type; other slice 0x14 or 1), the same stadium, the national teams and big clubs by the competition rule go into the list; a played fixture gives 0x16'),
         ('conflitos_do_jogo', 'FIXTURE_MANAGER::find_fixture_clashes', '0x5ad780', 'fix_man.cpp', 'thiscall, ret 0xc',
          'monta as nações a olhar (0 a 6, a do jogo, das ligas, do estádio, as parceiras e as das seleções, sem repetir), cria a lista e junta os conflitos dos dois dias antes, do anterior, do próprio e do seguinte; sem conflito, destrói a lista, 0x15 se a seleção não deixa e 0; senão a gravidade e a lista',
          'builds the nations to check (0 to 6, the fixture\'s, the leagues\', the stadium\'s, the partners and the national teams\', without repeats), creates the list and collects the clashes of the two days before, the day before, the day itself and the day after; with no clash, destroys the list, 0x15 if the national team forbids it and 0; otherwise the severity and the list'),
         ('carregar_conferencia_das_pernas', 'fix_man_load_fixture_leg_checks', '0x5af390', 'fix_man.cpp', 'cdecl',
          'esvazia a lista [0xae2a48] e a refaz com os registros de 15 bytes do check_fixture_legs.tmp',
          'clears the list at 0xae2a48 and refills it with the 15-byte records of check_fixture_legs.tmp')])
CAMPOS = [('jogo (0x4f bytes) +0 a competição (id), +0x14 a competição (+0x5d a nação), +0x18 o estádio, +0x1c / +0x20 os clubes, +0x28 / +0x2a a data, +0x3a o tipo, +0x3f a marca, +0x40 / +0x41 as fatias dos dois clubes, +0x43 jogado (0xff não); ponteiro e valor; lê (conflitos_do_dia, conflitos_do_jogo)',
           'fixture (0x4f bytes) +0 the competition (id), +0x14 the competition (+0x5d the nation), +0x18 the stadium, +0x1c / +0x20 the clubs, +0x28 / +0x2a the date, +0x3a the type, +0x3f the flag, +0x40 / +0x41 both clubs\' slices, +0x43 played (0xff no); pointer and value; reads (conflitos_do_dia, conflitos_do_jogo)'),
          ('clube +0x53 a nação, +0x57 a liga (+0 id), +0x80 a reputação; ponteiro e valor; lê (conflitos_do_dia, conflitos_do_jogo)',
           'club +0x53 the nation, +0x57 the league (+0 id), +0x80 reputation; pointer and value; reads (conflitos_do_dia, conflitos_do_jogo)'),
          ('LLIST (0xe bytes) +0 a conta, +2 a cabeça, +6 o fim, +0xa o cursor; nó +0 o dado, +6 o próximo; ponteiro; lê e grava o cursor das listas dos dias, acrescenta na lista dos conflitos (conflitos_do_dia)',
           'LLIST (0xe bytes) +0 count, +2 head, +6 tail, +0xa cursor; node +0 data, +6 next; pointer; reads and writes the day lists\' cursor, appends to the clash list (conflitos_do_dia)'),
          ('[0xae2a48] a lista das conferências das pernas (registros de 15 bytes); grava (carregar_conferencia_das_pernas)',
           '[0xae2a48] the fixture leg check list (15-byte records); writes (carregar_conferencia_das_pernas)')]
INTERESSA = []
