# O texto da notícia de aposentadoria (player_regen_write_retirement_announcement_text 0x7aa4c0, player_regen.cpp;
# leitor sma, 12/11/2026). Chamado pelas notícias (0x77bc80, o texto de cada tipo). Convenções conferidas:
# - a rotina: stdcall(notícia, corpo, Text), ret 0xc (o ecx não é usado);
# - 0x785830 (newsitem_get_slot): ecx = notícia -> thiscall; 0x53e140 (nome por ids): cdecl, 0x190, estilo 5/1;
# - o humano da tela: movsx ecx, word [0xb77c7a]; cmp eax, [ecx × 0x300 + 0xb74c26] (o técnico do clube, +0xcf);
# - 0x53ad00 (nome do cargo, Rust): push &gênero; push 0; push 0; push cargo (byte baixo) -> cdecl;
# - 0x66f4e0 em 0xde1f64, 0x4ae660 e 0x4ae8a0 (o Text), 0x53e990 (os ids batem): como nos outros textos.
# Comparador: compare_text_writer.
SAVE = False
ROTINAS = [('player_regen_write_retirement_announcement_text', 0x7aa4c0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0xa4, 0x01, 0x00, 0x00])]
DOCS = ('o texto da notícia de aposentadoria',
        'the retirement announcement text',
        [('player_regen_write_retirement_announcement_text', 'player_regen_write_retirement_announcement_text', '0x7aa4c0', 'player_regen.cpp', 'stdcall, ret 0xc',
          'o título ou o corpo da notícia de que alguém anuncia a aposentadoria: a manchete com o nome; o corpo pelo tipo (1, 2, 0x10, 0x20), pela ficha de não jogador, pelo cargo no clube (0xc, 0xd, 0xf, outros) ou na seleção (com o nome do cargo), pela idade, e em duas versões: dirigida ao humano quando o clube é o dele, ou com o nome curto do clube; o vínculo da pessoa quando os ids ainda batem',
          'the headline or body of the news that someone announces his retirement: the headline with the name; the body by the kind (1, 2, 0x10, 0x20), by the non-player sheet, by the club job (0xc, 0xd, 0xf, others) or the national job (with the job\'s name), by the age, in two versions: addressed to the human when the club is his, or with the club\'s short name; the person\'s link when the ids still match')])
