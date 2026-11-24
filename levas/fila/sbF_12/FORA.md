# sbF_12: os amistosos que ficaram de fora (leitor sbF, rodada 5, 24/11/2026)

Quatro rotinas do `friendly.cpp` da seção cx05 das sobras não foram escritas nesta rodada: são grandes, cheias de
objetos `Date` locais e de quadros SEH, e o tempo foi para as 21 rotinas dos itens sbF_1 a sbF_11. O que se apurou
pela desmontagem, para a próxima rodada:

| Endereço | Rotina | Tamanho | Convenção | Quem chama | Tipo |
|---|---|---|---|---|---|
| 0x5c5820 | `Friendly::process_tournament_replies(slot, force, clube)` | 1,8 KB | thiscall, ret 0xc | a tela de convites (0x474dd1), o dia dos amistosos (0x5c25c0) e a resposta automática das notícias (0x7843e3) | simulação; roda na carreira ENG em julho de 2001 (as minicopas do cx05_12/13) |
| 0x5c75a0 | `Friendly::get_club_date_status(clube, data, &torneio, &amistoso, &pedido, escolhidas, n, tipo, estádio)` → char | 3,2 KB | thiscall, ret 0x24 | as telas (0x45fcfd, 0x470870, 0x483238, 0x4834ca, 0x4837af), o dia dos amistosos (0x5c2bd0, 0x5c2c10) e ela mesma (0x5c7bd7, recursiva) | simulação (e telas) |
| 0x5c8a00 | `Friendly::load_from_temp_file(arquivo)` | 3,1 KB | thiscall, ret 4 | a vtable do Friendly 0x96ae7c, vaga +0x18 (0x96ae94); o construtor 0x5c1f80 a chama direto (0x5c2217) | SAVE (CARGA) |
| 0x5c9650 | `Friendly::save_to_temp_file(arquivo)` | 2,5 KB | thiscall, ret 4 | só a vtable, vaga +0x1c (0x96ae98) | SAVE |

Notas:
- `get_club_date_status`: devolve 4 quando a data está entre as escolhidas (`escolhidas`, `n` datas de 8 bytes);
  percorre os 127 torneios (0x69 bytes de +0xb1) duas vezes (os convidados em +0xc6, o tipo em +0xbd com o switch dos
  dias da linha 0xf18, a data em +0xb5) e devolve 2 ou 3 (em uso, +0x116); depois a lista de amistosos (+0x34c8,
  `LLIST::get_first` 0x68d1b0) e a de pedidos de datas. Os objetos Date locais pedem os invólucros `exe::date_*` com o
  ponteiro do resultado, como no `friendly_is_club_free_on_date`. O pseudocódigo do Ghidra está em
  `referencias/ferramentas/saida/0x5c6990_..._0x5c9650.c` (linhas 182 a 673) e confunde as cópias de datas: escrever
  pela desmontagem.
- `process_tournament_replies`: cria a MiniCup (0x765bf0, cx05_12) ou a MiniLeague (0x766500, cx05_13) quando todos
  aceitam; usa ponto flutuante (`dVar5` no pseudocódigo: conferir a ordem x87).
- `load_from_temp_file`/`save_to_temp_file`: o `Friendly.tmp` em CM3_TEMP, com os torneios, os amistosos, os pedidos de
  datas e as listas por nação; prova pelo `prova_save.py` (itens SAVE e CARGA).
- O `create_ai_tournament` (0x5c3be0) foi escrito no sbF_11; ele chama a `Friendly::send_tournament_invitations`
  0x5c5310 (ainda original, sem `FN_` no `exe_fns.rs`: o sbF_11 a chama pelo endereço, porque o cx05_20 já declara um
  `FN_FRIENDLY_SEND_TOURNAMENT_INVITATIONS` privado e os dois entram pela mesma âncora).
