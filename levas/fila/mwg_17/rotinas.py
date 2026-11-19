# A atualização diária da névoa de guerra, os lembretes das notas que vencem hoje e o construtor-base dos órgãos
# reguladores (fog_of_war.cpp, notes.cpp, ruling_body.cpp; leitor mwg, rodada 3 do mundo inteiro, 19/11/2026). Até
# 2002-08-01: FOG_OF_WAR::daily_update 383 vezes e Notes::post_due_reminders 383 (uma por dia), RulingBody::RulingBody
# 43 (os órgãos reguladores criados com a carreira) (cobertura_mundo.txt); rodam em qualquer carreira: LIGAS = 'ENG'.
# Convenções conferidas na desmontagem (0x5afce0..0x5afcfc, 0x795360..0x7953c6, 0x7e7760..0x7e784a):
# - FOG_OF_WAR::daily_update: thiscall, ret 0; os 16 ponteiros de this, cada um não nulo com FOG_OF_WAR_MAP::
#   daily_update (0x5b0260, thiscall, ret 0);
# - Notes::post_due_reminders: thiscall, ret 0; 16 vagas de 8 bytes, [vaga] = a lista (LLIST); o cursor +0xa = a cabeça
#   +2; contagem (+0, palavra com sinal, relida) vezes LLIST::get_next (0x68d1b0) e, se Date::operator== (0x4a7f20,
#   ecx = nota+0x10b, push [0xae2c94]; push [0xae2c90] -> thiscall, ret 8), Notes::post_reminder_news(k, nota)
#   (0x795e20, thiscall, ret 8, ecx = this);
# - RulingBody::RulingBody: thiscall, ret 4 (arquivo), devolve this; quadro SEH só para o new da lista: [this] =
#   0x96f104, +7 = new(0xe) + LLIST::LLIST (0x68d020) ou 0, +0xb, +0xf e +0x13 = 0; com arquivo, RulingBody::load
#   (0x7e78a0, thiscall, ret 4) e, se 0, a caixa (linha 0x36, 0xa7a858).
# Comparadores, só no original (as três mexem no que não volta: o conhecimento dos mapas, as notícias, a lista
# alocada), com o plano: os 16 mapas os mesmos; as listas de notas percorridas até o fim; a vtable, a lista e nenhuma
# entrada (sem arquivo).
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('fog_of_war_daily_update', 0x5afce0, [0x56, 0x57, 0x8b, 0xf1, 0xbf, 0x10, 0x00, 0x00, 0x00]),
           ('notes_post_due_reminders', 0x795360, [0x51, 0x53, 0x55, 0x56, 0x57]),
           ('construct_ruling_body', 0x7e7760, [0x6a, 0xff, 0x68, 0x5e, 0x08, 0x96, 0x00])]
DOCS = ('a névoa de guerra do dia, os lembretes das notas que vencem hoje e o construtor-base dos órgãos reguladores',
        "the day's fog of war, the reminders of the notes due today and the ruling bodies' base constructor",
        [('fog_of_war_daily_update', 'FOG_OF_WAR::daily_update', '0x5afce0', 'fog_of_war.cpp', 'thiscall, ret 0',
          'a névoa de guerra do dia: cada um dos 16 mapas de técnico humano que existe faz a sua atualização diária do conhecimento',
          "the day's fog of war: each of the 16 human manager maps that exists runs its daily knowledge update"),
         ('notes_post_due_reminders', 'Notes::post_due_reminders', '0x795360', 'notes.cpp', 'thiscall, ret 0',
          'os lembretes do dia: em cada uma das 16 vagas de técnico, cada nota cuja data de vencimento é hoje gera a sua notícia de lembrete',
          "the day's reminders: in each of the 16 manager slots, every note whose due date is today posts its reminder news"),
         ('construct_ruling_body', 'RulingBody::RulingBody', '0x7e7760', 'ruling_body.cpp', 'thiscall, ret 4',
          'o construtor-base de todo órgão regulador: a vtable dele, uma lista nova e nenhuma entrada; de um jogo salvo, o estado gravado (falhando, a caixa da linha 0x36)',
          "the base constructor of every ruling body: its vtable, a new list and no entries; from a saved game, its saved state (on failure, the box at line 0x36)")])
