# Os textos das noticias do prazo de transferencias da Italia e da Noruega (slot 0x30; leitor sx21, rodada 5,
# 24/11/2026). O texto so e escrito quando a noticia chega a um tecnico humano de clube da nacao: nao roda nas carreiras
# de teste (o humano e de clube ingles). Tipo TELAS, provado pelo Lucas jogando com um clube italiano ou noruegues.
# Caminho no jogo: novo jogo com a liga italiana (ou norueguesa) carregada, assumir um clube de la e avancar ate uma
# semana antes do fim de uma janela (Italia: 27 de setembro, 31 de janeiro; Noruega: 31 de agosto); abrir a noticia
# do prazo (a materia; a manchete vem do texto de base).
# Convencoes conferidas: thiscall(noticia, materia, texto) ret 0xc; 0x785830 thiscall(vaga) ret 4 (a Italia le as vagas
# 1, 2 e 5; a Noruega, 1 e 5); 0x6181d0 cdecl(nacao, saida, 100, forma); 0x4ae660 thiscall(texto) ret 4; 0x53b3d0
# cdecl(nacao); 0x4ae8a0 thiscall(texto, adjetivo, 0x7d5, selecao) ret 0xc.
# Ajudante comum: write_country_deadline_story (usado pela sx21_31).
# Depende da sx21_3 (transfer_rules_write_deadline_news_text): integrar junto ou depois dela.
# Comparadores: compare_text_writer (o texto, os links e o sorteio).
SAVE = False
TELAS = True
ROTINAS = [
    ('italy_rules_write_deadline_news_text', 0x661b40, [0x83, 0xec, 0x64, 0x53, 0x56]),
    ('norway_rules_write_deadline_news_text', 0x7948e0, [0x83, 0xec, 0x64, 0x53, 0x56]),
]
DOCS = ('os textos das notícias do prazo de transferências da Itália e da Noruega',
        'the transfer deadline news texts of Italy and Norway',
        [('italy_rules_write_deadline_news_text', 'italy_rules_write_deadline_news_text', '0x661b40', 'italy_rules.cpp', 'thiscall, ret 0xc',
          'regra: a manchete é a de base; a matéria pelo estado (3 voltam a comprar; 0 passou, 1 hoje, 2 na semana que vem), com o texto da janela de livres a partir da terceira janela, o adjetivo da nação e o link para a seleção',
          "rule: the headline is the base one; the story by the state (3 may buy again; 0 passed, 1 today, 2 next week), with the free transfer wording from the third window, the nation's adjective and the link to the national team"),
         ('norway_rules_write_deadline_news_text', 'norway_rules_write_deadline_news_text', '0x7948e0', 'norway_rules.cpp', 'thiscall, ret 0xc',
          'regra: a manchete é a de base; a matéria de passou, hoje e semana que vem com o adjetivo da nação e o link para a seleção; os outros estados, o texto de base',
          "rule: the headline is the base one; the passed, today and next week stories with the nation's adjective and the link to the national team; other states, the base text")])
CAMPOS = []
INTERESSA = []
