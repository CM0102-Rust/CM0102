# notes.cpp (leitor sx16, rodada 5, 24/11/2026): os lembretes das notas (a noticia do lembrete, o texto dela e os
# motivos validos para uma pessoa na tela de nota).
# - 0x795e20 Notes::post_reminder_news thiscall(notas, vaga, nota) ret 8 (ecx nao usado); quadro SEH pela noticia local
#   (0x77bbe0 NewsItem::NewsItem thiscall(item, 0x1393, 0) ret 8; sem destrutor); 0x785790 set_slot thiscall(item, k,
#   valor) ret 8; 0x785d40 News::deliver_to_manager thiscall([0xb7431c], &item, tecnico, 0) ret 0xc. Chamada so por
#   Notes::post_due_reminders 0x795360 (ja Rust; roda todo dia, mas sem notas na carreira automatica).
# - 0x795f50 notes_write_diary_reminder_news thiscall(notas, noticia, corpo, Text) ret 0xc: 0x785830 get_slot
#   thiscall(item, k) ret 4; 0x53e140 database_format_name_from_ids cdecl(nome1, nome2, comum, buf, 0x12c, 1, id);
#   0x795b90 (estilo 0, no buffer 0xde1f64, 1000 bytes); 0x66f4e0 langlib_sprintf; 0x4ae660 Text::set_string
#   thiscall(t, texto) ret 4; 0x53e990 database_staff_matches_names cdecl(pessoa, n1, n2, comum, 0); 0x4ae8a0
#   Text::add_link thiscall(t, nome, 0x7db, pessoa) ret 0xc. Tabela de saltos 0x7960ec (motivo - 1). Chamada pelo
#   NewsItem::write_text 0x77bc80 (0x782799), so ao abrir a noticia do lembrete.
# - 0x796100 notes_is_reason_valid_for_staff thiscall(notas, tecnico, pessoa, motivo char) ret 0xc (ecx nao usado);
#   tabela 0x796198 (5 entradas). Chamada pela tela de nota (0x7968aa).
# Tipo: telas (a carreira automatica nao poe notas). O lembrete de uma nota chega no dia: criar uma nota com lembrete
# "One Week" no perfil de uma pessoa (Create Manager Note), continuar o jogo 7 dias, abrir a noticia "Reminder about"
# na caixa de entrada; a tela de nota mostra os motivos validos (os outros em cinza).
TELAS = True
ROTINAS = [('notes_post_reminder_news', 0x795e20, [0x6a, 0xff, 0x68, 0x3b, 0xea, 0x95, 0x00]),
           ('notes_write_diary_reminder_news', 0x795f50, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('notes_is_reason_valid_for_staff', 0x796100, [0x0f, 0xbe, 0x44, 0x24, 0x0c])]
DOCS = ('os lembretes das notas sobre pessoas',
        'the staff note reminders',
        [('notes_post_reminder_news', 'Notes::post_reminder_news', '0x795e20', 'notes.cpp', 'thiscall, ret 8',
          'manda ao técnico humano da vaga a notícia de lembrete (tipo 5011) de uma nota: a pessoa, o motivo e os ids dos três nomes dela; nada com vaga fora de 0..15, sem nota ou com pessoa fora da tabela',
          "sends the slot's human manager the reminder news (type 5011) of a note: the person, the reason and the ids of his three names; nothing for a slot outside 0..15, without a note or with a person outside the table"),
         ('notes_write_diary_reminder_news', 'notes_write_diary_reminder_news', '0x795f50', 'notes.cpp', 'thiscall, ret 0xc',
          'escreve a manchete ("Reminder about ..." pelo motivo) ou o corpo da notícia de lembrete: a frase da secretaria do clube pelo motivo (decidir o futuro do jovem, decidir sobre o alvo de transferência, renovar o contrato, tratar do jogador do elenco, ou contatar a pessoa), com o nome ligado à pessoa quando ela ainda tem esses nomes',
          'writes the headline ("Reminder about ..." by the reason) or the body of the reminder news: the club secretary\'s sentence by the reason (decide the youngster\'s future, decide on the transfer prospect, renew the contract, deal with the squad member, or contact the person), with the name linked to the person when he still has those names'),
         ('notes_is_reason_valid_for_staff', 'notes_is_reason_valid_for_staff', '0x796100', 'notes.cpp', 'thiscall, ret 0xc',
          'se o motivo serve para a pessoa: nenhum e acima de 4 sempre; jovem promissor, um jogador com menos de 24 anos; alvo de transferência, um jogador de outro clube; renovação, alguém do clube do técnico; do elenco, um jogador do clube do técnico',
          "whether the reason fits the person: none and above 4 always; promising youngster, a player under 24; transfer prospect, a player of another club; renewal, anyone at the manager's club; squad member, a player at the manager's club")])
