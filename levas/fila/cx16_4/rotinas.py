# O construtor das eliminatórias europeias para a Copa do Mundo (wc_europe_league.cpp; leitor cx16, rodada 5,
# 24/11/2026). Simulação: roda na criação do jogo (a tabela das competições do setup.rs: (0x102, 0x918200, a chave da
# seção europeia)); LIGAS = 'ENG', ATE = '2002-08-01' para ir na mesma leva das outras eliminatórias (cx16_1..3, mwg_13).
# - 0x918200 WcEuropeLeague::WcEuropeLeague thiscall(ano, registro) ret 8 (quadro SEH só para os dois new): League::League
#   0x682200, a vtable 0x970cc4, o ano, o registro, +0x42 = 4, +0x1c = o valor da chave da seção africana (0x9cf77c),
#   +0x20 = -1, +0xf7/+0xee/+0xf2/+0xfb = 0, +0xf6 = 1, +0x50 = 0; o ano da campanha (a conta da
#   qualifying_campaign_year, nunca antes de 2004); a lista nova (operator new 0xe e LLIST::LLIST 0x68d020) em +0x10;
#   +0x51 = 0xf; League::load_from_temp_file 0x687b10 (push 1): carregou, devolve. Senão +0xd9 = 0x24, 0x919e30 (o
#   retorno não é lido), +0x2c pelo +0xf6 (0 -> 0xb, 1 e 2 -> 0xa, 3 -> 9, outro fica), +0x44 = 0xff, +0x30 = -1, +0xc =
#   malloc(4 x +0x2c) se positivo, +0x45 = 0; 0x9191e0 (0: caixa 0x7f); Comp_Stats (0x5ce; null: caixa 0x87); ano do jogo
#   >= o da campanha: 0x91b760 (load_2002_campaign; nunca: a campanha é no mínimo 2004 e o jogo começa em 2001).
# Convenções conferidas na desmontagem e no ponto de chamada (a tabela das competições, ret 8). Igual no cm0102-1080.exe.
# Comparador: só a original contra o plano (o retorno, a vtable, o registro, o ano da campanha, as seções acima e abaixo).
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('construct_wc_europe_league', 0x918200, [0x6a, 0xff, 0x68, 0x57, 0x59, 0x96, 0x00])]
DOCS = ('o construtor das eliminatórias europeias',
        'the European qualifying constructor',
        [('construct_wc_europe_league', 'WcEuropeLeague::WcEuropeLeague', '0x918200', 'wc_europe_league.cpp', 'thiscall, ret 8',
          'regra: as eliminatórias europeias nascem abaixo da seção africana, no ano do ciclo de quatro anos e nunca antes de 2004 (a campanha de 2002 já vem decidida); o número de fases sai do número de isentas: nenhuma, 11; uma ou duas, 10; três, 9',
          'rule: the European qualifying is born below the African section, in the year of the four-year cycle and never before 2004 (the 2002 campaign comes decided); the stage count comes from the exempt count: none, 11; one or two, 10; three, 9')])
CAMPOS = [('eliminatórias da Europa +0xf7 (dword), +0xee lista, +0xf2 contagem, +0xfb (dword), +0xf6 isentas (byte), +0x10 lista LLIST; valor/ponteiro; grava',
           'European qualifying +0xf7 (dword), +0xee list, +0xf2 count, +0xfb (dword), +0xf6 exempt (byte), +0x10 LLIST list; value/pointer; writes')]
