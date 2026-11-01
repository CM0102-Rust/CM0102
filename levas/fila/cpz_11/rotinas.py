# A folha salarial e a janela dos ingressos (finance.cpp, fin_prm.cpp no mapa):
# - 0x598370 (Finance::update_wage_bill): pelo passe diario das financas 0x59b9c0 (no sabado, todo clube com nacao: a
#   folha +0xc e lida logo depois para pagar os salarios da semana), pela assinatura de contrato 0x4cf720, pelo passe
#   dos contratos 0x4c7ac0, pelos emprestimos 0x4d01e0/0x4d07e0 (Rust) e pela 0x59cf50. A folha (+0xc), o maior salario
#   (+0x10) e o teto salarial (+0x14, Finance::get_max_wage 0x5968d0).
# - 0x59cf50 (FinanceTable::refresh_all_wage_bills): so pela criacao do jogo (0x828110), uma vez.
# - 0x59b4c0 (finance_get_days_to_season_start): pelo passe diario das financas, semanal por clube (depois do
#   merchandising 0x59b7e0, "cpv_5 merchandising" 16384 na base); devolvendo > 0, os ingressos da temporada
#   (0x599a20, "cpv_5 ingressos vendidos" 512); 0, zera +0x18 e +0x164.
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   0x4cf5f0 contrato_com_clube: thiscall (mov ecx, [0xae1050] em 0x5983c4), 2 args (push ebp = clube, push eax = pessoa).
#   0x864550 salario padrao (STAFF_CONTRACT::default_staff_contract): a original poe ecx = edi (o contrato, 0x5983ec) e
#   empurra 6 args (0, 0xb, -1, 0, clube, pessoa), ret 0x18, devolve double em st0 seguido do _ftol 0x945640; a rotina
#   sobrescreve o ecx na primeira instrucao (o mapa diz stdcall): declarada thiscall com o mesmo ecx (mesma pilha).
#   0x4cf6d0 ficha_de_contrato: thiscall (mov ecx, [0xae1050] em 0x5983fc e 0x598418), 1 arg; chamada duas vezes, como
#   na original. 0x4cf5a0 contrato_de: thiscall (mov ecx, [0xae1050]), 1 arg. 0x5968d0 get_max_wage: thiscall (mov ecx,
#   esi em 0x598693), 2 args (push -1, push 0: pessoa 0, cargo -1), ret 8. 0x945640 _ftol: x87, pelo ftol.
#   0x53b900 e_selecao: cdecl (add esp, 4 em 0x59cf70), 1 arg. 0x598370 folha: thiscall (mov ecx, edi em 0x59cf77), 0
#   args. 0x669360 linha_da_nacao_do_clube: cdecl (add esp, 4 em 0x59b4e0), 1 arg, so o al (test al, al; movsx).
#   0x54cfb0 mes_de: thiscall (mov ecx, 0xae2c90 em 0x59b596), 0 args, al. 0x54cdb0 dias_ate: thiscall (lea ecx,
#   [esi + 0xb63d8a]), 2 args (a data de hoje por valor: push [0xae2c94], push [0xae2c90]). A caixa de erro (aviso_em):
#   0x944cff/0x90d130/0x944c9f/0x5e8290 cdecl (add esp 0x14 e 0x24 em 0x59b502 e 0x59b53d).
#   A propria 0x59b4c0: o mapa diz stdcall(clube, &saida) ret 8; o unico chamador (0x59bfbc) poe ecx = a tabela antes do
#   call; a rotina nao le o ecx: declarada thiscall (mesma pilha). 0x598370 e 0x59cf50: thiscall, ret 0.
# Comparadores: a 0x598370 lado a lado na folha, no maior salario, no teto e em [0xae1080] (que a 0x864550 grava); a
# 0x59cf50 lado a lado na tabela de financas inteira (uma vez, na criacao); a 0x59b4c0 lado a lado na saida e na data
# estatica da 0x54cdb0 (0xae2470), com a devolucao (sem linha da nacao: so a original, a caixa). Sem SEH; x87 em
# precisao dupla. Iguais no cm0102-1080.exe.
SAVE = False
ROTINAS = [('folha_salarial_do_clube', 0x598370, [0x83, 0xec, 0x0c, 0x53, 0x55, 0x56]),
           ('refazer_todas_as_folhas', 0x59cf50, [0xa1, 0x5c, 0x23, 0xae, 0x00]),
           ('dias_ate_a_temporada', 0x59b4c0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a folha salarial e a janela dos ingressos: a folha do clube, as folhas refeitas na criação e os dias até a temporada',
        'the wage bill and the season ticket window: the club wage bill, the bills rebuilt at creation and the days to the season start',
        [('folha_salarial_do_clube', 'Finance::update_wage_bill', '0x598370', 'finance.cpp', 'thiscall',
          'soma os salários do elenco (menos a parte do outro clube nos emprestados) e do pessoal na folha (+0xc) e o maior salário (+0x10); com menos de 15 jogadores, as vagas que faltam por um salário pela reputação; mais reputação² × fator pelo tipo do clube e pela nação; o teto salarial (+0x14) pela 0x5968d0',
          'sums the squad wages (less the other club\'s share for loanees) and the staff wages into the wage bill (+0xc) and the highest wage (+0x10); with fewer than 15 players, the missing places at a wage by reputation; plus reputation² × a factor by club type and nation; the wage ceiling (+0x14) by 0x5968d0'),
         ('refazer_todas_as_folhas', 'FinanceTable::refresh_all_wage_bills', '0x59cf50', 'finance.cpp', 'thiscall',
          'na criação do jogo, a folha de cada clube até a primeira seleção',
          'at game creation, the wage bill of every club up to the first national team'),
         ('dias_ate_a_temporada', 'finance_get_days_to_season_start', '0x59b4c0', 'finance.cpp', 'thiscall, ret 8 (o ecx não é lido)',
          'os dias até o começo da temporada da nação do clube, na janela dos ingressos (até dois meses antes); marca se o ano da temporada já chegou',
          'the days until the season start of the club\'s nation, inside the season ticket window (up to two months before); flags whether the season year has come')])
CAMPOS = [('finanças do clube (0x167 bytes) +8 id do clube, +0xc folha semanal, +0x10 maior salário, +0x14 teto salarial; valores; grava (folha_salarial_do_clube); toda a tabela (refazer_todas_as_folhas)',
           'club finance (0x167 bytes) +8 club id, +0xc weekly wage bill, +0x10 highest wage, +0x14 wage ceiling; values; writes (folha_salarial_do_clube); the whole table (refazer_todas_as_folhas)'),
          ('clube +0xd7 50 vagas do elenco, +0x19f/+0x1b3/+0x1cf pessoal (5, 7, 3), +0x53 nação, +0x64 tipo, +0x80 reputação; ponteiros/valores; lê (folha_salarial_do_clube)',
           'club +0xd7 50 squad places, +0x19f/+0x1b3/+0x1cf staff (5, 7, 3), +0x53 nation, +0x64 type, +0x80 reputation; pointers/values; reads (folha_salarial_do_clube)'),
          ('contrato (0x50 bytes) +0xc salário semanal (−1: padrão), +0x35 & 0x3f tipo; valores; lê (folha_salarial_do_clube)',
           'contract (0x50 bytes) +0xc weekly wage (−1: default), +0x35 & 0x3f type; values; reads (folha_salarial_do_clube)'),
          ('nação +0 id (Inglaterra 0x9cf2e4, Itália 0x9cf360), +0x7e tipo; valores; lê (folha_salarial_do_clube)',
           'nation +0 id (England 0x9cf2e4, Italy 0x9cf360), +0x7e type; values; reads (folha_salarial_do_clube)'),
          ('[0xae1080] último contrato consultado; ponteiro; grava (pelo salário padrão 0x864550)', '[0xae1080] last contract looked up; pointer; writes (through the default wage 0x864550)'),
          ('linha da nação (72 bytes em 0xb63d60) +0x17 marca de 2002, +0x2a data do começo da temporada, +0x2c ano do começo, +0x34 mês do começo; valores; lê (dias_ate_a_temporada)',
           'nation row (72 bytes at 0xb63d60) +0x17 2002 flag, +0x2a season start date, +0x2c start year, +0x34 start month; values; reads (dias_ate_a_temporada)'),
          ('[0xdd6ec8] data do começo do jogo (ano em +2), [0xae2c90] hoje; valores; lê (dias_ate_a_temporada)', '[0xdd6ec8] game start date (year at +2), [0xae2c90] today; values; reads (dias_ate_a_temporada)')]
INTERESSA = ['CMRTE', 'SGE']
