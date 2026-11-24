# O órgão grego e o construtor do croata (leitor sx18, rodada 5, 24/11/2026). Usa as pecas do sx18_1 (RulingBodyClass, CardVerdict,
# run_card_rules, compare_card_rules, pending_points_ban, as datas): integrar depois dele.
# Rodam com a Grecia e a Croacia carregadas (ATE 2002-08-01).
# Convencoes conferidas na desmontagem: construtores thiscall(arquivo) ret 4 (0x7e7760 a base, depois os campos);
# regras thiscall(linha do registro, partida) ret 8 (0x7e82e0 a entrada, 0x7e8e10 make_points_ban por jogos, a data do
# pendente por 0x54c5b0/0x54c770/0x54a130/0x54cfb0/0x54a3f0, 0x68d0d0 add_tail 0x70, 0x556940 add_ban, 0x557280 a noticia).
# Comparadores: construtores so a original e os campos contra a classe; regras pelo veredito numa copia da entrada.
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('rb_greece_construct', 0x7de960, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_greece_process_match_cards', 0x7de9b0, [0x6a, 0xff, 0x68, 0x3c, 0x01, 0x96, 0x00]),
           ('rb_croatia_construct', 0x7dc700, [0x8b, 0x44, 0x24, 0x04, 0x56])]
DOCS = ('o órgão grego e o construtor do croata',
        "the Greek ruling body and the Croatian body's constructor",
        [('rb_greece_construct', 'RbGreece::RbGreece', '0x7de960', 'ruling_body/rb_greece.cpp', 'thiscall, ret 4',
          'monta o órgão grego (vtable 0x96ebf4, vaga 0, regra 0x20, escala 10, Grécia na Europa; liga 0x20, confederação 1) sobre a base',
          'builds the Greek (vtable 0x96ebf4, slot 0, rule 0x20, scale 10, Greece in Europe; league 0x20, confederation 1) ruling body on the base'),
         ('rb_greece_process_match_cards', 'RbGreece::process_match_cards', '0x7de9b0', 'ruling_body/rb_greece.cpp', 'thiscall, ret 8',
          'regras gregas: cada amarelo soma 1 e a cada terceiro a suspensão passa a conta/3 jogos (3 → 1, 6 → 2...); vermelho enfileira uma suspensão por pontos a partir do primeiro dia da semana 3 depois de hoje e dá 1 jogo a mais; a partir de hoje',
          'Greek rules: each booking adds 1 and at every third the ban becomes count/3 matches (3 → 1, 6 → 2...); a red card queues a points ban from the first weekday 3 after today and gives 1 more match; from today'),
         ('rb_croatia_construct', 'RbCroatia::RbCroatia', '0x7dc700', 'ruling_body/rb_croatia.cpp', 'thiscall, ret 4',
          'monta o órgão croata (vtable 0x96ead4, vaga 0, regra 0x21, escala 10, Croácia na Europa; liga 0x21, confederação 1) sobre a base',
          'builds the Croatian (vtable 0x96ead4, slot 0, rule 0x21, scale 10, Croatia in Europe; league 0x21, confederation 1) ruling body on the base')])
CAMPOS = []
INTERESSA = []
