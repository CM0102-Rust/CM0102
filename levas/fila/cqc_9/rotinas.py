# As expectativas da diretoria para a temporada, a vaga de tecnico e a noticia da oferta de emprego
# (manager_manager.cpp).
# - 0x6a3770 (MANAGER_MANAGER::set_season_expectations, 689 chamadas ate 2002-06-01 na cobertura de 17/10: o comeco da
#   temporada, o tecnico novo 0x694c80, as selecoes 0x775000..0x775220, a rotina diaria 0x68f9c0): a meta na liga, as
#   marcas das expectativas, os sorteios do presidente, as confiancas de volta a 6500 e a ficha do tecnico. As mutacoes
#   estao nela.
# - 0x69ba50 (MANAGER_MANAGER::add_job_vacancy, 34 chamadas): o escolhido para a vaga (a maior nota da 0x69d1b0 entre
#   todas as pessoas), as datas e o item na lista das vagas.
# - 0x6a0540 (MANAGER_MANAGER::post_job_offer_news; sem cobertura medida, pela vaga): passageira (a noticia 0x1775).
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   0x6a3770: thiscall (mov ecx, edi/esi/[0xb67a44] nos nove pontos), 2 na pilha (ret 8; o 2o so o byte). Dentro:
#   0x53b900, 0x5409b0 cdecl (add esp, 4), 1; 0x54cdb0 thiscall (mov ecx, 0xae2c90), 2 (a data por valor); 0x90cfc0 cdecl,
#   1; 0x68d0d0 thiscall (mov ecx, [this+4]), 2 (item, 6); 0x7751e0 thiscall (mov ecx, [0xb74318]), 1; o virtual +0x70
#   da competicao thiscall, 1 (clube); 0x6a44a0 thiscall (mov ecx, edi), 3 (clube, palavra das marcas, competicao);
#   0x905c60 thiscall (mov ecx, [0xde1f58]), 3 (nacao, &vagas no lugar do argumento, &outras); 0x4a4850 thiscall (mov
#   ecx, [comp + 8]), 1; 0x5a0590 thiscall (mov ecx, [0xae2a38]), 1; 0x5986b0 thiscall (mov ecx, financas), 0, le al;
#   0x598800 thiscall (mov ecx, financas), 5 (0, 0, 0, 1, 0), devolve edx:eax; os virtuais +0x98 e +0x9c thiscall, 0;
#   0x6a4a20 thiscall (mov ecx, edi), 2 (clube, palavra das marcas); 0x5a5930 thiscall (mov ecx, [0xae2a3c]), 1;
#   0x945640 _ftol.
#   0x69ba50: thiscall (mov ecx, [0xb67a44] ou esi/edi nos chamadores), 2 (ret 8). Dentro: 0x54a110 thiscall (lea ecx,
#   a data), 0; 0x53b900, 0x5409b0, 0x544820, 0x53ca90 cdecl, 1; 0x4cf6d0 e 0x4cf5a0 thiscall (mov ecx, [0xae1050]), 1;
#   0x69fc60 thiscall (mov ecx, edi = this), 1; 0x69f810 thiscall (mov ecx, edi), 4 (pessoa, clube e o double 3,5 ou
#   3,15 em dois dwords); 0x69d1b0 thiscall (mov ecx, edi), 4 (pessoa, clube, 0, 5); 0x90cfc0 cdecl, 1; 0x54cdb0
#   thiscall (mov ecx, 0xae2c90), 2; 0x547e70 cdecl (add esp, 0xc), 3; 0x600a50 cdecl (add esp, 0x14), 5; 0x6a0540
#   thiscall (mov ecx, edi), 2 (pessoa, clube); 0x944c9f cdecl (add esp, 0xc), 3; 0x68d0d0 thiscall (mov ecx, [this+8]),
#   2 (item, 0x26); 0x5fd210 thiscall (mov ecx, [0xb63c98]), 1; 0x5fea50 thiscall (mov ecx, [0xb63c98]), 2 (1, pessoa);
#   0x69ca00 thiscall (mov ecx, edi), 2 (clube, pessoa).
#   0x6a0540: thiscall (mov ecx, edi em 0x69c3af), 2 (ret 8). Dentro: 0x77bbe0 thiscall, 2; 0x785790 thiscall, 2;
#   0x7861e0 thiscall (mov ecx, [0xb7431c]), 2 (noticia, clube da pessoa).
# Comparadores: os tres so a original (a 0x6a3770 acrescenta na lista das saidas e posta noticias; a 0x69ba50 acrescenta a
# vaga, sorteia e chama as candidaturas; a 0x6a0540 entrega a noticia); as copias levam os contadores e se provam pela
# carreira e pelas mutacoes. Iguais no cm0102-1080.exe (conferido byte a byte).
SAVE = False
ROTINAS = [('expectativas_da_temporada', 0x6a3770, [0x83, 0xec, 0x10, 0x53, 0x55, 0x56]),
           ('vaga_de_tecnico', 0x69ba50, [0x6a, 0xff, 0x68, 0xa0, 0xcc, 0x95, 0x00]),
           ('noticia_da_oferta_de_emprego', 0x6a0540, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as expectativas da diretoria para a temporada, a vaga de técnico e a notícia da oferta de emprego',
        'the board season expectations, the manager vacancy and the job offer news',
        [('expectativas_da_temporada', 'MANAGER_MANAGER::set_season_expectations', '0x6a3770', 'manager_manager.cpp', 'thiscall',
          'a meta da diretoria na liga (a posição de perigo, a riqueza, a divisão anterior, o tempo do técnico) e as marcas das expectativas (título, vagas europeias, metade de cima, fuga do rebaixamento), os sorteios do presidente, a saída pelo tempo no cargo, as confianças que voltam a 6500 e a ficha do técnico; a seleção pega as da competição dela',
          'the board target in the league (danger position, wealth, previous division, manager tenure) and the expectation flags (title, European places, top half, avoid relegation), the chairman draws, the departure by tenure, the confidence values pulled back to 6500 and the manager record; a national team takes those of its competition'),
         ('vaga_de_tecnico', 'MANAGER_MANAGER::add_job_vacancy', '0x69ba50', 'manager_manager.cpp', 'thiscall',
          'recomeça a diretoria do clube e escolhe o técnico da vaga: a maior nota entre as pessoas livres que servem (ou o auxiliar, ou a primeira livre de trás para a frente), às vezes um sorteado melhor; as datas do anúncio e da chegada e o item na lista das vagas; os humanos se candidatam',
          'resets the club board and picks the manager for the vacancy: the best score among the free people who qualify (or the assistant, or the first free one counting backwards), sometimes a better drawn one; the announcement and arrival dates and the item in the vacancy list; the human managers apply'),
         ('noticia_da_oferta_de_emprego', 'MANAGER_MANAGER::post_job_offer_news', '0x6a0540', 'manager_manager.cpp', 'thiscall',
          'a notícia 0x1775 ao técnico do clube de que outro clube ofereceu o cargo de técnico a alguém da comissão dele',
          'news item 0x1775 telling the club manager that another club offered one of his staff the manager job')])
CAMPOS = [('diretoria (0x49 bytes em [this] + id × 0x49) +0, +1 riqueza, +2/+3/+4/+5 marcas, +6/+8/+0xa/+0xc confianças, +0xe expectativas, +0x10..+0x16, +0x28, +0x2c meta, +0x30/+0x38 doubles, +0x40; valor; lê e grava (expectativas_da_temporada, vaga_de_tecnico)',
           'board (0x49 bytes at [this] + id × 0x49) +0, +1 wealth, +2/+3/+4/+5 flags, +6/+8/+0xa/+0xc confidence, +0xe expectations, +0x10..+0x16, +0x28, +0x2c target, +0x30/+0x38 doubles, +0x40; value; reads and writes (expectativas_da_temporada, vaga_de_tecnico)'),
          ('[this+4] lista das saídas (item de 6 bytes {1, 6, id}); [this+8] lista das vagas (item de 0x26 bytes, cursor +0xa guardado); ponteiro; grava',
           '[this+4] departure list (6-byte item {1, 6, id}); [this+8] vacancy list (0x26-byte item, cursor +0xa kept); pointer; writes'),
          ('clube +0x53 nação (+0x75 continente, +0x88 ranking, +0x11c), +0x57/+0x5b divisão e anterior (+0x69 força), +0x80 reputação, +0xbf presidente (+0x69 ficha +0x1d), +0xcf técnico, +0xd3 auxiliar, +0x19f, +0x1db liga; ponteiro/valor; lê',
           'club +0x53 nation (+0x75 continent, +0x88 ranking, +0x11c), +0x57/+0x5b division and previous (+0x69 strength), +0x80 reputation, +0xbf chairman (+0x69 record +0x1d), +0xcf manager, +0xd3 assistant, +0x19f, +0x1db league; pointer/value; reads'),
          ('ficha de técnico (+0x69 da pessoa) +0xa/+0xc reputações; valor; lê e grava (expectativas_da_temporada); pessoa +0x59; valor; grava (+1 no técnico humano novo)',
           'non-player record (person +0x69) +0xa/+0xc reputations; value; reads and writes (expectativas_da_temporada); person +0x59; value; writes (+1 for a new human manager)'),
          ('pessoa +0x1a nação, +0x24/+0x28, +0x39 clube, +0x3d cargo, +0x3e data, +0x5f humano, +0x61 ficha; [0xae2d0c] estado (0x4f bytes, +0xb & 0x30); valor/ponteiro; lê',
           'person +0x1a nation, +0x24/+0x28, +0x39 club, +0x3d job, +0x3e date, +0x5f human, +0x61 record; [0xae2d0c] state (0x4f bytes, +0xb & 0x30); value/pointer; reads'),
          ('competição +0x3e clubes, +0xbe/+0xbf/+0xc1 vagas; linha da tabela da liga +0x72..+0x75; [0xae23d0] liga do clube (0x6b bytes); valor; lê',
           'competition +0x3e clubs, +0xbe/+0xbf/+0xc1 places; league table row +0x72..+0x75; [0xae23d0] club league (0x6b bytes); value; reads')]
INTERESSA = ['CMRTE']
