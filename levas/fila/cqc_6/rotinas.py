# Os jogos levados para a TV, a notícia dos grupos e a notícia da tabela (league.cpp):
# - 0x685640 (League::move_fixtures_for_tv, ~1,7 KB, quadro SEH sem destrutor no caminho normal): nas datas da tabela
#   da TV da liga, leva o jogo de maior nota de cada vaga para o dia e a fatia da TV (sai e volta ao calendário), pelo
#   dia da liga 0x6847c0 e pela montagem do calendário 0x683e30: 6533 chamadas na carreira automática até 2002-06-01
#   (docs/cobertura_simulacao.txt, 17/10). As datas e fatias das partidas mudam a carreira.
# - 0x684230 (League::send_group_draw_news, 634 bytes, quadro SEH sem destrutor no caminho normal): a notícia 0x15 do
#   grupo aos clubes de técnico humano, pelo sorteio dos grupos das competições: 40 chamadas; passageira.
# - 0x683ee0 (League::send_fixture_list_news, 845 bytes, quadro SEH sem destrutor no caminho normal): a notícia 0xd
#   com os 3 primeiros jogos aos clubes de técnico humano, pela criação do calendário 0x5cd1e0 e pelo dia da liga no
#   ano dela: 5 chamadas; passageira.
# Iguais no cm0102-1080.exe (as três inteiras). A 0x685640 chama originais não portadas (0x946464 bsearch, 0x5aa680,
# 0x85c260) e as já em Rust (0x5abd90, 0x5acbe0, 0x5abc30, 0x945333 qsort com 0x8b4e00, 0x90cfc0); a 0x683ee0 chama a
# 0x5acee0 (original, 18 argumentos).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x685640: 0x683ea6 e 0x684b08 (push &data; mov ecx, esi) -> thiscall(this, data), ret 4.
# - quem chama a 0x684230: 0x401a90, 0x402062, 0x40c2a2, 0x40c3a9 (mov ecx, esi / mov ecx, [eax]; sem push) ->
#   thiscall(this), ret 0.
# - quem chama a 0x683ee0: 0x5cd20a e 0x6848da (push 0; mov ecx, esi), 0x6841ee (push edi; mov ecx, eax) ->
#   thiscall(this, lista), ret 4.
# - dentro da 0x685640: 0x54a110 lea ecx, local -> thiscall, ret 0; 0x946464 push 0x685cf0; push 0x41; push conta;
#   push tabela; push &chave; add esp, 0x14 -> cdecl; 0x68d020 / 0x68d040 lea ecx, lista -> thiscall, ret 0; 0x5abd90
#   push lista; push tipo; push -1; push id; push fatia; push dia; push ano; mov ecx, [0xae2a58] -> thiscall, ret 0x1c;
#   0x945333 push 0x8b4e00; push 7; push conta; push base; add esp, 0x10 -> cdecl; 0x549ef0 push ano; push dia; lea ecx
#   -> thiscall, ret 8; 0x54a2d0 push destino; push dia da semana; lea ecx -> thiscall, ret 8; 0x54c5b0 / 0x54c8e0 push
#   n; push saída; lea ecx -> thiscall, ret 8; 0x417c10 push origem; push saída; lea ecx -> thiscall, ret 8; 0x68d1b0
#   lea ecx, lista -> thiscall, ret 0; 0x53b900 push clube; add esp, 4 -> cdecl; 0x90cfc0 push n; add esp, 4/8 ->
#   cdecl; 0x5acbe0 push -1; push -2; push -1; push 0; push id; push ano1; push dia1; push ano0; push dia0; mov ecx,
#   [0xae2a58] -> thiscall, ret 0x24; 0x77bbe0 push 0; push 3; lea ecx -> thiscall, ret 8; 0x785790 push v; push k;
#   lea ecx -> thiscall, ret 8; 0x7861e0 push clube; push &notícia; mov ecx, [0xb7431c] -> thiscall, ret 8; 0x5abc30
#   push &jogo; mov ecx, [0xae2a58] -> thiscall, ret 4; 0x85c260 push +0x24; push &jogo; push +5 da vaga; mov ecx,
#   [0xdd7ef4] -> thiscall, ret 0xc; 0x5aa680 push 1; push &jogo; mov ecx, [0xae2a58] -> thiscall, ret 8; 0x68d1f0
#   push item; lea ecx, lista -> thiscall, ret 4.
# - dentro da 0x684230: 0x7875e0 push clube; mov ecx, [0xb7431c] -> thiscall, ret 4; 0x944cf1 push 0xdf; add esp, 4
#   -> cdecl; 0x77bbe0 push 0; push 0x15; mov ecx, bloco -> thiscall, ret 8; 0x549ef0, 0x54a110, 0x417c10, 0x54a2d0,
#   0x785790 (mov ecx, notícia), 0x7861e0 como acima; 0x944c94 push notícia; add esp, 4 -> cdecl.
# - dentro da 0x683ee0: 0x4b0700 push id; add esp, 4 -> cdecl; 0x5acee0 (18 argumentos, ver o codigo.rs) mov ecx,
#   [0xae2a58] -> thiscall, ret 0x48; 0x68d0d0 push 4; push clube; mov ecx, lista -> thiscall, ret 8; ela mesma push
#   lista; mov ecx, fase -> thiscall, ret 4; as datas, a lista e a notícia como acima.
# Comparadores: a 0x685640 só a original (o calendário aloca, as notas sorteiam, as notícias vão aos técnicos), com o
# plano: pela entrada da data, cada vaga válida (fatia 0..2) antes da primeira vazia leva um jogo enquanto houver jogos
# da liga no dia e fatia (0x5abd90 numa lista provisória) e fica 0xff: as vagas 0xff depois são min(válidas, jogos);
# a 0x684230 e a 0x683ee0 só a original (só notícias e a lista dos avisados).
SAVE = False
ROTINAS = [('liga_jogos_para_a_tv', 0x685640, [0x6a, 0xff, 0x68, 0x72, 0xca, 0x95, 0x00]),
           ('liga_noticia_dos_grupos', 0x684230, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('liga_noticia_da_tabela', 0x683ee0, [0x6a, 0xff, 0x68, 0xbc, 0xc9, 0x95, 0x00])]
DOCS = ('a liga: os jogos de maior nota levados ao dia e à fatia da TV, a notícia do sorteio dos grupos e a notícia dos primeiros jogos',
        'the league: the highest-rated fixtures moved to the TV day and slot, the group draw news and the opening fixtures news',
        [('liga_jogos_para_a_tv', 'League::move_fixtures_for_tv', '0x685640', 'league.cpp', 'thiscall, ret 4',
          'nas datas da tabela da TV, cada vaga leva o jogo de maior nota (sorteio pelas reputações, zerado com jogo de um dos clubes 2..5 dias antes ou depois; 100/50 nas vagas de seleção) para o dia da semana e a fatia da vaga, com a notícia, o estádio e o calendário',
          'on the TV table dates, each slot takes the highest-rated fixture (a draw by reputation, zeroed when either club plays 2..5 days before or after; 100/50 in national team slots) to the weekday and time slot of the slot, with the news, the stadium and the calendar'),
         ('liga_noticia_dos_grupos', 'League::send_group_draw_news', '0x684230', 'league.cpp', 'thiscall, ret 0',
          'a notícia 0x15 do grupo (a competição, a primeira data, os adversários) a cada clube de técnico humano',
          'group news 0x15 (the competition, the first date, the opponents) to each human-managed club'),
         ('liga_noticia_da_tabela', 'League::send_fixture_list_news', '0x683ee0', 'league.cpp', 'thiscall, ret 4',
          'a notícia 0xd com os três primeiros jogos (data, adversário, mando) a cada clube de técnico humano ainda não avisado, e o mesmo nas fases seguintes',
          'news 0xd with the first three fixtures (date, opponent, venue) to each human-managed club not yet told, and the same in the following stages')])
CAMPOS = [('liga +0xba (tabela das datas da TV, 0x41 bytes por entrada; conta +0xa9) +0 dia, +2 ano − o da liga, +4 fatia, +5.. vagas de 7 bytes (+0 estádio, +4 dia da semana, +5 fatia da TV, +6 tipo: 0xff = vazia); valor; lê e grava (liga_jogos_para_a_tv: a vaga usada fica 0xff, a ordem pela 0x945333), lê (liga_noticia_dos_grupos, liga_noticia_da_tabela)',
           'league +0xba (TV date table, 0x41 bytes per entry; count +0xa9) +0 day, +2 year − the league one, +4 slot, +5.. 7-byte slots (+0 stadium, +4 weekday, +5 TV slot, +6 type: 0xff = empty); value; reads and writes (liga_jogos_para_a_tv: the used slot becomes 0xff, the order by 0x945333), reads (liga_noticia_dos_grupos, liga_noticia_da_tabela)'),
          ('liga +0x40 ano, +0x44 tipo/fase (0xff primeira fase), +0x3e times, +0xb1 times de 0x3b bytes (+0 clube), +0x24, +0x3a, +0xc/+0x30 as fases (+0x43); ponteiro e valor; lê (liga_jogos_para_a_tv, liga_noticia_dos_grupos, liga_noticia_da_tabela)',
           'league +0x40 year, +0x44 type/stage (0xff first stage), +0x3e teams, +0xb1 0x3b-byte teams (+0 club), +0x24, +0x3a, +0xc/+0x30 the stages (+0x43); pointer and value; reads (liga_jogos_para_a_tv, liga_noticia_dos_grupos, liga_noticia_da_tabela)'),
          ('partida (0x4f bytes no calendário) +0xc/+0x10 ids, +0x18 estádio, +0x1c/+0x20 clubes, +0x28 ano, +0x2a dia, +0x3b tipo, +0x3f fatia, +0x42 tipo da liga; valor; lê e grava (liga_jogos_para_a_tv), lê (liga_noticia_da_tabela)',
           'fixture (0x4f bytes in the calendar) +0xc/+0x10 ids, +0x18 stadium, +0x1c/+0x20 clubs, +0x28 year, +0x2a day, +0x3b type, +0x3f slot, +0x42 league type; value; reads and writes (liga_jogos_para_a_tv), reads (liga_noticia_da_tabela)'),
          ('clube +0x80 reputação, +0x53 nação; valor; lê (liga_jogos_para_a_tv)',
           'club +0x80 reputation, +0x53 nation; value; reads (liga_jogos_para_a_tv)')]
INTERESSA = ['CMRTE']
