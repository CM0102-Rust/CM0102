# O contrato desejado de uma pessoa num clube (STAFF_CONTRACT::set_to_desired_contract 0x85fc50, staff_contracts.cpp;
# fechamento do leitor D1 do mapa do mercado, 11/11/2026; as regras provadas passo a passo em
# levas/reorganizacao/mercado/fechamento_mkd1.md §2). Chamada por 0x47c0b0, 0x4cda70, 0x4ce450 (duas vezes), 0x4ce4e0,
# 0x4de5d0, 0x4e2280, 0x84a020, 0x866620, 0x866830, 0x8bf060, 0x8c0080, 0x8c9720.
# Igual no cm0102-1080.exe (0x2a90 bytes conferidos, com as quatro tabelas de salto 0x86268c..0x8626e0).
# Convenções conferidas na desmontagem (3.109 instruções; quadro C++ com tratador 0x962a76 só para as Datas locais,
# sem destrutor com efeito):
# - a rotina: thiscall(contrato; pessoa, clube, oferta, estatuto, cargo, motivos, &vontade), ret 0x1c, devolve int (eax);
#   o estatuto é empurrado como dword e só o byte baixo é regravado (os altos seguem para quem recebe o dword);
#   "motivos" (antes lido como "tipo") é o ponteiro dos bits de motivo que vai inteiro à 0x844780 e à 0x846920;
# - 0x4cf5a0 get_contract: ecx = [0xae1050]; push pessoa -> thiscall ret 4 (o resultado vai a [0xae1080], 6 vezes);
# - 0x4cf5f0 get_contract_for_club: ecx = [0xae1050]; push clube; push pessoa -> thiscall ret 8;
# - 0x540a50 club_get_linked_b_team: push 1; push &local; push clube; add esp, 0xc -> cdecl;
# - 0x4ca470 get_deserved_squad_status: ecx = [0xae1050]; push contrato; push pessoa -> thiscall ret 8 (al);
# - 0x849f00 get_squad_status_for_club: ecx = [[0xde1f54]+0x856]; push 0; push clube; push pessoa -> thiscall ret 0xc;
# - 0x53b620 is_player_job: push cargo (dword); add esp, 4 -> cdecl;
# - 0x4d0c60 calc_player_value: ecx = [0xae1050]; push 0; push -1; push 1; push clube; push pessoa -> thiscall ret 0x14;
# - 0x944c9f sprintf: cdecl (0xde1f64, 0xa8391c, id), add esp, 0xc;
# - 0x54cdb0 days_since: ecx = contrato + 0x2d; push [0xae2c94]; push [0xae2c90] -> thiscall ret 8;
# - 0x8cca10 get_future_transfer: ecx = [0xde1f54]; push 0; push pessoa -> thiscall ret 8;
# - 0x544520 is_transfer_listed_or_requested: push clube; push pessoa; add esp, 8 -> cdecl;
# - 0x864550 default_staff_contract: push tipo; push cargo; push estatuto; push flag; push clube; push pessoa -> stdcall
#   ret 0x18 (st0; o ecx que a original passa não conta);
# - 0x844780 staff_interest_in_job_at_club: ecx = [[0xde1f54]+0x856]; push motivos; push cargo; push 0; push clube;
#   push pessoa -> thiscall ret 0x14 (st0); 0x846920 staff_interest_in_joining_club: o mesmo ecx; push motivos;
#   push estatuto; push 0; push 0; push clube; push pessoa -> thiscall ret 0x18 (st0);
# - 0x5a0590 get_club_finance: ecx = [0xae2a38]; push clube -> thiscall ret 4; 0x5968d0 get_max_wage: push estatuto
#   (dword); push pessoa; ecx = finanças -> thiscall ret 8 (as quatro chamadas 0x861274/0x8612a2/0x8612d4/0x861302);
# - 0x549e50 club_is_in_major_nation: push clube; add esp, 4 -> cdecl (al);
# - 0x600a50 is_human_club: push 0; push 0; push 1; push 0; push clube; add esp, 0x14 -> cdecl;
# - virtuais da liga ([0xadadfc][divisão], tipo +0x43 = 2): call [vtable+0x98] / [vtable+0x9c], ecx = liga, sem args;
# - 0x8cf470 find_next_window_open_date: ecx = [0xde1f54]; push contrato+0x25; push clube -> thiscall ret 8 (Rust,
#   chamada direta); 0x8c61e0 get_offer_slot: ecx = [0xde1f54]; push oferta -> thiscall ret 4; 0x8cf0e0
#   find_next_allowed_transfer_date: ecx = [0xde1f54]; push contrato+0x25; push *vaga -> thiscall ret 8;
# - caminho de erro (0x861d72, inalcançável: lê o endereço 0 antes): 0x944cff _splitpath cdecl (0xa8360c, 0, 0, nome,
#   ext); 0x90d130 (nome, ext, 0xc4e) cdecl sem tirar os argumentos, que são o resto do sprintf "v%s %s.%s %d" em
#   0xae24d0; 0x5e8290 ("Error", 0xae24d0, 0) cdecl; [0xb67a34] = 0;
# - 0x417c10 Date::operator=: ecx = destino; push origem; push &saída -> thiscall ret 8;
# - 0x4d45f0 season_end_month: ecx = [0xae1050]; push clube -> thiscall ret 4 (deixa -1 e 0x7ce na pilha para a
#   0x549f70); 0x90cfc0 utils_random: push 0x1c; add esp, 4 -> cdecl; 0x549f70 Date(dia, mês, 0x7ce, -1) -> thiscall
#   ret 0x10 sobre a local; 0x669360 key_nation_index_of_club: push clube; add esp, 4 -> cdecl; 0x549ef0 Date(dia, ano)
#   -> thiscall ret 8 (só os 16 bits baixos contam); 0x945640 _ftol -> ftol.
# Comparador: compare_staff_contract_set_to_desired_contract, lado a lado (os comparadores de quem chama já a rodam duas
# vezes): o contrato (0x50 bytes), [0xae1080], o gerador (STATE_REGIONS), o texto 0xde1f64 (0x50), a pessoa +0x3d e os
# alvos de &vontade e dos motivos; o int devolvido igual.
# Determinismo: [esp+0x48] (a reputação das cláusulas e da duração) fica sem valor na original com ficha de jogador,
# cargo de comissão e sem ficha de não jogador (resto de pilha); a cópia usa 0 (vira 1). Ela só é lida nesse caso
# quando a pessoa tem job_for_club 11 (as cláusulas) ou um clube (a duração): o comparador acusa se acontecer.
SAVE = False
ROTINAS = [('staff_contract_set_to_desired_contract', 0x85fc50, [0x6a, 0xff, 0x68, 0x76, 0x2a, 0x96, 0x00])]
DOCS = ('o contrato desejado de uma pessoa num clube',
        'the desired contract of a person at a club',
        [('staff_contract_set_to_desired_contract', 'STAFF_CONTRACT::set_to_desired_contract', '0x85fc50', 'staff_contracts.cpp', 'thiscall, ret 0x1c',
          'preenche o contrato que a pessoa quer no clube e devolve o valor dele (int). O clube atual sai do contrato da pessoa (sem clube pedido, o atual ou o time B ligado); o contrato atual é o com esse clube; tipo 2, id, situação (−1 vira a merecida, a queixa 0x10 dá 2). O nível: reputação do clube / 500 + 1 (estatuto profissional 2: −1, entre 1 e 4), sem clube a da ficha / 500 ou a da seleção / 500 + 1 (mínimo 3), mais até 4 pelos jogos pela seleção. A luva (+8) de jogador: valor / 5 × 2, reduzida pela reputação, pela idade e pelo estatuto merecido na renovação (0 abaixo de 19 anos) e pelo clube atual fraco no clube novo. Comissão: o salário padrão, + 2500 com vontade (0x844780) abaixo de 1,75, × máx(3,5 / vontade; 0,9); 250000 (a marca de "indisponível") acima dos limites por cargo e de 250000. Jogador: o salário padrão (com o teto do clube na renovação "barata": sem queixa e perto do fim, jovem sem pressão e fora da lista, queixa leve, ou leal sem fama); acima do salário atual no máximo o dobro (ou + 8000), e o famoso em clube forte 25% acima de 75000/50000; o ambicioso de 19 a 31 anos em país grande sobe a uma fração do maior salário do clube (0,9 a 1,45 conforme posição, idade e caráter); q = 3,5 / vontade (0x846920), + 5000 para o inquieto, q mínimo 0,9 e, na renovação, os tetos pelas queixas, idade e estatuto (1,1 a 4,0); valor = q × salário, no mínimo o salário atual pela razão das reputações e 95% do maior salário com a queixa 0x1000000; marcas de 250000 por idade, goleiro, estatuto e clube humano; contrato atual mais caro em outro clube: a mistura (20 − a) / 20 entre o salário atual e o pedido (get_max_wage de duas a quatro vezes). Cláusulas (cargo 11): não promoção, rebaixamento, multa mínima e sem jogar, pelo valor de mercado e pelas ligas (virtuais +0x98/+0x9c); a cláusula do contrato atual que cai multiplica o valor por 1,05 ou 1,15. Datas: a janela (e logo hoje); o fim no dia sorteado (1 a 28) do mês do fim da temporada, ano seguinte nas ligas de duas datas, + ftol(L + 1) anos, L pela idade, posição, cláusulas, nível, estatuto, reputação, Finlândia/Turquia, salário e o máximo de 9; tipo 2 (bit 0x80 sem cargo no clube e nível baixo por sorteio fixo; até 20 anos, tipo 1 sem anos); a renovação não termina antes da data da pessoa. Bônus por posição (−1 sem), o bit 0x80 abaixo de 125 em clube de estatuto 2, o salário (+0xc) = ftol(valor); o retorno = salário (× 0,5 no tipo 1, × 1,05 com 0x80) + o menor entre bônus e cláusulas e 10% do valor. Defeitos mantidos: sem vaga da oferta lê o endereço 0 (&& no lugar de ||) antes da caixa de erro; a reputação das cláusulas fica sem valor para ficha de jogador com cargo de comissão e sem ficha de não jogador; a data da janela é sobrescrita com hoje; o ramo dos tipos 1/2 no clube novo nunca roda',
          'fills the contract the person wants at the club and returns its value (an int). The current club comes from the person\'s contract (without a club asked, the current one or its linked B team); the current contract is the one with that club; kind 2, id, squad status (−1 becomes the deserved one, grievance 0x10 gives 2). The level: club reputation / 500 + 1 (professional status 2: −1, within 1..4), without a club the sheet\'s / 500 or the national team\'s / 500 + 1 (at least 3), plus up to 4 by caps. The signing fee (+8) of a player: value / 5 × 2, cut by reputation, by age and deserved status on a renewal (0 under 19) and by a weak current club for a new club. Staff: the default wage, + 2500 with will (0x844780) below 1.75, × max(3.5 / will, 0.9); 250000 (the "not available" marker) above the limits by job and above 250000. Player: the default wage (with the club\'s cap on a "cheap" renewal: no grievance near the end, young without pressure and not listed, a light grievance, or loyal without fame); above the current wage at most double (or + 8000), and the famous at a strong club 25% above 75000/50000; the ambitious from 19 to 31 at a major nation rises to a share of the club\'s highest wage (0.9..1.45 by position, age and character); q = 3.5 / will (0x846920), + 5000 for the restless, q at least 0.9 and, on a renewal, the caps by grievances, age and status (1.1..4.0); value = q × wage, at least the current wage by the reputations\' ratio and 95% of the highest wage with grievance 0x1000000; 250000 markers by age, goalkeeper, status and human club; a dearer current contract at another club: the (20 − a) / 20 blend between the current wage and the asked one (get_max_wage two to four times). Clauses (job 11): non-promotion, relegation, minimum fee and not playing, by market value and the leagues (virtuals +0x98/+0x9c); a clause of the current contract that is dropped multiplies the value by 1.05 or 1.15. Dates: the window (then today anyway); the end on the drawn day (1..28) of the season end month, next year in two-year seasons, + ftol(L + 1) years, L by age, position, clauses, level, status, reputation, Finland/Turkey, wage and the maximum of 9; kind 2 (bit 0x80 without a club job and a low level by a fixed draw; up to 20 years, kind 1 without years); a renewal never ends before the person\'s date. Bonuses by position (−1 without), the 0x80 bit below 125 at a club of status 2, the wage (+0xc) = ftol(value); the return = wage (× 0.5 for kind 1, × 1.05 with 0x80) + the smaller of bonuses and clauses and 10% of the value. Defects kept: without an offer slot it reads the address 0 (&& for ||) before the error box; the clauses\' reputation is unset for a player sheet with a staff job and no non-player sheet; the window date is overwritten with today; the kind 1/2 branch of a new club never runs')])
CAMPOS = [('contrato +0x1e: cláusula de liberação "sem jogar" [inferência]; byte; grava (1 com a reputação acima do nível; multa mínima 1500000)',
           'contract +0x1e: the "not playing" release clause [inference]; byte; writes (1 with the reputation above the level; minimum fee 1500000)'),
          ('contrato +0x49: saída pela Bosman; byte; lê (0 é condição da renovação barata) e grava 0',
           'contract +0x49: leaving on a Bosman; byte; reads (0 is a condition of the cheap renewal) and writes 0'),
          ('finanças +0x10: maior salário da folha; valor; lê (a referência do reforço do ambicioso e o piso da queixa 0x1000000)',
           'finance +0x10: highest wage of the wage bill; value; reads (the reference of the ambitious one\'s rise and the floor of grievance 0x1000000)'),
          ('[0xae1080] último contrato consultado; ponteiro; grava', '[0xae1080] last contract looked up; pointer; writes')]
INTERESSA = ['CMRTE']
