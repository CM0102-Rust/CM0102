# Os convites dos torneios de amistosos e a substituta de um convidado que recusa (friendly.cpp; leitor cx05, rodada 5,
# 24/11/2026). Simulacao, carreira ENG (sem LIGAS): os torneios de pre-temporada de julho de 2001 (8 envios de convites
# na cobertura ate 2002-06-01; a substituicao, 0).
# - 0x5c5310 Friendly::send_tournament_invitations thiscall(torneio) ret 4: operator new 0x944cf1 e LLIST::LLIST
#   0x68d020, LLIST::remove 0x68d1f0 (push &torneio -> thiscall, ret 4), LLIST::add_tail 0x68d0d0 (3 pushes ->
#   thiscall, ret 0xc), news_club_has_human_manager 0x7875e0 (push clube -> stdcall), NewsItem::NewsItem 0x77bbe0,
#   NewsItem::set_slot 0x785790 e News::send_to_club_manager 0x7861e0 (thiscall). O quadro SEH so cobre o new da lista e
#   a noticia local (sem destrutor). Prologo mov eax, fs:[0] (6 bytes).
# - 0x5c6350 Friendly::pick_replacement_invitee thiscall(torneio, vaga) ret 8 (o this nao e usado; chamada pelo
#   process_tournament_replies 0x5c5a40): database_is_national_team 0x53b900 e database_get_national_team 0x53b3d0
#   (cdecl), utils_random 0x90cfc0 (cdecl); a divisao por 500 pelo magico 0x10624dd3 (truncada para zero). Prologo sub
#   esp, 8; push ebx; push ebp (5 bytes).
# Defeito do original: na substituta de um clube, a melhor marca guardada e a faixa do clube e nao a pontuacao com o
# sorteio (na de uma selecao, e a pontuacao): a comparacao seguinte usa a faixa (a copia faz o mesmo).
# Iguais no cm0102-1080.exe (as duas inteiras).
# Comparadores: so a original (a lista anda, as noticias saem, a substituta e sorteada), com o plano: o torneio na
# lista de pendentes e nenhum convidado humano ainda com a marca 1; a vaga sem a propria sede.
SAVE = False
ROTINAS = [('send_tournament_invitations', 0x5c5310, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('pick_replacement_invitee', 0x5c6350, [0x83, 0xec, 0x08, 0x53, 0x55])]
DOCS = ('os convites dos torneios de amistosos', 'the friendly tournament invitations',
        [('send_tournament_invitations', 'Friendly::send_tournament_invitations', '0x5c5310', 'friendly.cpp', 'thiscall, ret 4',
          'põe o torneio no fim da lista dos pendentes e manda a notícia de convite a cada convidado de técnico humano que ainda não a recebeu (marcado como enviado)',
          'puts the tournament at the end of the pending list and sends the invitation news to each human-run invitee that has not got it yet (marked as sent)'),
         ('pick_replacement_invitee', 'Friendly::pick_replacement_invitee', '0x5c6350', 'friendly.cpp', 'thiscall, ret 8',
          'regra: quando um convidado recusa, a vaga vai para um clube da nação da sede (ou, se a sede é uma seleção, uma seleção da mesma região) de reputação parecida: um clube tira 0 a 4 mais a sua faixa (reputação / 500) − 2 e não pode passar da faixa da sede (a dela + 1); uma seleção tira 0 a 4 mais a reputação da nação − 2, com a reputação da nação até a faixa da sede; o melhor fica com a vaga e a busca para quando ele passa da faixa da sede − 4',
          'rule: when an invitee declines, the place goes to a club of the host\'s nation (or, for a national team host, a national team of the same region) of similar reputation: a club draws 0 to 4 plus its band (reputation / 500) − 2 and may not go above the host\'s band (its own + 1); a national team draws 0 to 4 plus the nation\'s reputation − 2, the nation\'s reputation being up to the host\'s band; the best takes the place and the search stops once it is above the host\'s band − 4')])
CAMPOS = []
INTERESSA = []
