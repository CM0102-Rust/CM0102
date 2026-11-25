# O construtor, as regras e a tabela da Football Conference (eng_conf.cpp; leitor rx10, rodada 6, 25/11/2026).
# Carreira ENG com a base NORMAL: a Conference so e criada com a divisao extra inglesa carregada (o FA Trophy do sbF_9
# tambem); com a base padrao nenhuma roda. As tres rodam na criacao do jogo (e regras e tabela de novo na virada).
# - 0x56db60 EngConf::EngConf thiscall(ano, registro) ret 8: o molde construct_english_division (regras, tabela,
#   jogos, ordem, estatisticas, posicoes previstas 0x68a850; caixas 0x4c, 0x53, 0x5a, 0x63, 0x6b), mas o estadio
#   minimo (+0xe2 = 1000, +0xe4 = 600) so e gravado depois do teste do estado salvo (0x687b10 push 1); +0x2c = 0 e
#   +0xc = 0 (sem fase extra). Prologo push -1; push 0x958249 (7 bytes).
# - 0x56ede0 EngConf::set_up_rules thiscall() ret 0: os campos (sem o +0x52 das quatro divisoes; +0xc5 gravado duas
#   vezes com 1), acima a Third (0x9cf5c8), abaixo -1, e a tabela de rodadas pela virtual +0x3c(-1, +0xa9, +0x3a, 0).
#   Prologo push ebx; push esi; mov esi, ecx; mov al, 3 (6 bytes).
# - 0x56ecb0 EngConf::create_table thiscall() ret 0: eng_prm_fill_league_table com 22 clubes (0x9cf69c), caixa 0x1f3,
#   fundo 0x682f70(5000), TV 0x683010(100000, 0). Prologo sub esp, 0x200 (6 bytes).
# Iguais no cm0102-1080.exe. Comparadores: o construtor e a tabela so a original com o plano; as regras em ensaio.
SAVE = False
BASE = 'NORMAL'
ROTINAS = [('construct_eng_conf', 0x56db60, [0x6a, 0xff, 0x68, 0x49, 0x82, 0x95, 0x00]),
           ('eng_conf_set_up_rules', 0x56ede0, [0x53, 0x56, 0x8b, 0xf1, 0xb0, 0x03]),
           ('eng_conf_create_table', 0x56ecb0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o construtor, as regras e a tabela da Football Conference',
        'the Football Conference constructor, rules and table',
        [('construct_eng_conf', 'EngConf::EngConf', '0x56db60', 'eng_conf.cpp', 'thiscall, ret 8',
          'regra: a Football Conference é uma liga do ano do jogo com as regras disciplinares inglesas e estádio mínimo de 1.000 lugares (600 sentados); sem estado salvo, monta as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas',
          'rule: the Football Conference is a league of the game\'s year under the English discipline rules, with a minimum ground of 1,000 places (600 seated); with nothing saved, it builds the rules, the table, the fixtures, the order, the statistics and the predicted positions'),
         ('eng_conf_set_up_rules', 'EngConf::set_up_rules', '0x56ede0', 'eng_conf.cpp', 'thiscall, ret 0',
          'regra: a Conference tem turno e returno, 3 pontos por vitória e 1 por empate; o campeão sobe para a Third Division e três caem, sem liga abaixo; 5 reservas relacionados e 3 substituições',
          'rule: the Conference is played home and away, 3 points a win and 1 a draw; the champion goes up to the Third Division and three go down, with no league below; 5 substitutes named and 3 used'),
         ('eng_conf_create_table', 'EngConf::create_table', '0x56ecb0', 'eng_conf.cpp', 'thiscall, ret 0',
          'regra: a Conference tem 22 clubes; fundo de prêmios pela cota de 5.000 por posição; a TV paga 100.000 a cada clube',
          'rule: the Conference has 22 clubs; prize fund by the share of 5,000 per place; TV pays 100,000 to each club')])
CAMPOS = []
INTERESSA = []
