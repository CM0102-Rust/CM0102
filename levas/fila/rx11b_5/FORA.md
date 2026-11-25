# rx11b: o que ficou de fora (leitor rx11b, rodada 6, 25/11/2026)

Sem `rotinas.py`: esta pasta só guarda as notas. Das 11 funções da lista `rx11b.txt`, 8 viraram os itens rx11b_1 a
rx11b_4. As outras 3 ficaram para a próxima rodada, por tempo; nenhuma depende de decisão. As notas abaixo vêm da
desmontagem desta rodada.

## `Shortlist_Manager::club_wants_player` 0x843800 (1,5 KB): TELAS

- **Tipo:** as duas chamadas são de telas. A 0x47c0b0 é `open_add_part_exchange_player_screen`, o jogador oferecido em
  troca numa proposta do humano. A 0x8c8090 é `ai_respond_to_bid`, quando a IA responde a uma proposta do humano com
  jogador em troca. Por isso, 0 chamadas na carreira automática.
- **Caminho no jogo:** fazer uma proposta por um jogador da IA oferecendo um jogador do próprio elenco em troca.
- **Convenção:** thiscall(jogador, clube, valor, gravar) ret 0x10.
- **Corpo:** é quase o do irmão `try_add_player_to_club_shortlist` (0x842e10, já Rust em `shortlist_manager.rs`, com
  os auxiliares `shortlist_offer_margin` e `shortlist_offer_injury_penalty`, o mesmo x87 de 1,0 − a/b limitado a
  0,01..3,0, x 1/3 + 0,67):
  - o contrato do jogador vai para `[0xae1080]`, efeito colateral como no irmão;
  - a máscara de posição (0x5b7d20) ganha os bits derivados 2→4, 8→0x10, 0x20→0x10, 0x80→0x100 e 0x800→0x400;
  - a entrada do clube é `[this+0xdc]` + id x 9: byte +4 = 1 e byte +8 < 100;
  - o par (jogador, clube) é procurado pelo `bsearch` (0x946464, comparador 0x757ea0) e devolve 1 de cara;
  - vêm depois o visto (0x84b180), o `can_club_bid_for_player` (0x8ce990) e a vaga `[esi]` (0x25 bytes);
  - status 5: até 24 anos; nação: reputação 5000 e a reputação vista pelo técnico ≥ `utils_random`(1000) + 500 (há
    sorteio);
  - por fim, os dois cálculos de valor (0x840d80) e os preços pedidos (0x4d42d0 para humano e 0x4d1600 para IA; a
    segunda metade chama 0x5a0590 e 0x4d0c60, a conferir), a lesão (0x62ed70), o interesse (0x846920, comparado com
    `[0x9690a8]`) e, com gravar, 0x843e10 e 0x8423f0.
- **Comparador:** lado a lado só quando gravar = 0. Gravar mexe na lista de alvos: só a original, com o plano.

## `Shortlist_Manager::load` 0x83a9d0 (2,4 KB): SAVE com CARGA

- **Convenção:** thiscall(save) ret 4, chamada pelo construtor 0x83a730 (sx20_5) quando há save.
- **O que carrega:** os vetores de pares, as listas de posição e a tabela de 40 vagas de cada clube, convertendo os
  registros de vaga dos saves antigos.
- Não li o corpo nesta rodada.

## `Tactics::read_from_save` 0x8985c0 (4 KB): SAVE com CARGA

- **Convenção:** thiscall(caminho) ret 4, chamada por 0x893eff (gra_23).
- **O que carrega:** o tactics.dat do save para os blocos da IA, as predefinições, os registros dos clubes e as
  estatísticas, com o ramo de atualização dos saves antigos pela versão.
- Não li o corpo nesta rodada.
