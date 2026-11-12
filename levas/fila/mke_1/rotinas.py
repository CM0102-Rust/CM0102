# Um clube da IA põe na lista de compras um jogador que lhe é oferecido ou que ele encontra
# (Shortlist_Manager::try_add_player_to_club_shortlist 0x842e10, shortlist_manager.cpp; leitor mke do resto do mercado,
# 12/11/2026). Chamada pela oferta de jogadores aos clubes interessados (shortlist_manager.rs, a vitrine do jogador
# listado) e pelo processamento das ofertas; 12,6 milhões de chamadas até 2002-06-01 (cobertura de 17/10).
# Convenções conferidas na desmontagem (0x842e10..0x8437f4):
# - a rotina: thiscall(gerente de listas, jogador, clube, posições), ret 0xc, quadro SEH só com o STAFF_CONTRACT local
#   (0x85e410, sem destrutor);
# - cdecl: 0x600a50 (5 args), 0x53ca90, 0x5444a0 (jogador, 0), 0x5407e0 (jogador, técnico, 0; i16), 0x90cfc0,
#   0x946464 (bsearch: chave, base, n, 8, 0x757ea0);
# - 0x5fd210 push gerente; mov ecx, [0xb63c98] -> ret 4; 0x5fea50 push gerente; push 4 -> ret 8 (i8); 0x7f7490 push
#   gerente; push jogador; mov ecx, [0xb74448] -> ret 8; 0x4cf5a0 push pessoa; mov ecx, [0xae1050] -> ret 4 (a
#   primeira não guarda em 0xae1080, a segunda e a do alvo guardam, como a cópia faz);
# - 0x84b180 push clube; push clube do contrato; push jogador; mov ecx, this -> ret 0xc (u8); 0x8ce990 push 0; push 0;
#   push clube; push jogador; mov ecx, [0xde1f54] -> ret 0x10; 0x5a0590 push clube; mov ecx, [0xae2a38] -> ret 4;
#   0x598800 push 0; push 1; push 0; push 0; push 0; mov ecx, finanças -> ret 0x14 (só o dword baixo, com sinal);
# - 0x84b6f0 push &taxa; push clube; push contrato; push vaga; mov ecx, this -> ret 0x10 (sempre grava a taxa);
#   0x8c7560 push 1; push clube; push jogador; mov ecx, [0xde1f54] -> ret 0xc; 0x840d80 push 1; push avaliação; push
#   &nota; push vaga; push pessoa; push clube; mov ecx, this -> ret 0x18; 0x4d42d0 push pessoa; push clube; push
#   vendedor -> ret 0xc; 0x4d1600 push 1; push 1; push 0; push pessoa; push clube; push vendedor -> ret 0x18;
#   0x4d0c60 push 0; push -1; push 0; push vendedor; push jogador -> ret 0x14; 0x62ed70 push pessoa; mov ecx,
#   [0xb63d28] -> ret 4; 0x84af00 8 dwords (vendedor, jogador, valor i64, clube, pedido i64, 0) -> stdcall, ret 0x20;
# - 0x846920 push 0; push status; push 0; push 1|0; push clube; push jogador; mov ecx, this -> ret 0x18 (f64);
#   0x864550 push 0; push 0xb; push status; push 0; push clube; push jogador; lea ecx, local -> ret 0x18 (f64,
#   guardado como f32); 0x5968d0 push status; push jogador; mov ecx, finanças -> ret 8; 0x843e10 push 0; push 3; push
#   clube; push alvo antigo; mov ecx, this -> ret 0x10; 0x8423f0 push clube; push jogador; mov ecx, this -> ret 8.
# Fidelidade: o status vai para 0x864550 e 0x5968d0 com o eax inteiro (lixo do fnstsw e do 0x846920 em cima); as duas
# só leem o byte (get_max_wage: `job_arg as u8`; default_staff_contract repassa o status à get_max_wage e usa o byte),
# então a cópia passa o byte. A nota e o candidato ([esp+0x14]/[esp+0x28]) e o alvo antigo ([esp+0x24]) ficam no
# quadro entre as vagas, como na original.
# Comparador: só a original (sorteia, regrava o teto e o alvo das vagas, os pares da lista e o contrato guardado; não
# roda duas vezes).
SAVE = False
ROTINAS = [('try_add_player_to_club_shortlist', 0x842e10, [0x6a, 0xff, 0x68, 0xf8, 0x24, 0x96, 0x00])]
DOCS = ('um clube da IA põe na lista de compras um jogador que lhe é oferecido',
        'an AI club shortlisting a player it is offered',
        [('try_add_player_to_club_shortlist', 'Shortlist_Manager::try_add_player_to_club_shortlist', '0x842e10', 'shortlist_manager.cpp', 'thiscall, ret 0xc',
          'um clube da IA põe um jogador numa das 40 vagas da sua lista de compras (1 se pôs ou se ele já está na lista, 0 se não): sai o clube humano, o humano de férias com a opção 4 igual a 1 (ou 2 sem o jogador na lista do olheiro), a lista inativa ou com 100 dias ou mais, a regra de estrangeiros (0x84b180), quem o clube não pode comprar e o clube sem finanças; cada vaga de compra de jogador sem oferta cujas posições caibam nas pedidas: acima de 1,3 × o teto só o jogador listado de um vendedor humano com valor − 1,5 milhão abaixo do teto; o alvo igual ao substituído e a data da vaga não anterior a hoje pulam; numa vaga de status 5, mais de 24 anos pula e o estrangeiro fora da lista do olheiro precisa de reputação 1000 e da reputação vista pelo técnico ≥ 1000 + sorteio(2000); o visto fora dos status 1-3 pula; o teto que a verba passa cinco vezes vira avaliação² × 0,01536, no máximo 0,8 × a verba; o alvo atual com nota × a margem (1 − (pedido − valor)/(teto × fator de reputação), entre 0,01 e 3, acima de 1 vira y/3 + 0,67) já na avaliação mantém a vaga; senão a nota a bater é a avaliação menos a lesão; o jogador oferecido: o valor limitado no vendedor em crise, o preço pedido (0 com compensação), a previsão do vendedor 2, a nota × a margem menos a lesão abaixo da nota a bater e o interesse abaixo de 3,15 recusam (o teto volta); aceito, o salário desejado × max(3,5/interesse, 0,9) acima de 1,2 × o teto salarial encerra tudo com 0; senão ele vira o alvo, o alvo antigo sai (0x843e10 com 3) e ele entra nos pares (0x8423f0)',
          "an AI club puts a player in one of the 40 slots of its shortlist (1 when placed or already on it, 0 if not): out go a human club, a human on holiday with option 4 equal to 1 (or 2 with the player off the scout's list), an inactive list or one at 100 days or more, the foreigner rules (0x84b180), a player the club cannot bid for and a club without finances; each player purchase slot with no offer whose positions fit the ones asked: above 1.3 × the ceiling only a listed player of a human seller with value − 1.5 million below the ceiling; a target equal to the replaced person and a slot date not before today skip; in a status-5 slot, over 24 years skips and a foreigner off the scout's list needs reputation 1000 and the reputation seen by the manager ≥ 1000 + random(2000); the work permit outside status 1-3 skips; a ceiling the budget passes five times over becomes valuation² × 0.01536, at most 0.8 × the budget; the current target with score × the margin (1 − (asking − value)/(ceiling × reputation factor), between 0.01 and 3, above 1 becoming y/3 + 0.67) already at the valuation keeps the slot; otherwise the score to beat is the valuation less the injury; the player offered: the value capped at a seller in crisis, the asking price (0 with compensation), the seller's prediction 2, the score × the margin less the injury below the score to beat and an interest below 3.15 reject (the ceiling goes back); taken, a desired wage × max(3.5/interest, 0.9) above 1.2 × the wage cap ends everything with 0; otherwise he becomes the target, the old target leaves (0x843e10 with 3) and he enters the pairs (0x8423f0)")])
CAMPOS = [('vaga da lista (37 bytes) +7 teto: regravado (avaliação² × 0,01536, no máximo 0,8 × a verba) quando a verba passa cinco vezes o teto, e devolvido se o jogador for recusado; lê e grava (try_add_player_to_club_shortlist)',
           'shortlist slot (37 bytes) +7 ceiling: rewritten (valuation² × 0.01536, at most 0.8 × the budget) when the budget passes the ceiling five times over, and put back when the player is rejected; reads and writes (try_add_player_to_club_shortlist)')]
INTERESSA = ['Tweaks', 'CMRTE']
