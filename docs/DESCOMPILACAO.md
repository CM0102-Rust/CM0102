# CM 01/02 3.9.68: o que se sabe do binário

Endereços em VA (base 0x400000), referentes ao `cm0102.exe` salvo dito o contrário.

## 1. Os executáveis

| | `cm0102.exe` | `cm0102_GDI.exe` |
|---|---|---|
| MD5 | `ed0cb3bc9bcc8ac3964dad52f6007cb8` | `0ece18564ace3012e67912c9f8a30000` |
| Compilado | 21/11/2002 13:12 UTC | 21/11/2002 17:19 UTC |
| Vídeo | DirectDraw (`DDRAW.dll`) | GDI (`StretchDIBits`), sem DirectDraw |

- Compilador: **Visual C++ 6** (linker 6.0), C++ com CRT estática e exceções SEH (`fs:[0]` no prólogo).
- Seções: `.text` 0x401000–0x9660e5 (código, ~5,4 MB); `.rdata` 0x967000; `.data` 0x987000 (0x45d63c em memória, só 0x153000 no arquivo); `.rsrc` 0xde5000. Em `.text`, `.rdata` e `.data` o deslocamento no arquivo é VA − 0x400000.
- Interface: uma única janela (`CreateWindowExA`, classe `"main"`), todo o resto desenhado pelo próprio jogo. Não há controles do Windows. Isso permite reescrever a interface pixel a pixel.
- Outras dependências: `DSOUND` (som), `WSOCK32` (jogo em rede), `WINSPOOL`/`comdlg32` (impressão), `ADVAPI32` (só lê o registro).

### Código-fonte original

