# O órgão da liga belga e o construtor do da copa (leitor sx18, rodada 5, 24/11/2026). Usa as pecas do sx18_1 (RulingBodyClass, CardVerdict,
# run_card_rules, compare_card_rules, pending_points_ban, as datas): integrar depois dele.
# Rodam com a Belgica carregada (ATE 2002-08-01, a cadeia das ligas BEL HOL GER DEN).
# Convencoes conferidas na desmontagem: construtores thiscall(arquivo) ret 4 (0x7e7760 a base, depois os campos);
# regras thiscall(linha do registro, partida) ret 8 (0x7e82e0 a entrada, 0x7e8e10 make_points_ban por jogos, a data do
# pendente por 0x54c5b0/0x54c770/0x54a130/0x54cfb0/0x54a3f0, 0x68d0d0 add_tail 0x70, 0x556940 add_ban, 0x557280 a noticia).
# Comparadores: construtores so a original e os campos contra a classe; regras pelo veredito numa copia da entrada.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('rb_belgium_league_construct', 0x7dbca0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_belgium_league_process_match_cards', 0x7dbcf0, [0x6a, 0xff, 0x68, 0x46, 0xfe, 0x95, 0x00]),
           ('rb_belgium_cup_construct', 0x7db8f0, [0x8b, 0x44, 0x24, 0x04, 0x56])]
DOCS = ('o órgão da liga belga e o construtor do da copa',
        "the Belgian league ruling body and the Belgian cup body's constructor",
        [('rb_belgium_league_construct', 'RbBelgiumLeague::RbBelgiumLeague', '0x7dbca0', 'ruling_body/rb_belgium_league.cpp', 'thiscall, ret 4',
          'monta o órgão da liga belga (vtable 0x96ea74, vaga 0, regra 4, escala 5, Bélgica na Europa; liga 4, copa 5, confederação 1) sobre a base',
          'builds the Belgian league (vtable 0x96ea74, slot 0, rule 4, scale 5, Belgium in Europe; league 4, cup 5, confederation 1) ruling body on the base'),
         ('rb_belgium_league_process_match_cards', 'RbBelgiumLeague::process_match_cards', '0x7dbcf0', 'ruling_body/rb_belgium_league.cpp', 'thiscall, ret 8',
          'regras belgas da liga: vermelho enfileira uma suspensão por pontos a partir do dia 15 deste mês (do seguinte, se hoje passou dele) e, só se veio de amarelo na partida, dá 1 jogo na hora; cada terceiro amarelo dá 1 jogo (quantidade 3); a partir de hoje',
          'Belgian league rules: a red card queues a points ban from the 15th of this month (the next if today is past it) and, only after a booking in the match, gives 1 match on the spot; every third booking gives 1 match (amount 3); from today'),
         ('rb_belgium_cup_construct', 'RbBelgiumCup::RbBelgiumCup', '0x7db8f0', 'ruling_body/rb_belgium_cup.cpp', 'thiscall, ret 4',
          'monta o órgão da copa belga (vtable 0x96ea54, vaga 1, regra 5, escala 5) sobre a base',
          'builds the Belgian cup (vtable 0x96ea54, slot 1, rule 5, scale 5) ruling body on the base')])
CAMPOS = []
INTERESSA = []
