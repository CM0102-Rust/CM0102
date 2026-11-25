# A classificacao da UEFA (uefa_seeding.cpp; leitor sbI, rodada 5, 24/11/2026): o construtor, a ordem do euro.cfg e a
# conferencia do euro.cfg. Simulacao, carreira ENG (sem LIGAS).
# - 0x903360 UefaSeeding::UefaSeeding thiscall() ret 0 -> this: na criacao do jogo (setup, passo 8, new 0x698) e na
#   carga (passo 0xc, so this: le o UefaSeeding.tmp pela 0x905230, ja em Rust). O CD pelo find_game_cd (0x9033a2 e
#   0x9033e4: o patch sem CD do cm0102-1080 e do portatil troca esses 3 bytes; a rotina e desviada nos dois exes pelos
#   dois hashes). Na criacao monta a tabela das nacoes europeias (+0 id, +4 nacao, +8..+0x11 zerados).
# - 0x9059e0 UefaSeeding::apply_euro_cfg_order thiscall(tabela, conta) ret 8: so no collect_european_clubs 0x904040
#   (Rust; push conta; push tabela; mov ecx, ebx) com o ano da classificacao 2001, uma vez na carreira (a virada do ano
#   do jogo no comeco). fopen 0x945b00 de utils_build_file_path 0x90bde0 ("CM3_DATA", "euro.cfg") "r";
#   utils_read_config_line 0x90cba0 cdecl(arquivo, 0xde1f64); nation_get_english_name 0x5453f0 e club_get_english_name
#   0x545880 cdecl; strstr 0x945a60; bsearch 0x946464 (chave, tabela, conta short, 0xc, 0x905ce0); fclose 0x945a09.
# - 0x905d40 validate_euro_cfg cdecl(): so com "-verifyeurocfg" na linha de comando, na criacao do jogo (setup.rs,
#   FN_VALIDATE_EURO_CFG). Nao roda na carreira de teste: sem mutacao. A caixa: sprintf 0x944c9f no 0xde1f64 e
#   gui_show_message_box 0x5e8290 cdecl("Error" 0x9870e0, texto, 0).
# Prologos: sub esp, 0x474 (6); sub esp, 0x28; movsx eax, word [esp+0x30] (8); sub esp, 0xc8 (6). Sem relativas.
# 0x9059e0 e 0x905d40 iguais no cm0102-1080.exe; 0x903360 difere so nos 3 bytes do patch sem CD.
# Defeitos do original: 0x903360 sem limite de nacoes europeias (60 cabem; a conta e um byte com sinal); 0x9059e0 e
# 0x905d40 giram para sempre numa linha que nao e cabecalho onde se espera um (a primeira, ou a seguinte a uma nacao
# nao achada); 0x9059e0 nunca olha a ultima entrada da tabela; 0x905d40 nunca mostra o "club not found" (testa o
# ponteiro errado). A busca por nome e por trecho (strstr): a primeira nacao ou clube cujo nome esta na linha.
# Comparadores: o construtor so a original, com o plano da tabela nova (as nacoes europeias, finais vazias); a ordem lado
# a lado na tabela; a conferencia so a original, com a ultima caixa pelo 0xde1f64 contra a copia sem caixas.
# Usa o find_game_cd (sx02b_1, ja no HEAD desde a leva 330).
SAVE = False
ROTINAS = [('new_uefa_seeding', 0x903360, [0x81, 0xec, 0x74, 0x04, 0x00, 0x00]),
           ('apply_euro_cfg_order', 0x9059e0, [0x83, 0xec, 0x28, 0x0f, 0xbf, 0x44, 0x24, 0x30]),
           ('validate_euro_cfg', 0x905d40, [0x81, 0xec, 0xc8, 0x00, 0x00, 0x00])]
DOCS = ('a classificação da UEFA: o construtor, a ordem do euro.cfg e a conferência do euro.cfg',
        'the UEFA seeding: the constructor, the euro.cfg order and the euro.cfg check',
        [('new_uefa_seeding', 'UefaSeeding::UefaSeeding', '0x903360', 'uefa_seeding.cpp', 'thiscall, ret 0',
          'monta a classificação da UEFA: confere o CD, esvazia os voluntários da Intertoto e as 50 finais de copa nacional; carrega o arquivo temporário ou, no jogo novo, põe cada nação europeia da tabela de nações, na ordem, com o coeficiente, as vagas e os pontos zerados',
          'builds the UEFA seeding: checks the CD, empties the Intertoto volunteers and the 50 national cup finals; loads the temporary file or, in a new game, puts each European nation of the nation table, in order, with the coefficient, the places and the points zeroed'),
         ('apply_euro_cfg_order', 'UefaSeeding::apply_euro_cfg_order', '0x9059e0', 'uefa_seeding.cpp', 'thiscall, ret 8',
          'regra: no euro.cfg, cada "*nação" é seguida pelos clubes dela na ordem das vagas europeias reais; cada clube listado vai para a frente do bloco da nação na tabela dos clubes europeus, um depois do outro, trocando de lugar com o que estava lá; a nação e o clube são os primeiros cujo nome aparece na linha',
          'rule: in euro.cfg each "*nation" is followed by its clubs in the order of the real European places; each listed club goes to the front of the nation\'s block in the European clubs table, one after the other, swapping places with the one that was there; the nation and the club are the first whose name appears in the line'),
         ('validate_euro_cfg', 'validate_euro_cfg', '0x905d40', 'uefa_seeding.cpp', 'cdecl',
          'com "-verifyeurocfg" na linha de comando: confere o euro.cfg contra o banco de dados e mostra a caixa de erro de cada nação não achada (a do clube não achado nunca aparece, por defeito)',
          'with "-verifyeurocfg" on the command line: checks euro.cfg against the database and shows the error box of each nation not found (the one for a club not found never shows, by a defect)')])
CAMPOS = []
INTERESSA = []
