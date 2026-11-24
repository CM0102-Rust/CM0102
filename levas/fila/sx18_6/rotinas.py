# O órgão disciplinar da liga italiana e o construtor do da copa (leitor sx18, rodada 5, 24/11/2026). Usa as pecas do sx18_1 (RulingBodyClass, CardVerdict,
# run_card_rules, compare_card_rules, pending_points_ban, as datas): integrar depois dele.
# Rodam com a Italia carregada: os construtores na criacao do jogo, as regras depois de cada partida com cartao da Serie A/B.
# Convencoes conferidas na desmontagem: construtores thiscall(arquivo) ret 4 (0x7e7760 a base, depois os campos);
# regras thiscall(linha do registro, partida) ret 8 (0x7e82e0 a entrada, 0x7e8e10 make_points_ban por jogos, a data do
# pendente por 0x54c5b0/0x54c770/0x54a130/0x54cfb0/0x54a3f0, 0x68d0d0 add_tail 0x70, 0x556940 add_ban, 0x557280 a noticia).
# Comparadores: construtores so a original e os campos contra a classe; regras pelo veredito numa copia da entrada.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('rb_italy_league_construct', 0x7dfd70, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_italy_league_process_match_cards', 0x7dfdc0, [0x6a, 0xff, 0x68, 0xac, 0x02, 0x96, 0x00]),
           ('rb_italy_cup_construct', 0x7df960, [0x8b, 0x44, 0x24, 0x04, 0x56])]
DOCS = ('o órgão disciplinar da liga italiana e o construtor do da copa',
        "the Italian league ruling body and the Italian cup body's constructor",
        [('rb_italy_league_construct', 'RbItalyLeague::RbItalyLeague', '0x7dfd70', 'ruling_body/rb_italy_league.cpp', 'thiscall, ret 4',
          'monta o órgão da liga italiana (vtable 0x96ec94, vaga 0, regra 0x11, escala 8, Itália na Europa; liga 0x11, copa 0x12, confederação 1) sobre a base',
          'builds the Italian league (vtable 0x96ec94, slot 0, rule 0x11, scale 8, Italy in Europe; league 0x11, cup 0x12, confederation 1) ruling body on the base'),
         ('rb_italy_league_process_match_cards', 'RbItalyLeague::process_match_cards', '0x7dfdc0', 'ruling_body/rb_italy_league.cpp', 'thiscall, ret 8',
          'regras italianas da liga: vermelho dá 1 jogo e, para falta de código acima de 9, uma suspensão por pontos pendente a partir do primeiro dia da semana 3 depois de hoje; amarelos (um por partida) suspendem no 4º (quantidade 4), no 7º (3) e em cada ímpar a partir do 9º (2); a partir de hoje',
          'Italian league rules: a red card gives 1 match and, for an offence code above 9, a pending points ban from the first weekday 3 after today; bookings (one per match) ban at the 4th (amount 4), the 7th (3) and each odd one from the 9th (2); from today'),
         ('rb_italy_cup_construct', 'RbItalyCup::RbItalyCup', '0x7df960', 'ruling_body/rb_italy_cup.cpp', 'thiscall, ret 4',
          'monta o órgão da copa italiana (vtable 0x96ec74, vaga 1, regra 0x12, escala 8) sobre a base',
          'builds the Italian cup (vtable 0x96ec74, slot 1, rule 0x12, scale 8) ruling body on the base')])
CAMPOS = []
INTERESSA = []
