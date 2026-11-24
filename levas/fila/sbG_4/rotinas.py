# As respostas aos convites de um torneio amistoso e a criacao da competicao dele (friendly.cpp; leitor sbG, rodada 5,
# 24/11/2026). Simulacao, carreira ENG (sem LIGAS, como o cx05_12 e o cx05_13): os torneios de pre-temporada de julho
# de 2001 criam as minicopas e minicompeticoes de liga por aqui (3 MiniCup e 5 MiniLeague na cobertura do mundo; 8
# chamadas ate 2002-08-01; o pick_replacement_invitee 0x5c6350 nunca rodou: ninguem recusou com sede da IA).
# - 0x5c5820 Friendly::process_tournament_replies thiscall(torneio, forcar, clube) ret 0xc (quadro SEH para a noticia
#   local e os dois new): os convidados 1..+0x13 (byte com sinal) com clube e marca +0x25 = 1; humano (0x600a50 cdecl
#   (clube, 0, 1, 0, 0)) so responde se for o clube pedido, IA se for ele ou com forcar; a resposta: livre nas datas
#   (0x5c6b70 thiscall(&+4, clube, byte +0xc), ja em Rust) e a conta x87 (1.65 / 1.52625, x 0.85 por regiao e por
#   continente, x (+0x77 / ((distancia x 0.55 + 1) x 60000) + 1) com as duas cidades; recusa se a reputacao da sede
#   for menor que a do convidado / fator e a diferenca maior que fator x 600; o NaN conta como "menor"); sim marca 3;
#   nao zera clube e marca e, com sede da IA, 0x5c6350(torneio, k) e 0x5c5310(torneio); a noticia 10 (NewsItem
#   0x77bbe0, set_slot 0x785790 x6, News::send_to_club_manager 0x7861e0 com ecx = [0xb7431c]; sem destrutor no caminho
#   normal). Todos aceitos e sem competicao: as vagas repetidas e as que nao estao mais livres sao esvaziadas; de novo
#   todos aceitos: o registro da competicao (id = +0xaf do Friendly + byte +0x64; nome em +4 e +0x38, "Fr" de 0x9cd0f0
#   em +0x53, continente, nacao comum, cores +0x83/+0x87, reputacao 0) e a MiniCup (new 0xbc; 0x765bf0, 6 pushes; caixa
#   0xac0) ou a MiniLeague (new 0xf6; 0x766500, 11 pushes; caixa 0xaeb); +0x65 e a tabela [0xadadfc]; a copa com
#   Cup::set_up_round 0x51c800 (push 0), mesmo nula. O byte dos lados, a palavra do ano e a das regras vao em dwords com
#   lixo nos bytes altos (os construtores leem o byte ou a palavra).
# Prologo push -1; push 0x95a097 (7 bytes). Igual no cm0102-1080.exe (inteira).
# Comparador: so a original (noticias, convites, competicao nova) contra o plano das respostas, calculadas antes por
# friendly_tournament_reply_accepted: um sim deixa o clube na vaga com a marca 3 (ou a vaga vazia pela conferencia
# final), um nao, nao.
SAVE = False
ROTINAS = [('friendly_process_tournament_replies', 0x5c5820, [0x6a, 0xff, 0x68, 0x97, 0xa0, 0x95, 0x00])]
DOCS = ('as respostas aos convites de um torneio amistoso', 'the replies to a friendly tournament\'s invitations',
        [('friendly_process_tournament_replies', 'Friendly::process_tournament_replies', '0x5c5820', 'friendly.cpp', 'thiscall, ret 0xc',
          'regra: cada clube convidado (um humano só quando é ele que responde) aceita se estiver livre nas datas e a viagem compensar: o atrativo é 1,65 no mesmo país e 1,52625 fora, menor com outra região ou continente e maior para clubes de público alto e perto; recusa só quando a reputação da sede é bem menor que a dele; quem recusa é trocado por outro convidado (sede da IA) e a sede recebe a notícia de cada resposta; com todos aceitos, nasce a competição: uma minicopa (tipo 1) ou uma miniliga',
          'rule: each invited club (a human one only when it is the one replying) accepts if it is free on the dates and the trip pays off: the pull is 1.65 in the same country and 1.52625 abroad, lower with another region or continent and higher for clubs with big crowds and close by; it refuses only when the host\'s reputation is well below its own; a refusing club is replaced by another invitee (AI host) and the host gets news of each reply; with all accepted, the competition is born: a mini cup (kind 1) or a mini league')])
CAMPOS = []
INTERESSA = []
