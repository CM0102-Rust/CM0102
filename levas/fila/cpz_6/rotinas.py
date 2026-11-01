# O jogador a serviço da seleção (national_teams.cpp): NationalTeams::get_international_duty_club (0x7757d0: a seleção
# ou o time das estrelas que tem o jogador na data, de véspera ao dia seguinte; da escalação do computador 0x6dbb80 e
# 0x6dbc50, que deixa de fora quem está a serviço, dos olheiros, das transferências, do contrato e das telas),
# national_teams_player_is_eligible (0x76d9b0: o jogador serve à seleção do técnico; da busca de jogadores 0x7afa70 e
# 0x807450 e da tela de comparação) e NationalTeams::collect_duty_competitions (0x775580: as competições das convocações
# numa lista; dos olheiros 0x7f7010). Iguais no cm0102-1080.exe; sem x87. A 0x7757d0 tem quadro SEH pela lista e pelas
# datas locais: a cópia chama o construtor e o destrutor da lista (0x68d020/0x68d040) nos mesmos caminhos. O resto de
# national_teams.cpp são notícias (só para clubes humanos), a carga/gravação, o construtor, a escolha da convocação e o
# passe do dia (grandes, com muitas chamadas não portadas).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x68d020 / 0x68d040 (a lista provisória): lea ecx, [lista]; call; sem push -> thiscall(lista), ret 0.
# - 0x53b900 (é seleção): push clube; call; add esp, 4 -> cdecl(clube).
# - 0x5409b0 (segundo conjunto de seleções): push clube; call; add esp, 4 -> cdecl(clube).
# - 0x54c8e0 / 0x54c5b0 (data menos / mais n dias): push 1; push ret; mov ecx, data; call; sem add esp -> thiscall(data,
#   ret, 1), ret 8.
# - 0x5acbe0 (os jogos do calendário): mov ecx, [0xae2a58]; 9 pushes (dia0, ano0, dia1, ano1, clube, lista, -1, -2,
#   lado); sem add esp -> thiscall(this, 9 args), ret 0x24.
# - 0x68d1b0 (próximo da lista): mov ecx, lista; call; sem push -> thiscall(lista), ret 0.
# - 0x68d0d0 (acrescentar à lista): push 4; push item; mov ecx, saida; call; sem add esp -> thiscall(saida, item, 4), ret 8.
# - 0x66f4e0 (texto traduzido): push 0xad9c64; push formato; push destino; call; add esp, 0xc -> cdecl(destino, formato,
#   ...).
# - 0x549ef0 (Date::Date(dia, ano)): push ano; push dia; lea ecx, [data]; call; sem add esp -> thiscall(data, dia, ano),
#   ret 8.
# - quem chama a 0x7757d0: 0x6dbc0c mov ecx, [0xb74318]; push 0; push 0; push clube; push 0xae2c90; push jogador ->
#   thiscall(this, jogador, data, clube, comentario, adversario), ret 0x14; a 0x76d9b0: 0x7afcd8 push tecnico; mov ecx,
#   [0xb74318]; push jogador; sem add esp: o ecx vai, mas a rotina não o lê -> stdcall(jogador, tecnico) na pilha (como no
#   mapa), ret 8; a 0x775580: 0x7f70d3 mov ecx, [0xb74318]; push saida -> thiscall(this, saida), ret 4.
# Comparadores: a 0x7757d0 lado a lado (cada uma com a sua lista provisória, como a 0x5c6b70 da 254ª) no texto de
# 0xde1f64 e no do comentário e na devolução; a 0x76d9b0 pela devolução; a 0x775580 acrescenta nós (aloca): só a
# original, com o plano completo feito antes pelos nós (a devolução e os itens novos no fim de `saida`).
SAVE = False
ROTINAS = [('selecao_que_tem_o_jogador', 0x7757d0, [0x6a, 0xff, 0x68, 0xf0, 0xde, 0x95, 0x00]),
           ('jogador_serve_a_selecao', 0x76d9b0, [0x8b, 0x44, 0x24, 0x08, 0x56]),
           ('juntar_competicoes_das_convocacoes', 0x775580, [0x53, 0x55, 0x8b, 0xe9, 0x32, 0xdb])]
