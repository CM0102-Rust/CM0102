# O motor de busca da tela de procura: as pessoas (search_eng.cpp; leitor sx19b, rodada 5, 24/11/2026):
# SearchEngine::find_staff (0x805310..0x805f0a, ret 8, com as saidas de cada palavra especial) e
# search_text_matches_staff_name (0x806080..0x8061ea, ret).
# DEPENDE DO sx19b_10 (search_text_contains, os search_text_matches_*, SEARCH_ENG_*): integrar junto ou depois.
# Quem chama: a tela de procura (0x5a3c8c, 0x5a41fc: jogadores e nao-jogadores); o nome tambem pela busca das noticias
# (0x807ef8). A carreira automatica nao passa.
# TELAS: o caminho no jogo: Procurar, jogadores e comissao: um texto vazio, um nome, uma nacao, um clube, uma divisao,
# um continente; as palavras especiais "No Club", "Recurring", "School", "Retired", "Retiring", "Sedgeford", "awol",
# "skive", "big club", "contract", "dislikes" e "homesick" (exatas, com maiusculas).
# Convencoes conferidas na desmontagem:
# - 0x805310 thiscall(motor, texto, jogadores) ret 8; as palavras por strcmp exato (inline); 0x62ece0
#   Injury::get_recurring_injury_level thiscall([0xb63d28], pessoa) ret 4 -> al; 0x418f60 AWOL::is_player_awol
#   thiscall([0xad9c6c], pessoa, 0) ret 8 -> al; 0x418ff0 AWOL::has_missed_training thiscall([0xad9c6c], pessoa) ret 4
#   -> al; 0x4cf5f0 get_contract_for_club thiscall([0xae1050], pessoa, clube) ret 8; o estado da pessoa em [0xae2d0c]
#   + 0x4f x indice + 0xb (bits 0x30); o jogador (+0x61) +7 > 0xa0 e +5 > 0x8c no "Sedgeford";
# - 0x806080 cdecl(pessoa, texto): 0x53dba0 database_format_staff_name cdecl(pessoa, 0xdd6bfc, 200, estilo 1..7) e
#   0x806260 search_text_contains(0xdd6bfc, texto, 0).
# Defeito do original: nas palavras de contrato ("big club", "contract", "dislikes", "homesick") o ponteiro da pessoa
# so avanca dentro do "tem clube" (0x80596c): da primeira pessoa sem clube ate o fim da contagem o laco olha a mesma
# pessoa, e a busca so lista quem vem antes dela. A copia repete.
# Comparadores: lado a lado; find_staff sobre o vetor (o da original e guardado, a copia o refaz, compara e o da
# original volta).
TELAS = True
ROTINAS = [('find_staff', 0x805310, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('search_text_matches_staff_name', 0x806080, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c])]
DOCS = ('o motor de busca da tela de procura: as pessoas',
        'the find screen search engine: the people',
        [('find_staff', 'SearchEngine::find_staff', '0x805310', 'search_eng.cpp', 'thiscall, ret 8',
          'enche o vetor com as pessoas da busca (jogadores ou comissão; a classificação 3 sempre entra): uma palavra especial faz a sua passada — "No Club" sem clube, "School" com menos de 18, "Retired"/"Retiring" pelo estado, "Recurring" com lesão recorrente, "awol" e "skive" pelo AWOL, "Sedgeford" os jogadores bons sem clube, "big club"/"contract"/"dislikes"/"homesick" pela marca do contrato (parando, como o original, na primeira pessoa sem clube); senão as pessoas cujo nome, nação, segunda nação, clube, divisão ou continente contém o texto (texto vazio: todas)',
          'fills the array with the people of the search (players or staff; classification 3 always fits): a special word makes its own pass — "No Club" without a club, "School" under 18, "Retired"/"Retiring" by the state, "Recurring" with a recurring injury, "awol" and "skive" by AWOL, "Sedgeford" good players without a club, "big club"/"contract"/"dislikes"/"homesick" by the contract flag (stopping, like the original, at the first person without a club); otherwise the people whose name, nation, second nation, club, division or continent contains the text (empty text: all)'),
         ('search_text_matches_staff_name', 'search_text_matches_staff_name', '0x806080', 'search_eng.cpp', 'cdecl',
          'escreve o nome da pessoa nos sete estilos e diz se o texto aparece em algum',
          'writes the person\'s name in the seven styles and tells whether the text occurs in any')])
CAMPOS = []
INTERESSA = []
