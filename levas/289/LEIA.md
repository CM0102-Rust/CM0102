# 289ª leva — as regras de empréstimo da Finlândia: o período, o prazo e o outono sem devolução; o período do empréstimo e a regra de empréstimo da Irlanda do Norte (também Dinamarca, Coreia e País de Gales); o prazo dos empréstimos da Noruega (também Holanda); encerrar um empréstimo na Alemanha e na Irlanda; a conta de empréstimos da temporada na Irlanda

**Rotinas:**
- `finland_rules_get_loan_period()` = 0x5a9a80 (finland_rules.cpp, thiscall, ret 0x1c): regra: o empréstimo para um clube finlandês vai do mês seguinte ao mês do fim da temporada da nação do clube, neste ano se esse mês ainda está à frente do início, senão no ano seguinte (o mesmo resultado da regra base);
- `finland_rules_can_loan_player()` = 0x5a9af0 (finland_rules.cpp, thiscall, ret 0x10): regra: o clube finlandês só pega emprestado com a janela finlandesa aberta hoje (16 de novembro a 15 de agosto; fora dela, o motivo 9 e "The deadline for loans to Finnish clubs has passed."); o clube da IA, além disso, só entre os dias 90 e 300 do ano (1º de abril a 28 de outubro), recusando sem motivo nem texto;
- `finland_rules_can_end_loan()` = 0x5a9ba0 (finland_rules.cpp, thiscall, ret 0x14): regra: o empréstimo com um clube finlandês não termina (nem devolvido, nem chamado de volta) numa data estritamente entre 18 de agosto e 15 de novembro do ano da data: "This player cannot be sent back/recalled until 15th November";
- `northern_ireland_rules_get_loan_period()` = 0x793870 (northern_ireland_rules.cpp, thiscall, ret 0x1c): regra: o empréstimo para um clube norte-irlandês vai do mês de hoje + 3 ao mês do fim da temporada da nação do clube (no ano do início se ainda está à frente dele, senão no ano seguinte a hoje); o início depois do fim vira o fim;
- `northern_ireland_rules_can_loan_player()` = 0x793900 (northern_ireland_rules.cpp, thiscall, ret 0x10): regra: um clube da Dinamarca, da Coreia, da Irlanda do Norte ou do País de Gales sempre pode pegar emprestado (sem janela nem limite); só guarda o contrato principal do jogador;
- `norway_rules_can_loan_player()` = 0x794760 (norway_rules.cpp, thiscall, ret 0x10): regra: um clube da Noruega ou da Holanda só pega emprestado com uma janela da sua nação aberta hoje; fora dela, o motivo 9 e "The deadline for loans to ... clubs has passed.";
- `germany_rules_can_end_loan()` = 0x5e0210 (germany_rules.cpp, thiscall, ret 0x14): regra: o empréstimo entre clubes alemães não termina antes da hora pela tela (flag 0): "Players on loan from other German clubs cannot be sent back until the end of the loan." para quem pegou emprestado, "Players on loan to other German clubs cannot be recalled." para o dono; com a flag (a conferência do empréstimo e o pedido da IA), sempre pode;
- `ireland_rules_can_end_loan()` = 0x63c530 (ireland_rules.cpp, thiscall, ret 0x14): regra: o empréstimo com um clube irlandês só termina (devolvido ou chamado de volta) numa data dentro da janela irlandesa (10 de maio a 31 de janeiro); fora dela, "Players cannot be sent back/recalled ... until after the Irish transfer deadline";
- `ireland_rules_record_loan()` = 0x63c4e0 (ireland_rules.cpp, thiscall, ret 4): regra: cada empréstimo assinado por um clube irlandês soma 1 na conta da temporada dele (o limite de 5 da regra irlandesa; zerada em 20 de junho);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (7):** 7 acusadas.
- **Jogo real comparando até 2002-01-01:** adiado para a comparação acumulada da auditoria (decisão de 05/11/2026).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 cc7f6ccb, original d77c6c14 (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 288ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 289`

**Campos tocados:**
- regras de transferência +0xa (NATION_ID): a nação das regras, para o adjetivo do texto; ponteiro; lê
- staff +0x39 (club_job): o clube do jogador, para escolher "devolver" ou "chamar de volta"; ponteiro; lê
- [0xae2c90] (G_CURRENT_DATE): o dia do ano de hoje (palavra com sinal, 0 = 1º de janeiro); lê
- regras de transferência +0xa (NATION_ID): a nação das regras, para o adjetivo do texto; ponteiro; lê
- [0xae1080] (G_CONTRACT_LOOKUP_RESULT): o contrato principal do jogador, guardado pela regra norte-irlandesa; escreve
- staff +0x39 (club_job): o clube do jogador, a nação dele (a alemã) e "devolver" ou "chamar de volta"; ponteiro; lê
- [[0xde1f54]+0x91c] + id·16 (SEASON_LOAN_COUNTS): o byte 0, empréstimos do clube na temporada; soma 1

**Interessa a:** Tweaks
