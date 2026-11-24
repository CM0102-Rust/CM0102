# As regras dos cartões da copa japonesa e o órgão australiano (leitor sx18, rodada 5, 24/11/2026). Usa as pecas do sx18_1 (RulingBodyClass, CardVerdict,
# run_card_rules, compare_card_rules, pending_points_ban, as datas): integrar depois dele.
# Usa japan_card_verdict e JAPAN_CUP_RULING_BODY do sx18_10.
# Convencoes conferidas na desmontagem: construtores thiscall(arquivo) ret 4 (0x7e7760 a base, depois os campos);
# regras thiscall(linha do registro, partida) ret 8 (0x7e82e0 a entrada, 0x7e8e10 make_points_ban por jogos, a data do
# pendente por 0x54c5b0/0x54c770/0x54a130/0x54cfb0/0x54a3f0, 0x68d0d0 add_tail 0x70, 0x556940 add_ban, 0x557280 a noticia).
# Comparadores: construtores so a original e os campos contra a classe; regras pelo veredito numa copia da entrada.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('rb_japan_cup_process_match_cards', 0x7e0200, [0x6a, 0xff, 0x68, 0xc8, 0x02, 0x96, 0x00]),
           ('rb_australia_construct', 0x7db5b0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_australia_process_match_cards', 0x7db600, [0x6a, 0xff, 0x68, 0xdb, 0xfd, 0x95, 0x00])]
DOCS = ('as regras dos cartões da copa japonesa e o órgão australiano',
        'the Japanese cup card rules and the Australian ruling body',
        [('rb_japan_cup_process_match_cards', 'RbJapanCup::process_match_cards', '0x7e0200', 'ruling_body/rb_japan_cup.cpp', 'thiscall, ret 8',
          'regras da copa japonesa: cada segundo amarelo dá 1 jogo (quantidade 2); vermelho dá 1 a 4 jogos pela falta 9 a 12 (nada sem código)',
          'Japanese cup rules: every second booking gives 1 match (amount 2); a red card gives 1 to 4 matches for offence 9 to 12 (nothing with no code)'),
         ('rb_australia_construct', 'RbAustralia::RbAustralia', '0x7db5b0', 'ruling_body/rb_australia.cpp', 'thiscall, ret 4',
          'monta o órgão australiano (vtable 0x96ea34, vaga 0, regra 0x22, escala 10, Austrália na Oceania; liga 0x22, confederação 0x23) sobre a base',
          'builds the Australian (vtable 0x96ea34, slot 0, rule 0x22, scale 10, Australia in Oceania; league 0x22, confederation 0x23) ruling body on the base'),
         ('rb_australia_process_match_cards', 'RbAustralia::process_match_cards', '0x7db600', 'ruling_body/rb_australia.cpp', 'thiscall, ret 8',
          'regras australianas: partida com um só amarelo soma 1 e suspende no 5º e em cada ímpar a partir do 7º (quantidade = a conta); vermelho conta e dá 1 jogo, 2 a partir do terceiro vermelho; a partir de hoje',
          'Australian rules: a match with exactly one booking adds 1 and bans at the 5th and each odd one from the 7th (amount = the count); a red card counts and gives 1 match, 2 from the third red card; from today')])
CAMPOS = []
INTERESSA = []
