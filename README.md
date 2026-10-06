# Championship Manager 01/02: reconstrução

Championship Manager: Season 01/02 (Sports Interactive / Eidos, 2001), versão 3.9.68, liberado gratuitamente pela Eidos e pela SI.
Este repositório o mantém jogável no Windows 11 e prepara a reescrita do jogo em Rust, rotina por rotina, com a interface idêntica à original.

| Onde | O que é | No git? |
|---|---|---|
| `JOGAR.bat` | Abre o jogo em janela, sem CD e sem piscar a tela. | sim |
| `jogo/` | Instalação jogável: originais `cm0102.exe` / `cm0102_GDI.exe` e as cópias corrigidas `*-portatil.exe`. | não |
| `original/` | Downloads intocados (ISO, zips, patch 3.9.68) e cópia de segurança dos dois exe 3.9.68. | não |
| `referencias/` | Projetos de terceiros consultados (ex.: `CM0102Patcher` do nckstwrt). | não |
| `engenharia-reversa/` | Ferramentas de análise (Python) e o gerador dos exe portáteis. | sim |
| `docs/` | Documentação técnica; comece por [`DESCOMPILACAO.md`](docs/DESCOMPILACAO.md). | sim |
| `wiki/` | Estado do projeto, decisões e histórico. | sim |
| `CLAUDE.md` | Instruções para continuar o trabalho no Claude Code. | sim |

Os arquivos do jogo não entram no git: o jogo é gratuito para baixar, mas o código e a marca continuam da SI e da Eidos.

## Montar do zero

1. Instale o jogo a partir de `original/CM0102.iso` e aplique o patch 3.9.68 (`original/Championship-Manager-Season-0102_Patch_Win_EN_Patch-v3968/242.exe`). Coloque a instalação em `jogo/`.
2. Gere os exe portáteis (Python 3 com `pefile` e `capstone`):
   ```
   cd engenharia-reversa
   python patch_portatil.py ..\jogo\cm0102.exe ..\jogo\cm0102-portatil.exe
   python patch_portatil.py ..\jogo\cm0102_GDI.exe ..\jogo\cm0102_GDI-portatil.exe
   ```
3. Dê dois cliques em `JOGAR.bat`.

O que os exe portáteis corrigem, e por quê: [`docs/DESCOMPILACAO.md`](docs/DESCOMPILACAO.md), seção 3.
