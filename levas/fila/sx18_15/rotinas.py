# As regras dos cartões da copa alemã e o órgão dinamarquês (leitor sx18, rodada 5, 24/11/2026). Usa as pecas do sx18_1 (RulingBodyClass, CardVerdict,
# run_card_rules, compare_card_rules, pending_points_ban, as datas): integrar depois dele.
# Usa germany_card_verdict e as classes alemas do sx18_14.
# Convencoes conferidas na desmontagem: construtores thiscall(arquivo) ret 4 (0x7e7760 a base, depois os campos);
# regras thiscall(linha do registro, partida) ret 8 (0x7e82e0 a entrada, 0x7e8e10 make_points_ban por jogos, a data do
# pendente por 0x54c5b0/0x54c770/0x54a130/0x54cfb0/0x54a3f0, 0x68d0d0 add_tail 0x70, 0x556940 add_ban, 0x557280 a noticia).
# Comparadores: construtores so a original e os campos contra a classe; regras pelo veredito numa copia da entrada.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('rb_germany_cup_process_match_cards', 0x7de330, [0x6a, 0xff, 0x68, 0xc6, 0x00, 0x96, 0x00]),
           ('rb_denmark_construct', 0x7dcae0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_denmark_process_match_cards', 0x7dcb30, [0x6a, 0xff, 0x68, 0x46, 0xff, 0x95, 0x00])]
DOCS = ('as regras dos cartões da copa alemã e o órgão dinamarquês',
        'the German cup card rules and the Danish ruling body',
        [('rb_germany_cup_process_match_cards', 'RbGermanyCup::process_match_cards', '0x7de330', 'ruling_body/rb_germany_cup.cpp', 'thiscall, ret 8',
          'as regras alemãs na copa (regra 0xe na segunda vaga); o vermelho direto vale também na liga',
          'the German rules in the cup (rule 0xe in the second slot); a straight red card also counts in the league'),
         ('rb_denmark_construct', 'RbDenmark::RbDenmark', '0x7dcae0', 'ruling_body/rb_denmark.cpp', 'thiscall, ret 4',
          'monta o órgão dinamarquês (vtable 0x96eaf4, vaga 0, regra 8, escala 1, Dinamarca na Europa; liga 8, confederação 1) sobre a base',
          'builds the Danish (vtable 0x96eaf4, slot 0, rule 8, scale 1, Denmark in Europe; league 8, confederation 1) ruling body on the base'),
         ('rb_denmark_process_match_cards', 'RbDenmark::process_match_cards', '0x7dcb30', 'ruling_body/rb_denmark.cpp', 'thiscall, ret 8',
          'regras dinamarquesas por pontos: vermelho enfileira uma suspensão por pontos a partir do dia 24 e soma 9 (segundo amarelo) ou 24 pontos; cada amarelo soma 3, 4 ou 6 pela falta; cada limite (15, 30, depois de 14 em 14) cruzado na partida dá 1 jogo (motivo 0, quantidade = o total); a partir de hoje',
          'Danish points rules: a red card queues a points ban from the 24th and adds 9 (second booking) or 24 points; each booking adds 3, 4 or 6 by offence; each threshold (15, 30, then every 14) crossed in the match gives 1 match (reason 0, amount = the total); from today')])
CAMPOS = []
INTERESSA = []
