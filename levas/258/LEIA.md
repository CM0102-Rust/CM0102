# 258ª leva — a oferta passada aos clubes, a notícia da oferta aceita e a do empréstimo oferecido; a recusa dos termos pelo jogador, a notícia da confiança da diretoria e a das expectativas da diretoria; as ofertas em espera no fechamento da janela, a notícia das expectativas da federação e a do protesto da torcida; a negociação guardada da oferta, o boato do técnico em outro clube e a recusa do jogador em se mudar; as receitas da semana e o centro de treino: o merchandising, os ingressos da temporada e a obra do centro de treino; os eventos financeiros da semana: o passe dos eventos do clube, a diretoria que paga a dívida e a injeção de dinheiro; os salários do pessoal e os bônus: o teto salarial do pessoal, o pagamento dos bônus e o ajuste do teto salarial; o físico depois da partida: o ritmo de recuperação de cada jogador e as batidas que viram lesão, a batida do jogador na partida e o jogador de férias; o regen no clube: a vaga no elenco, o valor e o contrato do jovem; a aposentadoria pela lesão e a notícia da aposentadoria anunciada; o fim da lesão do jogador, a notícia do pedido de desculpas e a caixa de entrada esvaziada; o convocado machucado: tirar da seleção, a notícia ao clube dele e o pedido de substituto; a convocação pode mudar agora?, o jogador está na convocação e o adversário marcado no amistoso; o prêmio do mês no histórico (o técnico conta, o jogador agradece ou renova), a faixa de reputação e a folga do amistoso