DOCS = ('o jogador a serviço da seleção: a seleção ou o time das estrelas que o tem na data, o jogador que serve à seleção do técnico e as competições das convocações',
        "the player on international duty: the national side or all-star squad holding him on the date, the player eligible for the manager's national side and the duty competitions",
        [('selecao_que_tem_o_jogador', 'NationalTeams::get_international_duty_club', '0x7757d0', 'national_teams.cpp', 'thiscall, ret 0x14',
          'a seleção que tem o jogador da véspera ao dia seguinte (jogo da seleção dele na janela, contra o adversário se dado, ou convocação já começada) ou o time das estrelas da MLS na data do jogo; põe o comentário; senão 0 (e 0 para clube seleção ou time das estrelas)',
          'the national side holding the player from the day before to the day after (a national side fixture in the window, against the given opponent if any, or a call-up already started) or the MLS all-star squad on the game date; writes the comment; otherwise 0 (and 0 for a national side or all-star club)'),
         ('jogador_serve_a_selecao', 'national_teams_player_is_eligible', '0x76d9b0', 'national_teams.cpp', 'stdcall, ret 8',
          'o jogador pode jogar pela seleção do técnico: nacionalidade, idade para a sub-21, convocação atual e a regra da segunda nacionalidade',
          "the player may play for the manager's national side: nationality, under-21 age, current call-up and the second-nationality rule"),
         ('juntar_competicoes_das_convocacoes', 'NationalTeams::collect_duty_competitions', '0x775580', 'national_teams.cpp', 'thiscall, ret 4',
          'acrescenta a uma lista cada competição das convocações que ela ainda não tem e devolve quantas',
          'adds to a list each duty competition it does not yet hold and returns how many')])
CAMPOS = [('staff +0x24: a seleção que o convocou; ponteiro; lê (selecao_que_tem_o_jogador, jogador_serve_a_selecao)',
           'staff +0x24: the national side that called him up; pointer; reads (selecao_que_tem_o_jogador, jogador_serve_a_selecao)'),
          ('staff +0x5e bit 4: o lado nos jogos da seleção e se as convocações contam; valor; lê (selecao_que_tem_o_jogador)',
           'staff +0x5e bit 4: the side in national fixtures and whether call-ups count; value; reads (selecao_que_tem_o_jogador)'),
          ('staff +0x39: o clube; ponteiro; lê (selecao_que_tem_o_jogador)', 'staff +0x39: the club; pointer; reads (selecao_que_tem_o_jogador)'),
          ('staff +0x1a / +0x1e / +0x22 / +0x18 / +0x61: nacionalidades, os jogos pela seleção (com jogos, só a primeira nacionalidade vale), a idade e a ficha de jogador; ponteiro e valor; lê (jogador_serve_a_selecao)',
           'staff +0x1a / +0x1e / +0x22 / +0x18 / +0x61: nationalities, the international caps (with caps, only the first nationality counts), the age and the player record; pointer and value; reads (jogador_serve_a_selecao)'),
          ('[0xae2d0c] registro de 79 bytes da pessoa +0xc bit 2: só pela primeira nacionalidade; valor; lê (jogador_serve_a_selecao)',
           "[0xae2d0c] person's 79-byte record +0xc bit 2: first nationality only; value; reads (jogador_serve_a_selecao)"),
          ('clube +0x57: a liga; ponteiro; lê (selecao_que_tem_o_jogador); clube +0xd7: os 50 nomes do time das estrelas; ponteiro; lê',
           'club +0x57: the league; pointer; reads (selecao_que_tem_o_jogador); club +0xd7: the 50 all-star squad names; pointer; reads'),
          ('NationalTeams +0: a lista das convocações (+0 a seleção, +4 o adversário/competição, +8 a data); ponteiro; lê (selecao_que_tem_o_jogador, juntar_competicoes_das_convocacoes)',
           'NationalTeams +0: the duty list (+0 the national side, +4 the opponent/competition, +8 the date); pointer; reads (selecao_que_tem_o_jogador, juntar_competicoes_das_convocacoes)'),
          ('[0x9d00a4] / [0x9d00a8] os dois times das estrelas; id; lê. [0x9cf590] a liga da MLS; id; lê. [0x9cf434] a nação da regra da segunda nacionalidade; id; lê',
           '[0x9d00a4] / [0x9d00a8] the two all-star squads; id; reads. [0x9cf590] the MLS league; id; reads. [0x9cf434] the nation of the second-nationality rule; id; reads')]
INTERESSA = ['CMRTE']
