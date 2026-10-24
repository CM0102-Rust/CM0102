# Rodam na criação do jogo (carga da base): set_key_ids (0x601a40, chamada em 0x52d8fa) passa cada
# competição de clube e cada competição/prêmio de pessoa da base por elas. A gravação do .qst da prova do
# save cria o jogo lendo o banco (e a carreira automática também cria o seu).
# Só 2 rotinas: no index.cpp não há terceira rotina chamada na carreira que seja segura (set_key_ids,
# 9,5 KB com 0x220 de pilha e as caixas de erro, fica para depois; get_region_article_form começa com
# call, sem prólogo para o desvio).
SAVE = True
ROTINAS = [('casar_chave_de_competicao_de_clube', 0x60efd0, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('casar_chave_de_competicao_de_pessoa', 0x6115e0, [0x56, 0x8b, 0x74, 0x24, 0x08])]
DOCS = ('as chaves fixas das competições de clube e das competições e prêmios de pessoa pelo nome',
        'the fixed club competition and staff competition/award keys by name',
        [('casar_chave_de_competicao_de_clube', 'match_club_comp_key_id', '0x60efd0', 'index.cpp', 'cdecl',
          'casa o nome longo da competição de clube com as 301 chaves fixas (351 nomes em 3 cadeias, _stricmp; as duas divisões croatas gravam também 0x9cf8a4)',
          'matches the club competition long name with the 301 fixed keys (351 names in 3 chains, _stricmp; the two Croatian divisions also store 0x9cf8a4)'),
         ('casar_chave_de_competicao_de_pessoa', 'match_staff_comp_key_id', '0x6115e0', 'index.cpp', 'cdecl',
          'casa o nome da competição ou prêmio de pessoa com as 417 chaves fixas (442 nomes em 5 cadeias, _stricmp)',
          'matches the staff competition or award name with the 417 fixed keys (442 names in 5 chains, _stricmp)')])
CAMPOS = [('[0x9cf548..0x9cfa04]: as 301 chaves de competição de clube (nomes reais em CHAVES_DE_COMPETICAO_DE_CLUBE do codigo.rs: French Lower Division, German First Division, World Club Cup, UEFA Champions League, turkish premier division...; croatian first division e croatian a1 division gravam 0x9cf87c e 0x9cf8a4); id (linha da tabela de competições de clube); grava (casar_chave_de_competicao_de_clube)',
           '[0x9cf548..0x9cfa04]: the 301 club competition keys (real names in CHAVES_DE_COMPETICAO_DE_CLUBE of codigo.rs: French Lower Division, German First Division, World Club Cup, UEFA Champions League, turkish premier division...; croatian first division and croatian a1 division store 0x9cf87c and 0x9cf8a4); id (club competition table row); writes (casar_chave_de_competicao_de_clube)'),
          ('[0x9cfa20..0x9d00a0]: as 417 chaves de competição e prêmio de pessoa (nomes em CHAVES_DE_COMPETICAO_DE_PESSOA: FIFA World Player of the Year, European Footballer of the Year, Italian Serie A Team of the Year, Brazilian 1st Division Top Goalscorer...); id (linha da tabela de competições de pessoa); grava (casar_chave_de_competicao_de_pessoa)',
           '[0x9cfa20..0x9d00a0]: the 417 staff competition and award keys (names in CHAVES_DE_COMPETICAO_DE_PESSOA: FIFA World Player of the Year, European Footballer of the Year, Italian Serie A Team of the Year, Brazilian 1st Division Top Goalscorer...); id (staff competition table row); writes (casar_chave_de_competicao_de_pessoa)'),
          ('competição +nome longo / competição de pessoa +nome: o nome que a base dá (argumento); texto; lê (por _stricmp 0x951c50, sem distinguir maiúsculas)',
           'competition +long name / staff competition +name: the name the database gives (argument); text; reads (through _stricmp 0x951c50, case-insensitive)')]
INTERESSA = ['CMRTE', 'SGE']
