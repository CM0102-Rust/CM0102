# CM 01/02: instruções para o Claude Code

Projeto pessoal do Lucas: Championship Manager 01/02 (3.9.68) revertido e reescrito em Rust, no mesmo método dos projetos irmãos `D:\Sistemas\Elifoot98` e `D:\Sistemas\Elifoot II`.

- **Responda sempre em português (Brasil).**
- **Wiki primeiro:** leia `wiki/index.md` antes de trabalhar; ao fim da sessão, atualize `wiki/index.md` (estado) e registre em `wiki/log.md`.
- **Documentação bilíngue:** todo `.md` do projeto tem a versão em inglês ao lado (`.en.md`). Ao criar ou mudar um, atualize o par no mesmo commit. O português é o original; o código e os comentários no código seguem em português.
- **Documentação em dia a cada commit:** o estado (README, wiki, PROJETO, PLACAR, ROTINAS) e as instruções (este arquivo, a estrutura, as ferramentas) acompanham cada leva; nada fica falando de uma leva antiga.
- Leitura obrigatória: `docs/DESCOMPILACAO.md` (tudo o que se sabe do binário) e `docs/COMO_CONTINUAR.md` (o ciclo de uma leva do motor).

## Decisões em vigor (não mudar sem o Lucas)

- **Rust**, entrando rotina por rotina: uma DLL carregada pelo exe original, com desvios (hooks) nas funções reescritas. O jogo continua jogável o tempo todo e cada rotina é comparada com a original.
- **Interface idêntica antes de tudo:** modernizações (resolução, fontes, escala) só depois que a tela reescrita bater pixel a pixel com a original.
- **Original determinizado (15/10/2026):** rotinas originais que leem variável local sem valor (lixo de pilha que muda com o modo) recebem, em todos os modos, um desvio que zera a pilha abaixo da entrada (`ZERADAS` em `motor.rs`, `desvios::desviar_zerando`). A referência das carreiras é a original com essas zeragens; as cópias Rust usam zero. Cada zeragem nova é provada (sonda) e documentada.
- **Levas de 3 rotinas (15/10/2026):** cada leva porta três rotinas e roda uma cadeia só; divergência é localizada pelo comparador de cada rotina (ou `SEM_ROTINAS`).
- **Base de trabalho: `cm0102.exe` (DirectDraw).** É nele que estão os endereços mapeados pela comunidade. O `cm0102_GDI.exe` é a referência para o caminho de vídeo por GDI.
- **Os exe originais nunca são alterados.** Correções vão para cópias geradas por ferramenta (`*-portatil.exe`), que confere o MD5 do original e falha se um padrão esperado não bater.
- **Nada do jogo no git:** nem exe, nem `Data/`, nem código descompilado em bruto. O repositório `CM0102-Rust/CM0102` (organização; antes `lucastmpacheco/CM0102`) é privado.
- `referencias/CM0102Patcher` é fonte de endereços e formatos; não copiar o código dele (as ferramentas leem os dados de lá).
- **Versão de trabalho: 1920×1080 nativo** (`cm0102-1080.exe`, gerado por `resolucao.py`). Fotos de fundo desligadas nas definições do jogo ("trocar = não") até a camada de vídeo ajustar as fotos à resolução.
- **Os cliques no jogo são do Lucas.** Eu abro o jogo e ele conduz; diagnóstico pelo código (desmontagem, registro de falhas do Windows, `ddraw.log`), não por tentativa e erro clicando.

## Estrutura

