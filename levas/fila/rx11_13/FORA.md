# rx11: o que ficou de fora (leitor rx11, rodada 6, 25/11/2026)

Sem `rotinas.py`: esta pasta só guarda as notas. Dos 46 endereços da lista `rx11.txt`, 30 viraram os itens rx11_1 a
rx11_12. Os outros 16 estão abaixo.

## Fora de vez (não são rotinas a portar)

- **0x55f1a0 `thunk_area_release` e 0x55f1b0 `thunk_guio_free_images`:** um `jmp rel32` de 5 bytes cada, passados como
  destrutor de elemento ao `vector_destructor_iterator`, sem regra própria. Já decidido no `tel40_4/FORA.md`.
- **0x567000 e 0x567260:** não são entradas. O 0x567000 cai no meio de uma instrução (`00 66 8b ...`) e o 0x567260 é o
  `or ax, 0xffff` / epílogo de outro ramo do `Display::process_command_packet` 0x564080 (ver `tel46_3`, `tel26_2`).
- **0x9216d0 `WcSouthAmericanLeague::load_2002_campaign` e 0x922350 `add_fixed_fixture`:** código morto com o jogo
  começando em 2001 (a carga da campanha de 2002 nunca roda). Já decidido no `cx17_7/FORA.md`, que traz a prova e o
  caminho para quem pegar (um exe com o ano de início mudado).

## Não escritas nesta rodada (por tempo; nenhuma depende de decisão)

As notas abaixo vêm da desmontagem desta rodada; a próxima pode partir delas.

- **Physio, TELAS (pelo clique 0x7a2f10, o relatório do fisioterapeuta):**
  - 0x7a1a90 `physio_file_report_news` thiscall(fisio, jogador) ret 8, chamada por 0x7a3047: sem fisio ou jogador sem
    clube (+0x39), nada; a idade (0x54cdb0 em 0xae2c90 com +0x3e/+0x42); a faixa (bl: 1 com idade > 1000 e +0x59 > 10,
    senão 4 ou 2 por `idade >= 5 x (21 - (+0x56 com sinal)) + 0x5a`); 0x7a25c0(jogador), 0x7bd9d0 (cdecl, 4 args) e a
    avaliação 0x7a2650; o NewsItem 0x1392 (0x77bbe0 com 0) com 16 slots (0x785790: os ids [+0x33] de fisio e jogador,
    as bandeiras, a lesão recorrente 0x62ed70 (tipo e +0x2c) ou -1/-1) e a entrega 0x7861e0 em [0xb7431c] ao clube do
    fisio.
  - 0x7a2650 `physio_assess_player_fitness` thiscall(fisio, jogador, &bandeiras, &tipo) ret 0x10: grava 0 e 0xff nas
    saídas; ponto flutuante (0x968e00, _ftol) e efeitos colaterais: `[0xae1080]` (0x4cf5a0) e o bit 0 do byte +0xc do
    registro de 0x4f bytes do jogador em [0xae2d0c] (a marca do regen? conferir com `docs/MARCA_DO_REGEN.md`). Precisa
    de comparador que olhe esses dois.
  - 0x7a1cd0 `physio_write_report_news_text` (2,3 KB, quadro de 0x1160 bytes, `mov eax, 0x1160; call _chkstk`), o
    texto do relatório pelas bandeiras dos slots: `compare_text_writer`.
- **Staff_Records::format_staff_history_entry 0x86e0b0, TELAS** (4,6 KB, switch pelo byte de tipo, chamada por
  0x8864ea): rotina grande inteira num item, partida em pedaços por tipo (o molde do `gra_1`); comparador de texto.
