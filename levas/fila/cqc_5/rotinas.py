# Os jogos do confronto da copa, a notícia do confronto pendente e os avisos das janelas de transferência (Cup.cpp,
# transfer_manager.cpp):
# - 0x51cb90 (Cup::create_tie_fixtures, ~1,8 KB, quadro SEH sem destrutor no caminho normal): cria a partida (e a
#   volta, na rodada de duas mãos) de cada confronto marcado, pela montagem da rodada 0x51c800 (montar_rodada_da_copa,
#   em Rust; "cps_13 round matches" na base): 1148 chamadas na carreira automática até 2002-06-01
#   (docs/cobertura_simulacao.txt, 17/10). As partidas vão ao calendário (0x5aa680): quem joga, onde, quando.
# - 0x522950 (Cup::post_draw_news, 800 bytes, quadro SEH sem destrutor no caminho normal): a notícia 0x26 do confronto
#   com time ainda indefinido, só pela 0x51cb90: passageira.
# - 0x8c6bc0 (TRANSFER_MANAGER::update_transfer_window_news, 484 bytes, quadro SEH sem destrutor no caminho normal):
#   os avisos das janelas (uma semana antes, no dia, e no dia seguinte à abertura as ofertas em espera da nação,
#   0x8c4e10 ofertas_em_espera_na_janela, em Rust), pelo dia das transferências 0x8c5880: 322 chamadas na carreira.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x51cb90: 0x51c916 (push 1; push ebp; push edi; mov ecx, esi), 0x49719b, 0x49782a, 0x49787c (push 1;
#   push -1; push clube/rodada; mov ecx, edi) -> thiscall(this, rodada, confronto, adiar), ret 0xc.
# - quem chama a 0x522950: 0x51d2a1 push ecx; push ebx; mov ecx, esi -> thiscall(this, rodada, confronto), ret 8.
# - quem chama a 0x8c6bc0: 0x8c590f push ebp; mov ecx, esi -> thiscall(this, hoje), ret 4.
# - dentro da 0x51cb90: 0x522950 como acima; o virtual [[esi]+0x7c] push &partida; mov ecx, esi -> thiscall(copa,
#   partida); 0x85c260 push +0x24; push &partida; push +0x13; mov ecx, [0xdd7ef4] -> thiscall(estádios, regras,
#   partida, id), ret 0xc; 0x549ef0 push ano; push dia; lea ecx, local -> thiscall(data, dia, ano), ret 8; 0x54c5b0
#   push 1 (ou o byte +0x22); push saída; mov ecx, 0xae2c90 (ou a local) -> thiscall(data, saída, n), ret 8; 0x417c10
#   push origem; push saída; lea ecx, local -> thiscall(destino, saída, origem), ret 8; 0x54a3f0 mov ecx, data ->
#   thiscall, ret 0; 0x5aa680 push 1; push &partida; mov ecx, [0xae2a58] -> thiscall(calendário, partida, 1), ret 8;
#   0x7875e0 push clube; mov ecx, [0xb7431c] -> thiscall(notícias, clube), ret 4; 0x77bbe0 push 0; push 0x14; lea ecx,
#   notícia -> thiscall(notícia, tipo, 0), ret 8; 0x785790 push v; push k; lea ecx, notícia -> thiscall, ret 8;
#   0x7861e0 push clube; push &notícia; mov ecx, [0xb7431c] -> thiscall(notícias, notícia, clube), ret 8.
# - dentro da 0x522950: 0x77bbe0 (push 0; push 0x26), 0x785790 e 0x7861e0 como acima.
# - dentro da 0x8c6bc0: 0x54a110 lea ecx, local -> thiscall, ret 0; 0x54cfb0 mov ecx, hoje -> thiscall, ret 0 (al);
#   0x54c770 e 0x54c8e0 push 1; push saída; mov ecx, hoje -> thiscall(data, saída, n), ret 8; 0x40a7d0 push ano; push
#   saída; mov ecx, janela -> thiscall(janela, saída, ano), ret 8; 0x417c10 como acima; o virtual [[regras]+0x2c]
#   push dword do ano; push código; push janela; mov ecx, regras -> thiscall(regras, janela, código, ano); 0x8c4e10
#   push nação; mov ecx, ebp -> thiscall(this, nação), ret 4.
# Comparadores: a 0x51cb90 só a original (a 0x5aa680 aloca e as notícias vão aos técnicos), com o plano: o confronto
# (o −1 procura a marca 1 ou 2), a devolução, a marca (2 adiado, 3 criado) e os jogos do mandante na competição de hoje
# ao fim do ano seguinte (0x5acbe0) subindo 1, ou 2 na de duas mãos com a copa ainda ativa; a 0x522950 e a 0x8c6bc0 só
# a original (não gravam nada fora das locais: a notícia aos técnicos; os avisos das regras e as ofertas em espera).
SAVE = False
ROTINAS = [('copa_criar_jogos_do_confronto', 0x51cb90, [0x6a, 0xff, 0x68, 0x11, 0x77, 0x95, 0x00]),
           ('copa_noticia_do_confronto_pendente', 0x522950, [0x6a, 0xff, 0x68, 0xfb, 0x77, 0x95, 0x00]),
           ('janelas_avisos_do_dia', 0x8c6bc0, [0x6a, 0xff, 0x68, 0x48, 0x3d, 0x96, 0x00])]