**Rotinas:**
- `passar_a_oferta_aos_clubes()` = 0x8c2f50 (transfer_manager.cpp, thiscall): com todos os jogadores de acordo, cancela as ofertas rivais (reagendadas em 20 dias), espera a permissão de trabalho (18 a 24 dias) ou passa a oferta à decisão do comprador e do vendedor;
- `noticia_da_oferta_aceita()` = 0x8d76d0 (transfer_manager.cpp, thiscall): a notícia 0xfa3 da oferta aceita aos clubes humanos envolvidos;
- `noticia_do_emprestimo_oferecido()` = 0x8e1780 (transfer_manager.cpp, thiscall): a notícia 0xfb5 da oferta de empréstimo aos clubes humanos envolvidos; sem contrato, a caixa de erro;
- `recusa_dos_termos()` = 0x8c28c0 (transfer_manager.cpp, thiscall): o jogador recusa os termos: a oferta volta em 10 dias, saem as marcas de lance, a reputação dele sobe se o comprador é mais famoso; ou ele sai da troca e o valor passa ao preço;
- `noticia_da_confianca_da_diretoria()` = 0x6a4c40 (manager_manager.cpp, thiscall): a notícia 0x1772 da confiança da diretoria com os dias do técnico no cargo;
- `noticia_das_expectativas_da_diretoria()` = 0x6a4a20 (manager_manager.cpp, thiscall): a notícia 0x1771 das expectativas da diretoria para o técnico do clube;
- `ofertas_em_espera_na_janela()` = 0x8c4e10 (transfer_manager.cpp, thiscall): na janela que fecha, completa as ofertas aprovadas e põe em espera as da nação: notícia se há humano, senão retirada ou recusada; do computador, status 3 e de volta no dia seguinte;
- `noticia_das_expectativas_da_federacao()` = 0x6a44a0 (manager_manager.cpp, thiscall): a notícia 0x177e da expectativa da federação para a seleção numa competição;
- `noticia_do_protesto_da_torcida()` = 0x6a5d10 (manager_manager.cpp, thiscall): a notícia 0x1782 do protesto da torcida contra o técnico;
- `guardar_a_negociacao()` = 0x8be9d0 (transfer_manager.cpp, thiscall): guarda os termos da oferta numa negociação nova (o vetor cresce de 50; o id seguinte) e agenda a oferta em 2 dias, ou em 14 com notícia para o comprador humano;
- `noticia_do_boato_do_tecnico()` = 0x69fd50 (manager_manager.cpp, thiscall): com um sorteio em quatro, a notícia 0x1776 do boato que liga um técnico a outro clube;
- `noticia_da_recusa_em_se_mudar()` = 0x8d8310 (transfer_manager.cpp, thiscall): a notícia 0xfae do jogador que recusa a mudança, com o motivo e os técnicos dos dois clubes;
- `receita_do_merchandising()` = 0x59b7e0 (finance.cpp, thiscall, ret 4): público × reputação × fator (pelo estádio do clube de +0xbf ou pela reputação; Japão um degrau acima), × (sorteio(11) + 95)%, somado em +0xbc, +0x15c e no saldo;
- `ingressos_da_temporada()` = 0x599a20 (finance.cpp, thiscall, ret 4): estádio mínimo de 1500 lugares; a procura pelo valor no gerente de técnicos, público e reputação, até 87,5% dos lugares; os novos vendidos pela fração do passo (+0x164) e sorteio; razão em +0x160; receita (com aplicar) pelo preço, tipo da nação e clubes-chave; a loja com dois sorteios pela reputação;
- `melhorar_o_centro_de_treino()` = 0x59b630 (finance.cpp, thiscall, ret 4): o nível do centro de treino (+0x7f) sobe pela reputação (até 20) e cada degrau cobra a obra em +0x8c, +0x12c e no saldo;
- `eventos_financeiros_da_semana()` = 0x59ed70 (finance.cpp, thiscall, ret 8): pela faixa das finanças: encerra ou abre a crise (+0x165), move o aperto (+0x166), tenta o impulso, a dívida, a injeção e a troca da diretoria, desgasta o centro de treino; com aperto acima de 50, tira o aquecimento ou fecha 10% dos lugares;
- `diretoria_paga_a_divida()` = 0x59dd40 (finance.cpp, thiscall, ret 8): clube pobre (faixa −2/−1, ou 0 com saldo baixo): três sorteios; a diretoria paga 450, 300 ou 150 × reputação e manda a notícia;
- `injecao_de_dinheiro()` = 0x59e5a0 (finance.cpp, thiscall, ret 8): clube de reputação 4750 fora de Gales e das Irlandas: sorteios pela faixa e pelo estádio do clube de +0xbf; o valor pela reputação, até 10% da dívida e 4 milhões; notícia e fim da crise;
- `teto_salarial_do_pessoal()` = 0x597db0 (finance.cpp, thiscall, ret 4): o maior salário oferecido ao pessoal pelo cargo: padrão pelo cargo, nível da nação (+0x85) / tipo, ajustes das nações-chave, peso da liga × 0,2, faixa das finanças, teto do clube (+0x14) e faixa do cargo;
- `pagar_os_bonus()` = 0x599960 (finance.cpp, thiscall, ret 4): soma o valor nas despesas de bônus (+0x138 e +0x98) e o tira do saldo;
- `ajustar_o_teto_salarial()` = 0x598340 (finance.cpp, thiscall, ret 4): teto salarial (+0x14) × (percentual + 100)%, truncado;
- `efeitos_fisicos_da_partida()` = 0x62c080 (injury.cpp, thiscall, ret 8): para as 20 vagas de cada lado da partida: o ritmo (nível da vaga mais (10000 − condição) / 100; metade no jogo sem a marca +0x40; × 1,25 acima de 33 anos, o dobro abaixo de 23, com sorteio) vai para a recuperação (0x62bd10); as batidas da pessoa na lista da partida viram lesão (0x62d140, 0x62e700, 0x62d250);
- `batida_do_jogador()` = 0x62bf60 (injury.cpp, thiscall, ret 4): o item da lista das batidas da partida (+0x28) com o id da pessoa, ou 0;
- `jogador_de_ferias()` = 0x631550 (injury.cpp, thiscall, ret 4 (this não é lido)): 1 se a lesão atual da pessoa (0x62ed70 no [0xb63d28]) é de férias (tipo 0x51);
- `contratar_o_regen()` = 0x7a6f10 (player_regen.cpp, stdcall, ret 8): põe o regen na primeira vaga livre das 50 do elenco, com o valor (0x4d0c60; até 5000 no clube de +0x64 = 3), o fim do contrato (abaixo de 19 anos, daqui a dois anos; senão a sorteio(6) + 1) no mês do fim da temporada e dia sorteado, o contrato, o salário padrão e, fora do humano, o fim até daqui a cinco anos; devolve 1 (0 sem vaga);
- `aposentar_pela_lesao()` = 0x7a7f20 (player_regen.cpp, thiscall, ret 8): tira a aposentadoria pendente da pessoa e acrescenta a nova, pela lesão (motivo 8), daqui a sorteio(dias / 2) + 1 dias;
- `noticia_da_aposentadoria_anunciada()` = 0x7aa340 (player_regen.cpp, stdcall, ret 8): monta a notícia 0xbbb (id, nomes, motivo, clube do contrato, idade) e manda aos técnicos dos clubes interessados;
- `fim_da_lesao()` = 0x62e700 (injury.cpp, thiscall, ret 8): encerra a lesão atual da pessoa: o histórico (fora das férias), a notícia da volta no clube humano (com o pedido), o tipo da entrada 0xff e o +0xe do registro −1;
- `noticia_do_pedido_de_desculpas()` = 0x72bbf0 (media.cpp, stdcall, ret 0xc): monta a notícia 0xbdb (jogador, nomes, clube, história e a variante sorteada em algumas histórias) e manda ao técnico do clube;
- `esvaziar_caixa_de_entrada()` = 0x787630 (news.cpp, thiscall, ret 4): dá as respostas padrão do clube e da seleção do técnico (0x7874b0) e esvazia a caixa dele (as 100 notícias com +0xd6 = 0 e +0xda = 1);
- `tirar_convocado_machucado()` = 0x775cd0 (national_teams.cpp, thiscall, ret 8): com 14 dias de lesão ou mais, tira o convocado da tática e do elenco da seleção; antes da data do tipo, avisa o clube, abre a vaga e convoca o substituto ou pede ao técnico humano;
- `noticia_da_saida_do_convocado()` = 0x76f7d0 (national_teams.cpp, thiscall, ret 0xc): monta a notícia 0x20 (o jogador saiu da seleção machucado) e entrega ao clube dele e ao do contrato quando são de técnico humano;
- `noticia_do_pedido_de_substituto()` = 0x7701f0 (national_teams.cpp, thiscall, ret 0x10): monta a notícia 0x22 (é preciso um substituto) com o time, o elenco, o nível e o tipo, e entrega ao técnico humano da seleção;
- `pode_mudar_a_convocacao()` = 0x76db00 (national_teams.cpp, thiscall, ret 0x18): diz se o jogador pode entrar ou sair da convocação agora (aposentado, de outra nação, elenco cheio, machucado, janela do tipo, data e jogo do dia, técnico humano) e escreve o motivo;
- `esta_na_convocacao()` = 0x76da80 (national_teams.cpp, stdcall, ret 0xc): o jogador está no time do tipo (principal, segundo ou sub-21) com a marca do tipo?;
- `marcar_adversario_do_amistoso()` = 0x5c57d0 (friendly.cpp, thiscall, ret 8): grava o adversário na primeira vaga vazia da agenda de amistosos do clube e marca o pedido;
- `registrar_premio_do_mes()` = 0x86c8c0 (staff_records.cpp, thiscall, ret 0x14): grava a entrada 0x33 do histórico do premiado; o técnico conta +1 no registro dele; o jogador pode ter a renovação do contrato e, por sorteio, agradece ao técnico (notícia);
- `faixa_de_reputacao_do_amistoso()` = 0x5c52d0 (friendly.cpp, stdcall, ret 4): a faixa 1..5 da reputação do clube (2000, 4000, 6000 e 8000);
- `folga_do_amistoso()` = 0x5c74d0 (friendly.cpp, stdcall, ret 4): os dias de folga em volta do amistoso pelo tipo: 2 nos tipos 1 e 10, 3 no 2, senão 1;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (36):** 32 acusadas. `cpv3_status_1` escapou: as ofertas em espera no fechamento da janela (37 vezes) voltam à fila igual com o status 1 ou 3 até 2002-06-01 [inferência]. `cpv3_volta_em_3` escapou: a data de volta das ofertas em espera cai antes da próxima janela nos dois casos, sem mudar quem negocia [inferência]. `cpv7_sem_clube_grande` escapou: o aumento de 5% do teto salarial dos clubes grandes só pesa quando um deles procura pessoal, o que não decidiu nenhuma contratação até 2002-06-01 [inferência]. `cpv13_premio_em_dobro` escapou: a conta de prêmios do técnico no registro de pontos (+0x9e) não entra na impressão, e o prêmio do técnico não saiu até a data da confirmação comparando (o agente já previa) [inferência]. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (7c7b5d9a).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 9b926687, original 49f5dc41 (registrada; não barra a leva desde 31/10/2026).

