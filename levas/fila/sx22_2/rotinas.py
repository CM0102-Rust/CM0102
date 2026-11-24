# weather.cpp (leitor sx22, rodada 5, 24/11/2026): o texto da notícia de partida abandonada e o destrutor do tempo.
# - 0x924160 weather_write_abandoned_news_text: thiscall(this, notícia, completa, Text) ret 0xc (o escritor da notícia
#   de tipo 0x2a, tabela de news.rs). Manchete 0xad55f4 no molde da adiada (sx22_1). Corpo: motivo 0x10 (0xad5520), 4
#   (0xad544c), 8 (0xad5378), senão 0xad52b0 (sempre formata); argumentos conferidos na ordem de push: gêneros c, c,
#   h, h, v, v (+0x37); nomes c, h, v (+4); vínculos 0x4ae8a0 do mandante e do visitante. Usa os auxiliares do sx22_1
#   (weather_news_headline, weather_news_body): INTEGRAR_ANTES.txt.
# - 0x9227d0 Weather::~Weather: thiscall(this), ret 0: free 0x9452ca (cdecl, 1) de [this] e de [this+4] quando não
#   nulos, zerando cada um. Chamada só pelo free_game_data 0x82ff40 (0x830490; já em Rust, setup.rs, pelo exe): roda no
#   fim do jogo (ou ao liberar o jogo); fica no item por ser do mesmo arquivo; sem mutação (só libera).
# Iguais no cm0102-1080.exe. Tipo: simulação.
# Comparadores: o escritor por compare_text_writer; o destrutor só a original e os dois ponteiros nulos.
SAVE = False
ROTINAS = [('write_abandoned_news_text', 0x924160, [0x8b, 0x44, 0x24, 0x08, 0x53]),
           ('weather_free_tables', 0x9227d0, [0x56, 0x8b, 0xf1, 0x8b, 0x06])]
DOCS = ('o tempo: o texto da partida abandonada e a liberação das tabelas',
        'the weather: the abandoned match text and freeing the tables',
        [('write_abandoned_news_text', 'weather_write_abandoned_news_text', '0x924160', 'weather.cpp', 'thiscall, ret 0xc',
          'manchete "<mandante> vs <visitante> <liga> match abandoned"; corpo pelo motivo, nesta ordem: 0x10 falha dos refletores, 4 neve, 8 gramado encharcado, senão "será jogada outro dia"; vínculos dos dois clubes',
          'headline "<home> vs <away> <league> match abandoned"; body by the reason, in this order: 0x10 floodlight failure, 4 snowfall, 8 water-logged pitch, otherwise "will be replayed at a later date"; links of both clubs'),
         ('weather_free_tables', 'Weather::~Weather', '0x9227d0', 'weather.cpp', 'thiscall',
          'libera as duas tabelas do tempo (+0 e +4) e zera os ponteiros',
          "frees the weather's two tables (+0 and +4) and clears the pointers")])
CAMPOS = []
INTERESSA = []
