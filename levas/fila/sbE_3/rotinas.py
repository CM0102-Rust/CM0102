# A tabela da Bundesliga e a divisao da Regionalliga em Norte e Sul (ger_first.cpp; leitor sbE, rodada 5, 24/11/2026).
# Simulacao com a Alemanha (as ligas e a data dos itens mwc): a tabela na criacao do jogo (2001, com a reuniao das
# quatro divisoes regionais antigas) e na virada de 2002; a divisao na virada da Bundesliga (GerFirst::start_new_season
# 0x5db4a0, mwc_21, chamada direta), no verao de 2002. A Regionalliga nao e carregada no teste, mas os seus clubes
# existem (a tabela de 2001 os junta), entao a divisao roda.
# - 0x5db230 GerFirst::create_table thiscall() ret 0: +0x3e = 18; em 2001 os clubes de 0x9cf554, 0x9cf558, 0x9cf55c e
#   0x9cf560 (indice ate clubes - 2 x nacoes) vao para a Regionalliga (0x9cf564) com a divisao antiga na reserva
#   (+0x60); a tabela (malloc 18 x 0x3b; caixa 0x26e) com os clubes da Bundesliga (0x687430), cheia: fundo
#   0x682f70(0x30d40), +0xe1 = 1, TV 0x683010(0x4c4b40, 0) e 1. Prologo sub esp, 0x200 (6 bytes).
# - 0x5dbbd0 ger_first_split_regional_north_south thiscall() ret 0 (ecx nao lido): os 36 clubes da Regionalliga (senao
#   a caixa 0x419); as linhas 54.321 (0x96b1a8) e 47.55 (0x96b1a0) pela latitude da cidade (estadio +0x69, cidade
#   +0x38 dele, latitude +0x23), a do norte descendo 0.3 (0x968e80) e a do sul subindo 0.278 (0x96b198) a cada passada
#   (x87) ate todos os clubes com cidade terem lugar; os sem cidade pela secao antiga e depois onde houver lugar; as
#   secoes novas na divisao reserva; depois as trocas de volta em pares e em trios. Quadro com ebp e esp alinhado a 8.
#   Prologo push ebp; mov ebp, esp; and esp, -8 (6 bytes).
# Depende do sbE_2 (move_group_clubs_to_division): integrar depois dele.
# Iguais no cm0102-1080.exe (inteiras). Comparadores: a tabela so a original contra o plano (compare_nation_league_table)
# e nenhum clube nas divisoes antigas em 2001; a divisao lado a lado na divisao reserva dos 36 clubes.
# Duvida: as linhas da latitude andam em x87 (precisao do controle da FPU do jogo); a copia em f64. So difere num clube
# exatamente na linha.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('ger_first_create_table', 0x5db230, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ger_first_split_regional_north_south', 0x5dbbd0, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8])]
DOCS = ('a tabela da Bundesliga e a divisão da Regionalliga em Norte e Sul',
        'the Bundesliga table and the Regionalliga split into North and South',
        [('ger_first_create_table', 'GerFirst::create_table', '0x5db230', 'ger_first.cpp', 'thiscall, ret 0',
          'a Bundesliga tem 18 clubes; fundo de prêmios pela cota de 200 mil por posição; a TV paga 5 milhões a cada clube; em 2001 os clubes das quatro divisões regionais antigas são reunidos antes na Regionalliga',
          'the Bundesliga has 18 clubs; prize fund by the share of 200,000 per place; TV pays 5,000,000 to each club; in 2001 the clubs of the four old regional divisions are first gathered into the Regionalliga'),
         ('ger_first_split_regional_north_south', 'ger_first_split_regional_north_south', '0x5dbbd0', 'ger_first.cpp', 'thiscall, ret 0',
          'os 36 clubes da Regionalliga em duas seções de 18 pela latitude da cidade (uma linha ao norte que desce e uma ao sul que sobe até todos terem lugar); os sem cidade pela seção antiga; depois quem mudou de seção volta à antiga em pares ou trios, mantendo 18 em cada',
          'the 36 Regionalliga clubs into two sections of 18 by the latitude of their city (a north line moving down and a south line moving up until all are placed); those without a city by their old section; then clubs that changed section go back in pairs or threes, keeping 18 in each')])
CAMPOS = []
INTERESSA = []
