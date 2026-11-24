# Os textos das noticias do prazo de transferencias da Argentina, da Alemanha e da Belgica (slot 0x30; leitor sx21,
# rodada 5, 24/11/2026). O texto so e escrito quando a noticia chega a um tecnico humano de clube da nacao: nao roda nas
# carreiras de teste. Tipo TELAS, provado pelo Lucas jogando com um clube argentino, alemao ou belga.
# Caminho no jogo: novo jogo com a liga da nacao carregada, assumir um clube de la e avancar ate as noticias de prazo
# (Argentina: a abertura da segunda janela em 14 de dezembro; Alemanha e Belgica: uma semana antes, a vespera e o dia
# do fim de cada janela) e abrir a noticia (a materia; a manchete vem do texto de base).
# Convencoes conferidas: thiscall(noticia, materia, texto) ret 0xc; as tres leem as vagas 1, 2 e 5 antes de decidir;
# 0x6181d0 cdecl(nacao, saida, 100, forma); 0x66f4e0 cdecl variadico (com dois adjetivos no estado 0 da janela 0);
# 0x4ae660 thiscall(texto) ret 4. Sem link (ao contrario da Italia e da Noruega).
# Defeito do original (Alemanha e Belgica): janela sem formato propria poe no texto o que o buffer 0xde1f64 tiver.
# Depende da sx21_3 (transfer_rules_write_deadline_news_text, format_with_nationality): integrar junto ou depois dela.
# Comparadores: compare_text_writer.
SAVE = False
TELAS = True
ROTINAS = [
    ('argentina_rules_write_deadline_news_text', 0x40aa30, [0x83, 0xec, 0x64, 0x53, 0x56]),
    ('germany_rules_write_deadline_news_text', 0x5e0420, [0x83, 0xec, 0x64, 0x53, 0x56]),
    ('belgium_rules_write_deadline_news_text', 0x4250b0, [0x83, 0xec, 0x64, 0x53, 0x56]),
]
DOCS = ('os textos das notícias do prazo de transferências da Argentina, da Alemanha e da Bélgica',
        'the transfer deadline news texts of Argentina, Germany and Belgium',
        [('argentina_rules_write_deadline_news_text', 'argentina_rules_write_deadline_news_text', '0x40aa30', 'argentina_rules.cpp', 'thiscall, ret 0xc',
          'regra: a matéria da abertura da segunda janela (estado 3, janela 1): os clubes argentinos voltam a comprar, com duas contratações no máximo até fevereiro; senão o texto de base',
          'rule: the story of the second window opening (state 3, window 1): Argentine clubs may buy again, two signings at most until February; else the base text'),
         ('germany_rules_write_deadline_news_text', 'germany_rules_write_deadline_news_text', '0x5e0420', 'germany_rules.cpp', 'thiscall, ret 0xc',
          'regra: a manchete é a de base; a matéria pelo estado e pela janela (os prazos das divisões inferiores na janela 0, os das transferências entre clubes da nação na janela 1)',
          "rule: the headline is the base one; the story by the state and the window (the lower league deadlines in window 0, those of transfers between the nation's clubs in window 1)"),
         ('belgium_rules_write_deadline_news_text', 'belgium_rules_write_deadline_news_text', '0x4250b0', 'belgium_rules.cpp', 'thiscall, ret 0xc',
          'regra: como a da Alemanha, com a terceira janela (o prazo geral)',
          "rule: as Germany's, with the third window (the general deadline)")])
CAMPOS = []
INTERESSA = []
