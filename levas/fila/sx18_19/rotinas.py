# As regras dos cartões da copa finlandesa e o órgão da copa norueguesa (leitor sx18, rodada 5, 24/11/2026). Usa as pecas do sx18_1 (RulingBodyClass, CardVerdict,
# run_card_rules, compare_card_rules, pending_points_ban, as datas): integrar depois dele.
# Usa finland_card_verdict e FINLAND_CUP_RULING_BODY do sx18_18.
# Convencoes conferidas na desmontagem: construtores thiscall(arquivo) ret 4 (0x7e7760 a base, depois os campos);
# regras thiscall(linha do registro, partida) ret 8 (0x7e82e0 a entrada, 0x7e8e10 make_points_ban por jogos, a data do
# pendente por 0x54c5b0/0x54c770/0x54a130/0x54cfb0/0x54a3f0, 0x68d0d0 add_tail 0x70, 0x556940 add_ban, 0x557280 a noticia).
# Comparadores: construtores so a original e os campos contra a classe; regras pelo veredito numa copia da entrada.
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
ROTINAS = [('rb_finland_cup_process_match_cards', 0x7dd6b0, [0x6a, 0xff, 0x68, 0xf6, 0xff, 0x95, 0x00]),
           ('rb_norway_cup_construct', 0x7e0ef0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_norway_cup_process_match_cards', 0x7e0f40, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as regras dos cartões da copa finlandesa e o órgão da copa norueguesa',
        'the Finnish cup card rules and the Norwegian cup ruling body',
        [('rb_finland_cup_process_match_cards', 'RbFinlandCup::process_match_cards', '0x7dd6b0', 'ruling_body/rb_finland_cup.cpp', 'thiscall, ret 8',
          'as regras finlandesas na copa (regra 0xb na segunda vaga)',
          'the Finnish rules in the cup (rule 0xb in the second slot)'),
         ('rb_norway_cup_construct', 'RbNorwayCup::RbNorwayCup', '0x7e0ef0', 'ruling_body/rb_norway_cup.cpp', 'thiscall, ret 4',
          'monta o órgão da copa norueguesa (vtable 0x96ed34, vaga 1, regra 0x16, escala 10, Noruega na Europa; liga 0x15, copa 0x16, confederação 1) sobre a base',
          'builds the Norwegian cup (vtable 0x96ed34, slot 1, rule 0x16, scale 10, Norway in Europe; league 0x15, cup 0x16, confederation 1) ruling body on the base'),
         ('rb_norway_cup_process_match_cards', 'RbNorwayCup::process_match_cards', '0x7e0f40', 'ruling_body/rb_norway_cup.cpp', 'thiscall, ret 8',
          'regras da copa norueguesa: sempre 1 jogo; pelo vermelho (os amarelos da partida não contam) ou por todo amarelo a partir do segundo (quantidade 2 no segundo, depois 1); a partir de hoje',
          "Norwegian cup rules: always 1 match; for a red card (the match's bookings do not count) or for every booking from the second (amount 2 at the second, then 1); from today")])
CAMPOS = []
INTERESSA = []
