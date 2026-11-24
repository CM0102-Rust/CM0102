# As regras dos cartões da copa italiana e os construtores dos órgãos francês e brasileiro nacional (leitor sx18, rodada 5, 24/11/2026). Usa as pecas do sx18_1 (RulingBodyClass, CardVerdict,
# run_card_rules, compare_card_rules, pending_points_ban, as datas): integrar depois dele.
# Usa ITALY_CUP_RULING_BODY do sx18_6. A copa italiana roda com as partidas da Coppa Italia.
# Convencoes conferidas na desmontagem: construtores thiscall(arquivo) ret 4 (0x7e7760 a base, depois os campos);
# regras thiscall(linha do registro, partida) ret 8 (0x7e82e0 a entrada, 0x7e8e10 make_points_ban por jogos, a data do
# pendente por 0x54c5b0/0x54c770/0x54a130/0x54cfb0/0x54a3f0, 0x68d0d0 add_tail 0x70, 0x556940 add_ban, 0x557280 a noticia).
# Comparadores: construtores so a original e os campos contra a classe; regras pelo veredito numa copia da entrada.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('rb_italy_cup_process_match_cards', 0x7df9b0, [0x6a, 0xff, 0x68, 0x5c, 0x02, 0x96, 0x00]),
           ('rb_france_construct', 0x7ddcc0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_brazil_national_construct', 0x7dc080, [0x8b, 0x44, 0x24, 0x04, 0x56])]
DOCS = ('as regras dos cartões da copa italiana e os construtores dos órgãos francês e brasileiro nacional',
        "the Italian cup card rules and the French and Brazilian national bodies' constructors",
        [('rb_italy_cup_process_match_cards', 'RbItalyCup::process_match_cards', '0x7df9b0', 'ruling_body/rb_italy_cup.cpp', 'thiscall, ret 8',
          'regras da copa italiana: vermelho dá 1 jogo e, para falta acima de 9, uma suspensão por pontos pendente a partir do primeiro dia da semana 3 depois de hoje; cada segundo amarelo (um por partida) dá 1 jogo (quantidade 2); a partir de hoje',
          'Italian cup rules: a red card gives 1 match and, for an offence above 9, a pending points ban from the first weekday 3 after today; every second booking (one per match) gives 1 match (amount 2); from today'),
         ('rb_france_construct', 'RbFrance::RbFrance', '0x7ddcc0', 'ruling_body/rb_france.cpp', 'thiscall, ret 4',
          'monta o órgão francês (vtable 0x96eb94, vaga 0, regra 0xc, escala 5, França na Europa; liga 0xc, confederação 1) sobre a base',
          'builds the French (vtable 0x96eb94, slot 0, rule 0xc, scale 5, France in Europe; league 0xc, confederation 1) ruling body on the base'),
         ('rb_brazil_national_construct', 'RbBrazilNational::RbBrazilNational', '0x7dc080', 'ruling_body/rb_brazil_national.cpp', 'thiscall, ret 4',
          'monta o órgão brasileiro nacional (vtable 0x96ea94, vaga 0, regra 6, escala 5, Brasil na América do Sul; liga 6, copa 7, confederação 2) sobre a base',
          'builds the Brazilian national (vtable 0x96ea94, slot 0, rule 6, scale 5, Brazil in South America; league 6, cup 7, confederation 2) ruling body on the base')])
CAMPOS = []
INTERESSA = []
