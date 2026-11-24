# As financas iniciais de um clube, a tabela das financas e a obra do estadio pedida pela liga (fin_prm.cpp pelo mapa,
# finance.CPP pelos asserts; leitor sbA, rodada 5, 24/11/2026). Carreira ENG (sem LIGAS).
# - 0x5961d0 Finance::init_for_club thiscall(clube) ret 4: roda na criacao do jogo (pela tabela, um por clube). Saldo
#   inicial pela caixa (+0x65) ou pela tabela 0x9c8290 por reputacao, padrao da liga (+0x85) e desenvolvimento (+0x7e)
#   da nacao, com sorteios; contador de aperto, totais zerados (+0x20..+0x160), carnes e o teto salarial (so ENG, ITA e
#   ESP). Sem clube: a caixa de erro (finance.CPP 0x8e) e o fim do jogo (0x5cc7d0, 0x5a8f60, exit(-1)).
# - 0x59a370 FinanceTable::FinanceTable thiscall(save) ret 4: roda na criacao e na carga. O CD (0x59a3c3 e 0x59a406, o
#   patch sem CD do cm0102-1080 e do portatil troca esses 3 bytes: find_game_cd le os bytes do exe que roda; a rotina e
#   desviada nos dois exes pelos dois hashes), operator new(n * 0x167 + 4) e o vector constructor iterator (0x5961c0,
#   0x5a8f60), a carga pelo save (0x59b2a0) ou o init_for_club de cada clube.
# - 0x599e00 Finance::upgrade_stadium_for_league thiscall(lugares sentados, capacidade, competicao, opcional) ret 0x10
#   -> u8: quando um clube entra numa liga com estadio minimo (criacao do jogo e fim de temporada). A noticia dos planos
#   e 0x5a0410 (cx10_6, chamada pelo endereco).
# Convencoes conferidas: utils_random 0x90cfc0 (cdecl), _ftol 0x945640 (o saldo usa edx:eax inteiros), crt_strncmp
# 0x946fa0, get_wealth_band 0x5986b0 (thiscall, al), 0x5a0410 (ecx = [0xae2a38]; push tipo, competicao, id; ret 0xc).
# Prologos: 0x5961d0 push ebp; mov ebp, esp; and esp, -8 (6 bytes); 0x59a370 push -1; push imm32 (7); 0x599e00 sub esp,
# 0x14; push ebx; push ebp; push esi (6). Sem relativas. 0x5961d0 e 0x599e00 iguais no cm0102-1080.exe; 0x59a370
# difere so nos 3 bytes do patch sem CD.
# Defeito do original: a linha da tabela do saldo inicial passa da tabela com reputacao acima de 10999 (nao acontece).
# Comparadores: init_for_club e upgrade_stadium_for_league lado a lado (sem tecnico humano no segundo: a noticia so vai
# ao humano); o construtor so a original, conferindo a tabela que deixa (um registro por clube, a contagem na frente).
# Depende do sbA_1 (FINANCE_SIZE) e do sx02b_1 (find_game_cd, leva 330): integrar antes.
SAVE = False
ROTINAS = [('init_for_club', 0x5961d0, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8]),
           ('new_finance_table', 0x59a370, [0x6a, 0xff, 0x68, 0x0e, 0x90, 0x95, 0x00]),
           ('upgrade_stadium_for_league', 0x599e00, [0x83, 0xec, 0x14, 0x53, 0x55, 0x56])]
DOCS = ('as finanças iniciais de um clube, a tabela das finanças e a obra do estádio pedida pela liga',
        "a club's starting finances, the finance table and the league's ground upgrade",
        [('init_for_club', 'Finance::init_for_club', '0x5961d0', 'fin_prm.cpp (finance.CPP)', 'thiscall, ret 4',
          'monta as finanças de um clube no começo do jogo: seleção sem nada; o semiprofissional com objeto de competição na divisão vira status 2; o saldo pela caixa do banco de dados (abaixo de 1,5 × reputação², e sempre para o Rushden) ou pela tabela por reputação ajustada pelo padrão da liga e pelo desenvolvimento da nação, no máximo reputação² × 0,5, um terço no status 2 e um quinto no 3, com sorteios (na Coreia do Sul o que passa de 250 mil cai pela metade); o contador de aperto, os totais zerados, os carnês e o teto salarial (só Inglaterra, Itália e Espanha, reputação acima de 4750, faixas de riqueza 1 e 2)',
          "sets up a club's finances at the start of the game: a national team gets nothing; a semi-professional club with a competition object in its division becomes status 2; the balance from the database cash (under 1.5 × reputation², and always for Rushden) or from the table by reputation adjusted by the league standard and the nation's development, at most reputation² × 0.5, a third at status 2 and a fifth at 3, with draws (in South Korea the part over 250k is halved); the squeeze counter, the totals zeroed, the season tickets and the wage ceiling (only England, Italy and Spain, reputation above 4750, wealth bands 1 and 2)"),
         ('new_finance_table', 'FinanceTable::FinanceTable', '0x59a370', 'fin_prm.cpp (finance.CPP)', 'thiscall, ret 4',
          'monta a tabela das finanças: confere o CD, aloca um registro por clube e o carrega do save ou monta o de cada clube',
          'builds the finance table: checks the CD, allocates one record per club and loads it from the save or sets up each club\'s'),
         ('upgrade_stadium_for_league', 'Finance::upgrade_stadium_for_league', '0x599e00', 'fin_prm.cpp (finance.CPP)', 'thiscall, ret 0x10',
          'leva o estádio do clube aos lugares sentados e à capacidade mínimos de uma competição: os lugares que faltam (e, se a exigência é só de sentados, os de pé convertidos), o custo (6000 por mil lugares mais 125 por lugar, 75 por lugar convertido e 50 por lugar a mais), a aprovação do presidente pelos recursos quando a obra é opcional, a diretoria que às vezes paga, os custos de obra e a notícia dos planos',
          "takes the club's ground to a competition's minimum seats and capacity: the seats missing (and, for an all-seater requirement, the standing places converted), the cost (6000 per thousand seats plus 125 a seat, 75 per converted place and 50 per extra place), the chairman's approval by his resources when the works are optional, the board that sometimes pays, the ground costs and the plans news")])
CAMPOS = []
INTERESSA = []
