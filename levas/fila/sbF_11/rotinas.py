# O torneio amistoso da IA (friendly.cpp; leitor sbF, rodada 5, 24/11/2026). Simulacao, carreira ENG ate 2002-08-01:
# a unica chamada e a do planejamento da pre-temporada (Friendly::plan_preseason_tournaments_and_tours 0x5c3040, gra_21),
# que pula os torneios em 2001; os de 2002 saem nos dias de planejamento de julho de 2002 (cobertura do mundo: 10
# chamadas ate 2002-08-01). Medir na cadeia se a carreira ENG sozinha chega a chama-la antes da data; se nao, dar
# LIGAS com mais nacoes (as 35 entradas do G_LEAGUE_TABLE que passam preseason_league_open).
# - 0x5c3be0 Friendly::create_ai_tournament thiscall(grupo, n, tipo) ret 0xc -> 1/0: a janela comum dos candidatos
#   (+0xe/+0x10, de 0x16d), menos os dias do tipo (o switch inline, caixa 0xf18); a data = +0xa7 + fim
#   (Date::add_days 0x54c5b0, thiscall com o ponteiro do resultado), recuada ate o dia da semana 5
#   (Date::get_weekday 0x54a3f0, Date::sub_days 0x54c8e0, Date::operator= 0x417c10) quando a janela passa de 6 dias;
#   o anfitriao e os convidados por club_at; o nome no buffer 0xde1f64 (langlib_sprintf 0x66f4e0, cdecl) com o byte
#   +0x18 zerado; Friendly::add_tournament 0x5c6e00 (Rust, cqb_4) e Friendly::send_tournament_invitations 0x5c5310
#   (thiscall, ret 4, ainda original); as listas de datas (operator new 0xe + LLIST::LLIST 0x68d020), a contagem +4
#   (+2 copa, +3 liga) e os dias do torneio em cada lista (LLIST::add_tail 0x68d0d0, 8 bytes). SEH so para o new.
# Prologo push -1; push 0x959f5f (7 bytes). Igual no cm0102-1080.exe.
# Comparador: so a original (preenche a vaga, manda noticias e aumenta as listas), com o plano (o retorno pela janela,
# as listas e as contagens).
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('friendly_create_ai_tournament', 0x5c3be0, [0x6a, 0xff, 0x68, 0x5f, 0x9f, 0x95, 0x00])]
DOCS = ('o torneio amistoso da IA', 'the AI friendly tournament',
        [('friendly_create_ai_tournament', 'Friendly::create_ai_tournament', '0x5c3be0', 'friendly.cpp', 'thiscall, ret 0xc',
          'regra: um torneio de pré-temporada da IA para um grupo de clubes começa no último dia da janela de amistosos comum aos clubes que ainda deixa espaço para o torneio (uma copa amistosa leva 2 dias, uma liga amistosa 3), recuado até o sábado [inferência: dia da semana 5] quando a janela passa de uma semana; sem espaço, não há torneio. O anfitrião é o primeiro clube; o torneio se chama "Friendly Cup" (tipo 1) ou "Friendly League", os convites saem, e cada clube ganha os dias do torneio nas suas datas e 2 (copa) ou 3 (liga) amistosos a mais',
          'rule: an AI preseason tournament for a group of clubs starts on the last day of the clubs\' common friendly window that still leaves room for it (a friendly cup takes 2 days, a friendly league 3), moved back to a Saturday [inference: weekday 5] when the window is wider than a week; no room, no tournament. The host is the first club; it is named "Friendly Cup" (kind 1) or "Friendly League", the invitations go out, and every club gets the tournament days in its dates and 2 (cup) or 3 (league) more friendlies')])
CAMPOS = []
INTERESSA = []
