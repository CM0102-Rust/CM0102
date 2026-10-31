# O adiamento pelo clima (weather.cpp, finance.cpp, stadium.cpp): a decisão de adiar ou abandonar o jogo
# Weather::check_postponement 0x923630 (pelo dia dos jogos 0x6b3c10 e pela preparação da partida 0x6b7f20, as duas em
# Rust; todo jogo de liga com estádio: o sorteio das finanças nos jogos da noite e as cadeias de sorteio da chuva forte e
# do gelo), as finanças do clube FinanceTable::get_club_finance 0x5a0590 (centenas de chamadores: transferências,
# diretoria, técnicos, telas) e a ordem das reservas do estádio stadium_compare_bookings 0x85e380 (qsort das reservas,
# pela 0x85de20 da 255ª). Assim a 0x923630 só chama rotinas em Rust (0x669360, 0x5a0590, 0x5986b0 da cpu_7, 0x923010 da
# cpu_6, 0x90cfc0). Fora de fechar_partida_do_dia e de eventos_do_pos_jogo. Iguais no cm0102-1080.exe. Sem x87; a
# 0x5a0590 tem quadro de 0x204 bytes só para a caixa de erro (clube nulo: a original). A 0x923630 grava o clima na vaga do
# argumento e o lê de lá (sem regiões o clima não grava e ficam os bytes do ponteiro: a cópia faz igual). Comparadores: a
# 0x923630 só a original (sorteios), a cópia leva os ramos; a 0x5a0590 e a 0x85e380 pelo valor.
SAVE = False
ROTINAS = [('adiar_pelo_clima', 0x923630, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('financas_do_clube', 0x5a0590, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('ordem_das_reservas_do_estadio', 0x85e380, [0x53, 0x8b, 0x5c, 0x24, 0x08])]
DOCS = ('o adiamento pelo clima: a decisão de adiar ou abandonar o jogo, as finanças do clube e a ordem das reservas do estádio',
        'the weather postponement: the decision to postpone or abandon the match, the club finances and the stadium booking order',
        [('adiar_pelo_clima', 'Weather::check_postponement', '0x923630', 'weather.cpp', 'thiscall, ret 4',
          'só jogos da liga do mandante, com estádio descoberto (+0x4c) com cidade e lugares e fora do fim da temporada da nação-chave; na fatia da noite, sorteio(1000/2500/5000 pela faixa das finanças) zero devolve 0x12; pelo clima (0x923010): chuva 5 com 1 a 13 graus em estádio de até 20000 lugares sorteia o adiamento (9); chuva 8 e 9 (gelo), cadeias de sorteio que devolvem 0, 5 ou 6',
          'league matches of the home club only, at an uncovered stadium (+0x4c) with a city and places and outside the key nation season end; in the evening slot, random(1000/2500/5000 by the finance band) zero returns 0x12; by the weather (0x923010): rain 5 at 1 to 13 degrees in a stadium of up to 20000 places draws the postponement (9); rain 8 and 9 (ice), random chains returning 0, 5 or 6'),
         ('financas_do_clube', 'FinanceTable::get_club_finance', '0x5a0590', 'finance.cpp', 'thiscall, ret 4',
          'o registro de finanças (0x167 bytes) pelo id do clube; clube B com parceiro A, o do A (0x540a50); clube nulo, caixa de erro',
          'the finance record (0x167 bytes) by the club id; a B club with an A partner, the A one (0x540a50); null club, error box'),
         ('ordem_das_reservas_do_estadio', 'stadium_compare_bookings', '0x85e380', 'stadium.cpp', 'cdecl',
          'qsort das reservas: pela data (+4), depois pelo id do estádio (+0)',
          'bookings qsort: by date (+4), then by stadium id (+0)')])
CAMPOS = [('jogo +0x14 competição, +0x18 estádio, +0x1c mandante, +0x2a dia, +0x3f fatia do dia; ponteiros/valores; lê (adiar_pelo_clima)',
           'fixture +0x14 competition, +0x18 stadium, +0x1c home club, +0x2a day, +0x3f day slot; pointers/values; reads (adiar_pelo_clima)'),
          ('estádio +0x38 cidade, +0x3c lugares, +0x4c coberto, +0x4d marca; ponteiro/valores; lê (adiar_pelo_clima)',
           'stadium +0x38 city, +0x3c places, +0x4c covered, +0x4d flag; pointer/values; reads (adiar_pelo_clima)'),
          ('clube +0 id, +0x57 liga, +0x64 marca do mandante; id/ponteiro/valor; lê (adiar_pelo_clima, financas_do_clube)',
           'club +0 id, +0x57 league, +0x64 home flag; id/pointer/value; reads (adiar_pelo_clima, financas_do_clube)'),
          ('[0xb63d60] nações-chave (0x48 bytes) +0x39 dia do fim, +0x3b ano; valores; lê (adiar_pelo_clima)',
           '[0xb63d60] key nations (0x48 bytes) +0x39 end day, +0x3b year; values; reads (adiar_pelo_clima)'),
          ('finanças do clube (0x167 bytes em [[0xae2a38]]) +0x166 contador; valor; lê (adiar_pelo_clima)',
           'club finance (0x167 bytes at [[0xae2a38]]) +0x166 counter; value; reads (adiar_pelo_clima)'),
          ('reserva do estádio +0 id do estádio, +4 data; valores; lê (ordem_das_reservas_do_estadio)',
           'stadium booking +0 stadium id, +4 date; values; reads (ordem_das_reservas_do_estadio)')]
INTERESSA = ['CMRTE', 'GPF2']
