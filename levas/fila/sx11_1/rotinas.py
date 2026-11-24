# game.cpp (leitor sx11, rodada 5, 23/11/2026): os dias até a data de início, na criação do jogo.
# - 0x5cc850 game_run_to_start_date: cdecl(), ret 0; quadro SEH só pelas Date temporárias (estados 1 e 2, destrutor
#   trivial). Chamada uma vez pela criação do jogo (0x829fd9, create_game, já em Rust em setup.rs: a rotina roda dentro
#   da pilha particular, sem trocar de pilha). A carreira de teste cria o jogo do zero a cada execução: roda sempre.
# - Convenções conferidas na desmontagem: player_setup_cfg_load 0x7a30c0 cdecl(máscara, 0); set_year 0x5aa2d0
#   thiscall([0xae2a58], ecx) ret 4 (só o bx é lido; a palavra alta de ecx é lixo no original); operator new 0x38 e
#   set_up_the_day 0x6b3c10 thiscall(dia, &data, ecx = fatia com lixo na parte alta) ret 8; play_the_day 0x6b4360
#   thiscall(dia, mesmo nulo); fixture daily 0x5aaf00; Staff_Records 0x86dda0 ret 4; as competições pelo slot 0x14
#   (data, 0) e (data, 1) seguidas do fixture daily = game_update_comps_for_date(0/1), já em Rust (reaproveitada);
#   end_day 0x6b4190 e delete; CONTRACT_MANAGER 0x4c7ac0 thiscall(data, fatia relida, 1) ret 0xc; idade 0x53ab90
#   cdecl(pessoa, &data) -> al, movsx para a palavra +0x18; Weather 0x922ee0; langlib_sprintf(0xde1f64,
#   "Initialising Leagues") cdecl; ScreenManager::send_network_status_message 0x802f80 thiscall(0xde1f64, contador,
#   total, -1) ret 0x10; discipline 0x556240 ret 4; injury 0x62b7b0; UefaSeeding 0x9035a0(1) ret 4; awards 0x413540
#   ret 4; host 0x5fa850 ret 4; 0x5d5ae0 e 0x5d5f20 cdecl; NationalTeams 0x76b100 ret 4; finance 0x59b9c0 ret 4;
#   officials 0x79bf40; MANAGER_MANAGER 0x68f9c0; stadium 0x85b820 ret 4; Club_Records 0x448e30 (toda fatia); Date
#   add_days 0x54c5b0 / sub_days 0x54c8e0 thiscall(data, saída, n) ret 8 e operator= 0x417c10 thiscall(destino,
#   temporária, origem) ret 8; news 0x58a890 cdecl(1 ou 0); pump 0x68d520; fim: Shortlist_Manager 0x83a1b0 thiscall
#   ([[0xde1f54]+0x856]), cfg 0x203b, Injury::init_squad_fitness 0x62a920.
# - Locais: o ano anterior e o contador começam em 0; nenhum local lido sem valor.
# Tipo: simulação (roda na criação do jogo da carreira ENG).
SAVE = False
ROTINAS = [('game_run_to_start_date', 0x5cc850, [0x6a, 0xff, 0x68, 0x0b, 0xa4, 0x95, 0x00])]
DOCS = ('os dias até a data de início',
        'the run-up to the start date',
        [('game_run_to_start_date', 'game_run_to_start_date', '0x5cc850', 'game.cpp', 'cdecl',
          'na criação do jogo, carrega os arquivos de configuração de jogadores (máscaras 0x100, 0x1000, 0x800 e 0x200) e joga o mundo dia a dia, três fatias por dia, da data do jogo até a data de início escolhida: ano novo muda o ano dos calendários; em cada fatia o dia de partidas, calendários, registros, competições e contratos; na terceira, as idades, o tempo, um passo da barra "Initialising Leagues" (total = dias que faltam) e as atualizações diárias do mundo (disciplina, lesões, ranking da UEFA, prêmios, competições, sedes, temporadas, coeficientes, seleções, finanças, árbitros, técnicos, estádios); os recordes de clube; depois da terceira fatia o dia avança e as regras da FIFA entram em vigor se a data delas passou (contrato máximo de 5 anos e a notícia) ou dão o aviso 25 dias antes. No fim a última data processada vira a véspera, a fase do jogo vira 2, os clubes avaliam os elencos, a configuração 0x203b e a forma física dos elencos',
          'on game creation, loads the player setup files (masks 0x100, 0x1000, 0x800 and 0x200) and plays the world day by day, three slices a day, from the game date to the chosen start date: a new year sets the fixtures year; each slice the match day, fixtures, records, competitions and contracts; on the third, the ages, the weather, an "Initialising Leagues" progress step (total = days to go) and the world daily updates (discipline, injuries, UEFA seeding, awards, competitions, hosts, seasons, coefficients, national teams, finances, officials, managers, stadiums); the club records; after the third slice the day moves on and the FIFA rules come into force when their date has passed (5-year maximum contract and the news) or give the warning 25 days before. At the end the last processed date becomes the day before, the game phase becomes 2, the clubs assess their squads, setup file 0x203b and the squads fitness')])
CAMPOS = []
INTERESSA = []