O exe guarda os caminhos de **397 arquivos-fonte** da SI (mensagens de erro do tipo `arquivo.cpp linha`), todos sob `E:\dev\CM3\cm3 00-01\`. São o mapa de módulos do jogo:

| Pasta | Arquivos | Conteúdo |
|---|---|---|
| `cm3\code\comp\leagues` | 87 | uma liga por arquivo (`arg_prm.cpp`, `arg_second.cpp`…) |
| `cm3\code` | 76 | núcleo do jogo (motor de partida, gerentes, base de dados…) |
| `cm3\code\comp\cups` | 60 | copas nacionais |
| `cm3\code\ruling_body` | 44 | federações e regulamentos |
| `cm3\code\award` | 42 | prêmios por país |
| `cm3\code\transfer` | 25 | regras de transferência por país |
| `si\code` | 17 | biblioteca da SI: janela, vídeo, rede, `main.cpp` |
| `cm3\code\comp\eurocomp`, `intercomp`, `worldcup`, `friendly`, `comp` | 45 | competições internacionais |

Lista completa: `python engenharia-reversa\textos.py jogo\cm0102.exe "dev\\CM3\\.*\.(cpp|h)$"`.

## 2. Rotinas identificadas

| VA | O que é |
|---|---|
| 0x68d370 | provável `WinMain` (`si\code\main.cpp`): mutex "já está rodando", registro → `chdir`, inicialização |
| 0x68d7fc… | trecho do procedimento da janela que trata `WM_ACTIVATE`: em tela cheia, devolve o modo de vídeo do Windows ao perder o foco e volta a 800×600×16 ao recuperar |
| ~0x5e2800 | inicialização do vídeo (início exato da função ainda não fixado): `CreateWindowExA` em 0x5e2847, `DirectDrawCreate` em 0x5e28a5, nível de cooperação, `SetDisplayMode`, superfícies |
| 0x5e2ea0 | lê a flag de tela cheia (`[0xae319c]`) |
| 0x5d7xxx | carrega as preferências do `game.cfg` (assinatura 0x1553117c); trata `-windowed`, `-nosound` |
| 0x90bea0 | `get_user_folder`: pasta do usuário (saves) |
| 0x4c0aa0 / 0x4c0c10 | `_findfirst` / `_findclose` (envoltórios da CRT) |
| 0x953344 | `_chdir` (CRT) |
| 0x944c9f | `sprintf` (CRT) |
| 0x5e8290 | caixa de mensagem do jogo (título, texto, botões) |

## 3. Correções dos exe portáteis

Geradas por `engenharia-reversa/patch_portatil.py`, que confere o MD5 e localiza tudo por padrão (serve aos dois exe).

1. **CD.** A checagem foi copiada em **33 lugares** do código. Cada cópia percorre as unidades (`GetLogicalDriveStringsA`), procura `<unidade de CD>\cm0102.exe` (`_findfirst`) e, se não acha, mostra "Please insert the CM01/02 CD" e repete. Correção: o início de cada laço vira um salto para o caminho de "achou". Outras duas cópias, ligadas ao `auto_setup.cfg`, só mostram "CD Not Required" quando o CD está presente; ficaram como estão.
2. **Pasta.** O `WinMain` abre `HKLM\Software\Sports Interactive Ltd\Championship Manager 01-02` pedindo **acesso total** (`KEY_ALL_ACCESS`) e faz `chdir` para o valor `Location`. Sem administrador o Windows nega, o `chdir` não acontece e, fora da pasta do jogo, ele para em `Langlib.cpp 479` (o antigo "erro do registro"). Correção: `chdir` para a pasta do próprio exe (`GetModuleFileNameA`).
3. **`game.cfg`.** Guarda o caminho absoluto da pasta do usuário. Se a pasta antiga ainda existir, os saves vão para ela. Correção: o caminho gravado é ignorado e vale sempre a pasta do jogo.

Testado sem a ISO montada e com o jogo aberto a partir de outra pasta: chega ao menu e à escolha de campeonatos.

## 4. Vídeo e a "piscada" da tela

- O desenho do jogo é todo em **16 bits por pixel** (RGB565).
Comportamento **original** (sem a nossa `ddraw.dll`):

- **DirectDraw em janela:** `SetCooperativeLevel(DDSCL_NORMAL)` e em seguida `SetDisplayMode(largura_da_área_de_trabalho, altura, 16)` (0x5e2a53–0x5e2a83). No Windows 11 a área de trabalho continua em 32 bits (o modo é emulado), mas a chamada dispara uma reconfiguração da tela (`WM_DISPLAYCHANGE`) ao abrir e outra ao fechar: é a piscada. Sem essa chamada, as superfícies ficam em 32 bits e o jogo desenha errado (cores trocadas, imagem espremida). Testado.
- **DirectDraw em tela cheia:** 800×600×16 real (`SetDisplayMode` exclusivo); a piscada é inevitável nesse caminho.
- **GDI em janela:** monta a imagem em memória e copia com `StretchDIBits`, que converte para qualquer profundidade de cor. **Nenhuma reconfiguração de tela** (medido com `engenharia-reversa/escuta_tela.py`). Detalhe estético: sobra uma faixa branca no rodapé da janela.
- **GDI em tela cheia:** `ChangeDisplaySettingsA` para 800×600×16; também pisca.

### Uso real do DirectDraw (levantado com o espião, `CM_ESPIAO=1`)

| Método | Uso |
|---|---|
| `DirectDrawCreate`, `SetCooperativeLevel` | janela: `DDSCL_NORMAL` (0x8); tela cheia: `EXCLUSIVE|FULLSCREEN|ALLOWREBOOT` (0x13) |
| `SetDisplayMode` | janela: (largura da área de trabalho, altura, 16); tela cheia: (800, 600, 16) |
| `CreateSurface` ×2 | primária (`caps 0x200`) e quadro 800×600 na memória do sistema (`caps 0x840`) |
| `GetSurfaceDesc` (primária) | lê o formato de pixel: RGB565 |
| `CreateClipper`, `SetHWnd`, `SetClipper` | só em janela |
| `Lock`/`Unlock` (quadro) | o jogo desenha os pixels sozinho; com retângulo, o ponteiro aponta para o canto dele |
| `Blt` com `COLORFILL` (quadro) | pinta áreas de uma cor (`DDBLTFX.dwFillColor`, deslocamento 0x50) |
| `Blt` quadro → primária | apresentação; destino em coordenadas de tela |

Fora isso, o procedimento da janela chama `USER32!ChangeDisplaySettingsA` direto ao ganhar/perder o foco em tela cheia.

### Camada de vídeo própria (`rust/`, `jogo/ddraw.dll`)

O exe importa `DirectDrawCreate` da `ddraw.dll`, e o Windows procura a DLL primeiro na pasta do exe (`ddraw.dll` não é KnownDLL). A nossa `ddraw.dll` implementa só o que está acima:
- o quadro é um buffer nosso; `SetDisplayMode` é ignorado (**o modo do Windows nunca muda**);
- a apresentação converte RGB565 → 32 bits (replicação de bits) e desenha na janela com `StretchDIBits`: 1:1 em janela, escala proporcional com faixas pretas em tela cheia (a janela do jogo já cobre a tela inteira);
- em tela cheia, as mensagens de mouse são convertidas de volta para coordenadas 800×600 (subclasse do procedimento da janela);
- `ChangeDisplaySettingsA` é desviado na IAT do exe (primeiro desvio no executável) e não faz nada.

Em janela, a janela fica **redimensionável** (borda e maximizar): o quadro acompanha com escala proporcional (vizinho mais próximo em fator inteiro, suavizada em fator fracionário) e o mouse é convertido. O tamanho escolhido (fim de arrastar a borda, maximizar/restaurar) fica em `jogo/janela.cfg` (`largura altura maximizada`) e é reaplicado na abertura e sempre que o jogo reinicia o vídeo. Isso acontece, por exemplo, ao entrar em "Iniciar Novo Jogo", quando ele mesmo devolve a janela a 800×600.

Medido com `escuta_tela.py`: nenhuma reconfiguração de tela em janela, em tela cheia, nem na troca entre os dois. Só vale para `cm0102.exe`/`cm0102-portatil.exe`; a versão GDI não usa DirectDraw.
