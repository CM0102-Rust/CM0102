# 248ª leva — as ofertas pelo jogador: se o clube tem oferta por ele, a oferta do clube e se ele tem oferta pendente; as ofertas e o contrato do jogador: se ele está sob oferta, se o clube dele segue a lei Bosman e o veto da diretoria na oferta; os registros das transferências: guardar a transferência passada, dar a permissão de trabalho e guardar o prêmio por jogo no time; os contratos da comissão e as datas de decisão: achar a vaga de contrato, dar o contrato à pessoa nova e marcar a data da decisão; o contrato da comissão: montar a proposta pelo contrato atual, zerar a moral do contrato novo e a faixa do vencimento; o registro de histórias da mídia: registrar uma história, expirar as antigas e se a história já foi contada; os interesses de transferência da mídia: acrescentar um interesse e as comparações das chaves de ids e das histórias

**Rotinas:**
- `clube_tem_oferta_pelo_jogador()` = 0x8cdc90 (transfer_manager.cpp, thiscall, ret 8): 1 se a pessoa marcada como sob oferta (bit 0 de [+0x86e] + id × 8) tem, entre as ofertas em aberto (0x8c5cf0), uma do clube (+8 da oferta); senão 0;
- `oferta_do_clube_pelo_jogador()` = 0x8cdd80 (transfer_manager.cpp, thiscall, ret 0xc): a oferta do clube pela pessoa (com `todas`, também as encerradas e sem exigir a marca), ou 0;
- `jogador_com_oferta_pendente()` = 0x8cd150 (transfer_manager.cpp, thiscall, ret 4): 1 se a pessoa está sob oferta (0x8cde70) e uma oferta em aberto está na situação (+0x2c) 0, 1, 5 ou 6; senão 0;
- `jogador_sob_oferta()` = 0x8cde70 (transfer_manager.cpp, thiscall, ret 8): sem a visão do humano, a marca "sob oferta" (bit 0 de [+0x86e] + id × 8); com ela, 1 se alguma oferta é conhecida (+0x2d), é de clube do humano da vez ou pelo jogador de clube dele, se a pessoa está na shortlist dele ou se ele tem oferta de contrato aceita pela pessoa;
- `clube_do_jogador_com_lei_bosman()` = 0x8d0a70 (transfer_manager.cpp, stdcall, ret 4): 1 se a nação do clube do contrato da pessoa tem transferência livre no fim do contrato (0x544670); sem contrato, clube ou nação: 1;
- `veto_da_diretoria_na_oferta()` = 0x8cdb80 (transfer_manager.cpp, thiscall, ret 0x18): com a parte de troca da oferta vazia, a diretoria do clube que decide responde (0x696b10, pelo valor 0x8e7b50); 0x2000: motivo 0x24 e o jogador perde 10% das reputações (+0xb atual, +9 na nação, +0xd mundial); 0x800: 0x17; 0x200: 0x18; cada um devolve 1;
- `guardar_transferencia_passada()` = 0x8cb810 (transfer_manager.cpp, thiscall, ret 4): acrescenta a transferência passada (0x25 bytes, 0x8cb940) ao vetor +0x862 (capacidade +0x866, conta +0x86a), que cresce de 100 em 100; devolve o registro novo;
- `dar_permissao_de_trabalho()` = 0x8c7070 (transfer_manager.cpp, thiscall, ret 0x14): acrescenta a permissão (14 bytes: pessoa, ano +8, nação +0xa) ao vetor +0x93a (cresce de 30 em 30), liga a marca 0x20 da pessoa em +0x86e e, se pedido, reordena (0x8e55c0);
- `guardar_premio_por_jogo_no_time()` = 0x8cac40 (transfer_manager.cpp, thiscall, ret 4): acrescenta o prêmio por jogo (13 bytes) ao vetor +0x88c (cresce de 50 em 50) e reordena pela pessoa (0x8b4e00);
- `vaga_livre_de_contrato()` = 0x4c79a0 (contract_manager.cpp, thiscall, ret 4): a vaga do vetor de contratos de 0x50 bytes: a primeira vazia (id −1) ou, só no fim, a nova (cresce de 200 em 200; sem memória, caixa e −1);
- `acrescentar_contrato_da_pessoa()` = 0x4c7950 (contract_manager.cpp, thiscall, ret 8): a vaga do fim para a pessoa nova (técnico contratado, regen): liga a pessoa à vaga em [0xae2d0c], põe o id e gera o contrato inicial pelo valor (0x85e960);
- `acrescentar_data_de_decisao()` = 0x4cd6d0 (contract_manager.cpp, thiscall, ret 0x10): acrescenta (pessoa, data, fatia do dia, tipo) ao vetor de 14 bytes das datas de decisão (cresce de 100 em 100); o tipo 1 leva antes lembretes de tipo 2 pela moral do contrato;
- `zerar_moral_do_contrato()` = 0x866ee0 (staff_contracts.cpp, thiscall, ret 8): zera moral, queixas e felicidade do contrato, limpa os vínculos de opinião da pessoa (0x549c90), põe 4 em +0x4e e o nível do idioma da nação do clube (0x7c6830, até 10) em +0x4f;
- `contrato_pelo_atual()` = 0x85e590 (staff_contracts.cpp, thiscall, ret 0x18): a proposta pelo contrato atual: pessoa, clube, salário × percentual / 100, hoje, o fim, bits, a moral zerada como a 0x866ee0 e as duas cláusulas; sem contrato, caixa;
- `faixa_do_vencimento()` = 0x867d30 (staff_contracts.cpp, thiscall): a faixa 0..=14 do fim do contrato contra hoje e as datas-limite do CONTRACT_MANAGER (0xae1010..0xae1090);
- `registrar_historia_da_midia()` = 0x757a50 (media.cpp, thiscall, ret 0x14): acrescenta a história (data de hoje, ids dos sujeitos ou −1, tipo, subtipo, resposta) e reordena (0x757ee0); cresce de 20 em 20;
- `expirar_historias_da_midia()` = 0x757c70 (media.cpp, thiscall, ret 0): tira as histórias de mais de 100 dias (as do time, 0xbdf, de mais de 40 + sorteio(40)) pondo a última no lugar; reordena;
- `historia_ja_registrada()` = 0x757da0 (media.cpp, thiscall, ret 0x14): a do time: procura em ordem por id do 2º sujeito e subtipo; as outras: busca binária pela chave e, com a data, se não é anterior;
- `acrescentar_interesse_de_transferencia()` = 0x754740 (media.cpp, thiscall, ret 0x14): se a chave {jogador, clube} não está, acrescenta o interesse (data de hoje, valor até 50 milhões, avaliação × 100) e reordena; cresce de 20 em 20;
- `ordem_das_chaves_de_ids()` = 0x757ea0 (media.cpp, cdecl): nulo antes; pelo id +0 e, igual, pelo +4 (−1 casa com qualquer um);
- `ordem_das_historias()` = 0x757ee0 (media.cpp, cdecl): nulo antes; pelos sujeitos +8 e +0xc, o tipo +0x13 e o subtipo +0x11;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (19):** 18 acusadas. `cpl_contrato_valor` escapou: acrescentar_contrato_da_pessoa 0x4c7950: metade do valor passado à 0x85e960 (o contrato inicial) não muda a carreira até 2002-06-01 (contadores na 248ª DLL, carreira até 2002-06-01, impressão be54a138 igual: 128 ou mais chamadas, todas com clube e valor acima de 1); na 0x85e960 o valor só entra por ramos condicionais (a competição de tipo 2 pelas vtables +0x98/+0x9c e a comparação em 0x85ef76), que pela carreira não rodam ou dão no mesmo; a causa exata não foi isolada; escapou também no jogo comparando e na carga pt (0 divergências). (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (2f65fd6b68d1027b).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (be54a1386f51073a).

**Refazer**, sobre o commit da 247ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 248`

**Campos tocados:**
- gerente de transferências +0x86e: tabela de 8 bytes por id de pessoa (bit 0 = sob oferta); valor; lê (clube_tem_oferta_pelo_jogador, oferta_do_clube_pelo_jogador)
- gerente de transferências +0/+4: vetor de ponteiros das ofertas e a conta; ponteiro; lê (as três)
- oferta +8: id do clube que oferece; id; lê (clube_tem_oferta_pelo_jogador, oferta_do_clube_pelo_jogador)
- oferta +0x2c: situação da oferta (0, 1, 5, 6 = pendente); valor; lê (jogador_com_oferta_pendente)
- [0xae2364] número de pessoas; valor; lê
- gerente de transferências +0x86e: tabela de 8 bytes por id de pessoa (bit 0 = sob oferta); valor; lê (jogador_sob_oferta)
- oferta +8/+0xc: id do clube que oferece e da pessoa; id; lê (jogador_sob_oferta)
- oferta +0x2d: oferta conhecida do humano; valor; lê (jogador_sob_oferta)
- oferta +4: a parte de troca (palavra +0 = quantos); ponteiro; lê (veto_da_diretoria_na_oferta)
- contrato +4/+0x35: id do clube e situação (& 0x3f = 5: não vale); id e valor; lê (jogador_sob_oferta, clube_do_jogador_com_lei_bosman)
- clube +0x53: nação do clube; ponteiro; lê (clube_do_jogador_com_lei_bosman)
- jogador (staff +0x61) +9/+0xb/+0xd: as reputações na nação, atual e mundial (palavras com sinal); valor; lê e grava 90% (veto_da_diretoria_na_oferta)
- [0xae1080] o último contrato achado; ponteiro; grava (jogador_sob_oferta, clube_do_jogador_com_lei_bosman)
- humano da vez +0x39: o clube dele; ponteiro; lê (jogador_sob_oferta)
- gerente de transferências +0x862/+0x866/+0x86a: vetor de 0x25 bytes das transferências passadas, capacidade, conta; ponteiro e valores; lê e grava (guardar_transferencia_passada)
- gerente de transferências +0x93a/+0x942/+0x93e: vetor de 14 bytes das permissões de trabalho (+0 pessoa, +4/+6 zerados, +8 ano, +0xa nação), capacidade, conta; lê e grava (dar_permissao_de_trabalho)
- gerente de transferências +0x86e: tabela de 8 bytes por id de pessoa (bit 0x20 = tem permissão de trabalho); valor; grava (dar_permissao_de_trabalho)
- gerente de transferências +0x88c/+0x890/+0x894: vetor de 13 bytes dos prêmios por jogo no time principal, capacidade, conta; lê e grava (guardar_premio_por_jogo_no_time)
- contract manager +0/+4/+8: vetor de contratos de 0x50 bytes, capacidade, conta; ponteiro e valores; lê e grava (vaga_livre_de_contrato, acrescentar_contrato_da_pessoa)
- contrato +0: id da pessoa (−1 vaga vazia); id; lê e grava (vaga_livre_de_contrato, acrescentar_contrato_da_pessoa)
- [0xae2d0c] + id × 0x4f, +0: índice do contrato da pessoa; índice; lê e grava (acrescentar_contrato_da_pessoa), lê (acrescentar_data_de_decisao)
- contrato +0x44: moral; valor; lê (acrescentar_data_de_decisao, os lembretes do tipo 1)
- pessoa +0x61: registro de jogador; ponteiro; lê (acrescentar_data_de_decisao)
- contract manager +0xc/+0x10/+0x14: vetor de 14 bytes das datas de decisão (+0 id, +4 data, +0xc fatia, +0xd tipo), capacidade, conta; lê e grava (acrescentar_data_de_decisao)
- [0xae2364] número de pessoas; valor; lê (acrescentar_data_de_decisao)
- contrato +0/+4/+8/+0xc: id da pessoa, id do clube, 0, salário; id e valor; grava (contrato_pelo_atual)
- contrato +0x10/+0x14/+0x18: −1; grava (contrato_pelo_atual)
- contrato +0x1c: cláusula sem acesso; +0x1d: multa mínima; +0x1e..+0x24: 0; valor; grava (contrato_pelo_atual)
- contrato +0x25: data do início (hoje, 0xae2c90); +0x2d: data do fim; data; grava (contrato_pelo_atual), lê +0x2d (faixa_do_vencimento)
- contrato +0x35: bits (& 0x3f tipo); valor; grava & 0xc6 | 6 (contrato_pelo_atual), lê (faixa_do_vencimento)
- contrato +0x36: salário (cópia); +0x3a, +0x49: 0; +0x3b (16 bits): 0; +0x4a: −1; grava (contrato_pelo_atual)
- contrato +0x3e, +0x3f (felicidade), +0x40, +0x44 (moral), +0x45 (queixas): 0; +0x4e: 4; +0x4f & 0xf: nível do idioma; grava (zerar_moral_do_contrato, contrato_pelo_atual)
- [0xae2d0c] + id × 0x4f, +0x27/+0x37/+0x3b/+0x4b: vínculos de opinião; −1; grava pela 0x549c90 (zerar_moral_do_contrato, contrato_pelo_atual)
- clube +0x53: nação; ponteiro; lê; nação +0x77: idioma; valor; lê (zerar_moral_do_contrato, contrato_pelo_atual)
- [0xae1010..0xae1090] datas-limite do CONTRACT_MANAGER; [0xae2c90] hoje; data; lê (faixa_do_vencimento, contrato_pelo_atual)
- Media +0 registro de histórias (23 bytes: +0 data, +8 id do sujeito a ou −1, +0xc id do sujeito b ou −1, +0x10 resposta, +0x11 subtipo palavra, +0x13 tipo), +4 capacidade, +8 conta; grava (registrar_historia_da_midia, expirar_historias_da_midia), lê (historia_ja_registrada)
- [0xae2c90] data de hoje; valor; lê (registrar_historia_da_midia, expirar_historias_da_midia)
- Media +0xc interesses de transferência (25 bytes: +0 id do jogador, +4 id do clube, +8 data, +0x10 boato saiu, +0x11 valor até 50.000.000, +0x15 avaliação × 100), +0x10 capacidade, +0x14 conta; grava (acrescentar_interesse_de_transferencia)
- chave de ids (+0 id, +4 id ou −1 coringa); lê (ordem_das_chaves_de_ids)
- Media +0 registro de histórias (+8, +0xc sujeitos, +0x11 subtipo, +0x13 tipo); lê (ordem_das_historias)

**Interessa a:** CMRTE, CMScout, SGE, GPF2