- **ReserveComp, simulação (roda na carreira ENG; a marcação dos jogos reservas é diária):**
  - 0x7e6380 `schedule_reserve_fixtures` thiscall(data) ret 4: três LLIST locais (0x68d020/0x68d040), o dia da semana
    (0x54a3f0 = 4: a data + 2 por 0x54c5b0, senão a data - 2 por 0x54c8e0), para as 35 nações-chave da tabela 0xb63d60
    (0x48 bytes cada) com liga escolhida (+0x11c) e que não sejam a França: três passagens (byte +0x14) pelos jogos da
    data (0x5ae6a0 do FIXTURE_MANAGER [0xae2a58]); cada jogo de liga (0x4b0700) que passe em 0x7e6ba0 vira um jogo
    reserva: a cópia de 0x4f bytes do Fixture, os lados trocados fora da competição 0x9cf4dc, a competição reserva, as
    datas, os campos zerados, os clubes reservas (+0x6e ou +0x69 do clube, livres pela 0x5ae870), a marca 0x44 nas
    regras com a competição 0x9cf244, e 0x5aa680 (adiciona). Atenção à cópia do LLIST local em [esp+0x10..0x1e] (o
    `mov eax, [esp+0x12]; mov [esp+0x1a], eax` copia bytes de outro campo).
  - 0x7e6720 `add_b_or_under21_fixture` (848 bytes), 0x7e6f70 `add_result` (784 bytes, a notícia ao técnico humano) e
    0x7e7280 `reserve_result_news_text` (1,2 KB, `compare_text_writer`).
  - O construtor 0x7e5fc0 está no rx11_12.
- **Shortlist_Manager, simulação e save:**
  - 0x843800 `club_wants_player` thiscall(clube, jogador, ?, commit) ret 0x10 (1,5 KB, ponto flutuante: 0x967890,
    0x967898, 0x968930, 0x968a80, 0x968f80, 0x9690a8): 0 chamadas na cobertura do mundo até 2002-08-01; chamada pela
    oferta de troca da IA (0x8c843b) e por uma tela (0x47ed88). Escrever como simulação só se a carreira a alcançar;
    senão TELAS pelo caminho da tela.
  - 0x83a9d0 `Shortlist_Manager::load` thiscall(save) ret 4 (2,4 KB), item `SAVE` com `CARGA`.
- **Tactics::read_from_save 0x8985c0** thiscall(caminho) ret 4 (4 KB, chamada por 0x893eff): item `SAVE` com `CARGA`;
  o ramo da atualização de saves antigos pela versão.
- **Eliminatórias de 2006 que já estavam fora da lista:** 0x90f210, 0x90f600 e 0x9111d0 estão no `sbG_2`; 0x9106a0 e
  0x910120 no `cx15_10`; 0x910f00 e 0x911110 no `cx15_12`.

## Defeitos do original achados (para o `docs/DEFEITOS_DO_ORIGINAL.md`)

- **0x9109e0 `WcAfricanCup::create_group_stages`:** com o byte de formato +0xba fora de 0..3, o número de grupos e o
  tamanho deles ficam com o que estava na pilha (nunca iniciados); a cópia não monta grupos nesse caso. Num erro de
  `new`, a tabela de times e a de rodadas não são liberadas.
- **0x917320 `WcConcacafCup::create_semi_final_groups`:** o quarto do grupo B vem da lista de inscritos no índice
  `team_count` da copa (+0x3e), não do primeiro do grupo 1 como no grupo C [inferência: engano]; com menos de três
  sobreviventes no mata-mata caribenho ou na repescagem, as vagas 1 e 2 leem lixo da pilha (a cópia passa 0); sem
  cabeça de chave sobrando, a vaga 0 fica com o lixo do `malloc` (a cópia também).
- **0x911ba0 `WcAfricanCup::get_qualification_status_change`:** com o código atual 1 devolve sempre -1, mesmo quando o
  anterior também era 1 (a da Oceania devolve 0 nesse caso) [inferência: engano].
- **0x5788c0 e as outras `init_awards`:** o índice do prêmio só anda quando o `new` dá certo; um `new` que falha
  desloca os índices dos seguintes (a cópia mantém as vagas fixas).
- **0x7a2650 (não escrita):** grava `[0xae1080]` como efeito colateral de uma consulta.
