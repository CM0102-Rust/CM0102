# As noticias do prazo de transferencias da Franca, da Italia e da Noruega (slot 0x2c; leitor sx21, rodada 5,
# 24/11/2026). O gestor de transferencias (update_transfer_window_news, 0x8c6bc0) passa por todas as janelas de todas as
# nacoes, carregadas ou nao: rodam na carreira ENG (na abertura, na vespera e uma semana antes de cada janela), mas a
# noticia so vai aos tecnicos dos clubes da nacao.
# Convencoes conferidas: thiscall(janela, tipo, ano) ret 0xc (quadro SEH sem destrutor); 0x40a7d0 thiscall(saida, ano)
# ret 8; 0x77bbe0 thiscall(item, 0xfb2, 0); 0x785790 thiscall(vaga, valor) ret 8; 0x786520 thiscall(item, nacao)
# ret 8. A Franca usa a chave KEY_NATION_FRANCE na vaga 5 e no envio; Italia e Noruega, a nacao das regras (+0xa).
# Ajudante comum: post_country_deadline_news (usado pela sx21_25).
# Comparadores: so a original (as noticias sao postadas). Sem mutacao: a noticia nao volta a simulacao e o comparador
# nao ve a copia (ver mutacoes.py).
SAVE = False
ROTINAS = [
    ('france_rules_post_deadline_news', 0x5c1a70, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
    ('italy_rules_post_deadline_news', 0x661a30, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
    ('norway_rules_post_deadline_news', 0x7947d0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
]
DOCS = ('as notícias do prazo de transferências da França, da Itália e da Noruega',
        'the transfer deadline news of France, Italy and Norway',
        [('france_rules_post_deadline_news', 'france_rules_post_deadline_news', '0x5c1a70', 'france_rules.cpp', 'thiscall, ret 0xc',
          'regra: a notícia 0xfb2 aos técnicos dos clubes franceses, menos na janela restrita (número 1)',
          'rule: news item 0xfb2 to the managers of French clubs, except in the restricted window (number 1)'),
         ('italy_rules_post_deadline_news', 'italy_rules_post_deadline_news', '0x661a30', 'italy_rules.cpp', 'thiscall, ret 0xc',
          'regra: a notícia 0xfb2 aos técnicos dos clubes italianos, menos o aviso de suspensão (3) da terceira janela',
          'rule: news item 0xfb2 to the managers of Italian clubs, except the lifted notice (3) of the third window'),
         ('norway_rules_post_deadline_news', 'norway_rules_post_deadline_news', '0x7947d0', 'norway_rules.cpp', 'thiscall, ret 0xc',
          'regra: a notícia 0xfb2 aos técnicos dos clubes noruegueses, menos os avisos de suspensão (3)',
          'rule: news item 0xfb2 to the managers of Norwegian clubs, except the lifted notices (3)')])
CAMPOS = []
INTERESSA = []
