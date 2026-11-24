# notes.cpp (leitor sx16, rodada 5, 24/11/2026): as notas dos tecnicos humanos sobre pessoas (tela de nota do perfil).
# O objeto (G_NOTES 0xb7433c, 0x1070 bytes): 16 vagas de 8 bytes (+0 a lista LLIST das notas, +4 a segunda lista) e
# os 16 textos de edicao de 255 bytes a partir de +0x80. A vaga e G_STAFF_COUNT - id do tecnico - 1 (as 16 ultimas
# linhas da tabela de pessoas); fora de 0..15, nada. A nota (0x114 bytes): +0 id da pessoa, +4 texto (255), +0x103
# data em que foi escrita, +0x10b data do lembrete, +0x113 motivo. (O notes.rs da tela chama +0x103 de
# NOTE_REMINDER_DATE e +0x10b de NOTE_WRITTEN_DATE: os nomes estao trocados; o codigo da tela le os dois na mesma
# ordem da original, entao so o nome esta errado.)
# Convencoes conferidas na desmontagem (todas thiscall; as de rotulo nao usam ecx):
# - 0x7953d0 add_note(tecnico, &id, texto, lembrete char, motivo char) ret 0x14; quadro SEH so pelas datas locais;
#   0x54a110 Date::Date(); 0x9466d0 strncpy cdecl; 0x417c10 Date::operator= thiscall(dest, &tmp, origem) ret 8;
#   0x54c770 Date::add_weeks thiscall(&hoje, &tmp, n) ret 8; 0x549f70 Date::Date(23, 7, 1971, -1) ret 0x10 (sem
#   lembrete); 0x68d0d0 LLIST::add_tail thiscall(lista, &nota, 0x114) ret 8;
# - 0x795550 add_note_dated(tecnico, &id, texto, &lembrete, &escrita, motivo) ret 0x18;
# - 0x795660 remove_note(tecnico, &id) ret 8: 0x68d1b0 get_next, 0x68d1f0 LLIST::remove thiscall(lista, dado) ret 4;
# - 0x7956d0 replace_note ret 0x14 e 0x795710 replace_note_dated ret 0x18: chamam 0x795660 e 0x7953d0/0x795550
#   direto (pelo desvio);
# - 0x795750 find_note(tecnico, &id) ret 8 -> nota ou 0;
# - 0x7957b0 format_note_text(tecnico, &id, saida) ret 0xc -> saida ou 0; 0x795b90 (estilo 2, 100 bytes); 0x54a470
#   Date::get_numeric_date_string thiscall(data, saida, 0x64, 0xff) ret 0xc; 0x66f4e0 langlib_sprintf e 0x944c9f
#   sprintf cdecl;
# - 0x7959e0 get_edit_text(tecnico) ret 4; 0x795a10 set_edit_text(tecnico, texto) ret 8; 0x795a60
#   clear_edit_text(tecnico) ret 4;
# - 0x795a90 get_reminder_label(saida, tamanho, escolha char) ret 0xc e 0x795b90 get_reason_label(saida, tamanho,
#   motivo char, estilo char) ret 0x10; as caixas "Error" do notes.cpp (0x944cff, 0x90d130, sprintf, 0x5e8290,
#   [0xb67a34] = 0) = show_error_box (transfer_manager.rs). Tabelas de salto 0x795de4, 0x795df4, 0x795e04 (motivo - 1).
# Quem chama: a tela de nota (0x796340, 0x796b30, ja Rust), o perfil (find_note, format_note_text), a tela de
# comparar jogadores (0x87db9a), e remove_note tambem na carreira: TRANSFER_MANAGER::finalise_transfer 0x8c97a4
# (12.731 vezes ate 2002-06-01, sempre com a lista vazia na carreira automatica: so a tela poe notas).
# Comparadores: add_note e add_note_dated pela nota que a original pos no fim da lista contra o registro que a copia
# monta; remove_note so a original (libera) com a contagem conferida pela busca da copia; replace_* so a original; os
# outros lado a lado (o cabecalho da lista, a saida, os textos de edicao).
TELAS = True
ROTINAS = [('notes_add_note', 0x7953d0, [0x6a, 0xff, 0x68, 0x04, 0xea, 0x95, 0x00]),
           ('notes_add_note_dated', 0x795550, [0x6a, 0xff, 0x68, 0x23, 0xea, 0x95, 0x00]),
           ('notes_remove_note', 0x795660, [0x8b, 0x44, 0x24, 0x04, 0x53]),
           ('notes_replace_note', 0x7956d0, [0x53, 0x8b, 0x5c, 0x24, 0x08]),
           ('notes_replace_note_dated', 0x795710, [0x53, 0x8b, 0x5c, 0x24, 0x08]),
           ('notes_find_note', 0x795750, [0x8b, 0x44, 0x24, 0x04, 0x53]),
           ('notes_format_note_text', 0x7957b0, [0x8b, 0x44, 0x24, 0x0c, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('notes_get_edit_text', 0x7959e0, [0x8b, 0x54, 0x24, 0x04, 0xa1, 0x64, 0x23, 0xae, 0x00]),
           ('notes_set_edit_text', 0x795a10, [0x8b, 0x54, 0x24, 0x04, 0xa1, 0x64, 0x23, 0xae, 0x00]),
           ('notes_clear_edit_text', 0x795a60, [0x8b, 0x54, 0x24, 0x04, 0xa1, 0x64, 0x23, 0xae, 0x00]),
           ('notes_get_reminder_label', 0x795a90, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('notes_get_reason_label', 0x795b90, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as notas dos técnicos humanos sobre pessoas',
        "the human managers' staff notes",
        [('notes_add_note', 'Notes::add_note', '0x7953d0', 'notes.cpp', 'thiscall, ret 0x14',
          'acrescenta a nota de um técnico humano sobre uma pessoa, escrita hoje, com o lembrete em uma semana (1), quatro semanas (2) ou nenhum (a data de 23/07/1971), o texto (até 255 bytes) e o motivo; fora das 16 vagas humanas, nada',
          "appends a human manager's note about a staff member, written today, with the reminder in one week (1), four weeks (2) or none (the date 23/07/1971), the text (up to 255 bytes) and the reason; outside the 16 human slots, nothing"),
         ('notes_add_note_dated', 'Notes::add_note_dated', '0x795550', 'notes.cpp', 'thiscall, ret 0x18',
          'acrescenta a nota com as datas dadas (lembrete e escrita), para a nota editada manter as suas',
          'appends the note with the given dates (reminder and written), so an edited note keeps its own'),
         ('notes_remove_note', 'Notes::remove_note', '0x795660', 'notes.cpp', 'thiscall, ret 8',
          'tira a nota do técnico sobre a pessoa (a primeira com o id dela); roda também a cada transferência concluída, para o técnico do clube comprador',
          "removes the manager's note about the person (the first with his id); also runs on every finished transfer, for the buying club's manager"),
         ('notes_replace_note', 'Notes::replace_note', '0x7956d0', 'notes.cpp', 'thiscall, ret 0x14',
          'troca a nota: tira a antiga e acrescenta outra escrita hoje',
          'replaces the note: removes the old one and adds another written today'),
         ('notes_replace_note_dated', 'Notes::replace_note_dated', '0x795710', 'notes.cpp', 'thiscall, ret 0x18',
          'troca a nota mantendo as datas dadas',
          'replaces the note keeping the given dates'),
         ('notes_find_note', 'Notes::find_note', '0x795750', 'notes.cpp', 'thiscall, ret 8',
          'a nota do técnico sobre a pessoa, ou 0',
          "the manager's note about the person, or 0"),
         ('notes_format_note_text', 'Notes::format_note_text', '0x7957b0', 'notes.cpp', 'thiscall, ret 0xc',
          'escreve a nota para exibir: "(motivo) texto (Lembrete: data)", cada parte só quando existe; sem a nota, a saída vazia e 0',
          'writes the note for display: "(reason) text (Reminder: date)", each part only when present; without the note, the output emptied and 0'),
         ('notes_get_edit_text', 'Notes::get_edit_text', '0x7959e0', 'notes.cpp', 'thiscall, ret 4',
          'o texto de edição (255 bytes) da vaga do técnico, ou 0 fora das 16 vagas',
          "the manager slot's edit text (255 bytes), or 0 outside the 16 slots"),
         ('notes_set_edit_text', 'Notes::set_edit_text', '0x795a10', 'notes.cpp', 'thiscall, ret 8',
          'copia o texto de uma nota para o texto de edição da vaga',
          "copies a note's text into the slot's edit text"),
         ('notes_clear_edit_text', 'Notes::clear_edit_text', '0x795a60', 'notes.cpp', 'thiscall, ret 4',
          'esvazia o texto de edição da vaga antes de uma nota nova',
          "empties the slot's edit text before a new note"),
         ('notes_get_reminder_label', 'notes_get_reminder_label', '0x795a90', 'notes.cpp', 'thiscall, ret 0xc',
          'o rótulo da escolha de lembrete: "No Reminder", "One Week", "One Month"; sem buffer ou com menos de 50 bytes, a caixa de erro e 0',
          'the label of the reminder choice: "No Reminder", "One Week", "One Month"; without a buffer or under 50 bytes, the error box and 0'),
         ('notes_get_reason_label', 'notes_get_reason_label', '0x795b90', 'notes.cpp', 'thiscall, ret 0x10',
          'o rótulo do motivo pelo estilo: o nome (2), o título da notícia de lembrete (0) ou a etiqueta curta (os outros); motivos 1 jovem promissor, 2 alvo de transferência, 3 renovação de contrato, 4 do elenco',
          'the reason label by style: the name (2), the reminder news title (0) or the short tag (others); reasons 1 promising youngster, 2 transfer prospect, 3 contract renewal, 4 squad member')])
