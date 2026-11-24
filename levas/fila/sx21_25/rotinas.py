# As noticias do prazo de transferencias da Belgica e da Alemanha (slot 0x2c; leitor sx21, rodada 5, 24/11/2026). O
# gestor de transferencias (0x8c6bc0) passa por todas as janelas de todas as nacoes: rodam na carreira ENG, mas a
# noticia so vai aos tecnicos dos clubes da nacao.
# Convencoes conferidas: thiscall(janela, tipo, ano) ret 0xc (quadro SEH sem destrutor); a vaga 5 e a chave da nacao
# (KEY_NATION_BELGIUM, KEY_NATION_GERMANY), o envio vai a nacao das regras (+0xa).
# Depende da sx21_24 (post_country_deadline_news): integrar junto ou depois dela.
# Comparadores: so a original (as noticias sao postadas). Sem mutacao (ver mutacoes.py).
SAVE = False
ROTINAS = [
    ('belgium_rules_post_deadline_news', 0x424fa0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
    ('germany_rules_post_deadline_news', 0x5e0310, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
]
DOCS = ('as notícias do prazo de transferências da Bélgica e da Alemanha',
        'the transfer deadline news of Belgium and Germany',
        [('belgium_rules_post_deadline_news', 'belgium_rules_post_deadline_news', '0x424fa0', 'belgium_rules.cpp', 'thiscall, ret 0xc',
          'regra: a notícia 0xfb2 aos técnicos dos clubes belgas, menos o aviso de suspensão (3) das janelas depois da primeira',
          'rule: news item 0xfb2 to the managers of Belgian clubs, except the lifted notice (3) of the windows after the first'),
         ('germany_rules_post_deadline_news', 'germany_rules_post_deadline_news', '0x5e0310', 'germany_rules.cpp', 'thiscall, ret 0xc',
          'regra: a notícia 0xfb2 aos técnicos dos clubes alemães, menos o aviso de suspensão (3) das janelas depois da primeira',
          'rule: news item 0xfb2 to the managers of German clubs, except the lifted notice (3) of the windows after the first')])
CAMPOS = []
INTERESSA = []
