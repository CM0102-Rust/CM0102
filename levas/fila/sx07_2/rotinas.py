# O texto da noticia da resposta a uma proposta de contrato (contract_manager_write_offer_response_news_text 0x4d4fb0,
# contract_manager.cpp, 5.424 bytes; leitor sx07, rodada 5, 23/11/2026). A rotina vai de 0x4d4fb0 ao ret 0xc de
# 0x4d64cf (0x4d64d2.. e enchimento), sem tabela de saltos. Chamada so pelo despacho das noticias (NewsItem::write_text
# 0x77bc80, tipo 0xbb8, tabela NEWS_WRITERS: Global(0xae1050, 0x4d4fb0)); 44 vezes na carreira ENG ate 2002-06-01.
# Partida em pedacos com nome: a leitura das vagas e o que o original prepara antes de escrever (offer_text_read), a
# manchete (offer_text_headline), o corpo aceito (offer_text_accepted_body, com os motivos offer_text_accept_reasons e
# os dois ramos offer_text_accept_with_reasons / offer_text_accept_plain), o corpo recusado (offer_text_rejected_body)
# e os vinculos (offer_text_links).
# Convencoes conferidas na desmontagem (cada push e cada [esp+k] seguidos ate cada chamada):
# - a rotina: thiscall(gerente de contratos, noticia, corpo, Text), ret 0xc; o ecx nao e lido; quadro de 0x118c bytes
#   pelo __chkstk (0x945610, fora do prologo);
# - 0x785830 NewsItem::get_slot thiscall(k) ret 4; 0x549ef0 Date::Date thiscall(dia, ano) ret 8;
# - 0x43f460 Cash::get_display_amount thiscall(&valor, 6) -> edx:eax e 0x441c10 cash_format_money cdecl(buf, 0x64,
#   i64, 6) (o primeiro push 6 e do segundo); _ftol(dias x 0x968e10 + 0x968968);
# - 0x53e140 database_format_name_from_ids cdecl(n1, n2, n3, buf, tamanho, modo, id): o nome em modo 1 e 5 (0x190) e o
#   do tecnico com a vaga 17 como id (assim no original); 0x54ba20 Date::get_long_date_string thiscall(buf, 0x12c,
#   0xff) ret 0xc;
# - 0x66f4e0 langlib_sprintf cdecl e 0x944c9f sprintf cdecl: os argumentos dos 55 formatos conferidos push a push (os
#   generos sao movsx do byte +0x52 com o nome curto +0x38, ou +0x37 com o nome inteiro +4, empilhados duas vezes);
# - 0x4ae660 Text::set_string thiscall ret 4; 0x4ae8a0 Text::add_link thiscall(texto, tipo, dado) ret 0xc; 0x53e990
#   database_staff_matches_names cdecl(pessoa, n1, n2, n3, 0).
# Defeitos do original: o nome do tecnico usa a vaga 17 (o codigo do fim do contrato: 0, 0x3e7 ou 0x3e8) como id da
# pessoa e so e montado se ela nao for negativa (negativa: o buffer fica com lixo da pilha; a copia usa vazio); o corpo
# de um estado fora de 1, 2 e 3 mostra o que o buffer compartilhado 0xde1f64 tinha.
# Comparador: compare_text_writer (texto, vinculos e gerador).
SAVE = False
ROTINAS = [('contract_manager_write_offer_response_news_text', 0x4d4fb0, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o texto da noticia da resposta a uma proposta de contrato',
        'the text of the news item of the answer to a contract offer',
        [('contract_manager_write_offer_response_news_text', 'contract_manager_write_offer_response_news_text', '0x4d4fb0', 'contract_manager.cpp', 'thiscall, ret 0xc',
          'escreve a manchete ou o corpo da noticia 0xbb8, de que uma pessoa aceitou ou recusou uma proposta de contrato ou de teste: le das vagas a pessoa, o clube, o estado, o fim e o inicio do contrato, o tipo (5 teste), os motivos, o tecnico, o codigo do fim do contrato, se a proposta e propria, o sorteio, a renovacao, o cargo, o salario, o valor de assinatura, o veterano e o rebelde; o valor do contrato (semanas inteiras x salario + assinatura; a partir de 3 milhoes arredondado ao milhao, acima de 1 milhao ao meio milhao) e os anos (dias / 365, arredondado); "em casa" se o clube e do tecnico humano na tela. A manchete por estado (aceita, recusada; outro estado: a caixa de erro e nada escrito): teste, proposta propria, renovacao de cargo 11 pelo sorteio (com os anos; o rebelde na recusa), fora de casa com o clube, em casa sem. O corpo aceito: teste com a data de fim; os motivos (amor ao clube, admiracao pelo tecnico, ou a crenca no tecnico de quem tem menos de 24 anos); com motivos, o veterano (por tecnico e valor acima de (sorteio + 1) milhoes) e o amor ao clube pelo sorteio e pelos anos, senao o rebelde de salario 5.000 ou mais; sem motivos, a renovacao ou o contrato novo pelo valor e pelo sorteio contra o id da pessoa modulo 3. O corpo recusado: teste; os motivos (insatisfeito com o tecnico, quer sair logo) e, fora de casa, pelo codigo do fim do contrato (0x3e7 especulacao, 0x3e8 futuro em duvida), senao a recusa simples. O estado 3: disposto a negociar. Depois os vinculos do clube, da pessoa (dois nomes) e do tecnico',
          'writes the headline or body of news item 0xbb8, that a person accepted or rejected a contract or trial offer: reads from the slots the person, the club, the state, the contract end and start, the type (5 a trial), the reasons, the manager, the contract end code, whether the offer is custom, the draw, the renewal, the job, the wage, the signing value, the veteran and the rebel; the contract value (whole weeks x wage + signing; from 3 million down to the million, above 1 million to the half million) and the years (days / 365, rounded); "at home" when the club is the human manager\'s on screen. The headline by state (agreed, rejected; another state: the error box and nothing written): trial, custom offer, job 11 renewal by the draw (with the years; the rebel when rejected), away with the club, at home without. The agreed body: a trial with its end date; the reasons (love for the club, admiration for the manager, or the belief in the manager of one under 24); with reasons, the veteran (by manager and a value above (draw + 1) million) and the love for the club by the draw and the years, otherwise the rebel on a wage of 5,000 or more; without reasons, the renewal or the new contract by the value and by the draw against the person\'s id modulo 3. The rejected body: trial; the reasons (unhappy with the manager, wants to leave soon) and, away, by the contract end code (0x3e7 speculation, 0x3e8 future in doubt), otherwise the plain rejection. State 3: willing to negotiate. Then the links of the club, the person (two names) and the manager')])