| Pasta | Conteúdo |
|---|---|
| `jogo/` | instalação jogável (não versionada) |
| `original/` | downloads e exe 3.9.68 intocados (não versionado) |
| `referencias/` | projetos de terceiros (não versionado) |
| `engenharia-reversa/` | `pe_info.py`, `textos.py`, `refs.py`, `desmonta.py`, `descompilar.py` (pseudocódigo do Ghidra, portátil em `referencias/ferramentas`), `desvios.py`, `escuta_tela.py`, `patch_portatil.py`, `resolucao.py`, `simular.py` (banco de provas), `cobertura.py`, `laboratorio.py`, `prologos.py`, `candidatos.py` (próximas rotinas; `--simulacao` fora do motor) e `fronteiras.py` (placar), `pacote.py` (o pacote de leitura de uma rotina em `pacotes/`), `mutacoes.py` + `mutacoes/` (autoteste com mutações), `cadeia.sh` (a cadeia de testes de uma leva), `aridade.py` (argumentos das chamadas por endereço), `save_grande.py` e `enchimento.py` e `assinatura_save.py` (o save acima de 2 GB: prova sem cliques e enchimento para o teste no jogo), `gen_docs.py`, `glossario.py` e `pendencias.py` (documentação gerada) |
| `rust/` | núcleo em Rust = `ddraw.dll` (i686, CRT estática, sem crates externos): `video.rs` (DirectDraw próprio), `apresentacao.rs` (quadro → janela, mouse), `ganchos.rs` (IAT), `desvios.rs` (desvios no exe, pilha particular), `simulacao.rs` + `alocador.rs` (simulação reproduzível), `aleatorio.rs`, `data.rs`, `motor.rs` (rotinas portadas, tabela `PROLOGOS`), `conteiner.rs` (o contêiner do save, usado também por `save/`), `comparacao.rs` (comparação original × Rust, fita), `jogo.rs`, `reducao.rs`, `registro.rs`, `win.rs`, `extensao.rs` (ponto de extensão: `botao.dll`, ver `docs/EXTENSAO.md`), `espiao.rs` (diagnóstico, `CM_ESPIAO=1`) |
| `save/` | DLL só do save (`ddraw.dll` repassadora com as 3 rotinas do contêiner de `rust/src/conteiner.rs`, assinatura do código antes de desviar; para os exes do NickPatcher): `cd save && cargo build --release`; ver `docs/SAVE_2GB.md` |
| `levas/` | uma pasta por leva desde a 143ª: `codigo.rs`, `integrar.py`, `docs.py`, `cadeia.txt` (resultado dos testes), `LEIA.md` |
| `pacotes/`, `jogo_testes/` | pacotes de leitura das rotinas (desmontagem, pseudocódigo) e a cópia do jogo para os testes automáticos (não versionados) |
| `docs/` | `DESCOMPILACAO.md`, `COMO_CONTINUAR.md`, `ROTINAS.md`, `PROJETO.md`, `PLACAR.md` (quanto já está em Rust), `PENDENCIAS.md` (lacunas das mutações), `GLOSSARIO.md`, `CAMPOS.md`, `FORMATOS.md`, `MAPA_MODULOS.md`, `ENDERECOS_COMUNIDADE.md`, `EXTENSAO.md`, `SAVE_2GB.md`, `VERSAO_SEM_TELA.md`, `mapa_funcoes.csv`, `cobertura_motor.txt` |
| `wiki/` | `index.md` (estado e próximos passos), `log.md` |

## Ambiente

- Python 3 com `pefile`, `capstone`, `psutil`, `Pillow` (instalados no Python do usuário).
- Endereços sempre como VA (base 0x400000). Em `.text`, `.rdata` e `.data` o deslocamento no arquivo é VA − 0x400000.
- Rust: `cd rust && cargo build --release` (sai em `target/i686-pc-windows-msvc/release/ddraw.dll`; copiar para `jogo/`), `cargo test --release`.
- **Armadilha:** restaurar um `.rs` com `Copy-Item` mantém a data antiga e o cargo não recompila (a DLL fica velha). Depois de restaurar, atualizar a data (`(Get-Item f).LastWriteTime = Get-Date`) ou conferir se apareceu "Compiling".
- Validação de rotina reescrita: `CM_AUTOTESTE=1` (entradas sintéticas no início do jogo, sem cliques) e `CM_COMPARAR=1` (durante o jogo real). **Banco de provas sem o Lucas:** carreira automática de férias (`auto_setup.cfg` com `ON_HOLIDAY=YES`, ver `docs/VERSAO_SEM_TELA.md`) + `CM_COMPARAR=1` + `CM_JANELA=960x540`, com cópia e restauração do `game.cfg`. Provar que o teste discrimina com uma mutação proposital antes de confiar no "zero divergências".
- **Máquina compartilhada com outras sessões pesadas (Elifoot):** em 06/10/2026 o PC travou por vazamento de driver no pool do kernel (~105 GB), alimentado por muita E/S de disco. Testes pesados só pelo `simular.py`, que usa `engenharia-reversa/laboratorio.py`: trava compartilhada `D:\Sistemas\.laboratorio.lock` (um teste pesado por vez entre as sessões) e vigia de memória (não roda com pool não paginado > 3 GB ou RAM livre < 4 GB). Um jogo de cada vez; nunca vários em paralelo. `python laboratorio.py` mostra o estado.
- Métodos COM são stdcall: cada método da vtable precisa do número exato de argumentos, mesmo os não suportados.
- **Sempre em janela (`-windowed`)**, no atalho e nos testes; nunca testar em tela cheia nem maximizar (cobre a tela do Lucas enquanto ele usa o PC). O que só der para ver em tela cheia, pedir para ele testar.
- **Experimentos que possam alterar preferências** (configuração automática, mudança de idioma...): copiar `jogo/game.cfg` antes e restaurar depois. Não deixar `auto_setup.cfg`/`quick_start.cfg` na pasta do jogo. Ver `docs/VERSAO_SEM_TELA.md`.
- A janela abre no tamanho/estado do `jogo/janela.cfg` do Lucas (pode estar maximizada e cobrir a tela dele). **Nos testes, usar `CM_JANELA=960x540`**: tamanho fixo, sem maximizar e sem gravar no `janela.cfg`.
- O Lucas às vezes mexe na janela durante os testes: mudança inesperada de tamanho pode ser ele.
- Para testar o jogo sem atrapalhar o Lucas: abra com `-windowed` e mande cliques por `PostMessage` (não mexe no mouse dele).
