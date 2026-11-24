# O órgão da liga japonesa e o construtor do da copa (leitor sx18, rodada 5, 24/11/2026). Usa as pecas do sx18_1 (RulingBodyClass, CardVerdict,
# run_card_rules, compare_card_rules, pending_points_ban, as datas): integrar depois dele.
# Rodam com o Japao carregado.
# Convencoes conferidas na desmontagem: construtores thiscall(arquivo) ret 4 (0x7e7760 a base, depois os campos);
# regras thiscall(linha do registro, partida) ret 8 (0x7e82e0 a entrada, 0x7e8e10 make_points_ban por jogos, a data do
# pendente por 0x54c5b0/0x54c770/0x54a130/0x54cfb0/0x54a3f0, 0x68d0d0 add_tail 0x70, 0x556940 add_ban, 0x557280 a noticia).
# Comparadores: construtores so a original e os campos contra a classe; regras pelo veredito numa copia da entrada.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('rb_japan_league_construct', 0x7e0490, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_japan_league_process_match_cards', 0x7e04e0, [0x6a, 0xff, 0x68, 0xe8, 0x02, 0x96, 0x00]),
           ('rb_japan_cup_construct', 0x7e01b0, [0x8b, 0x44, 0x24, 0x04, 0x56])]
DOCS = ('o órgão da liga japonesa e o construtor do da copa',
        "the Japanese league ruling body and the Japanese cup body's constructor",
        [('rb_japan_league_construct', 'RbJapanLeague::RbJapanLeague', '0x7e0490', 'ruling_body/rb_japan_league.cpp', 'thiscall, ret 4',
          'monta o órgão da liga japonesa (vtable 0x96ecd4, vaga 0, regra 0x13, escala 10, Japão na Ásia; liga 0x13, copa 0x14, sem confederação) sobre a base',
          'builds the Japanese league (vtable 0x96ecd4, slot 0, rule 0x13, scale 10, Japan in Asia; league 0x13, cup 0x14, no confederation) ruling body on the base'),
         ('rb_japan_league_process_match_cards', 'RbJapanLeague::process_match_cards', '0x7e04e0', 'ruling_body/rb_japan_league.cpp', 'thiscall, ret 8',
          'regras japonesas da liga: os amarelos da partida somam e cada terceiro dá 1 jogo (quantidade 3); vermelho dá 1 jogo pela falta 9 ou sem código, 2 pela 10, 3 pela 11, 4 pela 12; a partir de hoje',
          "Japanese league rules: the match's bookings add up and every third gives 1 match (amount 3); a red card gives 1 match for offence 9 or no code, 2 for 10, 3 for 11, 4 for 12; from today"),
         ('rb_japan_cup_construct', 'RbJapanCup::RbJapanCup', '0x7e01b0', 'ruling_body/rb_japan_cup.cpp', 'thiscall, ret 4',
          'monta o órgão da copa japonesa (vtable 0x96ecb4, vaga 1, regra 0x14, escala 10) sobre a base',
          'builds the Japanese cup (vtable 0x96ecb4, slot 1, rule 0x14, scale 10) ruling body on the base')])
CAMPOS = []
INTERESSA = []
