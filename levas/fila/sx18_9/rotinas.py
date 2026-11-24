# As regras dos cartões brasileiras, nacional e regional, e o construtor do órgão regional (leitor sx18, rodada 5, 24/11/2026). Usa as pecas do sx18_1 (RulingBodyClass, CardVerdict,
# run_card_rules, compare_card_rules, pending_points_ban, as datas): integrar depois dele.
# Usa BRAZIL_NATIONAL_RULING_BODY do sx18_7.
# Convencoes conferidas na desmontagem: construtores thiscall(arquivo) ret 4 (0x7e7760 a base, depois os campos);
# regras thiscall(linha do registro, partida) ret 8 (0x7e82e0 a entrada, 0x7e8e10 make_points_ban por jogos, a data do
# pendente por 0x54c5b0/0x54c770/0x54a130/0x54cfb0/0x54a3f0, 0x68d0d0 add_tail 0x70, 0x556940 add_ban, 0x557280 a noticia).
# Comparadores: construtores so a original e os campos contra a classe; regras pelo veredito numa copia da entrada.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('rb_brazil_national_process_match_cards', 0x7dc0d0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('rb_brazil_regional_construct', 0x7dc3c0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_brazil_regional_process_match_cards', 0x7dc410, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as regras dos cartões brasileiras, nacional e regional, e o construtor do órgão regional',
        "the Brazilian card rules, national and regional, and the regional body's constructor",
        [('rb_brazil_national_process_match_cards', 'RbBrazilNational::process_match_cards', '0x7dc0d0', 'ruling_body/rb_brazil_national.cpp', 'thiscall, ret 8',
          'regras brasileiras nacionais: vermelho conta na entrada e dá 1 jogo, e a partir do segundo vermelho uma suspensão por pontos pendente a partir de 7 dias (os amarelos da partida não contam); senão os amarelos somam e a cada terceiro dão 1 jogo (quantidade 3), tirando 3 da conta; a partir de hoje',
          "Brazilian national rules: a red card counts in the entry and gives 1 match, and from the second red card a pending points ban from 7 days on (the match's bookings do not count); otherwise bookings add up and every third gives 1 match (amount 3), taking 3 off the count; from today"),
         ('rb_brazil_regional_construct', 'RbBrazilRegional::RbBrazilRegional', '0x7dc3c0', 'ruling_body/rb_brazil_regional.cpp', 'thiscall, ret 4',
          'monta o órgão brasileiro regional (vtable 0x96eab4, vaga 1, regra 7, escala 5) sobre a base',
          'builds the Brazilian regional (vtable 0x96eab4, slot 1, rule 7, scale 5) ruling body on the base'),
         ('rb_brazil_regional_process_match_cards', 'RbBrazilRegional::process_match_cards', '0x7dc410', 'ruling_body/rb_brazil_regional.cpp', 'thiscall, ret 8',
          'as mesmas regras brasileiras, na regra regional (segunda vaga)',
          'the same Brazilian rules, under the regional rule set (second slot)')])
CAMPOS = []
INTERESSA = []