DOCS = ('a copa: as partidas de cada confronto (a ida, a volta de mando invertido, o estádio, a data e a notícia), a notícia do confronto com time indefinido e os avisos das janelas de transferência',
        'the cup: the fixtures of each tie (first leg, second leg with home and away swapped, the stadium, the date and the news), the news of a tie with an undecided team and the transfer window notices',
        [('copa_criar_jogos_do_confronto', 'Cup::create_tie_fixtures', '0x51cb90', 'Cup.cpp', 'thiscall, ret 0xc',
          'cria a partida do confronto (ou só o marca como adiado): competição, rodada, regras, clubes, data (amanhã se já passou), fatia, estádio e calendário; a notícia aos técnicos humanos; na de duas mãos, a volta +0x22 dias depois, com o mando invertido',
          'creates the fixture of the tie (or only marks it as deferred): competition, round, rules, clubs, date (tomorrow if already past), slot, stadium and calendar; the news to human managers; in a two-legged round, the second leg +0x22 days later, with home and away swapped'),
         ('copa_noticia_do_confronto_pendente', 'Cup::post_draw_news', '0x522950', 'Cup.cpp', 'thiscall, ret 8',
          'a notícia 0x26 do confronto com time indefinido: a copa, a rodada e os times possíveis, aos técnicos de cada um',
          'news 0x26 of a tie with an undecided team: the cup, the round and the possible teams, to the managers of each'),
         ('janelas_avisos_do_dia', 'TRANSFER_MANAGER::update_transfer_window_news', '0x8c6bc0', 'transfer_manager.cpp', 'thiscall, ret 4',
          'confere cada janela de transferência nos meses anterior, atual e seguinte: no dia, o aviso de abertura ou fechamento; no dia seguinte à abertura, o aviso e as ofertas em espera da nação; uma semana antes da abertura, o aviso',
          'checks each transfer window in the previous, current and next months: on the day, the opening or closing notice; the day after the opening, the notice and the waiting offers of the nation; a week before the opening, the notice')])
CAMPOS = [('copa +0xad (confrontos de 0x35 bytes por rodada) +0/+2 times, +0xa/+0xc/+0xe/+0x10 origem, +0x34 marca (1 adiar, 2 criar, 3 criado); id e valor; lê e grava (copa_criar_jogos_do_confronto), lê (copa_noticia_do_confronto_pendente)',
           'cup +0xad (0x35-byte ties per round) +0/+2 teams, +0xa/+0xc/+0xe/+0x10 origin, +0x34 mark (1 defer, 2 create, 3 created); id and value; reads and writes (copa_criar_jogos_do_confronto), reads (copa_noticia_do_confronto_pendente)'),
          ('copa +0xa3 (rodadas de 0x68 bytes) +3 dia, +5 ano, +6 fatia, +7 código, +9/+0xb, +0xd regras, +0x13 estádio, +0x17, +0x1a confrontos, +0x20, +0x21 mãos, +0x22 dias até a volta, +0x6b/+0x6d a data da seguinte; valor; lê (copa_criar_jogos_do_confronto)',
           'cup +0xa3 (0x68-byte rounds) +3 day, +5 year, +6 slot, +7 code, +9/+0xb, +0xd rules, +0x13 stadium, +0x17, +0x1a ties, +0x20, +0x21 legs, +0x22 days to the second leg, +0x6b/+0x6d the next date; value; reads (copa_criar_jogos_do_confronto)'),
          ('copa +0xa7 (times de 6 bytes) +0 clube, +4 nível; +0x3a, +0x3c rodadas, +0x40 ano, +0x44 liga, +0x24; ponteiro e valor; lê (copa_criar_jogos_do_confronto)',
           'cup +0xa7 (6-byte teams) +0 club, +4 level; +0x3a, +0x3c rounds, +0x40 year, +0x44 league, +0x24; pointer and value; reads (copa_criar_jogos_do_confronto)'),
          ('partida (0x4f bytes, para a 0x5aa680) +0 id da competição, +8, +0xc/+0x10 ids, +0x14 competição, +0x1c/+0x20 clubes, +0x24/+0x26 a data seguinte, +0x28 ano, +0x2a dia, +0x2c, +0x30 código, +0x32..+0x38, +0x3a..+0x4c marcas, +0x4e bit 2; valor; grava (copa_criar_jogos_do_confronto)',
           'fixture (0x4f bytes, for 0x5aa680) +0 competition id, +8, +0xc/+0x10 ids, +0x14 competition, +0x1c/+0x20 clubs, +0x24/+0x26 the next date, +0x28 year, +0x2a day, +0x2c, +0x30 code, +0x32..+0x38, +0x3a..+0x4c flags, +0x4e bit 2; value; writes (copa_criar_jogos_do_confronto)'),
          ('gerência de transferências +0x8a4/+0x8a8 as janelas (+0 nação das regras, +3 dia, +4 mês, +5 fecha), +0x8ac as 28 regras (+0xa nação); ponteiro e valor; lê (janelas_avisos_do_dia)',
           'transfer manager +0x8a4/+0x8a8 the windows (+0 rules nation, +3 day, +4 month, +5 closes), +0x8ac the 28 rules (+0xa nation); pointer and value; reads (janelas_avisos_do_dia)'),
          ('[0xae2c90] hoje; [0xadadfc] as competições ativas; valor; lê (copa_criar_jogos_do_confronto)',
           '[0xae2c90] today; [0xadadfc] the active competitions; value; reads (copa_criar_jogos_do_confronto)')]
INTERESSA = ['CMRTE']
