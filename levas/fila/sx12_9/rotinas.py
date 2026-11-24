# Os rotulos de temporada e os nomes da divisao extra das nacoes-chave (key_nation.cpp; leitor sx12, rodada 5,
# 23/11/2026). Nao rodam na carreira de ferias (0 chamadas na cobertura do mundo): so nas telas.
# Caminho no jogo para o Lucas provar (JOGAR_COMPARANDO.bat):
# - key_nation_format_club_season 0x669630: as telas de historia e recordes do clube (club_records 0x4499d1..,
#   club_screens 0x46b45f.., o historico do jogador 0x875634): abrir a historia de um clube e de um jogador;
# - key_nation_format_comp_season 0x6697c0: as telas de historia das competicoes (0x499c77.., 0x4a5873..) e das
#   noticias (0x7d81fe): abrir a historia da Premier League e de uma copa;
# - key_nation_extra_division_name 0x669f50: a escolha das ligas na criacao do jogo (Setup 0x81ce00): marcar a
#   Inglaterra, a Alemanha, a Italia, Portugal, a Espanha ou a Suecia e ver a opcao da divisao extra.
# Convencoes conferidas na desmontagem (cdecl):
# - format_club_season(ano (so a palavra baixa), clube, texto, tamanho) e format_comp_season(ano, competicao, texto,
#   tamanho): eax 1 ou 0 (caixa "Error", linhas 0x30b e 0x350); 0x944c9f sprintf "%d/%d" (ano, (ano + 1) mod 10) ou
#   "%d"; o ano - 1 e "add dword, 0xffff" (so a palavra baixa vale, "and 0xffff" depois); 0x669360 e 0x669470 cdecl.
# - extra_division_name(nacao, texto, tamanho): nacao nula 0 sem caixa; texto nulo nao confere o tamanho e devolve 1 nas
#   nacoes com divisao extra; 0x66f4e0 langlib_sprintf cdecl (o modelo da Inglaterra com o texto de 0xad9c64).
SAVE = False
TELAS = True
ROTINAS = [('key_nation_format_club_season', 0x669630, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('key_nation_format_comp_season', 0x6697c0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('key_nation_extra_division_name', 0x669f50, [0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os rotulos de temporada e a divisao extra das nacoes-chave',
        'the season labels and the extra division of the key nations',
        [('key_nation_format_club_season', 'key_nation_format_club_season', '0x669630', 'key_nation.cpp', 'cdecl',
          'escreve o rotulo da temporada de um clube: "2001/2" quando a linha da nacao do clube atravessa dois anos, senao "2001"; ano 0 e a temporada de hoje (o ano anterior antes do fim da temporada da linha); clube nulo ou selecao, o ano simples; texto nulo ou curto, a caixa "Error" e 0',
          "writes a club's season label: \"2001/2\" when the row of the club's nation spans two years, else \"2001\"; year 0 is today's season (the year before, before the row's season end); a null club or a national team, the plain year; a null or short text, the \"Error\" box and 0"),
         ('key_nation_format_comp_season', 'key_nation_format_comp_season', '0x6697c0', 'key_nation.cpp', 'cdecl',
          'escreve o rotulo da temporada de uma competicao do mesmo jeito, pela linha da competicao (as linhas do mundo 0 e 1 dao o ano simples)',
          "writes a competition's season label the same way, by the competition's row (the world rows 0 and 1 give the plain year)"),
         ('key_nation_extra_division_name', 'key_nation_extra_division_name', '0x669f50', 'key_nation.cpp', 'cdecl',
          'escreve o nome da divisao extra da nacao e devolve 1: Inglaterra "Conference Division", Alemanha "Regional Divisions", Italia "Serie C2 A, B, C", Portugal e Espanha "Second Division B", Suecia "Second Division"; outra nacao ou nula, 0; texto de menos de 20 bytes, a caixa "Error" e 0',
          "writes the name of the nation's extra division and returns 1: England \"Conference Division\", Germany \"Regional Divisions\", Italy \"Serie C2 A, B, C\", Portugal and Spain \"Second Division B\", Sweden \"Second Division\"; another or a null nation, 0; a text under 20 bytes, the \"Error\" box and 0")])
