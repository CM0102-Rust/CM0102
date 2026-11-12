# O dia da Copa da Ásia, do futebol olímpico e da Copa das Confederações (asia_nations.cpp, olympics.cpp,
# fifa_confed.cpp). Leitor cqz (12/11/2026), lista de 8 ligas.
# - 0x410170 (AsiaNations::update_for_date), 0x7a11c0 (Olympics::update_for_date), 0x58c4b0 (FifaConfed::update_for_date).
# O mesmo molde da Copa das Nações da Oceania (0x79a100, já em Rust: oceania_nations_draw_date), com as diferenças
# conferidas na desmontagem: a Ásia roda o dia da liga direto sem `full` e zera +0xf2 como word, dois grupos-fase (0
# e 1); a Olimpíada e a Copa das Confederações testam a data mesmo sem `full` (antes do sorteio devolvem 0 sem o dia
# da liga) e só sorteiam num dia SEM `full` (estranheza do original: num dia com `full` o sorteio não roda), zeram
# +0xf2 como dword; a Olimpíada monta 3 grupos-fase (0..2), a Copa das Confederações um só (sem argumento).
# Convenções conferidas: thiscall, ret 8 (virtual +0x14); o sorteio, a tabela do grupo A e os grupos-fase sem
# argumento (mov ecx -> thiscall, ret 0) ou com o índice (push k -> ret 4); 0x6835c0 (ret 0), 0x6827d0 (push 0 ->
# ret 4), 0x6847c0 (2 pushes -> ret 8), 0x9452ca (cdecl).
# Comparadores: só a original (o sorteio, as tabelas e o dia da liga não rodam duas vezes); no dia do sorteio, o plano.
# Atenção da prova: os sorteios (5/6/2004, 20/3/2004, 20/2/2003) ficam depois de 2002-06-01 e o dia da liga de uma copa
# inativa devolve 0 sem efeito: até lá a rotina não muda nada; as mutações antecipam o sorteio (ver mutacoes.py).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('asia_nations_update_for_date', 0x410170, [0x6a, 0xff, 0x68, 0x66, 0x39, 0x95, 0x00]),
           ('olympics_update_for_date', 0x7a11c0, [0x6a, 0xff, 0x68, 0x66, 0xf0, 0x95, 0x00]),
           ('fifa_confed_update_for_date', 0x58c4b0, [0x6a, 0xff, 0x68, 0xeb, 0x8d, 0x95, 0x00])]
DOCS = ('o dia da Copa da Ásia, do futebol olímpico e da Copa das Confederações',
        'the daily update of the Asian Cup, the Olympic football and the Confederations Cup',
        [('asia_nations_update_for_date', 'AsiaNations::update_for_date', '0x410170', 'asia_nations.cpp', 'thiscall, ret 8', 'regra: no dia do sorteio da Copa da Ásia (5 de junho do ano da copa) as finalistas são sorteadas em três grupos; o grupo A ganha a tabela e os jogos e os outros dois as suas fases', 'rule: on the draw day of the Asian Cup (5 June of the cup year) the finalists are drawn into three groups; group A gets its table and fixtures and the other two their stages'),
         ('olympics_update_for_date', 'Olympics::update_for_date', '0x7a11c0', 'olympics.cpp', 'thiscall, ret 8', 'regra: no dia do sorteio olímpico (20 de março do ano olímpico) as finalistas são sorteadas em quatro grupos; antes do sorteio nada roda, nem o dia da liga; o sorteio só acontece num dia sem atualização completa', 'rule: on the Olympic draw day (20 March of the Olympic year) the finalists are drawn into four groups; before the draw nothing runs, not even the league day; the draw only happens on a day without the full update'),
         ('fifa_confed_update_for_date', 'FifaConfed::update_for_date', '0x58c4b0', 'fifa_confed.cpp', 'thiscall, ret 8', 'regra: no dia do sorteio da Copa das Confederações (20 de fevereiro do ano da copa) as seleções são escolhidas e sorteadas em dois grupos; como na Olimpíada, nada roda antes e o sorteio só num dia sem atualização completa', 'rule: on the draw day of the Confederations Cup (20 February of the cup year) the nations are chosen and drawn into two groups; as in the Olympics, nothing runs before and the draw only on a day without the full update')])
CAMPOS = [('competição +0x45 ativa, +0xee lista sorteada (liberada), +0xf2 contagem (word na Ásia, dword nas outras), +0xb6/+0xb8 data do sorteio; valor/ponteiro; lê e grava', 'competition +0x45 active, +0xee drawn list (freed), +0xf2 count (word in Asia, dword in the others), +0xb6/+0xb8 draw date; value/pointer; reads and writes')]
