# Os textos das notícias de nomeação e de pressão sobre o técnico (manager_manager.cpp; leitor sma, 12/11/2026).
# Chamados pelas notícias (o texto de cada tipo); as notícias são postadas pelas Rust post_manager_appointment_news
# (0x6a2e00) e post_manager_under_pressure_news (0x6a0e70). Convenções conferidas:
# - as duas: stdcall(notícia, corpo, Text), ret 0xc (o ecx não é usado);
# - 0x785830 (newsitem_get_slot) e 0x785a20 (get_slot_string, Rust): ecx = notícia -> thiscall;
# - 0x53e140 (nome por ids): push id; push estilo; push 0x190; push buf; push id3; push id2; push id1 -> cdecl;
#   0x53e2e0 (nome por textos, humano): push -1; push 1; push 0x190; push buf; push 0; push segundo; push primeiro;
# - 0x53ad00 (nome do cargo, Rust): push &gênero; push 0; push 0; push cargo (dword com o byte baixo) -> cdecl;
# - 0x6181d0 (adjetivo da nação): push 9; push 0x64; push buf; push nação -> cdecl; 0x944c9f (sprintf "%s"): cdecl;
# - 0x53e990 (os ids ainda batem): push 0; push id3; push id2; push id1; push pessoa -> cdecl;
# - 0x66f4e0 em 0xde1f64 (o formato da língua), 0x4ae660 e 0x4ae8a0 (o Text): como nos outros textos.
# Comparadores: compare_text_writer.
SAVE = False
ROTINAS = [('news_manager_appointment_text', 0x6a3060, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x14, 0x02, 0x00, 0x00]),
           ('news_manager_under_pressure_text', 0x6a1190, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x34, 0x05, 0x00, 0x00])]
DOCS = ('os textos das notícias de nomeação e de pressão sobre o técnico',
        'the manager appointment and pressure news texts',
        [('news_manager_appointment_text', 'news_manager_appointment_text', '0x6a3060', 'manager_manager.cpp', 'stdcall, ret 0xc',
          'o título ou o corpo da notícia de que um clube nomeou ou promoveu alguém a técnico: a manchete com o nome curto do clube; o corpo pelo código (1 jogador, 2 comissão, 3 técnico), pelo clube anterior (o mesmo ou outro) e pelo cargo antigo (com o nome do cargo); o nome do humano vem dos textos da notícia; vínculos da pessoa (quando os ids ainda batem), do clube e do clube anterior',
          'the headline or body of the news that a club appointed or promoted someone to manager: the headline with the club\'s short name; the body by the code (1 player, 2 staff, 3 manager), by the previous club (the same or another) and by the old job (with the job\'s name); a human\'s name comes from the news strings; links for the person (when the ids still match), the club and the previous club'),
         ('news_manager_under_pressure_text', 'news_manager_under_pressure_text', '0x6a1190', 'manager_manager.cpp', 'stdcall, ret 0xc',
          'o título ou o corpo da notícia de pressão sobre o técnico: pelo tipo (boato), pelo motivo (1 a 4), pelo presidente (com ou sem nome) e pelo boato; a federação no lugar do clube numa seleção com nação (com o adjetivo da nação); vínculos do técnico e do clube',
          'the headline or body of the story about pressure on the manager: by the kind (rumour), by the reason (1 to 4), by the chairman (named or not) and by the rumour; the FA instead of the club at a national team with a nation (with the nation\'s adjective); links for the manager and the club')])
