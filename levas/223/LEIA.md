# 223ª leva — a ordem do histórico de competições, o vetor de atributos do jogador e se a nação é britânica ou a Irlanda; se o clube é basco e se a pessoa está na lista de transferências do clube ou na de empréstimos; se a pessoa está na lista a pedido, a nota de um atributo relativo e se o clube pode ter time reserva

**Rotinas:**
- `ordem_do_historico_de_competicao()` = 0x540780 (Database.cpp, cdecl): comparação do qsort do histórico de competições: sem competição no fim, pelo id da competição e pela temporada da mais nova para a mais velha;
- `vetor_de_atributos_do_jogador()` = 0x53a610 (Database.cpp, cdecl): estende posições, lados e atributos do jogador num vetor de 63 células (valor e peso) e devolve os fins dos grupos (9, 0xc, 0x20);
- `nacao_britanica_ou_irlanda()` = 0x5446d0 (Database.cpp, cdecl): se a nação é Inglaterra, Escócia, Irlanda, País de Gales ou Irlanda do Norte;
- `clube_basco()` = 0x53d7f0 (Database.cpp, cdecl): se o clube é da Espanha e um dos 23 clubes bascos citados no código;
- `pessoa_listada_pelo_clube()` = 0x5444a0 (Database.cpp, cdecl): se o vínculo da pessoa com o clube (ou o do contrato) está na lista de transferências pelo clube; função 0xf em clube sem técnico humano não conta;
- `pessoa_listada_para_emprestimo()` = 0x544590 (Database.cpp, cdecl): se o vínculo da pessoa com o clube (ou o do contrato) está na lista de empréstimos;
- `pessoa_listada_a_pedido()` = 0x544600 (Database.cpp, cdecl): se o vínculo da pessoa com o clube (ou o do contrato) está na lista de transferências a pedido;
- `nota_relativa_do_atributo()` = 0x544380 (Database.cpp, cdecl): a nota de 1 a 20 de 2 × byte do atributo + habilidade atual do jogador;
- `clube_pode_ter_reservas()` = 0x542740 (Database.cpp, cdecl): se o clube pode ter time reserva: não para seleções, pares A/B, clubes de Espanha, Alemanha e Portugal e algumas segundas seleções;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (9):** 9 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 222ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 223`

**Campos tocados:**
- histórico de competição +0x04: competição; ponteiro; lê (ordem_do_historico_de_competicao)
- histórico de competição +0x08 [2]: temporada (sem nome; −1 = sem); valor; lê (ordem_do_historico_de_competicao)
- competição +0x00: id; id; lê (ordem_do_historico_de_competicao)
- jogador +0x0f..+0x1a: posições e lados (sem nome em CAMPOS.md; +0x0f goleiro); valor; lê (vetor_de_atributos_do_jogador)
- jogador +0x1b..+0x44: atributos (aceleração +0x1b, antecipação +0x1e, regularidade +0x21, drible +0x26, cabeceio +0x2b, impulsão +0x2e, liderança +0x2f, pé esquerdo +0x30, chute de longe +0x31, marcação +0x32, movimentação sem bola +0x33, mano a mano +0x35, velocidade +0x36, passe +0x37, pé direito +0x3b, resistência +0x3c, desarme +0x3e, trabalho em equipe +0x3f, técnica +0x40, criatividade +0x43, disposição +0x44; os demais sem nome); valor; lê (vetor_de_atributos_do_jogador)
- pessoa +0x58: sem nome (o atributo 10 de 0x543540); valor; lê (vetor_de_atributos_do_jogador)
- nação +0x00: id; id; lê (nacao_britanica_ou_irlanda: comparado com [0x9cf2e4] Inglaterra, [0x9cf45c] Escócia, [0x9cf2dc] Irlanda, [0x9cf510] País de Gales, [0x9cf3e4] Irlanda do Norte)
- clube +0x00: id; id; lê (clube_basco: comparado com 23 clubes de [0x9d0120]..[0x9d0338])
- clube +0x53: nação; ponteiro; lê (clube_basco)
- nação +0x00: id; id; lê (clube_basco: comparado com [0x9cf488] Espanha)
- pessoa +0x3d: função no clube (sem nome em CAMPOS.md; 0xf conta só em clube de técnico humano); valor; lê (pessoa_listada_pelo_clube)
- contrato (0x4cf5a0) +4: id do clube; id; lê (pessoa_listada_pelo_clube, pessoa_listada_para_emprestimo)
- vínculo pessoa-clube (0x4cf5f0) +0x4e: marcas (1 lista de transferências, 2 lista de empréstimos, 8 a pedido); valor; lê (pessoa_listada_pelo_clube: 1; pessoa_listada_para_emprestimo: 2)
- [0xae1080] último contrato consultado; ponteiro; grava (pessoa_listada_pelo_clube, pessoa_listada_para_emprestimo, sem clube)
- contrato (0x4cf5a0) +4: id do clube; id; lê (pessoa_listada_a_pedido)
- vínculo pessoa-clube (0x4cf5f0) +0x4e: marcas (8 a pedido); valor; lê (pessoa_listada_a_pedido)
- [0xae1080] último contrato consultado; ponteiro; grava (pessoa_listada_a_pedido, sem clube)
- jogador +0x05 [2]: habilidade atual; valor; lê (nota_relativa_do_atributo)
- clube +0x00: id (≥ [0xae235c] − [0xae2350] seleção; ≥ [0xae235c] − 2 × [0xae2350] segunda seleção); id; lê (clube_pode_ter_reservas)
- clube +0x53: nação; ponteiro; lê (clube_pode_ter_reservas)
- clube +0x244: sem nome (marca de parceiro A/B, zerada por 0x540a50 ao recusar); valor; lê e pode zerar (clube_pode_ter_reservas, por 0x540a50)
- nação +0x00: id; id; lê (clube_pode_ter_reservas: comparado com [0x9cf488] Espanha, [0x9cf314] Alemanha, [0x9cf434] Portugal, [0x9cf408] Noruega, [0x9cf360], [0x9cf260] Brasil, [0x9cf218] Argentina)
- nação +0x71: continente; ponteiro; lê (clube_pode_ter_reservas: [0x9cfa18] e [0x9cfa08])
- [0xae235c] / [0xae2350] total de clubes / nações; valor; lê (clube_pode_ter_reservas)

**Interessa a:** CMScout, CMRTE
