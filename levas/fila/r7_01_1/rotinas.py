# O modificador de gramática do Langlib.cpp (leitor r7_01, rodada 7, 28/11/2026): a 0x6701f0, a maior rotina que faltava
# (73.328 bytes no mapa: ~64 KB de código, 14 ret, e ~170 tabelas de saltos e índices de 0x67fdc4 a 0x682060).
# - 0x6701f0 langlib_apply_grammar_modifier (cdecl, 5 dwords: o texto, a marca antes, a marca depois e os dois bytes de
#   gênero, lidos com movsx; add esp, 0x14 em quem chama). Devolve 0xb64804 (a saída, zerada na entrada), 0xad9c64 (texto
#   nulo) ou 0. Chamada pelo langlib_sprintf (rx04_2, todo texto traduzido com gênero), pelos nomes de competição
#   (comp_util) e pela narração (match_events).
# - Portada: a entrada, o ramo inglês (0, 0x670276), o português (9, 0x67b506..0x67c2e5) e as línguas sem regra (1, -1:
#   0x670974, a saída = o texto). Francês (2), alemão (3), italiano (4), espanhol (5), dinamarquês (6), holandês (7),
#   norueguês (8) e sueco (10) continuam na original pelo trampolim (original_fn), como a area_layout: ver FORA.md.
# - Os textos e as marcas lidos do exe pelo endereço (as tabelas de 5 formas do português conferidas pelas tabelas de
#   saltos 0x6810f8..0x6812e4; o índice dos gêneros de "do"/"di" em 0x6811dc: 100 = nada, 5..99 = " ").
# - Iguais no cm0102-1080.exe (0x6701f0..0x682060 inteiro). Prólogo: mov eax, 0x2808 (5 bytes, sem relativo; o call do
#   _chkstk vem depois).
# - Prova no emulador (unicorn, rascunho r7_01\check_gram.py): a original contra um modelo da cópia que lê as tabelas
#   deste codigo.rs, 20.592 casos (inglês, português e língua 1; todas as marcas, 8 textos, 10 gêneros), 0 diferenças;
#   com a tabela "deste" corrigida, 48 diferenças (o teste discrimina).
# Defeitos do original: (1) o teste do texto nulo vem depois da cópia do texto (repne scasb no nulo): um nulo derruba antes;
# (2) a tabela de "deste" está um gênero atrasada ("desta " para o masculino, "destes " para o feminino...).
# Comparador: lado a lado, a cópia antes em cópias das marcas e num buffer próprio, depois a original; a resposta, o texto
# e as marcas que sobram (o português apaga a marca que não conhece) iguais.
SAVE = False
ROTINAS = [('langlib_apply_grammar_modifier', 0x6701f0, [0xb8, 0x08, 0x28, 0x00, 0x00])]
DOCS = ('o modificador de gramática: artigos, preposições e concordância das marcas dos textos',
        'the grammar modifier: the articles, prepositions and agreement of the text marks',
        [('langlib_apply_grammar_modifier', 'langlib_apply_grammar_modifier', '0x6701f0', 'Langlib.cpp', 'cdecl',
          'regra: devolve, na saída do modificador, o prefixo, o texto e o sufixo que a marca antes e a marca depois pedem na língua do jogo. Inglês: "upper" põe a 1a letra em maiúscula, "s"/"S" depois põem o possessivo ("\'" depois de um s final, senão "\'s"; o de um texto vazio devolve nulo) e "an"/"An" antes dão "an "/"An " antes de vogal e "a "/"A " no resto. Português: "upper" e "lower" mudam a caixa; o artigo ou a preposição da marca antes ("no", "o", "O", "do", "Do", "DO", "ao", "pelo", "um", "Um", "deste") e o particípio da marca depois ("promovido", "relegado", "foi promovido", "despromovido", "eliminado", "foi despromovido", "foi premiado", "está qualificado") concordam com os dois bytes de gênero (0 masculino, 1 feminino, 2 e 3 plurais, 4 sem artigo; outro valor dá " ", e "do" com 100 não dá nada); a marca que não conhece é apagada na string de quem chama. Língua sem regra: o texto como está. As outras línguas continuam na original',
          'rule: returns, in the modifier\'s output, the prefix, the text and the suffix the mark before and the mark after ask for in the game language. English: "upper" puts the first letter in upper case, "s"/"S" after add the possessive ("\'" after a final s, otherwise "\'s"; that of an empty text returns null) and "an"/"An" before give "an "/"An " before a vowel and "a "/"A " otherwise. Portuguese: "upper" and "lower" change the case; the article or preposition of the mark before ("no", "o", "O", "do", "Do", "DO", "ao", "pelo", "um", "Um", "deste") and the participle of the mark after ("promovido", "relegado", "foi promovido", "despromovido", "eliminado", "foi despromovido", "foi premiado", "está qualificado") agree with the two gender bytes (0 masculine, 1 feminine, 2 and 3 plural, 4 without article; any other value gives " ", and "do" with 100 gives nothing); a mark it does not know is erased in the caller\'s string. A language without rules: the text as it is. The other languages still run the original')])
INTERESSA = []
