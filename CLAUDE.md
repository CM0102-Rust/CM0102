# CM 01/02: instruções para o Claude Code

Projeto pessoal do Lucas: Championship Manager 01/02 (3.9.68) revertido e reescrito em Rust, no mesmo método dos projetos irmãos `D:\Sistemas\Elifoot98` e `D:\Sistemas\Elifoot II`.

- **Responda sempre em português (Brasil).**
- **Wiki primeiro:** leia `wiki/index.md` antes de trabalhar; ao fim da sessão, atualize `wiki/index.md` (estado) e registre em `wiki/log.md`.
- Leitura obrigatória: `docs/DESCOMPILACAO.md` (tudo o que se sabe do binário).

## Decisões em vigor (não mudar sem o Lucas)

- **Rust**, entrando rotina por rotina: uma DLL carregada pelo exe original, com desvios (hooks) nas funções reescritas. O jogo continua jogável o tempo todo e cada rotina é comparada com a original.
- **Interface idêntica antes de tudo:** modernizações (resolução, fontes, escala) só depois que a tela reescrita bater pixel a pixel com a original.
- **Base de trabalho: `cm0102.exe` (DirectDraw).** É nele que estão os endereços mapeados pela comunidade. O `cm0102_GDI.exe` é a referência para o caminho de vídeo por GDI.
- **Os exe originais nunca são alterados.** Correções vão para cópias geradas por ferramenta (`*-portatil.exe`), que confere o MD5 do original e falha se um padrão esperado não bater.
- **Nada do jogo no git:** nem exe, nem `Data/`, nem código descompilado em bruto. O repositório `lucastmpacheco/CM0102` é privado.
- `referencias/CM0102Patcher` é fonte de endereços e formatos; não copiar o código dele.

## Estrutura

| Pasta | Conteúdo |
|---|---|
| `jogo/` | instalação jogável (não versionada) |
| `original/` | downloads e exe 3.9.68 intocados (não versionado) |
| `referencias/` | projetos de terceiros (não versionado) |
| `engenharia-reversa/` | `pe_info.py`, `textos.py`, `refs.py`, `desmonta.py`, `escuta_tela.py`, `patch_portatil.py` |
| `rust/` | núcleo em Rust = `ddraw.dll` (i686, CRT estática, sem crates externos): `video.rs` (DirectDraw próprio), `apresentacao.rs` (quadro → janela, mouse), `ganchos.rs` (desvios no exe), `espiao.rs` (diagnóstico, `CM_ESPIAO=1`) |
| `docs/` | `DESCOMPILACAO.md` |
| `wiki/` | `index.md` (estado e próximos passos), `log.md` |

## Ambiente

- Python 3 com `pefile`, `capstone`, `psutil`, `Pillow` (instalados no Python do usuário).
- Endereços sempre como VA (base 0x400000). Em `.text`, `.rdata` e `.data` o deslocamento no arquivo é VA − 0x400000.
- Rust: `cd rust && cargo build --release` (sai em `target/i686-pc-windows-msvc/release/ddraw.dll`; copiar para `jogo/`), `cargo test --release`.
- Métodos COM são stdcall: cada método da vtable precisa do número exato de argumentos, mesmo os não suportados.
- Para testar tela cheia sem mexer nas preferências do Lucas: cópia temporária de `jogo/` com hard links, sem `game.cfg` (o jogo abre nas definições; escolher "Full").
- Para testar o jogo sem atrapalhar o Lucas: abra com `-windowed` e mande cliques por `PostMessage` (não mexe no mouse dele).
