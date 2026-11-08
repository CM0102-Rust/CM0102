*[English version](README.en.md). Toda a documentação tem versão em inglês (`.en.md`).*

# Championship Manager 01/02: reconstrução

Championship Manager: Season 01/02 (Sports Interactive / Eidos, 2001), versão 3.9.68, liberado gratuitamente pela Eidos e pela SI.
Este repositório o mantém jogável no Windows 11 e prepara a reescrita do jogo em Rust, rotina por rotina, com a interface idêntica à original.

| Onde | O que é | No git? |
|---|---|---|
| `JOGAR.bat` | Abre o jogo em 1920×1080 nativo, em janela redimensionável, sem CD e sem trocar a resolução do Windows. | sim |
| `rust/` | Núcleo em Rust, compilado como `ddraw.dll` (camada de vídeo própria; ponto de entrada dos desvios). As rotinas do jogo reescritas ficam em `rust/src/motor/`, um módulo por arquivo de origem do jogo; **como ler o código:** o cabeçalho de [`rust/src/motor.rs`](rust/src/motor.rs). A base comum das DLLs fica em `cm0102-base/`. | sim |
| `save/` | DLL só do save (saves acima de 2 GB, até 4 GB), para os exes modificados (NickPatcher, CLZ): [`docs/SAVE_2GB.md`](docs/SAVE_2GB.md). | sim |
| CM0102-Rust-Tweaks | A `ddraw.dll` de ajustes para a comunidade (save acima de 2 GB, idade e distribuição dos regens), num repositório à parte, `CM0102-Rust/CM0102-Rust-Tweaks` (privado até a abertura); a regra original dos regens em [`docs/DISTRIBUICAO_DOS_REGENS.md`](docs/DISTRIBUICAO_DOS_REGENS.md). | à parte |
| `jogo/` | Instalação jogável: originais `cm0102.exe` / `cm0102_GDI.exe` e as cópias corrigidas `*-portatil.exe`. | não |
| `original/` | Downloads intocados (ISO, zips, patch 3.9.68) e cópia de segurança dos dois exe 3.9.68. | não |
| `referencias/` | Projetos de terceiros consultados (ex.: `CM0102Patcher` do nckstwrt). | não |
| `engenharia-reversa/` | Ferramentas de análise (Python) e o gerador dos exe portáteis. | sim |
| `docs/` | Documentação técnica; créditos em [`CREDITOS.md`](CREDITOS.md); comece por [`DESCOMPILACAO.md`](docs/DESCOMPILACAO.md). Para continuar a reescrita: [`COMO_CONTINUAR.md`](docs/COMO_CONTINUAR.md). Nomes do código em inglês: [`GLOSSARIO.md`](docs/GLOSSARIO.md). Campos dos registros: [`CAMPOS.md`](docs/CAMPOS.md). | sim |
| `levas/` | Uma pasta por leva da reescrita: o código, a integração e o resultado dos testes. | sim |
| `CLAUDE.md` | Instruções para continuar o trabalho no Claude Code. | sim |

**Para quem lê em inglês.** O projeto começou em português; o código agora é em inglês (com cabeçalhos bilíngues), mas
alguns nomes de pasta e de documento ficaram em português; o README em inglês traz um guia desses nomes e explica o que
é uma leva (a unidade de trabalho: algumas rotinas reescritas, desviadas no exe original e testadas juntas antes de
publicar; `levas/N/` guarda os resultados de cada uma).

**Estado:** 283 levas, 2065 rotinas em Rust, cada uma comparada com a original, direto ou pela rotina que a chama (o motor de partidas sem as telas está 100% em Rust; o save passa de 2 GB, até 4 GB); quanto falta, módulo a módulo: [`docs/PLACAR.md`](docs/PLACAR.md).

Os arquivos do jogo não entram no git: o jogo é gratuito para baixar, mas o código e a marca continuam da SI e da Eidos.

## Montar do zero

1. Instale o jogo a partir de `original/CM0102.iso` e aplique o patch 3.9.68 (`original/Championship-Manager-Season-0102_Patch_Win_EN_Patch-v3968/242.exe`). Coloque a instalação em `jogo/`.
2. Gere os exe portáteis (Python 3 com `pefile` e `capstone`):
   ```
   cd engenharia-reversa
   python patch_portatil.py ..\jogo\cm0102.exe ..\jogo\cm0102-portatil.exe
   python patch_portatil.py ..\jogo\cm0102_GDI.exe ..\jogo\cm0102_GDI-portatil.exe
   ```
   E a versão em 1920×1080 (também converte as imagens de fundo; as fotos originais ficam em `jogo\Pictures-800x600`):
   ```
   python resolucao.py ..\jogo\cm0102-portatil.exe ..\jogo\cm0102-1080.exe 1920 1080 ..\jogo
   ```
3. Compile o núcleo em Rust (alvo `i686-pc-windows-msvc`, já configurado em `rust/.cargo/config.toml`) e copie a DLL para o jogo:
   ```
   cd rust
   cargo build --release
   copy target\i686-pc-windows-msvc\release\ddraw.dll ..\jogo\
   ```
4. Dê dois cliques em `JOGAR.bat`.

Diagnóstico: com `set CM_ESPIAO=1` antes de abrir o jogo, a DLL repassa tudo ao DirectDraw do Windows e registra cada chamada em `jogo\ddraw.log`.

O que os exe portáteis corrigem, e por quê: [`docs/DESCOMPILACAO.md`](docs/DESCOMPILACAO.md), seção 3.
