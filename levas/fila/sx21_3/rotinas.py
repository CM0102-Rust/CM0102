# As regras de transferencia de base, parte 3 (transfer_offer.cpp; leitor sx21, rodada 5, 24/11/2026): a noticia do
# prazo de transferencias de uma janela (slot 0x2c) e o texto dela (slot 0x30, tambem a resposta de base das nacoes
# com texto proprio). Rodam na carreira ENG: a Inglaterra usa as de base (as noticias de janela do gestor de
# transferencias, a uma semana, na vespera e no dia do prazo).
# Convencoes conferidas: 0x8e9080 thiscall(janela, tipo, ano) ret 0xc; 0x8e91b0 thiscall(noticia, materia, texto)
# ret 0xc (o this nao e lido); 0x77bbe0 thiscall(item, 0xfb2, 0); 0x785790 thiscall(vaga, valor) ret 8; 0x785830
# thiscall(vaga) ret 4; 0x786520/0x786640 thiscall(item, nacao/continente) ret 8; 0x786780 thiscall(item) ret 4;
# 0x6181d0 cdecl(nacao, saida, 100, 2); 0x66f4e0 cdecl variadico; 0x4ae660 thiscall(texto) ret 4.
# Comparadores: a noticia so a original (ela posta); o texto pelo compare_text_writer.
SAVE = False
ROTINAS = [
    ('transfer_rules_post_deadline_news', 0x8e9080, [0x6a, 0xff, 0x68, 0x46, 0x4a, 0x96, 0x00]),
    ('transfer_rules_write_deadline_news_text', 0x8e91b0, [0x83, 0xec, 0x64, 0x53, 0x56]),
]
DOCS = ('as regras de transferência de base: a notícia do prazo de transferências e o texto dela',
        'the base transfer rules: the transfer deadline news and its text',
        [('transfer_rules_post_deadline_news', 'transfer_rules_post_deadline_news', '0x8e9080', 'transfer_offer.cpp', 'thiscall, ret 0xc',
          'regra: a notícia 0xfb2 com a regra, o estado do prazo, o número da janela, a data da janela no ano dado e a nação, para os técnicos dos clubes da nação, senão do continente, senão para todos',
          "rule: news item 0xfb2 with the rule, the deadline state, the window number, the window's date in the given year and the nation, to the managers of the nation's clubs, else of the continent, else to all"),
         ('transfer_rules_write_deadline_news_text', 'transfer_rules_write_deadline_news_text', '0x8e91b0', 'transfer_offer.cpp', 'thiscall, ret 0xc',
          'regra: pelo estado (vaga 1: 0 passou, 1 hoje, 2 em uma semana, 3 suspenso): a matéria com o adjetivo da nação (vaga 5) ou a manchete "Transfer deadline", "... today", "... in one week", "... lifted"',
          'rule: by the state (slot 1: 0 passed, 1 today, 2 in a week, 3 lifted): the story with the nation\'s adjective (slot 5) or the headline "Transfer deadline", "... today", "... in one week", "... lifted"')])
CAMPOS = []
INTERESSA = []
