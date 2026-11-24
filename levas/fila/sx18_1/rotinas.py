# O orgao disciplinar ingles (rb_england.cpp: o construtor e as regras dos cartoes) e as regras dos cartoes do orgao
# europeu (rb_europe.cpp), com as pecas que os orgaos de todos os paises usam (leitor sx18, rodada 5, 24/11/2026):
# RulingBodyClass (os campos que cada construtor grava sobre a base RulingBody), CardVerdict (o que as regras decidem
# para um jogador: a suspensao na hora e a pendente da lista do orgao), run_card_rules / apply_card_verdict e o
# comparador compare_card_rules. Os outros itens sx18_* de orgaos usam estas pecas: integrar este primeiro.
# Rodam na carreira ENG: o construtor ingles na criacao do jogo (Discipline::create_rule_sets, com a Inglaterra
# selecionada), as regras inglesas depois de cada partida com cartao (RulingBody::process_match 0x7e7ff0, vtable +8,
# por linha do registro da partida) e as europeias nas partidas das copas europeias.
# Convencoes conferidas na desmontagem:
# - 0x7dcf90 thiscall(arquivo) ret 4: 0x7e7760 (base, ja em Rust: construct_ruling_body), Date::Date em +0x23 (o objeto
#   ingles e o unico alocado com 0x2b bytes em 0x55d733), vtable 0x96eb14, +4 = 0, +5 = 9, +6 = 5, +0x17 Inglaterra,
#   +0x1b Europa, +0x1f = 9, +0x20 = 0xff, +0x21 = 1, +0x22 = 0;
# - 0x7dd010 / 0x7dd450 thiscall(linha do registro, partida) ret 8; 0x7e82e0 thiscall(id, 1) ret 8; Date::Date
#   0x549ef0 (dia, ano) e 0x549f70 (1, mes, ano da temporada + 1, -1); 0x54a3f0 get_weekday; 0x54c5b0 add_days
#   thiscall(saida, dias) ret 8; 0x417c10 operator= thiscall(saida, fonte) ret 8; 0x556940 add_ban (Rust) thiscall(id,
#   registro) ret 8; 0x557280 post_ban_news thiscall(jogador, registro, partida) ret 0xc.
# Defeito do original: o byte de marcas da suspensao (+0x13) sai com os bits 0 e 3..7 do lixo da pilha (o bit 0 o
# add_ban grava; os outros ninguem le): a copia grava 0, e o comparador olha so o bit 2.
SAVE = False
ROTINAS = [('rb_england_construct', 0x7dcf90, [0x6a, 0xff, 0x68, 0x58, 0xff, 0x95, 0x00]),
           ('rb_england_process_match_cards', 0x7dd010, [0x6a, 0xff, 0x68, 0xa0, 0xff, 0x95, 0x00]),
           ('rb_europe_process_match_cards', 0x7dd450, [0x6a, 0xff, 0x68, 0xb8, 0xff, 0x95, 0x00])]
DOCS = ('o órgão disciplinar inglês e as regras dos cartões do órgão europeu',
        'the English ruling body and the European card rules',
        [('rb_england_construct', 'RbEngland::RbEngland', '0x7dcf90', 'ruling_body/rb_england.cpp', 'thiscall, ret 4',
          'monta o órgão disciplinar inglês sobre a base: uma data vazia em +0x23, a tabela virtual 0x96eb14, a vaga 0 do registro, a regra 9, a escala 5, a Inglaterra na Europa e as regras da liga (9) e da confederação (1)',
          'builds the English ruling body on the base: an empty date at +0x23, the vtable 0x96eb14, record slot 0, rule set 9, scale 5, England in Europe and the league (9) and confederation (1) rule sets'),
         ('rb_england_process_match_cards', 'RbEngland::process_match_cards', '0x7dd010', 'ruling_body/rb_england.cpp', 'thiscall, ret 8',
          'aplica os cartões de um jogador numa partida pelas regras inglesas: os amarelos somam e o limite é o próximo de 5, 10 e 15 (depois, (conta mod 5 + 1) × 5), com suspensão de limite/5 jogos; o 5º só vale em partida antes de 1º de março do segundo ano da temporada e o 10º só antes do dia seguinte ao segundo dia da semana 6 a partir de 1º de abril; o vermelho dá 1 jogo, mais 2 para falta de código 9 a 11 e mais 3 para o código 12, mais um por vermelho anterior; a suspensão começa em 14 dias, com a notícia',
          'applies a player\'s cards of a match under the English rules: bookings add up and the threshold is the next of 5, 10 and 15 (then (count mod 5 + 1) × 5), banning for threshold/5 matches; the 5th only counts in a match before 1 March of the season\'s second year and the 10th only before the day after the second weekday 6 from 1 April; a red card gives 1 match, 2 more for an offence code 9 to 11 and 3 more for code 12, plus one per earlier red card; the ban starts in 14 days, with its news'),
         ('rb_europe_process_match_cards', 'RbEurope::process_match_cards', '0x7dd450', 'ruling_body/rb_europe.cpp', 'thiscall, ret 8',
          'aplica os cartões de um jogador numa partida pelas regras europeias: um jogo pelo vermelho e um a cada terceiro amarelo (um por partida), a partir de hoje, com a notícia',
          'applies a player\'s cards of a match under the European rules: one match for a red card and one for every third booking (one per match), from today, with its news')])
CAMPOS = [('linha do registro da partida +0x48: um byte por jogador; = 0 liga o bit 2 das marcas da suspensão (lê)',
           'match record row +0x48: a per-player byte; 0 sets bit 2 of the ban flags (reads)'),
          ('órgão disciplinar +0x1f, +0x20, +0x21: as regras da liga, da copa e da confederação da área; +0x22 marca zerada (grava)',
           'ruling body +0x1f, +0x20, +0x21: the area\'s league, cup and confederation rule sets; +0x22 a cleared flag (writes)')]
INTERESSA = []
