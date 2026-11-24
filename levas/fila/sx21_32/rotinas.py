# O texto da noticia do prazo de transferencias da Franca (slot 0x30; leitor sx21, rodada 5, 24/11/2026). So e escrito
# quando a noticia chega a um tecnico humano de clube frances: nao roda nas carreiras de teste. Tipo TELAS.
# Caminho no jogo: novo jogo com a liga francesa carregada, assumir um clube frances e avancar ate as noticias de prazo
# (uma semana antes, a vespera e o dia de 31 de agosto e de 10 de janeiro; a janela restrita, numero 1, nao tem
# noticia); abrir a noticia (a materia; a manchete vem do texto de base).
# Convencoes conferidas: thiscall(noticia, materia, texto) ret 0xc; le as vagas 1, 2 e 5 antes de decidir; 0x53b3d0
# cdecl(nacao); 0x4ae8a0 thiscall(texto, adjetivo, 0x7d5, selecao) ret 0xc.
# Defeito do original: numa janela sem formato, o link vai com um buffer de pilha nunca escrito (inalcancavel: a
# noticia francesa pula a janela 1).
# Depende da sx21_31 (write_window_deadline_story) e da sx21_3 (transfer_rules_write_deadline_news_text): integrar junto
# ou depois delas.
# Comparadores: compare_text_writer.
SAVE = False
TELAS = True
ROTINAS = [
    ('france_rules_write_deadline_news_text', 0x5c1b90, [0x83, 0xec, 0x64, 0x53, 0x55]),
]
DOCS = ('o texto da notícia do prazo de transferências da França',
        'the transfer deadline news text of France',
        [('france_rules_write_deadline_news_text', 'france_rules_write_deadline_news_text', '0x5c1b90', 'france_rules.cpp', 'thiscall, ret 0xc',
          'regra: a manchete é a de base; a matéria pelo estado e pela janela (os prazos gerais na janela 0, as restrições na janela 2), com o adjetivo da nação e o link para a seleção',
          "rule: the headline is the base one; the story by the state and the window (the general deadlines in window 0, the restrictions in window 2), with the nation's adjective and the link to the national team")])
CAMPOS = []
INTERESSA = []
