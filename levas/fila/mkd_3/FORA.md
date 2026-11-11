# mkd_3: 0x85fc50 `STAFF_CONTRACT::set_to_desired_contract` fica fora (11/11/2026)

Reservado para a 0x85fc50 (o contrato desejado: salário pedido, luva, cláusulas, bônus e duração). A desmontagem
inteira foi lida e as regras provadas (ver `levas/reorganizacao/mercado/fechamento_mkd1.md`), mas a rotina **não foi
portada** nesta rodada:

- 3.109 instruções num quadro com tratador de exceção C++ (0x962a76, duas Datas locais com destrutor, `trylevel`
  0 → 1), 0x258 bytes de locais;
- chama três originais fora da CRT: `staff_interest_in_job_at_club` (0x844780, o ramo da comissão), e no caminho de
  erro 0x944cff/0x90d130/0x5e8290 (a caixa "Error" de staff_contracts.cpp linha 3150);
- chama métodos virtuais de competição (`[0xadadfc][div]`, slots +0x98 e +0x9c, com a liga de tipo 2) e
  `get_max_wage` até quatro vezes seguidas no mesmo cálculo;
- sorteia (`utils_random(0x1c)` 0x90cfc0, o dia do fim do contrato), então o comparador precisa restaurar o gerador
  do jogo e o estado inteiro do contrato (0x50 bytes), de [0xae1080], do texto 0xde1f64 e da pessoa (+0x3d).

Um porte fiel ao byte disso pede uma rodada própria (duas ou três rotinas auxiliares: o valor das cláusulas, a
duração, o ramo da comissão), com o comparador escrevendo o contrato inteiro lado a lado. Defeitos achados que o porte
tem de manter: o `&&` no lugar do `||` em 0x861d6b..0x861d70 (o caminho da caixa de erro lê o endereço 0) e a
variável de reputação em [esp+0x48] lida sem valor quando a pessoa tem ficha de jogador, cargo de comissão e nenhuma
ficha de não jogador (ver o fechamento).

Ponto de extensão que dispensa o porte (provado na desmontagem): o salário pedido sai em `contrato +0xc` = ftol do
valor final (0x862483..0x862493) e a luva em `contrato +8` (0x85ffe2..0x86016b); um desvio que chame a original e
ajuste os dois na saída muda o pedido sem reescrever a rotina; o retorno (int) é o valor do contrato (salário ajustado
+ o menor entre a soma de bônus e cláusulas e 10% do valor), que quem chama compara.
