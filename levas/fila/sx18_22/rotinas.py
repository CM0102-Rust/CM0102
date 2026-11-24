# O órgão argentino e o construtor do sul-coreano (leitor sx18, rodada 5, 24/11/2026). Usa as pecas do sx18_1 (RulingBodyClass, CardVerdict,
# run_card_rules, compare_card_rules, pending_points_ban, as datas): integrar depois dele.
# Rodam com a Argentina e a Coreia do Sul carregadas (ATE 2002-08-01).
# Convencoes conferidas na desmontagem: construtores thiscall(arquivo) ret 4 (0x7e7760 a base, depois os campos);
# regras thiscall(linha do registro, partida) ret 8 (0x7e82e0 a entrada, 0x7e8e10 make_points_ban por jogos, a data do
# pendente por 0x54c5b0/0x54c770/0x54a130/0x54cfb0/0x54a3f0, 0x68d0d0 add_tail 0x70, 0x556940 add_ban, 0x557280 a noticia).
# Comparadores: construtores so a original e os campos contra a classe; regras pelo veredito numa copia da entrada.
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
ROTINAS = [('rb_argentina_construct', 0x7db100, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_argentina_process_match_cards', 0x7db150, [0x6a, 0xff, 0x68, 0x88, 0xfd, 0x95, 0x00]),
           ('rb_korea_construct', 0x7e0730, [0x8b, 0x44, 0x24, 0x04, 0x56])]
DOCS = ('o órgão argentino e o construtor do sul-coreano',
        "the Argentine ruling body and the South Korean body's constructor",
        [('rb_argentina_construct', 'RbArgentina::RbArgentina', '0x7db100', 'ruling_body/rb_argentina.cpp', 'thiscall, ret 4',
          'monta o órgão argentino (vtable 0x96e9f4, vaga 0, regra 3, escala 10, Argentina na América do Sul; liga 3, confederação 2) sobre a base',
          'builds the Argentine (vtable 0x96e9f4, slot 0, rule 3, scale 10, Argentina in South America; league 3, confederation 2) ruling body on the base'),
         ('rb_argentina_process_match_cards', 'RbArgentina::process_match_cards', '0x7db150', 'ruling_body/rb_argentina.cpp', 'thiscall, ret 8',
          'regras argentinas: vermelho dá 1 jogo; os amarelos da partida somam e cada múltiplo de 5 dá 1 jogo (quantidade 5); a partir de hoje',
          "Argentine rules: a red card gives 1 match; the match's bookings add up and each multiple of 5 gives 1 match (amount 5); from today"),
         ('rb_korea_construct', 'RbKorea::RbKorea', '0x7e0730', 'ruling_body/rb_korea.cpp', 'thiscall, ret 4',
          'monta o órgão sul-coreano (vtable 0x96ecf4, vaga 0, regra 0x2a, escala 8, Coreia do Sul na Ásia; liga 0x2a, confederação 0x29) sobre a base',
          'builds the South Korean (vtable 0x96ecf4, slot 0, rule 0x2a, scale 8, South Korea in Asia; league 0x2a, confederation 0x29) ruling body on the base')])
CAMPOS = []
INTERESSA = []
