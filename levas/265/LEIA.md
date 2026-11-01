# 265ª leva — o save das finanças e do gerente de elencos: gravar e carregar o finance.dat e gravar o squad_manager.dat; os temporários do save: as sementes da UEFA e da CONMEBOL e os países anfitriões; o save dos históricos: o histórico dos clubes, as estatísticas dos árbitros e as estatísticas simuladas; o save do calendário: gravar e carregar as datas dos clubes e gravar a conferência das pernas

**Rotinas:**
- `gravar_financas()` = 0x59b3b0 (fin_prm.cpp, thiscall): grava no item finance.dat as finanças de todos os clubes (0x167 bytes × [0xae235c] de [this]); sem contêiner, nada; erro: a caixa "Error" (finance.CPP 0x115e, 0x1166) sem fechar o item;
- `carregar_financas()` = 0x59b2a0 (fin_prm.cpp, thiscall): lê do item finance.dat as finanças de todos os clubes em [this] e devolve 1; sem contêiner 0; erro: a caixa "Error" (0x1137, 0x113f) e 0 (o construtor refaz as finanças do zero);
- `gravar_gerente_de_elencos()` = 0x859cc0 (squad_manager.cpp, thiscall): grava no item squad_manager.dat o u16 em +0 estendido a 4 bytes e devolve 1; erro: a caixa "Error" (squad_manager.cpp 0x68, 0x6f) e 0;
- `gravar_sementes_da_uefa()` = 0x905510 (uefa_seeding.cpp, thiscall): grava CM3_TEMP\uefa_seeding.tmp: quantos países (+0x438), 0x12 bytes por país de +0, +0x439 (u16), quantos (+0x43f) u32 de [+0x43b] e 50 registros de 0xc de +0x440; erro: a caixa "Error" sem fechar o arquivo;
- `gravar_sementes_da_conmebol()` = 0x4c6b70 (conmebol_seeding.cpp, thiscall): grava CM3_TEMP\conmebol_seeding.tmp: quantos países (+0x6e), 0xa bytes por país de +0, +0x6f (u16) e +0x71 (u32); erro: a caixa "Error" sem fechar o arquivo;
- `gravar_paises_anfitrioes()` = 0x5fb320 (host_country.cpp, thiscall): grava CM3_TEMP\HostCountry.tmp: quantos (+4, u16) e 0x22 bytes por anfitrião de [+0], sem conferir a gravação; erro ao abrir: a caixa "Error";
- `gravar_historico_dos_clubes()` = 0x443f50 (club_history.cpp, thiscall): grava no item "club history.dat" a quantidade (+0xc) e os registros de 0x10 bytes de [+8] e devolve 1; sem contêiner a caixa "Error" e 0; erro ao abrir ou gravar: a caixa e o jogo para;
- `gravar_estatisticas_dos_arbitros()` = 0x79bb80 (officials_manager.cpp, thiscall): grava no item off.dat [0xae2390] árbitros (0x11 bytes de [+0]), [0xae2350] nações (0x102 bytes de [+0xc]) e um byte por árbitro de [+8]; devolve 1 no byte baixo; sem árbitros ou erro: a caixa "Error", fecha o item e 0;
- `gravar_estatisticas_simuladas()` = 0x84c680 (simulated_stats.cpp, thiscall): grava no item simulated_stats.dat a quantidade (+0) e os registros de 0x44 bytes de [+4]; erro: a caixa "Error" sem fechar o item;
- `gravar_datas_dos_clubes()` = 0x5af040 (fix_man.cpp, cdecl): grava em CM3_TEMP\club_dates.tmp a tabela [0xae2a5c], uma palavra por clube ([0xae235c]); sem a tabela ou com gravação curta, a caixa "Error" (fix_man.cpp 0x1011, 0x101b);
- `carregar_datas_dos_clubes()` = 0x5aee90 (fix_man.cpp, cdecl): aloca a tabela [0xae2a5c] (uma palavra por clube) e a lê de CM3_TEMP\club_dates.tmp, ou a zera sem o arquivo; já alocada, sem memória ou com leitura curta, a caixa "Error" (0xfe4, 0xfec, 0xff6);
- `gravar_conferencia_das_pernas()` = 0x5af4a0 (fix_man.cpp, cdecl): com a lista [0xae2a48] não vazia, grava em CM3_TEMP\check_fixture_legs.tmp a conta (2 bytes) e os itens de 15 bytes; gravação curta de um item, a caixa "Error" (0x10e0);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Prova do save** (o `.qst` gravado em modo Rust com e sem as rotinas da leva, fora dos bytes que variam sozinhos, e o save de férias carregado até 2001-08-28 com e sem elas, mesma impressão; as mutações também foram por ela): IGUAL fora dos campos ignorados; carga IGUAL (parou em 2001-239, 729127830 sorteios, impressao 57d1b826c3d85ea4).
- **Mutações (12):** 12 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (0bb74e04).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 6299f556, original da4329bf (registrada; não barra a leva desde 31/10/2026).

**Refazer**, sobre o commit da 264ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 265`

**Campos tocados:**
- FinanceTable +0: as finanças dos clubes (0x167 bytes por clube, [0xae235c] clubes, nat_club inclusive); ponteiro; lê e grava no save (gravar_financas), grava do save (carregar_financas)
- SQUAD_MANAGER +0: o valor do gerente de elencos (u16; 0 no jogo novo); valor; lê e grava no save em 4 bytes (gravar_gerente_de_elencos)
- [0xae235c] quantidade de clubes; valor; lê (gravar_financas, carregar_financas)
- UefaSeeding +0: países (0x12 bytes cada); +0x438 quantos (i8); +0x439 (u16); +0x43b ponteiro dos u32 e +0x43f quantos (i8); +0x440 50 registros de 0xc; bytes; lê e grava no uefa_seeding.tmp (gravar_sementes_da_uefa)
- ConmebolSeeding +0: países (0xa bytes cada); +0x6e quantos (i8); +0x6f (u16); +0x71 (u32); bytes; lê e grava no conmebol_seeding.tmp (gravar_sementes_da_conmebol)
- HostCountry +0: anfitriões (0x22 bytes cada); ponteiro; +4 quantos (i16); lê e grava no HostCountry.tmp (gravar_paises_anfitrioes)
- Club_History +8: os registros do histórico de liga e copa dos clubes (0x10 bytes cada); ponteiro; +0xc quantos (u32); lê e grava no save (gravar_historico_dos_clubes)
- OfficialsManager +0: estatísticas por árbitro (0x11 bytes); +0xc por nação (0x102 bytes); +8 um byte por árbitro; ponteiros; [0xae2390] árbitros e [0xae2350] nações; lê e grava no off.dat (gravar_estatisticas_dos_arbitros)
- SimulatedStats +0: quantos (u32); +4 os registros de 0x44 bytes; ponteiro; lê e grava no save (gravar_estatisticas_simuladas)
- [0xae2a5c] as datas dos clubes (uma palavra por clube: bit 0 o primeiro ano, o resto o último); ponteiro; lê e grava no save (gravar_datas_dos_clubes), aloca e grava da carga (carregar_datas_dos_clubes)
- [0xae2a48] a lista de conferência das pernas (LLIST: +0 conta, +2 cabeça, +0xa cursor; itens de 15 bytes); lê e grava o cursor (gravar_conferencia_das_pernas)
- [0xae235c] quantidade de clubes; valor; lê (gravar_datas_dos_clubes, carregar_datas_dos_clubes)

**Interessa a:** SGE, CMRTE