**Refazer**, sobre o commit da 257ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 258`

**Campos tocados:**
- oferta +0: lista das ofertas pelo mesmo jogador (índices); +4: lista dos jogadores da troca; ponteiro; lê (passar_a_oferta_aos_clubes)
- oferta +8: clube comprador; +0xc: jogador; id; lê (as três)
- oferta +0x10: estado dos termos (2 = de acordo); +0x20: índice no vetor; +0x2f: confirmação direta; valor; lê (passar_a_oferta_aos_clubes)
- oferta +0x2c: status (0xa espera da permissão, 0xb com os clubes, 0xf cancelada); +0x2e: tentativas (3 nas rivais); valor; lê e grava (passar_a_oferta_aos_clubes)
- oferta +0x30: marcas (& 0xf: em espera); valor; lê (passar_a_oferta_aos_clubes)
- oferta +0x1f, +0x20, +0x24, +4; termos +0xc, +0x2d, +0x2f; valor; lê (as notícias)
- contrato +4: clube; id; +0x35 & 0x3f: tipo (5 sem clube); +0xc: salário; lê (as três)
- [0xae1080] o último contrato pedido; ponteiro; grava (as três)
- oferta +0x10: estado dos termos (4 = recusados); +0x2c: status (0xf); +0x2e: tentativas; valor; grava (recusa_dos_termos)
- oferta +0x1b: preço; valor; lê e grava (recusa_dos_termos)
- oferta +4: lista dos jogadores da troca (liberada se vazia); ponteiro; lê e grava (recusa_dos_termos)
- [this+0x86e] registro de 8 bytes por jogador: bit 0 de +0 (em negociação), +2 (ofertas); valor; grava (recusa_dos_termos)
- clube +0x80: reputação; valor; lê (recusa_dos_termos)
- ficha do jogador (pessoa +0x61) +0xb: reputação atual (até 10000); valor; lê e grava (recusa_dos_termos)
- clube +0xcf: técnico; ponteiro; técnico +0x3e: data de chegada (dia, ano +0x40); lê (noticia_da_confianca_da_diretoria)
- [0xdd6ec8] data a partir da qual saem as notícias (dia, ano +2); valor; lê (as duas notícias)
- [0xae1080] o último contrato pedido; ponteiro; grava (recusa_dos_termos)
- [this+0x8ac] as 28 regras de transferência por nação (+0xa: nação; virtual +0xc: segurar a oferta); ponteiro; lê (ofertas_em_espera_na_janela)
- oferta +0x2c: status (0xe completar, 0x13 fora, 1 em espera, 3 reagendada); valor; lê e grava (ofertas_em_espera_na_janela)
- oferta +0x30: marcas (bit 0 em espera, bits 1..3 limpos); +0x10: termos (zerados); valor; grava (ofertas_em_espera_na_janela)
- oferta +4: lista da troca (item +4: estado, zerado); +0x20: vaga; lê e grava (ofertas_em_espera_na_janela)
- clube +0x53: nação; ponteiro; lê (ofertas_em_espera_na_janela)
- pessoa +0x61: ficha de jogador; ponteiro; lê (ofertas_em_espera_na_janela)
- clube +0xcf: técnico; ponteiro; lê (noticia_do_protesto_da_torcida)
- [0xdd6ec8] data a partir da qual saem as notícias; valor; lê (noticia_do_protesto_da_torcida)
- [0xae1080] o último contrato pedido; ponteiro; grava (ofertas_em_espera_na_janela)
- transferências +0xc: vetor das negociações (capacidade +0x10, conta +0x14); +0x85a: id seguinte; ponteiro e valor; lê e grava (guardar_a_negociacao)
- oferta +0x28: vaga da negociação (−1 sem); +0x2c: status (1); +0x2d: marcada; valor; lê e grava (guardar_a_negociacao)
- negociação (0x41 bytes): cópia dos termos +0..+0x40 (data +0x12; 4 bits baixos de +0x30 e +0x31), +0x20 da oferta, +0x24 id, +4 lista nova; grava (guardar_a_negociacao)
- pessoa +0x39: clube; +0x3d: cargo; lê (noticia_do_boato_do_tecnico)
- clube +0xcf: técnico; ponteiro; lê (noticia_da_recusa_em_se_mudar)
- [0xdd6ec8] data a partir da qual saem as notícias; valor; lê (noticia_do_boato_do_tecnico)
- [0xae1080] o último contrato pedido; ponteiro; grava (noticia_da_recusa_em_se_mudar)
- finanças do clube (0x167 bytes em [[0xae2a38]]) +0 saldo (i64), +0xbc e +0x15c receita da loja, +0xac e +0x14c receita dos ingressos, +0x8c e +0x12c obras; valores; grava (as três)
- finanças do clube +8 id do clube, +0x18 ingressos da temporada vendidos, +0x160 razão vendidos/lugares (f32), +0x164 passo da venda; valores; lê e grava (ingressos_da_temporada)
- clube +0x53 nação, +0x57 liga, +0x69 estádio, +0x6d byte, +0x73 público, +0x80 reputação, +0xbf clube ligado; ponteiros/valores; lê (receita_do_merchandising, ingressos_da_temporada)
- clube +0x7f nível do centro de treino; valor; lê e grava (melhorar_o_centro_de_treino)
- estádio +0xf nível, +0x3c lugares; valores; lê e grava os lugares abaixo de 1500 (ingressos_da_temporada)
- nação +0 id (Japão 0x9cf36c, Alemanha 0x9cf314), +0x7e tipo; valores; lê (receita_do_merchandising, ingressos_da_temporada)
- [[0xb67a44]] + 73·id + 8: valor do clube no gerente de técnicos (i16); lê (ingressos_da_temporada)
- finanças do clube (0x167 bytes) +0 saldo (i64), +0xb4 e +0x154 dinheiro recebido da diretoria; valores; lê e grava (diretoria_paga_a_divida, injecao_de_dinheiro)
- finanças do clube +0x165 estado (0 normal, 1 crise, 2 aperto), +0x166 contador de aperto (i8); valores; lê e grava (eventos_financeiros_da_semana; a injeção grava 0 no estado)
- clube +0x53 nação, +0x57 liga, +0x69 estádio, +0x6d byte, +0x7f centro de treino, +0x80 reputação, +0x82 marca, +0xbf clube ligado; ponteiros/valores; lê, grava +0x7f e +0x82 (eventos_financeiros_da_semana)
- estádio +0xf nível, +0x20 nível, +0x3c lugares, +0x40 resto, +0x44 lugares fechados, +0x4d aquecimento; valores; lê e grava (eventos_financeiros_da_semana; lê +0xf/+0x20 nas outras)
- jogador +0x52 valor; contrato +0x4e bit 4; [0xae1080] último contrato; valores; grava (eventos_financeiros_da_semana, crise do clube humano)
- [0xadadfc] ligas ativas (dword por id); valor; lê (eventos_financeiros_da_semana)
- finanças do clube (0x167 bytes) +8 id do clube, +0x14 teto salarial; valores; lê (teto_salarial_do_pessoal), lê e grava o teto (ajustar_o_teto_salarial)
- finanças do clube +0 saldo (i64), +0x98 e +0x138 despesas de bônus; valores; grava (pagar_os_bonus)
- clube +0x53 nação, +0x57 liga, +0x64 tipo, +0x80 reputação; ponteiros/valores; lê (teto_salarial_do_pessoal)
- nação +0 id (nações-chave 0x9cf244..0x9cf4f8), +0x7e tipo, +0x85 nível dos salários; valores; lê (teto_salarial_do_pessoal)
- liga +0x69 peso (i16); valor; lê (teto_salarial_do_pessoal)
- partida +0x1c0 + 0x625 × lado + 0x49 × vaga: −0x30 id da pessoa; −0x11 marca da batida; +0, +2, +4 níveis (0xff = nenhum); +6 condição (16 bits); id e valor; lê (efeitos_fisicos_da_partida)
- partida +0x4c: marca (nível 90 nas onze primeiras); +0x4d: nível padrão; valor; lê (efeitos_fisicos_da_partida)
- jogo +0x40: marca do lado da casa (sem ela, o ritmo pela metade); +0: id (−2 sem jogo para a lesão); valor; lê (efeitos_fisicos_da_partida)
- pessoa +0x18: idade; +0x61: ficha (+0xf nota com sinal, 15 ou mais para a batida de tipo 7); valor; lê (efeitos_fisicos_da_partida)
- Injury +0x28: lista das batidas da partida (item: +0 id, +4 tipo, +5 gravidade; cursor +0xa recomeça de +2); ponteiro, id e valor; lê e grava o cursor (efeitos_fisicos_da_partida, batida_do_jogador)
- [0xb63d28] Injury; tabela de lesões de 0x30 bytes (+0 tipo, 0x51 férias); ponteiro e valor; lê (jogador_de_ferias)
- clube +0xd7: as 50 vagas do elenco (ponteiro de pessoa, 0 livre); ponteiro; lê e grava (contratar_o_regen)
- clube +0x64: estatuto (3: valor do regen até 5000); valor; lê (contratar_o_regen)
- pessoa +0x39: clube; +0x4e: 0; +0x52: valor; +0x46: fim do contrato (data de 8 bytes); +0x18: idade; ponteiro e valor; grava, lê a idade (contratar_o_regen)
- contrato +0x2d/+0x2f: fim (dia e ano, até o ano + 5 fora do humano); +0x3a: 0; +0x35 e +0x4f: bits do clube humano; valor; grava (contratar_o_regen)
- PLAYER_REGEN +4: aposentadorias pendentes (26 bytes: +0 id, +4 clube, +8 e +0x10 data, +0x18 motivo 8, +0x19 0), +8 conta, +0xc capacidade; id e valor; tira e acrescenta (aposentar_pela_lesao)
- [0xae1080] o último contrato lido; ponteiro; grava (noticia_da_aposentadoria_anunciada)
- Injury +0: registro de 31 bytes por pessoa (+0xe índice da lesão atual, −1 nenhuma; +0x14/+0x18 histórico); +4: tabela de lesões de 0x30 bytes (+0 tipo, 0xff livre, 0x51 férias); +0xc: arquivo do histórico; índice e valor; lê e grava (fim_da_lesao)
- pessoa +0x39: clube (humano: a notícia da volta); +0x24: seleção; +0x61: ficha; ponteiro; lê (fim_da_lesao, esvaziar_caixa_de_entrada, noticia_do_pedido_de_desculpas)
- News +8: caixas dos humanos (12 bytes: +0 primeira notícia, +4 cópia, +8 0); +0: notícias de 0xdf bytes (+0xd6 0, +0xda 1); índice e valor; grava (esvaziar_caixa_de_entrada)
- staff +0x24: o clube da seleção do convocado; ponteiro; lê (tirar_convocado_machucado; zerado pela 0x7762f0)
- staff +0x28: a marca de convocado (0xb); valor; lê (tirar_convocado_machucado)
- staff +0x5e: as marcas de elenco (bit 3 = tipo 1); valor; lê (tirar_convocado_machucado)
- staff +0x39: o clube do jogador; ponteiro; lê (noticia_da_saida_do_convocado)
- registro da nação nas seleções ([NationalTeams+4] + 0x6e × id) + 20 × tipo: +0x3a a data do tipo; valor; lê; +0x45 as vagas a preencher (byte); valor; lê e grava +1 (tirar_convocado_machucado)
- [NationalTeams+0] a lista dos clubes com data (id, ?, data em +8); ponteiro; lê e anda o cursor +0xa (tirar_convocado_machucado)
- lesão (0x62ed70) +0: o tipo da lesão (0x51 = doença: só sai); valor; lê (tirar_convocado_machucado, as notícias)
- clube +0xcf: o técnico do clube; ponteiro; lê (tirar_convocado_machucado)
- [0xae1080] o contrato achado por último; ponteiro; grava (noticia_da_saida_do_convocado)
- staff +0x1a: a nação do jogador; ponteiro; lê (pode_mudar_a_convocacao)
- staff +0x24: o clube da seleção; ponteiro; lê (pode_mudar_a_convocacao, esta_na_convocacao)
- staff +0x5e: as marcas de elenco (4 principal e sub-21, 8 o segundo); valor; lê (esta_na_convocacao)
- registro da nação nas seleções ([NationalTeams+4] + 0x6e × id) + 20 × tipo: +0x3a a data do tipo, +0x42 o teto do elenco, +0x44 a janela, +0x45 as vagas abertas; valor; lê (pode_mudar_a_convocacao)
- [NationalTeams+0] a lista dos times com data; ponteiro; lê e anda o cursor +0xa (pode_mudar_a_convocacao)
- agenda de amistosos +8: as vagas (byte); +0x19 + 25·i o adversário; +0x21 + 25·i a marca do pedido; ponteiro e valor; lê e grava (marcar_adversario_do_amistoso)
- histórico da pessoa (FILE_LLIST [Staff_Records], 0x58e340): a entrada 0x33 com a data, o clube ou −1 (+8), o id do prêmio (+0xd), a4 (+0x11) e a5 (+0x15); valor; grava (registrar_premio_do_mes)
- staff +0x3d e +0x28: o cargo (5 ou 0xc = técnico); valor; lê (registrar_premio_do_mes)
- registro de pontos do técnico (0x86f5b0, 0xd2 bytes) +0x9e: os prêmios do mês (palavra); valor; lê e grava +1 (registrar_premio_do_mes)
- staff +0x59 e +0x5b: os bytes que liberam o sorteio do agradecimento (acima de 15 e de 10); +0x59 é o teto do sorteio; valor; lê (registrar_premio_do_mes)
- contrato (0x4cf5f0) +0x25/+0x29: o começo (mais de 300 dias para a renovação); +0x45 a marca que barra o agradecimento; valor; lê (registrar_premio_do_mes)
- [0xae1080] o contrato achado por último; ponteiro; grava (registrar_premio_do_mes)
- clube +0x80: a reputação; valor; lê (faixa_de_reputacao_do_amistoso)

**Interessa a:** CMRTE, GPF2, SGE
