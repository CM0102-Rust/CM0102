*[English version](COMO_ESCREVER_UM_ITEM.en.md).*

# Como escrever um item da fila

Um item é um pacote de até 3 rotinas portadas, escrito antes da hora em `levas/fila/<prefixo>_<n>/` e juntado numa leva pelo `fila.py` (o formato dos arquivos está no cabeçalho de [`engenharia-reversa/fila.py`](../engenharia-reversa/fila.py); o ciclo da leva, em [COMO_CONTINUAR.md](COMO_CONTINUAR.md)). Este guia junta as lições dos briefs dos agentes (outubro e novembro de 2026), para não ter de reescrevê-las a cada rodada.

## Escolher

- **Onde procurar:** `python engenharia-reversa/candidatos.py` (`--simulacao`, `--competicoes`), `docs/cobertura_*.txt`, `docs/MAPA_COMPETICOES.md` e `docs/PLACAR_ARQUIVOS.md`. Prefira rotinas que só chamam o que já é Rust (falta 0 ou 1).
- **Só o que a carreira de teste exercita, com efeito até 2002-06-01.** Rotina que não roda (contador 0), torneio passageiro (nada lido depois depende dos jogos dele) ou registro que ninguém lê até junho não tem mutação que acuse: o item fica de fora. Item a menos é melhor que item que não roda.
- **Sem repetir:** nada do `motor.rs` nem de `levas/fila/*`. Reserve criando a pasta com um `rotinas.py` mínimo assim que escolher.
- **Mapa da comunidade** (`referencias/cm0102-function-map`): entradas "fragment" ou "not a function" são pedaços de outra rotina, não rotinas. Não leia código de terceiros cuja licença proíba esse uso.
- **Mesmo slot em várias ligas:** juntar o mesmo slot de 3 ligas num item é bom; um ajudante comum com os parâmetros de cada uma serve, desde que cada endereço tenha o seu `Prologue` e seja fiel byte a byte.

## Portar

- **Acesso à memória:** use as funções de `rust/src/motor/mem.rs` (`read_u32`, `write_u8`...); não crie cópias `<item>_d/_b/_w` no item (desde 07/11/2026).
- **Convenções conferidas no ponto de chamada** (thiscall/stdcall/cdecl, `ret N`); `engenharia-reversa/aridade.py` ajuda. Anote no `rotinas.py`.
- **Tamanho do que se grava:** a 0x549ef0 grava 8 bytes (a data e o ano bissexto); um local de 4 bytes corrompe a pilha. Use o tamanho que a original escreve.
- **Embaralhamento e contagens:** confira o número exato de vagas (uma cópia embaralhava 9 e a original 8).
- **Ordem que a original supõe:** a `init_table_row` portada (0x687430) supõe as linhas em ordem; não a inverta.
- **Rótulos de campo:** confira em [CAMPOS.md](CAMPOS.md); o que não estiver provado leva "[inferência]".
- **DOCS:** em cada rotina, a regra em palavras (quantos sobem e descem, datas, vagas, desempate), em português e inglês. É a fonte do glossário e dos cabeçalhos.

## Nomes e cabeçalho (desde 08/11/2026)

- Código em inglês: a rotina com o nome do método original (o mapa de maxevans60, como em `docs/NOMES.csv`), os auxiliares com nome em inglês e sem prefixo de item, as constantes em MAIÚSCULAS em inglês.
- Cabeçalho em inglês em cada rotina (desde 11/11/2026 sem o parágrafo `[pt]`; o português vai no `DOCS`, que gera o `ROTINAS.md`): o texto completo (condições, campos pelo nome, defeitos do original), não um resumo, e a linha `Original: Classe::metodo (endereço, arquivo.cpp, convenção)`. Os campos e funções pelos nomes do código, não pelo deslocamento ou endereço.
- Rótulo de contador de ramo em inglês, com o código do item na frente: `crate::comparison::branch("cqx_3 rival offer")` (o mesmo texto no quarto campo da lista de mutação).
- Função do exe chamada pela cópia: `exe::nome(...)` (o invólucro tipado em `rust/src/motor/exe_calls.rs`, gerado pelo `involucros_exe.py`; quando a assinatura foi conferida na desmontagem, ela está em `levas/reorganizacao/assinaturas_conferidas.json` e vence), não `let f: unsafe extern "thiscall" fn(..) = core::mem::transmute(FN_X);` repetido.
- Campos com nome também nas variáveis de uma letra: `p + staff::NATION`, não `p + 0x..`, quando a variável guarda um registro do docs/FORMATOS.md a função inteira (o `aplicar_tipos_variaveis.py` faz isso depois, mas o item já pode vir assim).
- Use o que já tem nome: `read_u32`/`write_u8` (`mem.rs`), `KEY_*` (`keys.rs`), `G_*` (`globals.rs`), `club::DIVISION` e `club::SIZE` (`fields.rs`), `FN_*` (`exe_fns.rs`) e `original_fn::<F>(IDX_<NOME>)`; constante de ponto flutuante lida do exe pelo nome do valor
  (`read_f64(rdata::F64_0_5)`, `rdata_consts.rs`).
