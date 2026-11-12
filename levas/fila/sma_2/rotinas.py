# A nomeação de um técnico num clube (MANAGER_MANAGER::appoint_manager_to_club 0x69ebc0, manager_manager.cpp; leitor
# sma, 12/11/2026). Chamada pelas notícias (0x784100, a resposta a uma oferta de emprego), pela tela do técnico
# (0x6b2080) e pela criação do jogo (0x826f20), todas originais; na carreira de teste roda 1 vez (cobertura até
# 2002-06-01): o humano assumindo o clube na criação do jogo. Convenções conferidas:
# - a rotina: thiscall(this, clube, técnico), ret 8;
# - 0x53b900 (seleção): push clube -> cdecl;
# - 0x69ba50 (add_job_vacancy): push pessoa; push clube velho; ecx = this -> thiscall, ret 8;
# - 0x4d0b50 (remove_contract): ecx = [0xae1050]; push pessoa -> thiscall, ret 4;
# - 0x549f70 (Date 23/7/1971): push -1; push 0x7b3; push 7; push 0x17; lea ecx, local -> thiscall, ret 0x10;
# - 0x417c10 (Date::operator=): push origem; push local; ecx = destino -> thiscall, ret 8 (a linha da diretoria +0x20
#   e a data de entrada +0x29 / +0x3e);
# - 0x690890 / 0x690730 (tirar da comissão / do elenco): push clube velho; push pessoa; ecx = this -> thiscall, ret 8;
# - 0x6a2e00 (a notícia da nomeação, Rust): push código; push clube; push clube velho (ou 0); push pessoa; ecx = this
#   -> thiscall, ret 0x10;
# - 0x7874b0 (respostas automáticas): ecx = [0xb7431c]; push clube velho; push pessoa -> thiscall, ret 8;
# - 0x7f7270: ecx = [0xb74448]; push pessoa -> thiscall, ret 4; 0x8b2470: ecx = [0xde1f44]; push clube; push velho;
#   push pessoa -> thiscall, ret 0xc;
# - 0x694c80 (reset_board_for_new_manager, Rust): push clube; ecx = this -> thiscall, ret 4;
# - 0x4cf5a0 (get_contract): ecx = [0xae1050]; push pessoa -> thiscall, ret 4; 0x8674e0 (clear_grievances, original):
#   push bit; ecx = contrato -> thiscall, ret 4;
# - 0x5e8290 e companhia: a caixa de erro do jogo (show_error_at, as linhas do original).
# Comparador: só a original roda (a mudança não pode rodar duas vezes) e confere o que ela garante: o clube com o
# técnico e o cargo dele gravado (5, ou 12 com a vaga no elenco).
SAVE = False
ROTINAS = [('appoint_manager_to_club', 0x69ebc0, [0x81, 0xec, 0x14, 0x02, 0x00, 0x00])]
DOCS = ('a nomeação de um técnico num clube',
        'appointing a manager to a club',
        [('appoint_manager_to_club', 'MANAGER_MANAGER::appoint_manager_to_club', '0x69ebc0', 'manager_manager.cpp', 'thiscall, ret 8',
          'leva o técnico ao clube: o emprego antigo (o clube, ou a seleção que ele dirigia) ganha vaga se a liga do país estiver selecionada (ou, numa seleção, com reputação acima de 3500 e sem continente); o técnico-jogador e o papel 15 perdem o contrato de jogador ao ir para um clube; a data +0x20 da linha da diretoria vira 23/7/1971; ele sai do emprego antigo conforme o papel (do elenco, da comissão ou dos dois) e a notícia da nomeação sai com o código 1 (jogador), 2 (comissão) ou 3 (técnico); com um humano, as respostas automáticas e as marcas de liga do humano na nação; a data de entrada vira hoje, o clube ganha o técnico e ele o cargo 5, ou 12 (técnico-jogador, na primeira vaga livre do elenco) quando a reputação de jogador + 750 passa a do clube; o humano refaz as listas de observação e os treinos; a diretoria recomeça e cada jogador do elenco perde as queixas 8, 0x200, 0x400, 0x2000, 0x10000, 0x80000 e 0x800000 do contrato',
          'takes the manager to the club: the old job (the club, or the national team he managed) gets a vacancy when the country\'s league is selected (or, at a national team, with reputation above 3500 and no continent); the player-manager and role 15 lose the playing contract when going to a club; date +0x20 of the board row becomes 23/7/1971; he leaves the old job by role (the squad, the staff or both) and the appointment news goes out with code 1 (player), 2 (staff) or 3 (manager); with a human, the automatic replies and the human\'s league marks on the nation; the joining date becomes today, the club gets the manager and he gets job 5, or 12 (player-manager, in the first free squad slot) when his playing reputation + 750 is above the club\'s; the human redoes the scout lists and the training; the board restarts and each squad player loses the grievances 8, 0x200, 0x400, 0x2000, 0x10000, 0x80000 and 0x800000 of the contract')])
CAMPOS = [('linha da diretoria +0x20: data (23/7/1971 ao nomear); valor; escreve', 'board row +0x20: date (23/7/1971 on appointment); value; writes'),
          ('contrato +0x45: marcas de queixas (8, 0x200, 0x400, 0x2000, 0x10000, 0x80000, 0x800000 limpas pelo técnico novo); valor; lê', 'contract +0x45: grievance flags (8, 0x200, 0x400, 0x2000, 0x10000, 0x80000, 0x800000 cleared by the new manager); value; reads')]
