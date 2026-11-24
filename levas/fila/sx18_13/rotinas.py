# As regras dos cartões da copa belga e o órgão holandês (leitor sx18, rodada 5, 24/11/2026). Usa as pecas do sx18_1 (RulingBodyClass, CardVerdict,
# run_card_rules, compare_card_rules, pending_points_ban, as datas): integrar depois dele.
# Usa BELGIUM_CUP_RULING_BODY do sx18_12.
# Convencoes conferidas na desmontagem: construtores thiscall(arquivo) ret 4 (0x7e7760 a base, depois os campos);
# regras thiscall(linha do registro, partida) ret 8 (0x7e82e0 a entrada, 0x7e8e10 make_points_ban por jogos, a data do
# pendente por 0x54c5b0/0x54c770/0x54a130/0x54cfb0/0x54a3f0, 0x68d0d0 add_tail 0x70, 0x556940 add_ban, 0x557280 a noticia).
# Comparadores: construtores so a original e os campos contra a classe; regras pelo veredito numa copia da entrada.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('rb_belgium_cup_process_match_cards', 0x7db940, [0x6a, 0xff, 0x68, 0x16, 0xfe, 0x95, 0x00]),
           ('rb_holland_construct', 0x7ded90, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_holland_process_match_cards', 0x7dede0, [0x6a, 0xff, 0x68, 0x8c, 0x01, 0x96, 0x00])]
DOCS = ('as regras dos cartões da copa belga e o órgão holandês',
        'the Belgian cup card rules and the Dutch ruling body',
        [('rb_belgium_cup_process_match_cards', 'RbBelgiumCup::process_match_cards', '0x7db940', 'ruling_body/rb_belgium_cup.cpp', 'thiscall, ret 8',
          'regras da copa belga: vermelho enfileira uma suspensão por pontos a partir do dia 15 e dá 1 jogo na hora; cada terceiro amarelo dá 1 jogo (quantidade 3)',
          'Belgian cup rules: a red card queues a points ban from the 15th and gives 1 match on the spot; every third booking gives 1 match (amount 3)'),
         ('rb_holland_construct', 'RbHolland::RbHolland', '0x7ded90', 'ruling_body/rb_holland.cpp', 'thiscall, ret 4',
          'monta o órgão holandês (vtable 0x96ec14, vaga 0, regra 0xf, escala 0x11, Holanda na Europa; liga 0xf, confederação 1) sobre a base',
          'builds the Dutch (vtable 0x96ec14, slot 0, rule 0xf, scale 0x11, Holland in Europe; league 0xf, confederation 1) ruling body on the base'),
         ('rb_holland_process_match_cards', 'RbHolland::process_match_cards', '0x7dede0', 'ruling_body/rb_holland.cpp', 'thiscall, ret 8',
          'regras holandesas: cada amarelo soma 1; o 3º e cada um a partir do 5º dão 1 jogo (quantidade = a conta); vermelho enfileira uma suspensão por pontos a partir do primeiro dia da semana 3 depois de hoje e dá 1 jogo; a partir de hoje',
          'Dutch rules: each booking adds 1; the 3rd and each one from the 5th give 1 match (amount = the count); a red card queues a points ban from the first weekday 3 after today and gives 1 match; from today')])
CAMPOS = []
INTERESSA = []
