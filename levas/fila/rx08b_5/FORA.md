# rx08b: o que ficou de fora da lista (rodada 6, 25/11/2026)

O leitor rx08b escreveu os itens rx08b_1 a rx08b_4 (13 rotinas, todas `TELAS`). As duas rotinas abaixo, os dois
escritores de texto de 12 KB, não foram escritas por tempo: ficam para a próxima rodada, cada uma num item próprio,
partida em pedaços com nome e comparada pelo `compare_text_writer`. Nenhuma está em `rust/src` nem em outro item da
fila (conferido em 25/11/2026). Ferramentas do rx08b no rascunho da sessão (`scratchpad\rx08b\`): `frame.py` (o
deslocamento de cada `[esp+x]` em relação à entrada, com o `_chkstk` 0x945610 e os textos da `.rdata`) e
`compact.py` (a listagem com os registros seguidos dentro de cada bloco e os `push` dobrados na chamada); as saídas
`f_487f60.txt`, `c_487f60.txt` e `d_449810.asm`.

## 0x487f60 `write_coach_report_news_text` (coach.cpp, 12.240 bytes) — TELAS

- Quem chama: só o escritor das notícias (`NewsItem::write_text` 0x77bc80, gra_1), item 0x1391 (`Object(0xadadf4,
  0x487f60)` no `NEWS_WRITERS` do news.rs): thiscall(objeto 0xadadf4, notícia, corpo, Text) ret 0xc. A notícia vem
  do rx08b_1 (`coach_post_report_news`), que só a tela "Request Coach Report" dispara.
- Prólogo `mov eax, 0x1bb8` (5 bytes) e `call 0x945610` (_chkstk): o quadro tem 0x1bb8 bytes de locais. Igual no
  cm0102-1080.exe.
- Lê as vagas 0..0x16 (`NewsItem::get_slot` 0x785830): 0 o id do treinador, 1..3 os nomes dele, 4 o id do jogador,
  5..7 os nomes dele, 8 a adaptação (1/2/4), 9 a fase da carreira (1/2/4/8/16), 0xa o veredito, 0xb o potencial,
  0xc e 0xd as características, 0xe os pontos fortes, 0x10 a máscara de posições, 0x11 a idade (byte), 0x12 a
  reputação do clube, 0x13 o id do clube do treinador (palavra com sinal), 0x14 transferência marcada (byte), 0x15
  ausente sem licença (byte). A vaga 0xf (pontos fracos) e a 0x16 são lidas e descartadas.
- Monta: o nome do treinador (estilos 1 e 5) e o do jogador (1 e 5) por `database_format_name_from_ids` 0x53e140; a
  posição (`write_position_name` 0x5461b0 da vaga 0x10, devolve o gênero) e a descrição do jogador
  (`write_player_description` 0x5465a0 (jogador, posições, pontos fortes, buf, 200, gênero, 1)) no mesmo buffer; o
  texto das características 0x7bdee0 (vaga 0xc, vaga 0xd só quando o jogador é regen — bit 0x40 do +0xb do registro
  de 79 bytes da tabela 0xae2d0c —, a reputação, a idade, saída, nome 5 do treinador, nome 5 do jogador).
- Corpo (arg 2 ≠ 0): fora do idioma 9, os textos de brincadeira dos desenvolvedores (0x84b8b0 com "Paul Norman",
  "Oliver Collyer", "Paul Collyer", "Kevin Turner", "Marc Duffy", "Mark Woodger", "Marc Vaughan", "Pete Sottrel",
  "Svein Kvernoey", "Miles Dineen", "Keith Flannery", "Marlon Davidson", "Grant Appleyard"), cada um só para quem
  nasceu antes de 11/11/1977 (0x549f70 (11, 11, 1977, −1) contra o +0x10/+0x12 do registro); o "Paul Norman" tem
  dois textos pela paridade do dia. Depois a árvore pelos bits da adaptação, da fase, do veredito e do potencial, da
  transferência marcada e da ausência; os textos vão para 0xde1f64 (langlib 0x66f4e0, alguns pelo sprintf do CRT
  0x944c9f no idioma 0) e terminam nos rótulos 0x48ac01/0x48ac05/0x48abfe/0x48abf4 (o `Text::set_string` e os
  vínculos). O título (arg 2 = 0) começa em 0x48ada7.
- O comparador: `compare_text_writer` (o texto, os vínculos e o gerador).

## 0x449810 `club_records_format_record_text` (club_records.cpp, 11.840 bytes) — TELAS

- Quem chama: `club_records_get_record_text` 0x4496d0 (rx08_9, pelo exe) e as telas do clube (0x46c3b8, 0x46c89b,
  0x46cd6b, 0x46d225, 0x46d717, 0x46dbd5). thiscall(clube, registros, tipo, Text) ret 0x10; quadro SEH (prólogo
  `mov eax, fs:[0]`, 6 bytes); igual no cm0102-1080.exe.
- Sem Text: a caixa de club_records.cpp 0x99a e 0; o Text é limpo (0x4afa40) e recebe "-" (0x98ba0c); sem
  registros: a caixa 0x9a6 e 0. A tabela de saltos 0x44c5cc despacha os 32 tipos (o mesmo número dos rótulos do
  rx08_9, `CLUB_RECORDS_TYPE_LABELS`); cada tipo escreve o seu recorde com vínculos a clubes, jogadores e partidas.
  Fim em 0x44c5c8 (`ret 0x10`).
