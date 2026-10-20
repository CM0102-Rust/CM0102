# Endereços da comunidade (patches de texto, finanças e telas)

Pontos do `cm0102.exe` 3.9.68 que a comunidade já identificou com patches de bytes. Não fazem parte do motor de partida, mas indicam onde ficam regras e textos que talvez queiramos mudar depois da migração.

**Fonte:**
- melanzalko, com contribuições de davdav, no tópico "Miscellaneous CM0102 Patches & Fixes" do fórum champman0102.co.uk (viewtopic.php?t=10877, páginas 1 a 3, agosto a outubro de 2026).
- Obrigado aos dois.

**Convenção dos posts:**
- Os números dos posts são **deslocamentos no arquivo**. Aqui estão convertidos em VA (VA = arquivo + 0x400000 em `.text`, `.rdata` e `.data`).
- Cada byte "antigo" foi conferido contra o exe original em 16/10/2026.
- A rotina é a que contém o endereço em `docs/mapa_funcoes.csv` (o módulo, quando o mapa o tem).

## Regras de jogo

### Instalações de treino
- **VA 0x461532, 0x461552, 0x461572, 0x461592** — rotina 0x460590 (`club_screens.cpp`).
- Limiares da descrição das instalações de treino: 0x190 / 0x258 / 0x320 / 0x3e8, ou seja bom / adequado / precisa de algum / precisa de muito.
- O patch baixa para 285 / 435 / 585 / 950.
- Só muda a descrição, não o nível.

### Situação financeira na diretoria
- **VA 0x5986ee (rico, multiplicador 0x1770) e VA 0x598716 (seguro, multiplicador 0x2904)** — rotina 0x5986b0.
- São os multiplicadores pela reputação do clube.
- O patch troca por 0x9398 e 0x9538, para que nem todo clube fique "rico".

### Dividendos (na prática um imposto sobre a receita)
Ficam na rotina 0x59cfa0, que ainda não tem módulo no mapa.

| VA | O que é | Patch |
|---|---|---|
| 0x59db0c | `je` da decisão aleatória do presidente | `jmp` tira o sorteio |
| 0x59dbf7 | `add ecx, 0x7d0` | — |
| 0x59dca3 | `add ecx, 0x5dc` | — |
| 0x59dc66 / 0x59dcc9 | `cmp word [edi+0x80], 0xfa0` (reputação 4000) | — |
| 0x59dcbb | `shl eax, 3`, a escala do limiar de entrada | vira `imul eax, eax, n`, com n de 1 a 8 |
| 0x59dcc1 / 0x59dcc7 | as duas guardas de caixa | com NOP, todo clube paga |

- O pagamento soma em +0x94 e +0x134 do registro financeiro e subtrai do caixa (64 bits em +0 e +4).
- Depois chama 0x59fe10.
- A faixa de impostos: **VA 0x59714d e VA 0x597a92**, constantes 0x1e46 e 0x1c52 na rotina 0x5968d0. davdav sugere 3500 e 2500.

### Transferências e contratos
- **VA 0x8edb67** — rotina 0x8e9550 (`transfer_screens.cpp`): o teto de 25 (0x19) da porcentagem da próxima venda. davdav: 0x32.
- **VA 0x867abe** — rotina 0x867aa0 (`staff_contracts.cpp`): o bônus por jogo sem sofrer gol fica restrito a goleiros. O patch faz a rotina devolver 1 sempre.

### Excursões
- **VA 0x471ef1** — rotina 0x4719a0 (`club_screens.cpp`).
- É o limiar de reputação para excursões a qualquer continente: o byte 4 multiplica 1000.

## Telas e interface

| Área | VA | Rotina | O que é |
|---|---|---|---|
| Próxima partida | 0x45e8c3, 0x45e8ce, 0x45e920 | 0x45ccf0 (`club_screens.cpp`) | estado do gramado com 13 dias de antecedência |
| Tela do clube | 0x456415 | 0x455942 (`club_screens.cpp`) | botão IMPRIMIR |
| Tela da competição | 0x490219, 0x490220 | 0x48f8e0 (`comp_screens.cpp`) | botão IMPRIMIR |
| Tela do clube | 0x456f85 | 0x456900 (`club_screens.cpp`) | ponteiro do texto "Stats" na tela da partida |
| Jogos do clube | 0x483f55 (casa), 0x483f7f (fora), 0x483fa0 (neutro) | 0x483a90 (`club_screens.cpp`) | cores das letras H/A/N: `mov reg, [cor]` vira `mov reg, imediato` |
| **Tela de tática** | 0x899f00, 0x899efb, 0x899ef7 | 0x899e50 (`tactics_screens.cpp`) | R, G, B da camisa de goleiro 1 |
| **Tela de tática** | 0x89e4ad | 0x899e50 | o salto para o código que pinta a camisa de goleiro 2 |
| Menu do técnico | 0x75e8e5, 0x75ebd4 | 0x75db00 (`Menubar.cpp`) | opções "controlar todas as seleções" e "controlar reservas" |
| Pedidos à diretoria | 0x4254fa | 0x425390 | `push` do texto "Meet with the Board" |

