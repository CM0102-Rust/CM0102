# O gerente de listas curtas: montar, as listas iniciais e refazer a lista de um clube (shortlist_manager.cpp; leitor
# sx20, rodada 5, 24/11/2026): Shortlist_Manager::Shortlist_Manager (0x839a80), create_initial_shortlists (0x83a1b0)
# e rebuild_club_shortlist (0x84abe0). Rotinas inteiras (0x839a80..0x839d17 ret 4 mais a saida fatal ate 0x839d84;
# 0x83a1b0..0x83a72e ret; 0x84abe0..0x84aea2 ret 4).
# Quem chama: o construtor, o TRANSFER_MANAGER (0x8bbd52 jogo novo, 0x8bc7c5 carga com o save); as listas iniciais, a
# criacao do jogo (0x5ccc91); refazer, 0x695147 (manager_manager) e 0x762513. Na cobertura ENG as tres rodam 1 vez.
# Convencoes conferidas na desmontagem:
# - 0x839a80 thiscall(this, save) ret 4 -> this; 0x944e46 malloc(clubes * 9) e malloc(0x5c8) por clube, sem memoria a
#   caixa (shortlist_manager.cpp 0x54 / 0x6f), 0x5cc7d0, 0x5a8f60 e exit(-1); 0x549ef0 Date::Date thiscall(data, 1,
#   0x7cd) ret 8 (os dois push vem antes da vaga, no comeco do laco) e 0x417c10 Date::operator= thiscall(vaga+0x1b,
#   rascunho, data) ret 8; com save 0x83a9d0 load thiscall(this, save) ret 4; sem save 0x839e60 build_position_lists
#   thiscall(this, 1) ret 4;
# - 0x83a1b0 thiscall(this): 0x66f4e0 langlib_sprintf no mesmo buffer de 0x64 bytes ("Assessing Club Squads", depois
#   "Creating Shortlists"); 0x802f80 send_network_status_message thiscall(0xb74450, buf, i, clubes, -1) ret 0x10 a
#   cada 100 clubes e no fim (buf, clubes, clubes, -1); 0x83cb40 assess_club_squad thiscall(this, entrada, clube, 0, 1)
#   ret 0x10 na ordem de [0xae2d1c]; 0x540a50 club_get_linked_b_team cdecl(clube, &saida, 1); 0x4cf5f0
#   get_contract_for_club thiscall([0xae1050], pessoa, clube) ret 8 (chamado tambem sem clube); 0x862720
#   calc_unset_default_wage thiscall(contrato, pessoa, clube, 1, +0x4f >> 4, 0xb, 0) ret 0x18 -> st0, _ftol (0x945640);
#   as comparacoes x87 "valor > limite" (fcom; test ah, 0x41) e (v - limite) * fator + limite por _ftol; 0x600a50
#   human_manager_is_human_club cdecl(clube, 0, 1, 0, 0); 0x4d0c60 calc_player_value thiscall([0xae1050], pessoa,
#   clube, 0, -1, humano) ret 0x14 -> +0x52; 0x83a780 sort_position_lists thiscall(this); 0x83b6e0
#   allocate_transfer_budget thiscall(this, clube) ret 4;
# - 0x84abe0 thiscall(this, clube) ret 4, quadro SEH so pelo LLIST local (0x68d020 / 0x68d040); 0x53b900
#   database_is_national_team cdecl(clube); 0x844660 release_club_slot_targets thiscall(this, clube, entrada) ret 8;
#   0x8420e0 collect_club_shortlist_entries thiscall(this, clube, lista) ret 8; o cursor da lista volta a cabeca
#   (+0xa = +2) e 0x68d1b0 get_next; 0x946464 bsearch(par, [this+0]/[this+4], [this+0x10]/[this+0x14], 8, 0x757ea0 /
#   0x844740) e a remocao em linha (o remove_sorted_pair); 0x53ca90 database_get_club_manager cdecl(clube); 0x5fd210
#   is_on_holiday thiscall([0xb63c98], tecnico) ret 4; 0x5fea50 get_holiday_option thiscall([0xb63c98], 0, tecnico)
#   ret 8 -> al; 0x4cf5a0 get_contract thiscall([0xae1050], pessoa) ret 4.
# Defeitos do original: os bits 6-7 das bandeiras de cada vaga nunca sao postos (fica o que o malloc deixou); o status
# do elenco vai num registrador com os bytes altos de sobra (lido como byte).
ROTINAS = [('shortlist_manager_shortlist_manager', 0x839a80, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('shortlist_manager_create_initial_shortlists', 0x83a1b0, [0x83, 0xec, 0x74, 0x8d, 0x44, 0x24, 0x10]),
           ('shortlist_manager_rebuild_club_shortlist', 0x84abe0, [0x6a, 0xff, 0x68, 0xc8, 0x25, 0x96, 0x00])]
DOCS = ('montar o gerente de listas curtas, as listas iniciais e refazer a lista de um clube',
        "building the shortlist manager, the initial shortlists and rebuilding one club's shortlist",
        [('shortlist_manager_shortlist_manager', 'Shortlist_Manager::Shortlist_Manager', '0x839a80', 'shortlist_manager.cpp', 'thiscall, ret 4',
          'monta o gerente: a tabela de 9 bytes por clube (sem memoria, a caixa e o jogo para) e as 32 listas de posicao zeradas; com save, carrega tudo dele; senao cada clube ganha 40 vagas vazias (+0 = 0xb, sem posicao, avaliacao, teto e preco, sem alvo, substituto e oferta, -1, bits 1-4 das bandeiras postos e 0 e 5 tirados, indice -1, data 1/1997), inativo, a reputacao copiada e o contador de dias 0; os pares e as contagens zerados e as listas de posicao montadas; devolve this',
          'builds the manager: the 9-byte table per club (out of memory, the box and the game stops) and the 32 position lists cleared; with a save, loads everything from it; otherwise each club gets 40 empty slots (+0 = 0xb, no position, valuation, ceiling and price, no target, replacement or offer, -1, flag bits 1-4 set and 0 and 5 cleared, index -1, date 1/1997), inactive, its reputation copied and the day counter 0; the pairs and counts cleared and the position lists built; returns this'),
         ('shortlist_manager_create_initial_shortlists', 'Shortlist_Manager::create_initial_shortlists', '0x83a1b0', 'shortlist_manager.cpp', 'thiscall',
          'as listas curtas do jogo novo: avalia o elenco de cada clube na ordem de reputacao ("Assessing Club Squads"); para cada pessoa ("Creating Shortlists"), o clube (o time B quando e ligado ao principal) e, com contrato sem salario (-1), o salario padrao pelo status no elenco, limitado para estrangeiro pela nacao do clube (Coreia do Sul metade acima de 1000 e de 2500; Alemanha, Franca, Escocia e Holanda um quarto acima de 20000; Suecia um quarto acima de 250, 500 profissional; Gales, Finlandia, Irlanda e Irlanda do Norte 0,15 acima de 100, 200; Dinamarca e Noruega 0,1 acima de 250, 350; Belgica e Portugal piso de 5 x (reputacao / 500 + 1)² e 0,2 acima de 250, 500; Inglaterra, Italia, Espanha, Japao e EUA nada; as outras um quarto acima de 25000); jogador ganha o valor (clube humano marcado); ordena as listas de posicao e divide o orcamento de transferencias de cada clube nas vagas; progresso a cada 100 clubes e no fim',
          "the new game's shortlists: assesses every club squad in reputation order (\"Assessing Club Squads\"); for each person (\"Creating Shortlists\"), the club (its B team when linked to the main club) and, with a contract without a wage (-1), the default wage by squad status, capped for a foreigner by the club's nation (South Korea halves past 1000 and past 2500; Germany, France, Scotland and Holland a quarter past 20000; Sweden a quarter past 250, 500 professional; Wales, Finland, Ireland and Northern Ireland 0.15 past 100, 200; Denmark and Norway 0.1 past 250, 350; Belgium and Portugal a floor of 5 x (reputation / 500 + 1)² and 0.2 past 250, 500; England, Italy, Spain, Japan and the USA none; the others a quarter past 25000); a player gets his value (human club flagged); sorts the position lists and shares each club's transfer budget across its slots; progress every 100 clubs and at the end"),
         ('shortlist_manager_rebuild_club_shortlist', 'Shortlist_Manager::rebuild_club_shortlist', '0x84abe0', 'shortlist_manager.cpp', 'thiscall, ret 4',
          'refaz a lista curta de um clube (nada para selecao): solta os alvos das vagas, tira cada par da lista do clube dos dois vetores ordenados, avalia o elenco de novo e, clube da maquina, divide o orcamento de novo; depois o valor de cada um dos 50 do elenco com id valido: clube humano marcado como humano, salvo com o tecnico de ferias com a opcao de ferias e o contrato sem o bit 4 do status de transferencia (fica como esta)',
          "rebuilds one club's shortlist (nothing for a national team): releases the slot targets, takes each pair of the club's list out of both sorted arrays, assesses the squad again and, for a computer club, shares the budget again; then the value of each of the 50 squad members with a valid id: a human club flagged as human, except with its manager on holiday with the holiday option and the contract without bit 4 of the transfer status (left as is)")])