- **O estilo dos registros tipados (desde 10/11/2026):** o registro com o tipo dele e os campos como métodos
  (`club.reputation()`, `p.set_age(..)`, `person.club_job().manager()`), não `read_i16(club + club::REPUTATION)`; as tabelas
  do jogo por `club_at(i)` e `staff_at(id)`; o jogador em campo por `m.player(lado, k)` e a casa da grade por `m.cell(x, y)`;
  os códigos de narração por `event::NOME` e os do motor por `match_codes.rs`; o acesso cru que sobrar como `read_T(E)` /
  `write_T(E, v)`, nunca `*((E) as *const T)`. Nomes locais, mensagens de log e de comparador em inglês. O
  `padronizar.sh` passa tudo isso no código de uma leva logo depois de integrá-la (o `publicar_leva.py` chama), mas o item
  já pode vir assim.
- O `DOCS` do `rotinas.py` continua em português e inglês (é a fonte do ROTINAS e do glossário).

## Comparar

- O comparador confere **só o que a rotina garante na saída**. Campo que outra rotina reescreve depois (ex.: a 0x6827d0 reescreve +0xa7) acusa à toa.
- Quando a rotina não pode rodar duas vezes, roda só a original e o plano é conferido depois (o modelo das `cqt`/`cqk`); os calendários vão lado a lado (a máquina da `cqp_11`: `Cqp11Op`, `cqp11_tabela`, `cqp11_comparar`).
- Pedir de novo a tabela de rodadas de uma copa mostra a caixa "Cup.cpp 1278": não refaça o plano na comparação.

## Provar (na sua cópia, nunca nas bancadas)

- A cópia leva `rust/` e `cm0102-base/` lado a lado (o `rust/` depende da base pelo caminho `../cm0102-base`).
- **Comparação** com `simular.py --modo comparar` (com `--ligas "ENG ESP ITA FRA BRA JPN AUS USA"` nos itens de competições) até a data em que a rotina roda; qualquer `DIVERGENCIA` ou `EXCECAO` barra o item.
- **Carreiras Rust ×2** com `--repetir 2 --detector` até 2002-06-01: iguais e "detector de lixo: igual".
- **Mutações** pela carreira até 2002-06-01, com o motor de lances ligado (como o `mutacoes.py` da cadeia; use `--limite 3000`, porque a de 8 ligas passa de 900 s):
  - mude **quem joga, quem passa ou quem entra**, não só quando;
  - mutação de data muda **7 dias**: o jogo leva as rodadas ao dia da semana, e 1 ou 2 dias caem no mesmo lugar;
  - evite a mutação que trava ou derruba o jogo (só é acusada no limite e atrasa a cadeia);
  - mutação de caminho que não roda nas ligas do teste não serve;
  - a que escapa da carreira mas é pega pelo comparador da rotina conta como acusada; as outras precisam de explicação, ou o item sai.
- Testes pesados só pelo `simular.py` (trava do laboratório, `CM_VAGAS`); nunca dois jogos na mesma pasta.

## Entregar

- `PRONTO.txt` com uma linha de prova (ex.: "comparação até 2001-10-01 0 div; Rust x2 iguais; mutações 3/3"). Depois dele, não mexa mais no item.
- Item que não passa: `FORA.md` dizendo por quê (a próxima rodada lê antes de reaproveitar).
- Relatório por item: endereços, nomes, comparador, convenções, a regra em uma linha, a prova e as mutações que escaparam.
- Sem commit; sem mexer em `rust/src` da principal, `levas/<N>`, `ordem.txt` ou `copias/bancada*`. Apague a cópia do jogo e o `target` ao terminar.