A tela de tática interessa para o elemento gráfico novo que o Lucas cogitou: a 0x899e50 desenha as camisas, com a cor em RGB.

## Textos (na `.data`)

| VA | Texto |
|---|---|
| 0x98f9b0 | "Dividends" (finanças) |
| 0x9c9720 | "Shareholder dividend payout" |
| 0x9c96b8 | "The board have announced…" |
| 0x98fa94 | "Season Tickets" |
| 0x98ee70 | "Stats" |
| 0x98ff14 | "Non-Playing Staff" |
| 0xaa0dec | "Set Staff Transfer Status" |
| 0x98f8d0, 0x990d54, 0x98f834, 0x990dc4, 0x9908c3, 0x9908d3, 0x99091b | "Tour" e frases da excursão |
| 0x98a1a8 | instrução dos pedidos à diretoria |
| 0x9c740c | instrução dos pedidos à federação |
| 0xa0fd54, 0xa0fd7b, 0xa0fea6 | opções e notícias de investimento |

## Áreas usadas como "caverna" (depende do exe)
- **VA 0x58ff74 (arquivo 0x18ff74):** NOPs dentro da rotina 0x58f500 (`file_screens.cpp`). O patch "Stats" grava um texto ali.
- **Arquivo 0x5e8200 e 0x6dd254:** não batem com o exe original. O primeiro não é zero na `.data`; o segundo fica depois do fim do arquivo. Só valem num exe já expandido pelo CM0102Patcher. Não usar sem conferir.

## Eliminatórias da Copa (o travamento das bases que começam cedo)

Fonte: Danek1985, no tópico "I've been studying the match engine, happy to share" do mesmo fórum (viewtopic.php?t=10790, 12/07/2026).

- **O ano em que cada confederação sorteia as eliminatórias** é uma palavra (`cmp`/`mov` com o ano) dentro das rotinas de cada liga das eliminatórias:
  - Ásia: VA 0x911c4d, 0x911c88, 0x911ca4 e 0x911cb8, na rotina 0x911bf0 (`wc_asia_league.cpp`);
  - Europa: VA 0x9182dc, na rotina 0x918200 (`wc_europe_league.cpp`);
  - América do Sul: VA 0x92036e, na rotina 0x9202b0 (`wc_south_american_league.cpp`).
- **No exe 3.9.68 original** valem 2000 (0x7d0) nos três primeiros pontos da Ásia e 2004 (0x7d4) nos outros. O post corrige um exe com o início mudado para 1994, em que ficam em 1994 (0x7ca); a correção põe 1996 (0x7cc).
- **A causa, segundo ele:** os sorteios ficam escalonados (Ásia em 1995, Europa em 1996). No intervalo, o código das eliminatórias asiáticas consulta o grupo europeu ainda vazio e lê um ponteiro nulo: leitura de 0x3e em VA 0x91df0e, na rotina 0x91def0. Só vale para jogo novo.
- **Para nós:** é um caso real de referência cruzada entre competições, a lembrar quando portarmos as eliminatórias ou mudarmos o ano inicial.

## Textos em português (language.ldb e estatísticas)

Fonte: o mesmo tópico do Danek1985, página 2 (t=10790&start=25, 18/07/2026), com observações de tafo.

- **Pé preferido em português:** o `Data\language.ldb` (204 MB) guarda os rótulos traduzidos. Em 0x070B0198 do arquivo está "Esquerdo"; trocar por "Só Esque" corrige o "Só esquerdo" exibido errado. Segundo tafo, cada rótulo pode aparecer duas vezes no arquivo, a alguns KB de distância.
- **Coluna "Key" vazia nas estatísticas da partida em português:** VA 0xa161cc (arquivo 0x6161cc), na lista de abreviações "Won", "Tck", "Key", "Cmp" da `.data`. O patch troca "Key" por "Imp".
- **Para nós:** quando a interface for nossa, os rótulos e abreviações traduzidos vêm do `language.ldb`, e as abreviações em inglês servem de chave.

## O mapa de funções e globais de maxevans60 (18/10/2026)

