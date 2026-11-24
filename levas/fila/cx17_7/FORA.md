# cx17_7: a campanha sul-americana de 2002, fora da rodada 5

O leitor cx17 (rodada 5, 24/11/2026) deixou de fora as duas rotinas abaixo (wc_south_american_league.cpp). Nenhuma foi
escrita; nenhuma está em `rust/src` nem em outro item da fila (conferido em 24/11/2026).

| Endereço | Bytes | Nome | Situação |
|---|---|---|---|
| 0x9216d0 | 3200 | WcSouthAmericanLeague::load_2002_campaign | thiscall, ret 0. Virtual +0x5c (as posições previstas), +0x45 = 1 e 90 chamadas à 0x922350 com os jogos reais de 2000 e 2001 (fase -1, dia, mês, ano 0x7d0/0x7d1, as duas seleções pelas chaves `KEY_NATION_*` e os gols; -1 nos jogos sem placar). Prólogo push esi; mov esi, ecx; mov eax, [esi] (5 bytes). |
| 0x922350 | 880 | WcSouthAmericanLeague::add_fixed_fixture | thiscall(fase, dia, mês, ano, casa, gols, fora, gols) ret 0x20. Monta um Fixture na pilha (quadro de 0x464 bytes), cria pelo FIXTURE_MANAGER 0x85c260 e, com a data já passada, grava o resultado (virtual +4 e 0x6827d0). Caixas nas linhas 0x3c7 e 0x3ce. Prólogo mov eax, fs:[0] (6 bytes). |

**Por quê:** as duas são inalcançáveis no jogo como ele é. Só o construtor (0x9202b0, já em Rust: `construct_wc_south_american_league`) chama a 0x9216d0, quando o ano do jogo não é anterior à edição; e a edição é a do ciclo do ano, nunca antes de 2004 (`qualifying_campaign_year(year).max(0x7d4)`). Com o jogo começando em 2001 a condição nunca vale: a cobertura do mundo até 2002-08-01 contou 0 chamadas das duas, e nenhuma carreira de teste chega lá. Só um exe com o ano de início mudado por patch (2004 ou depois) as rodaria, e aí carregaria os jogos de 2000 e 2001 numa campanha de 2004 ou mais.

**Defeitos do original [inferência]:** (1) código morto: a carga da campanha de 2002 ficou presa a uma condição que o mínimo de 2004 da edição tornou impossível no jogo normal; (2) com o ano de início em 2004 ou depois (patch), a tabela da campanha nova receberia os 90 resultados de 2000-2001; (3) a 0x922350 lê a palavra das substituições do Fixture local (`mov ecx, [esp+0x61]` em 0x9223c1) antes de iniciá-la e só troca os 12 bits de baixo (os dois nibbles das substituições e o nibble zerado por `and ax, 0xf0ff`): os 4 bits de cima ficam com sobra de pilha que o original não inicia (para o `DEFEITOS` e o comparador, quando for portada).

**Para quem pegar:** a prova só com um exe de ano de início mudado (o caminho no jogo: criar uma carreira com esse exe, jogando comparando, e abrir a tabela das eliminatórias sul-americanas). A 0x9216d0 é uma tabela de dados (a ordem das 90 chamadas e os seus 8 argumentos): gerar a tabela por script a partir da desmontagem. A 0x922350 precisa do leiaute do Fixture (`fixture::*` no `fields.rs`) e reproduzir o byte de pilha não iniciado.
