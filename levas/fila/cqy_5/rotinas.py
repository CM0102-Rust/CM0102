# As linhas de notícia das copas: um clube vence ou cai numa rodada e uma seleção/clube se classifica ou não de um
# grupo (comp_util.cpp; leitor cqy, 12/11/2026, a continuação pedida depois do compare_text_writer, commit 759c98f9d).
# Rodam 3 e 12 vezes na carreira de 8 ligas e 79 vezes (a primeira) na de ENG até 2002-06-01 (docs/cobertura_*.txt),
# chamadas pelo update_staff_records_for_status de cada copa (20 e 51 pontos de chamada).
# - 0x4b4420 comp_util_write_cup_win_or_defeat_news (368 bytes) e 0x4b4590 comp_util_write_group_qualify_news (704
#   bytes), lidas instrução a instrução (o pseudo-C do Ghidra erra as duas assinaturas: são cdecl de 4 e de 6
#   argumentos; o Text é o último).
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada (0x908552: 4 pushes, add esp, 0x10;
# 0x90839c: 6 pushes, add esp, 0x18 -> cdecl, sem devolução usada); dentro: 0x4bfa30 e 0x4b4850 (cqy_3), 0x4ba8f0
# (push forma; push id -> cdecl, byte), 0x66f4e0 langlib_sprintf (cdecl variádica: 8 e 11 argumentos), Text::set_string
# 0x4ae660 (push texto; mov ecx, text -> thiscall, ret 4) e Text::add_link 0x4ae8a0 (push dado; push tipo; push texto;
# mov ecx, text -> thiscall, ret 0xc).
# Comparadores: compare_text_writer (compare/media.rs): a original no Text de verdade e a cópia num Text local, lado a
# lado; o texto, os vínculos e o gerador iguais. As mutações são acusadas pelo comparador (o texto não volta para a
# simulação), como diz o guia.
# Determinismo: só os registros (clube, competição, jogo).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('comp_util_write_cup_win_or_defeat_news', 0x4b4420, [0x81, 0xec, 0xf4, 0x01, 0x00, 0x00]),
           ('comp_util_write_group_qualify_news', 0x4b4590, [0x8a, 0x44, 0x24, 0x10, 0x83, 0xec, 0x64])]
DOCS = ('as linhas de notícia das copas: vencer ou cair numa rodada e classificar-se ou não de um grupo',
        'the cup news lines: winning or losing in a round and qualifying or not from a group',
        [('comp_util_write_cup_win_or_defeat_news', 'comp_util_write_cup_win_or_defeat_news', '0x4b4420', 'comp_util.cpp', 'cdecl',
          'regra: a linha curta "<clube> vence em <rodada>" ou, com o resultado 2, 4 ou −1, "<clube> cai em <rodada>"; a rodada é o rótulo competição + fase + rodada (forma 2, a rodada sem o dígito da variante), com o gênero da rodada, da fase ou da competição; o texto ganha dois vínculos: o clube e a competição',
          'rule: the short line "<club> win in <round>" or, with result 2, 4 or −1, "<club> defeat in <round>"; the round is the competition + stage + round label (form 2, the round without its variant digit), with the gender of the round, the stage or the competition; the text gets two links: the club and the competition'),
         ('comp_util_write_group_qualify_news', 'comp_util_write_group_qualify_news', '0x4b4590', 'comp_util.cpp', 'cdecl',
          'regra: com o resultado 6 ou 7, "<time> se classifica do grupo"; com −2, "<time> não se classifica"; outro resultado, nada; com a fase, o nome completo dela e o nome longo da competição, e um terceiro vínculo para a fase; sem a fase, o nome curto da competição',
          'rule: with result 6 or 7, "<team> qualify from the group"; with −2, "<team> fail to qualify"; any other result, nothing; with the stage, its full name and the competition\'s long name, and a third link for the stage; without it, the competition\'s short name')])
CAMPOS = []
INTERESSA = []