O mapa público [cm0102-function-map](https://github.com/maxevans60/cm0102-function-map) (maxevans60, licença CC BY 4.0, com permissão extra para uso com IA) dá nome, arquivo-fonte, convenção, propósito e confiança às 6.863 funções do `cm0102.exe` 3.9.68, e nome, tipo e leitores/escritores aos 4.428 globais. Foi feito para o "Bosman Edition", um porte recompilado do jogo para Mac, Windows e Linux.

- Clonado em `referencias/cm0102-function-map` (fora do git, como as outras referências).
- `engenharia-reversa/pacote.py` põe no `LEIA.md` de cada pacote o nome, a convenção e o propósito da rotina no mapa e os globais que ela usa.
- Conferência de 18/10/2026: os 391 inícios de rotina já portados aqui são inícios de função no mapa; dos 6.071 inícios em comum com o `docs/mapa_funcoes.csv`, só 44 têm arquivo diferente. Os nomes das rotinas da noite de 17/10 batem com os nossos (por exemplo 0x5b7d20 `formation_get_position_mask`, 0x9326e0 `zopen`).
- Nomes de confiança média ou baixa são pista, não prova: o critério continua sendo a comparação com a original.
- Obrigado a maxevans60, e aos que ele credita (John Locke, Tapani, Saturn e o tópico Offset Index).

## Tópicos antigos do fórum (arquivo nic.opik.net, lidos em 20/10/2026)

O arquivo [OldThreads](https://nic.opik.net/open/cm0102/OldThreads/) guarda em PDF os tópicos antigos do champman0102.co.uk. Foram lidos inteiros: o tópico **Offsets** (2.357 posts, 2012–2020, na versão "New Rip"), o do **Nicks CM0102Patcher** (New Rip, 1.681 posts), **Save Game Queries** (1.329 posts) e 14 tópicos menores (Database Structure, CM Secrets, Adding space to an exe, Transfer Window, Resetting regens, Using Development Benchmark Mode, Add Fonts, entre outros). Os PDFs, o texto extraído e os relatórios por parte ficam em `referencias/forum_antigo/` (fora do git; `achados/` tem os relatórios). Os endereços abaixo são VA e foram conferidos no `cm0102.exe` original; os que só valem em exes com patch ficaram de fora.

**Linha de comando e ambiente** (varredura das chaves testadas sobre `g_command_line_text` 0xae2c98 por 0x945a60):
- `-load <save>` (0x81bdb1, perto de `setup_run_front_end`) abre um save sem cliques. Outras chaves: `-verifypics` (0x41b4b3, 0x8282fb), `-verifyhist`, `-verifyplcfg`, `-verifyeurocfg` (0x828316, 0x828331, 0x828356), `-verifysounds` (0x6da68c), `norealplhist`, `checkgks` (0x7c0292, 0x7c0429), `-media` (0x7da35c), `-nonetwork` (0x81b9bf), `-dontcheckram` (0x81dff0), `-ignoreenv` (0x90bc57), `-defaultfont` (0x5d79ba), `nomemcheck` (strcmp em 0x5cc6d0).
- Variáveis de ambiente lidas em 0x90bc50: `SI_DATA`, `CM3_DATA`, `CM3_TEMP`, `CM3_DEBUG`, `CM3_PREFS` (pasta do `game.cfg`), `CM3_TACTICS`, `CM3_HISTORY`, `CM3_PICTURES`, `CM3_SOUNDS`, `CM3_QSTART`, `CM3_HELP`, `CM3_TRAINING`.
- 0x68d3b6 (`jne`): a trava de instância única. 0x9472cd (`push 0x2710`): a espera do laço principal.

**Competições** (estrutura da fase e da copa):
- 0x48c580: construtor padrão da fase de liga (+0x1c/+0x20 = −1, +0x49 = 5 reservas, +0x4a = 3 substituições, +0x42, +0x51, +0x45, +0x30).
- Fase de liga: +0xc2 pontos por vitória, +0xc3 por empate, +0xc7 não-comunitários, +0xbf repescagem de acesso, +0xc0 repescagem de rebaixamento (com +0xbe e +0xc1), +0x40 ano da temporada.
- Competição: +0x2c = estágios − 1 (liga 0, liga com playoff 1, Copa do Mundo 8, Liga dos Campeões 0x0d); +0x42 tipo (4 seleções, 2 clubes internacional, 1 nacional); +0x50 grupo de suspensões (9 Inglaterra, 0xff nenhum); +0x13 limite de estrangeiros nas contratações; +0x4c "spareggio" da Série A; +0xab desempate da liga.
- Rodada de copa (registro de 0x68): +0x0b tipo de sorteio (1/3 o menor manda, 4 cabeças de chave, 8 o melhor colocado manda); +0x0d/+0x0f/+0x11 desempate (byte baixo 0x83 gol de ouro, 0x03 prorrogação e pênaltis; byte alto 0x02 gols fora); fase de copa +0x229 jogos e +0x22a dias até a volta; copa +0xf2 número de clubes.
- Cup.cpp: 0x521e60 data do sorteio, 0x521eb0 data do jogo (dia, mês 0–11, dia da semana, turno, ano 0/1 da temporada), 0x521f10.
- Setup.cpp 0x830520: um `je` por competição (114, de 0x8322a2 a 0x838672) antes do construtor dela; vetor de competições 0xae23d0 (0x6c bytes).
- Sedes (host_country.cpp 0x5f9970): Copa do Mundo 0x5f99e9.., Euro 0x5f9c47.. (ano em 0x5f9c0a), Copa Africana 0x5f9eea.., Olimpíada 0x5fa030.., Copa América 0x5fa1b5.., Copa da Ásia 0x5fa563...
- Mundial de Clubes: ano 0x9291b2; vagas por 0x92ae60 (corte de reputação 5000; liga selecionada pela nação +0x11c).
- Globais preenchidos pelo nome: nações em 0x605144.. (dentro de 0x601a40), clubes em 0x60a973.. (dentro de 0x606060); região da nação (+0x75: 1 África … 24 Espanha) e continentes (0 África … 5 América do Sul).

**Jogadores e transferências:**
- Atributo interno → nota 1..20: `t = atributo/10 + CA/k + 10`, `nota = t²/30 + t/3 + 0,5`, limitada a 1..20 (0x5442f0, 0x543540).
- 0x4d0c60 valor do jogador: valor-base por posição (atacante 13 M, pico aos 28; goleiro 6 M, pico aos 33; os demais de 8 a 11 M), cortes de reputação 5000/8000, tabela 0x9a7f30.
- 0x8c5880 segunda nacionalidade: 3 anos (Portugal e Espanha), 6 (Europa), 11 (Itália, Finlândia, Rússia), nunca fora da Europa. 0x53d740 "é da UE?" (nação +0x7f = 2; exceções de Suíça, Alemanha, Itália e Dinamarca).
- Empréstimos: 0x579c70 limite de 4 meses (byte +0x18 < 24, data 19/5); Inglaterra 0x579e58 (5 por temporada) e 0x579f16 (simultâneos). Regras de estrangeiros por país (registro de 42 bytes com −1: Inglaterra 0x579c50, França 0x5c1a30, Bélgica 0x424f80, Itália 0x6619e0, Portugal 0x7d43c0, México/Turquia 0x902180); nação sem regras usa as inglesas.
- Regens: 0x7abac0 (CA por posição, 15 a 20 anos), troca de nacionalidade só com nação de menos de 100 pessoas (0x7abcd7), promoção bloqueada com elenco de 50 ou mais (0x7ad1bd); pregens 1,035 (0x524de9).
- Treino: 0x8b2be0 grava os atributos; 0x8b4e50 o treino padrão (níveis 0, 0x0a, 0x19, 0x32).

**Motor de partida:**
- Chute: 0x702050 chama 0x700bd0 (chute) e 0x700420 (goleiro); `100·chute/(chute+goleiro)` comparado com 71,0 (0x96d808). Gol de ouro em 0x6d4250. Códigos de evento = código do `events_eng.cfg` + 8000.
- Em `atualizar_atributos` (0x6ec010, já em Rust), com CA de 125 ou mais o salto em 0x6eda39 deixa a criatividade com o valor do atributo anterior. A comunidade (MadScientist) trata como defeito; a nossa cópia reproduz o original (fica para a modernização).
- Gerador: srand 0x9469fa, rand 0x946a04 (semente em 0xad6c30). Com semente fixa, laços do tipo `while(rand(10)!=5)` travam.

**Save** (confirma o `SAVE_2GB.md`): o índice do contêiner visto por Hugo9 no hex (`injury_history`: posição 0xec20fb, tamanho 0x17d90); itens `injury_history.tmp`, `club cup history.tmp`, `club league history.tmp`, `club records history.tmp`, `comp records history.tmp`, `ban_history.tmp`. Gravação segura em `save_game_data` (0x82e430): temporário, apaga o antigo, renomeia; a falha é o erro Setup.cpp 11431 (0x82f626). Setup.cpp 9090 (0x82ce4a) é a falha ao desempacotar os `.tmp`. A causa mais comum de save "sumido" é o VirtualStore do Windows.

**Espaço livre no exe:** fim do `.text` (0x9660e5..0x966fff); `cc` em 0x953472.. e 0x953fb3..; "Tapani Space" (0x601a4d, compactado pelos patches); seção extra do NickPatcher (2 MB em 0xde7000), o motivo de endereços de exes com patch não baterem.

Obrigado aos autores dos tópicos: Tapani, John Locke, saturn, Golly, Hugo9, MadScientist, Nick (nckstwrt), Craig Forrest, djole2mcloud, giovanisantana29, xeno, Patinoz, Zozoa, Dmitri, Fiestita, Kingsley e os demais; e a quem mantém o arquivo em nic.opik.net.
