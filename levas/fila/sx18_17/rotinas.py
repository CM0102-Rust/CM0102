# As regras por pontos e a virada da temporada do órgão norte-irlandês (leitor sx18, rodada 5, 24/11/2026). Usa as pecas do sx18_1 (RulingBodyClass, CardVerdict,
# run_card_rules, compare_card_rules, pending_points_ban, as datas): integrar depois dele.
# Usa NORTHERN_IRELAND_RULING_BODY do sx18_16. A virada roda na virada da temporada norte-irlandesa (julho de 2002, dentro de ATE 2002-08-01, se a temporada virar antes); o comparador zera uma copia da tabela e confere com a original.
# Convencoes conferidas na desmontagem: construtores thiscall(arquivo) ret 4 (0x7e7760 a base, depois os campos);
# regras thiscall(linha do registro, partida) ret 8 (0x7e82e0 a entrada, 0x7e8e10 make_points_ban por jogos, a data do
# pendente por 0x54c5b0/0x54c770/0x54a130/0x54cfb0/0x54a3f0, 0x68d0d0 add_tail 0x70, 0x556940 add_ban, 0x557280 a noticia).
# Comparadores: construtores so a original e os campos contra a classe; regras pelo veredito numa copia da entrada.
SAVE = False
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
ROTINAS = [('rb_northern_ireland_process_match_cards', 0x7e0b40, [0x6a, 0xff, 0x68, 0x40, 0x03, 0x96, 0x00]),
           ('rb_northern_ireland_start_new_season', 0x7e0ec0, [0x8b, 0x51, 0x0b, 0x56, 0x8b, 0x71, 0x13])]
DOCS = ('as regras por pontos e a virada da temporada do órgão norte-irlandês',
        'the Northern Irish points rules and new season',
        [('rb_northern_ireland_process_match_cards', 'RbNorthernIreland::process_match_cards', '0x7e0b40', 'ruling_body/rb_northern_ireland.cpp', 'thiscall, ret 8',
          'regras norte-irlandesas por pontos: vermelho soma 12 se veio de amarelo e dá 1 jogo; cada amarelo soma 4; o primeiro limite de 51, 42, 32 e 20 pontos cruzado na partida dá 6, 4, 2 ou 1 jogo a mais (motivo 0, quantidade = o limite), e 51 zera os pontos; começa no primeiro dia da semana 0 a partir de hoje',
          'Northern Irish points rules: a red card adds 12 when it came from a booking and gives 1 match; each booking adds 4; the first threshold of 51, 42, 32 and 20 points crossed in the match gives 6, 4, 2 or 1 more match (reason 0, amount = the threshold), and 51 clears the points; it starts on the first weekday 0 from today'),
         ('rb_northern_ireland_start_new_season', 'RbNorthernIreland::start_new_season', '0x7e0ec0', 'ruling_body/rb_northern_ireland.cpp', 'thiscall',
          'na virada da temporada, zera os pontos de todas as entradas',
          'at the new season, clears the points of every entry')])
CAMPOS = []
INTERESSA = []
