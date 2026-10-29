# Notícias e olheiros: o texto de um dado da notícia (NewsItem::get_slot_string 0x785a20, pelos textos das notícias
# 0x77bc80/0x6a1d60/0x6a3060; strncpy 0x9466d0), a notícia do clube em apuro financeiro (Media::post_financial_trouble_news
# 0x74ae60, pelo passe diário das finanças 0x59b9c0; constrói a notícia local com 0x77bbe0, original; 0x785790,
# 0x786780 e 0x7863f0 já em Rust) e se os olheiros do clube cobrem a nação (SCOUT_MANAGER::can_club_scout_nation
# 0x7f9190, pelas 0x7b7490/0x7f8ff0; finanças pela 0x5a0590). Simulação: as mutações pela carreira e a confirmação no
# jogo real comparando. A 0x74ae60 tem quadro SEH sem destrutor (a notícia local não tem) e sorteia uma vez
# (0x90cfc0); caixa só com dado fora da faixa, destino nulo ou sem clube (a original). Comparadores: o texto lado a
# lado pelo destino; a notícia só pela original (sorteia e entrega); a cobertura pelo valor.
SAVE = False
ROTINAS = [('texto_do_dado_da_noticia', 0x785a20, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('noticia_de_apuro_financeiro', 0x74ae60, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('olheiros_cobrem_a_nacao', 0x7f9190, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o texto de um dado da notícia, a notícia do clube em apuro financeiro e se os olheiros do clube cobrem a nação',
        'the text of a news slot, the news of a club in financial trouble and whether the club scouts cover a nation',
        [('texto_do_dado_da_noticia', 'NewsItem::get_slot_string', '0x785a20', 'news.cpp', 'thiscall, ret 0xc',
          'copia o texto guardado a partir do dado (+5 + 4 × dado) para o destino (strncpy) e zera o último byte; devolve 1',
          'copies the text stored from the slot (+5 + 4 × slot) into the buffer (strncpy) and clears the last byte; returns 1'),
         ('noticia_de_apuro_financeiro', 'Media::post_financial_trouble_news', '0x74ae60', 'media.cpp', 'thiscall, ret 4',
          'notícia 0x1790 (id, reputação, sorteio de 2) para os técnicos da nação, ou para todos num clube grande (9000+)',
          'news 0x1790 (id, reputation, draw of 2) to the managers of the nation, or to all for a big club (9000+)'),
         ('olheiros_cobrem_a_nacao', 'SCOUT_MANAGER::can_club_scout_nation', '0x7f9190', 'scout_manager.cpp', 'thiscall, ret 8',
          'clube pobre (saldo < 1.000.000) e pequeno (reputação < 5000) fora da região da nação: 0; senão 1',
          "poor (balance < 1,000,000) and small (reputation < 5000) club outside the nation's region: 0; else 1")])
CAMPOS = [('notícia +5 + 4 × k dados (texto a partir do dado k); lê (texto_do_dado_da_noticia)',
           'news +5 + 4 × k slots (text from slot k); reads (texto_do_dado_da_noticia)'),
          ('clube +0 id, +0x53 nação, +0x57 divisão, +0x69 ficha (+0x38), +0x80 reputação; divisão +0x69; nação +0x76 região; finanças do clube (0x5a0590) +0 saldo de 64 bits; lê (noticia_de_apuro_financeiro, olheiros_cobrem_a_nacao)',
           'club +0 id, +0x53 nation, +0x57 division, +0x69 record (+0x38), +0x80 reputation; division +0x69; nation +0x76 region; club finances (0x5a0590) +0 64-bit balance; reads (noticia_de_apuro_financeiro, olheiros_cobrem_a_nacao)'),
          ('[0xb74c26 + 0x300 × [0xb77c7a]] técnico humano da vez (+0x39 clube); ponteiro; lê (olheiros_cobrem_a_nacao)',
           '[0xb74c26 + 0x300 × [0xb77c7a]] current human manager (+0x39 club); pointer; reads (olheiros_cobrem_a_nacao)')]
INTERESSA = ['CMRTE']
