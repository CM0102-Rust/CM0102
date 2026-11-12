# As eliminatórias europeias da Copa: a lista de inscritas e o dia (wc_europe_league.cpp). Leitor cqz (12/11/2026), lista
# de 8 ligas.
# - 0x919e30 (WcEuropeLeague::build_entry_pool), 0x91a500 (WcEuropeLeague::update_for_date).
# A lista: o molde das listas da Oceania e da Ásia com 50 vagas e o continente europeu, mas a lista é sempre de 50
# posições (a antiga é liberada) e o laço para em 50 (não na contagem); abaixo da contagem devolve 0; o formato pelo
# número de isentas: nenhuma 8/9/10 (nove grupos), uma ou duas 7/8/9, mais 6/7/8 (+0xff..+0x101, lidos pela tabela de
# rodadas). O dia: o teste da data do molde da Oceania; no sorteio, inativa e com `full`, o sorteio, a tabela do
# próprio grupo, os jogos, os outros grupos (10, 9 ou 8 menos um, pelo número de isentas), a fase dos segundos
# colocados e as previsões; nos outros dias, no dia do último resultado, a tabela dos segundos (0x91a240).
# Convenções conferidas: thiscall, ret 0 e ret 8; 0x9196d0 (3 pushes; mov ecx -> thiscall, ret 0xc), as outras ret 0.
# Comparadores: só a original (a lista alocada; o sorteio e as fases), com o plano.
# Atenção da prova: a campanha de 2004 começa depois de 2002-06-01; a mutação do dia antecipa o sorteio.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('wc_europe_league_build_entry_pool', 0x919e30, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('wc_europe_league_update_for_date', 0x91a500, [0x6a, 0xff, 0x68, 0x76, 0x5a, 0x96, 0x00])]
DOCS = ('as eliminatórias europeias da Copa: a lista de inscritas e o dia',
        'the European World Cup qualifying: the entry pool and the day',
        [('wc_europe_league_build_entry_pool', 'WcEuropeLeague::build_entry_pool', '0x919e30', 'wc_europe_league.cpp', 'thiscall, ret 0', 'regra: as eliminatórias europeias têm 50 vagas menos as isentas (as sedes europeias da Copa seguinte e o último campeão, se europeu), com as outras seleções europeias na ordem do banco, ordenadas pela reputação; o número de isentas decide o formato: nenhuma, nove grupos; uma ou duas, oito; mais, sete', 'rule: the European qualifying has 50 places minus the exempt (the European hosts of the next World Cup and the last winner, when European), with the other European national teams in database order, sorted by reputation; the exempt count sets the format: none, nine groups; one or two, eight; more, seven'),
         ('wc_europe_league_update_for_date', 'WcEuropeLeague::update_for_date', '0x91a500', 'wc_europe_league.cpp', 'thiscall, ret 8', 'regra: na data do sorteio das eliminatórias europeias, com a atualização completa e a liga parada, os grupos são sorteados e montados com a fase dos segundos colocados; nos dias de resultado a tabela dos segundos colocados é atualizada; antes do sorteio nada roda', 'rule: on the draw date of the European qualifying, with the full update and the league not yet started, the groups are drawn and built with the runners-up stage; on result days the runners-up table is refreshed; before the draw nothing runs')])
CAMPOS = [('eliminatórias europeias +0xee lista (50 posições), +0xf2 contagem (dword), +0xf6 isentas (byte), +0xff/+0x100/+0x101 formato, +0xf7 grupos sorteados; +0x45, +0xa7, +0xc9; valor/ponteiro; lê e grava', 'European qualifying +0xee list (50 slots), +0xf2 count (dword), +0xf6 exempt (byte), +0xff/+0x100/+0x101 format, +0xf7 drawn groups; +0x45, +0xa7, +0xc9; value/pointer; reads and writes')]
