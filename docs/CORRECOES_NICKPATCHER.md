# NickPatcher: patches que corrigem erros ou quedas do original

Inventário para o exe todo em Rust. Fontes lidas como dados (nada de código copiado): o dicionário `patches` do `referencias/CM0102Patcher/Patcher.cs`, os textos do `PatcherForm.Designer.cs` e os `.patch`/`.txt` do `MiscPatches.zip` (lidos com `ler_catalogo`/`opcoes_do_patcher` do Tweaks). Endereços em VA: os deslocamentos do catálogo são de arquivo (VA = deslocamento + 0x400000); o código novo dos patches vai para a seção acrescentada pelo `addextraspaceheader` (VA = arquivo + 0x70b000). Rotina: `docs/mapa_funcoes.csv` e o nome da comunidade (`referencias/cm0102-function-map/map/functions.tsv`). "Rust" = a cópia em `rust/src/motor/` (achada pelo cabeçalho `Original: ... (0x...)`). Desmontagem conferida contra `jogo/cm0102.exe` (3.9.68) com capstone. 08/11/2026.

**Critério.** Tabela 1: o alvo do patch é um defeito do código 3.9.68 (queda, ponteiro nulo, conta que dá a volta, competição que trava, CPU desperdiçada). Tabela 2: o que muda regra, limite, calendário, banco ou gosto; o que conserta defeito introduzido por outro patch (Tapani, Saturn, o próprio patcher); o que só serve à troca do ano de início; e o que só cala a caixa de erro.

## Tabela 1: correções de defeitos do original

| # | Patch (fonte) | VA alterado | Rotina | O defeito | Como o patch corrige | Rust | Em DEFEITOS_DO_ORIGINAL.md |
|---|---|---|---|---|---|---|---|
| 1 | `UnlockYourCreativity` (zip, Misc; também no `bugfixes` do Patcher.cs, "Apply known bug fixes") | 0x6eda3a (1 byte) | 0x6ec010 `MATCH_PLAYER::calc_match_attributes` (nosso `update_attributes`), match_pl.cpp | Com CA ≥ 125 e criatividade intrínseca > 10, um salto pula o cálculo e a criatividade fica com o valor anterior (o dos reflexos). | Muda o destino do salto para cair no cálculo da criatividade. | portada, reproduz: `rust/src/motor/match_pl.rs:9334-9341` (com CA ≥ 125 o `l28` não é recalculado) | sim (0x6ec010) |
| 2 | `MarkingAndPositionClampTo100` (zip, Misc; também no `bugfixes`); variante `MarkingAndPositionClampTo75` | 0x6ef644, 0x6ef692 (desvios) + rotina nova na seção acrescentada | dentro de 0x6eecb0 `MATCH_PLAYER::calc_ratings` (o mapa local parte em 0x6ef372, sem arquivo), match_pl.cpp | Marcação e posicionamento calculados acima do teto são gravados num byte (+0x110, +0x111) e dão a volta para um valor baixo. | Antes de gravar, limita o valor a 100 (ou 75). | na original (0x6eecb0 não portada) | sim (0x6ef644/0x6ef692), sem o nome da rotina |
| 3 | `EnsureCashDoesNotResetToZero` (zip, Misc; também no `bugfixes`) | 0x5986e3 (desvio) + código em 0x602f05 (espaço do `tapanispacemaker` em `set_key_ids`) | 0x5986b0 `Finance::get_wealth_band`, finance.cpp | O saldo do clube muito rico passa de ~2 bilhões e dá a volta. | Ali, se a palavra baixa do saldo ficou abaixo de −1,5 bilhão, repõe 2 bilhões. | portada, reproduz (não repõe): `rust/src/motor/comp/leagues/fin_prm.rs:3112` | sim (0x5986b0) |
| 4 | `memorycheckfix` (Patcher.cs, parte de "Make executable portable") | 0x7a1737..0x7a1744 | 0x7a1710 `os_get_free_memory_mb`, os.cpp | Soma memória física livre + arquivo de troca livre (GlobalMemoryStatus) em 32 bits antes de dividir por 1 MB: com 4 GB ou mais a soma dá a volta e o jogo acha que tem pouca memória (liga [0xdd7138]). | Divide cada parcela por 1 MB antes de somar. | na original; a DLL só fixa 2047 MB no modo simulação (`rust/src/simulation.rs:380-412`) | não |
| 5 | `idlesensitivity` + `idlesensitivitytransferscreen`, `idlesensitivitytransferstatusscreen`, `idlesensitivitydisciplinescreen` (Patcher.cs, "Enable idle sensitivity") | 0x4e243a, 0x9472d5, 0x8ec743, 0x8f76fe, 0x5a8a10 + rotinas nos vãos 0x966120.. e na seção acrescentada | laço de espera da interface e telas (0x4e2280 contract_screens.cpp, 0x8ec420 transfer_screens.cpp, entre outras) | Parado, o jogo gira o laço sem esperar: CPU a 100%. | Põe uma espera quando não há evento e ajusta as telas que dependiam do giro rápido. | na original (telas) | não |
| 6 | `protect_against_virtual_staff_195_error` (zip, Protection) | 0x85d891..0x85d89d, 0x85d8b8 | 0x85d8a0 `Stadium::get_travel_distance_factor`, stadium.cpp | Lê [nação do visitante +0x7e] sem conferir; clube sem nação → nulo → queda ("virtual staff 195"). | Nação nula conta como 0. | portada, reproduz: `rust/src/motor/comp/stadium.rs:1034` | sim (0x85d8a0) |
| 7 | `StupidNULLPointerCrash` (zip, Protection) | 0x8c3caa, 0x8c3e13, 0x8c3e4b + código em 0x60301e (espaço do Tapani) | 0x8c39f0 `TRANSFER_MANAGER::update_appearance_clauses_after_match`, transfer_manager.cpp | Soma jogo (+4) e falta (+6) no registro de permissão de trabalho achado por bsearch (0x946464) sem conferir se achou: nulo → queda (comum com todas as ligas). | Só soma se o registro existe. | portada e **já protegida** (diverge da original só no caso da queda): `rust/src/motor/transfer_manager.rs:3782-3786` | não |
| 8 | `danny_bhoy67_crash_squad_manager_protection_patch` (zip, Protection) | 0x85a556 + código novo | 0x85a470 `SQUAD_MANAGER::allocate_squad_number`, squad_manager.cpp | Lê a ficha de jogador (+0x61) da pessoa e as posições dela sem conferir nulo. | Ficha nula: sai sem dar número. | portada, reproduz: `rust/src/motor/comp/squad_manager.rs:307` | não |
| 9 | `luke_transfer_offer_protection_patch` (zip, Protection) | 0x8e8827 + código novo | 0x8e8800 `TRANSFER_RULES::can_make_offer`, transfer_offer.cpp | Desreferencia um ponteiro (eax) sem conferir nulo ao comparar o clube da oferta. | Nulo: segue pelo ramo "não é o mesmo". | na original (a cópia só a chama: `rules_can_make_offer`, `transfer_manager.rs:7533`) | não |
| 10 | `NullNewsItemProtection` (zip, Protection) | 0x4aeb0a + código novo | 0x4aeaf0 `Text::draw_wrapped`, comp_text.cpp | Texto nulo de notícia vai para o `strlen` (repne scasb): queda. A causa do nulo não é conhecida. | Texto nulo vira texto vazio. | na original (interface) | não |
| 11 | `Teebo78_ProtectionPatch` (zip, Protection) | 0x48ce46 + código novo | 0x48ce10 `Comp::all_stages_complete`, comp.cpp | Percorre a cadeia de divisões (+0x20) pela tabela [0xadadfc] e chama o método +0x10 de uma entrada que pode ser nula (competição não carregada). | Entrada nula: dá a cadeia por completa. | portada, reproduz: `rust/src/motor/comp/comp.rs:198-199` | não |
| 12 | `transfer_manager_protection` (zip, Protection) | 0x84aebb + vão 0x84aea5 | 0x84aeb0 `Shortlist_Manager::clear_bid_markers_for_player`, shortlist_manager.cpp | Lê o id do clube por um ponteiro de clube nulo. | Clube nulo: sai. | portada, reproduz: `rust/src/motor/shortlist_manager.rs:4206` | não |
| 13 | `Transfer_Offer_Protection_Patch` (zip, Protection) | 0x8e7bef + código novo | 0x8e7b50 `TRANSFER_OFFER::get_weighted_value`, transfer_offer.cpp | Lê dados de jogador (+0x61) de uma pessoa sem ficha. | Sem ficha: pula essa parte do valor. | na original | sim (linha 0x8e7bec em "Save e banco", sem o nome do patch nem da rotina) |
| 14 | `vascobenny_staff_records_protection_patch` (zip, Protection) | 0x86d606, 0x8c94c5 + código novo | 0x86cd70 `Staff_Records::add_transfer_records` e 0x8c9390 `TRANSFER_MANAGER::finalise_transfer` | Jogador dado em troca cujo contrato acaba antes da chegada fica sem clube (+0x39 nulo) e as duas rotinas leem [clube]. | Clube nulo: pula o histórico e a retirada do elenco desse jogador. | portadas, reproduzem: `rust/src/motor/staff_records.rs:1772` e `rust/src/motor/transfer_manager.rs:5079-5081` | não |
| 15 | `woerd86_netherlands_protection_patch` (zip, Protection) | 0x5f74ac + código novo | 0x5f73d0 `HolFirst::set_club_progress`, hol_first.cpp | A busca da linha de Comp_Stats (0x4a4850, copa holandesa KPN) volta 0 e o código segue sem conferir. | Zero: sai da rotina. | na original | não |
| 16 | `WorldCup_NULL_Crash_Protection` (zip, Protection) | 0x4b07b0 + código novo | 0x4b07b0 `compare_clubs_by_reputation_desc` (qsort), comp_util.cpp | O comparador lê +0x80 de ponteiro de clube nulo na lista da Copa (raro). | Nulo: copia o outro clube no lugar (duplica um time em vez de cair). | portada (desviada), reproduz: `rust/src/motor/comp/comp_util.rs:44-46` (`order_by_80`) | não |
| 17 | `ArgentinaFixes` (Protection) e `Possible Argentinian Aug 1st Points Issue` (Misc): o mesmo byte | 0x409d7e (jle → jmp) | 0x409c20 `ArgSecond::update_relegation_averages`, arg_second.cpp | Na tabela das médias da 2ª argentina, procura o clube lendo [ponteiro do clube] de cada linha; linha sem clube derruba o jogo (no 1º de agosto). [inferência sobre a linha nula] | Pula a procura e vai direto à vaga vazia (−1). | na original | não |
| 18 | `Australian League Bug Fix` (zip, 2021; aplicado pelos "All Tested 2021..2026") | 0x411cd5..0x411cdc, 0x411d24, 0x411d82..0x411d8a | 0x411ce0 `AusNsl::set_up_rules`, aus_nsl.cpp | A NSL não é zerada no fim da temporada (falta −1 em +0x30) e grava 3 em +0x3c. | Grava 2 em +0x3c e −1 em +0x30 antes de sair. | portada, reproduz: `rust/src/motor/comp/leagues/aus_nsl.rs:172` (+0x3c = 3) e `:219-226` (sem +0x30) | sim (0x411ce0; a fonte citada é o fórum) |
| 19 | 3º lugar cancelado (Xeno): linhas soltas dos "All Tested 2021..2026 + Saturn Patches" | 0x797477, 0x40aee1, 0x40cfd1 (3 → 2) | 0x79746c oceania_club_champ.cpp; 0x40ae90 `AsiaClubChamp::create_round_table`; 0x40cf90 `AsiaCupWinner::create_round_table` (o comentário do patch chama a 3ª de "CAF Champions League") | A fase final tem 3 rodadas (semi, 3º lugar, final) e o 3º lugar às vezes cai depois da final e trava a competição. | Reduz a fase final a 2 rodadas, sem 3º lugar. | Oceania na original; as asiáticas portadas, reproduzem: `rust/src/motor/comp/eurocomp/asia_club_champ.rs:508` e `:537` (`rodadas: 3`) | sim (0x797474...), mas a situação "na original" está velha para as duas asiáticas |
| 20 | `Stack Increase` (zip, Misc, sem .txt). Mitigação genérica | cabeçalho PE (0x400150 checksum, 0x40015a SizeOfStackReserve) | — | [inferência] Estouro da pilha de 1 MB (bancos grandes). | Reserva de pilha de 1 MB → 20 MB. | não se aplica; as rotinas desviadas já rodam em pilha própria (`rust/src/detours.rs`) | não |
| 21 | `DoubleHeapMemory` (zip, 2021, sem .txt). Mitigação genérica | 0x944ee1 (malloc da CRT) + código novo | 0x944e84 (CRT, alocação no heap) | [inferência] Escritas além do bloco alocado derrubam o jogo. | Dobra o tamanho de toda alocação. | na original (CRT) | não |

**Contagem.** 19 correções de defeito localizado (24 nomes de patch: `bugfixes`, a variante 75, o par argentino, as 4 opções de idle e as 3 linhas do 3º lugar) + 2 mitigações genéricas. Já em `DEFEITOS_DO_ORIGINAL.md`: 7 (linhas 1, 2, 3, 6, 13, 18, 19). Faltam 12 (4, 5, 7, 8, 9, 10, 11, 12, 14, 15, 16, 17) e as 2 mitigações. Na cópia Rust: 10 portadas que reproduzem a falha (1, 3, 6, 8, 11, 12, 14, 16, 18 e as asiáticas da 19), 1 já protegida (7), o resto ainda na original.

**Sugestões para `DEFEITOS_DO_ORIGINAL.md`** (nada alterado): a linha do 3º lugar está "na original", mas 0x40ae90 e 0x40cf90 já são Rust; a linha 0x6ef644 pode ganhar o nome `MATCH_PLAYER::calc_ratings` (0x6eecb0); a linha 0x8e7bec é o `Transfer_Offer_Protection_Patch` em `TRANSFER_OFFER::get_weighted_value`; a da NSL é o `Australian League Bug Fix`; a 7 já está corrigida na cópia sem linha "já corrigido" (o comentário de `increment_permit_counter` diz que a original cairia).

## Tabela 2: regra, limite, calendário, banco, gosto, ou conserto de outro patch

### Opções do `Patcher.cs`

| Opção | Classificação |
|---|---|
| `colouredattributes` | gosto (atributos coloridos) |
| `disablecdremove`, `disablesplashscreen` | gosto (aviso do CD, tela de abertura) |
| `disableunprotectedcontracts`, `hideprivatebids` | regra |
| `sevensubs`, `remove3playerlimit`, `manageanyteam`, `jobsabroadboost`, `transferwindowpatch` | regra |
| `showstarplayers`, `positionintacticsview`, `addadditionalcolumns`, `addadditionalcolumns_italy`, `changenamecolour` | gosto (interface) |
| `allowclosewindow` | gosto (o original desliga o "fechar" de propósito); o `CleanUpOnCloseWindow` conserta o efeito colateral |
| `forceloadallplayers`, `findallplayers` | recurso/banco |
| `regenfixes` | [discutível] o texto diz "fix", mas é regra: limite de 50 → 45 jogadores para receber regen (0x7ad1bf em `pick_best_club_for_player`), tira duas condições de 0x7acfc0 (`build_nation_club_lists`) e o ramo de reputação ≥ 350 em 0x7abeab (`regenerate_as_youth_player`, a origem dos regens velhos de nações pequenas) |
| `fixsuperkeepers` | [discutível] fora da interface; troca a constante 71,0 → 69,0 em 0x96d808 (.rdata), o limiar da defesa do goleiro em `take_shot` (0x706d1d, 0x706dee): equilíbrio. A cópia Rust lê a constante do exe (`match_eng.rs:4867`) e acompanha um exe alterado |
| `to1280x800` | gosto (resolução) |
| `tapanispacemaker`, `addextraspaceheader` | infraestrutura (espaço no exe) |
| `tapaninewregencode`, `makeyourpotential200` | regra (regens, potencial) |
| `englishleaguenorthpatch`, `englishleaguenorthpatchrelegation`, `englishleaguenorthawards`, `englishleaguesouthawards`, `chinapatch` | banco/competições |
| `restricttactics`, `changegeneraldat` | regra (modo restrito) |
| `changeregistrylocation`, `removemutexcheck` | infraestrutura ("portátil") |
| `datecalcpatch`, `datecalcpatchjumps`, `comphistory_datecalcpatch`, `tapanieurofix`, `fixworldcuppre2000` | só para o trocador de ano (corrigem o que a troca do ano quebra); `fixworldcuppre2000` já citado em DEFEITOS (0x91df0e) |


### Patches do `MiscPatches.zip` fora da tabela 1 (pastas Misc, Protection, fairedinkum, hodgy, 3.9.60, 3.9.65)

| Pasta | Patch | Classificação / primeira linha do .txt |
|---|---|---|
| 3.9.60 | `ColoredAttributes_3_9_60` | outra versão do exe (3.9.60/3.9.65): porte de uma opção do Patcher.cs |
| 3.9.60 | `HidePrivateBids_DisableUnprotectedContracts_3_9_60` | outra versão do exe (3.9.60/3.9.65): porte de uma opção do Patcher.cs |
| 3.9.65 | `3_9_65_AllowClosingGameWindow` | outra versão do exe (3.9.60/3.9.65): porte de uma opção do Patcher.cs |
| 3.9.65 | `3_9_65_ColorAttribs` | outra versão do exe (3.9.60/3.9.65): porte de uma opção do Patcher.cs |
| 3.9.65 | `3_9_65_ForceLoadAllPlayers` | outra versão do exe (3.9.60/3.9.65): porte de uma opção do Patcher.cs |
| 3.9.65 | `3_9_65_HidePrivateBids` | outra versão do exe (3.9.60/3.9.65): porte de uma opção do Patcher.cs |
| 3.9.65 | `3_9_65_IdleSensitivity` | outra versão do exe (3.9.60/3.9.65): porte de uma opção do Patcher.cs |
| 3.9.65 | `3_9_65_SpeedHack_x4` | outra versão do exe (3.9.60/3.9.65): porte de uma opção do Patcher.cs |
| 3.9.65 | `3_9_65_UnprotectedContracts` | outra versão do exe (3.9.60/3.9.65): porte de uma opção do Patcher.cs |
| fairedinkum | `AllowInstantSquadStatusBiddingReset` | regra/gosto: "What this patch does is allow you to change a players squad status - https://ibb.co/2ccLPHy - during contract talks, th |
| fairedinkum | `EPLFixtureFixes` | regra/gosto: "Here's another patch that people might find handy (and Nick himself might want to include in his patcher), it's an upda |
| fairedinkum | `SaturnPatchUpdateEuroBidsHostsFor2019` | regra/gosto: From fairedinkum: |
| fairedinkum | `SaturnPatchUpdateWCHostsFor2019` | regra/gosto: Here's another patch that people might find handy (and Nick himself might want to include in his patcher), it's an updat |
| hodgy | `CombineApps` | regra/gosto: Will combine appearances on the Player History screen, so instead of 10(10) apps, it will simply show 20. |
| hodgy | `CombineAppsLeagueOnly` | regra/gosto: Will do same as CombineApps.patch but show league appearances only like the game stores old histories (full details are  |
| Misc | `95SubsForAllApartFromEngland` | regra/gosto: This patch should make all competitions have 9 Named Subs and 5 Usable Subs apart from the English competitions. |
| Misc | `9and5SubsForEverything` | regra/gosto: Patch that makes every competition have 9 subs on the bench and 5 that can be used. |
| Misc | `All Tested 2020 + Saturn Patches` | regra/gosto (sem .txt; pelo nome) |
| Misc | `AllAvailableJobsInOnePlace` | regra/gosto: All available jobs listed in one place. |
| Misc | `AllowFutureTransfersToBeFarOut` | regra/gosto (sem .txt; pelo nome) |
| Misc | `AllowJanuaryTransfersInChampionsLeague` | regra/gosto: Normally players bought in January would not be eligble to play in Champions League games that season. |
| Misc | `AllowLoansToPlayInEurope` | regra/gosto: Taken from a Tapani patch, this allows your loans to register to play in Europe. |
| Misc | `AllowNonEULoans` | regra/gosto (sem .txt; pelo nome) |
| Misc | `AllowVeryYoungPlayers` | regra/gosto (sem .txt; pelo nome) |
| Misc | `AllowYoungAndOld` | regra/gosto: Some managers/non-players you might want to include from real-life, but the game might consider them too old or too youn |
| Misc | `AlwaysRegenToNewGoodCountry` | regra/gosto: This checks if a regen player's potential is 120 or higher and if so changes their nation to a different nation (but one |
| Misc | `Android11Patch` | compatibilidade de plataforma |
| Misc | `Asia Cup add only reputable teams` | regra/gosto (sem .txt; pelo nome) |
| Misc | `AttributeMaskingOffByDefault` | regra/gosto (sem .txt; pelo nome) |
| Misc | `Australia to Asia V2` | regra/gosto: However, this patch actually does |
| Misc | `BrazilSubsTo9` | regra/gosto: Changes Brazil's subs from 7 to 9. Taken from the Tapani patch. |
| Misc | `BTeamsIncreaseTo50Players` | regra/gosto: Teams that have B teams only have 48 players taken from the database by default. |
| Misc | `CleanUpOnCloseWindow` | conserto de outro patch: efeito colateral do "fechar a janela" do Tapani/patcher (.tmp largados) |
| Misc | `CopaLibertadores9Subs` | regra/gosto (sem .txt; pelo nome) |
| Misc | `Coup de France and Trophee des Champions Finals 95 Subs` | regra/gosto: Makes the finals of the Coup de France and Trophee des Champions have 9+5 Subs |
| Misc | `Disable virtual_staff.cpp 195 error` | só cala a caixa de erro (0x90d4bc, virtual_staff.cpp); não corrige |
| Misc | `DontAskAboutPlayerComparison` | regra/gosto: When trying to select a 1st and 2nd player for comparison CM0102 would confirm you are sure you want to set that player  |
| Misc | `DontRejectOffersV2` | regra/gosto: Fed up of your board cancelling your transfers at the last minute because they deem they are unrealistic? |
| Misc | `EnableSeasonTicketMoneyOnFirstYear` | [discutível] o 1º ano sem carnê vem da marca "passou" de 0x59b4c0 (portada, `fin_prm.rs:2009`); parece intencional |
| Misc | `English Subs to 9 Named 5 Used` | regra/gosto (sem .txt; pelo nome) |
| Misc | `FindAllPlayers` | recurso (a busca acha todos) |
| Misc | `FitnessPatch` | regra/gosto: Fitness Patch - changes the dates when players have slacking vs training portions of the year. |
| Misc | `FixGoalieStatsV3` | [discutível] experimental, equilíbrio (manuseio x reflexos na carga, Database.cpp); bytes de origem não batem com o 3.9.68 puro |
| Misc | `FixNetworkGames` | conserto de outro patch: queda introduzida pelos patches Saturn v6+ (rede em modo Win95) |
| Misc | `FixSaturnFrenchJokerWindowPatch` | conserto de outro patch: corrupção de memória do Saturn v6 (janela coringa francesa) |
| Misc | `GiveMoreOptionsInOfferDropDown` | regra/gosto: When offering to buy players this gives more options in the dropdown so you can quickly get to the value you want to off |
| Misc | `GiveMoreOptionsInOfferDropDownV2` | regra/gosto: When offering to buy players this gives more options in the dropdown so you can quickly get to the value you want to off |
| Misc | `GoldCupCrashFixV2` | conserto de outro patch: congela a Gold Cup que o próprio patcher (out/2021) acrescentou |
| Misc | `HalveSigningOnFees` | regra/gosto: Xeno pointed out that clubs were spending a significant amount of revenue just on signing on fees, especially for contra |
| Misc | `HideAttributesPlayerScreen68TapaniSaturn` | regra/gosto: Use this patch if you want to play blind mode: It hides all attributes in player screen. |
| Misc | `hodgy Replace Northern Ireland With Austria` | regra/gosto: Load Austrian leagues instead of Northern Ireland |
| Misc | `Human Manager Attributes` | regra/gosto: "Human manager attributes. If you apply below patch form, you will see "your" attributes in the game." --Xeno |
| Misc | `IgnorePlayerHistoriesOnLoad` | desempenho (pula o histórico na criação) |
| Misc | `Include SanMarino in Euro Qualifiers (Tapani)` | regra/gosto (sem .txt; pelo nome) |
| Misc | `IncreaseLoans` | regra/gosto: Increases the number of loans from 2 -> 9 (at least for English clubs) |
| Misc | `IncreasePregensMultiplierTo1.30` | regra/gosto: Discovered by Saturn, this will MILDY increase the number of "pregens" added when you start a new game. |
| Misc | `IncreasePregensMultiplierTo1.50` | regra/gosto: Discovered by Saturn, this will SIGNIFICANTLY increase the number of "pregens" added when you start a new game. |
| Misc | `IncreasePregensMultiplierTo1.90` | regra/gosto: Discovered by Saturn, this will GREATLY increase the number of "pregens" added when you start a new game. |
| Misc | `IncreasePromotionToDivision3To2Places` | regra/gosto: This patch changes the promotions to the English Division 3 from the Conference to 2. |
| Misc | `Increase_Font_Size_By_1` | regra/gosto: This patch will increase the size of all fonts by 1 |
| Misc | `Italy Subs to 9 Named 5 Used` | regra/gosto (sem .txt; pelo nome) |
| Misc | `JL_MakeAttributesShowAsStars` | regra/gosto: From John Locke: |
| Misc | `LoansUpdate` | regra/gosto: This patches the number of loans England, Scotland, France and Ireland can do to 9 (rather than 2 by default) |
| Misc | `MakeExeRequireAdmin` | infraestrutura (manifesto) |
| Misc | `MakeFutureLineProperlyTransparent` | conserto de outro patch: defeito gráfico "que não existe no original" |
| Misc | `match_eng_1225_error_turn_off` | só cala a caixa de erro (0x6dd41c, em 0x6dbc50 match_man.cpp); não corrige |
| Misc | `NicksFitnessPatchV3` | regra/gosto: This is like Fiestita's fitness patch but instead removes the idea of a "slacking" portion altogether. |
| Misc | `No Complaining About Training New Positions` | regra/gosto: Without this patch players complain constantly about training new positions. |
| Misc | `No MLS Foreign Restrictions` | regra/gosto: Removes the foreign player restrictions in the US Major League Soccer league |
| Misc | `NoForeignRestrictionsForAll` | regra/gosto: Removes the foreign player restrictions from ALL teams. |
| Misc | `NoForeignRestrictionsForEU` | regra/gosto: Removes the foreign player restrictions from all EU teams. |
| Misc | `NoMoreRetirements` | regra/gosto (sem .txt; pelo nome) |
| Misc | `Northern Ireland and Interamerican Renaming Fix` | conserto de outro patch: índices quebrados pelo renomeador do patcher |
| Misc | `NoTransferFundCountryBiasV2` | regra/gosto: This patch ensures the percentage of bank balance available for transfer funds for each nation is equal. |
| Misc | `NoWorkPermits` | regra/gosto: Removes the need for any work permits when buying players. |
| Misc | `PageDownPatch` | regra/gosto (sem .txt; pelo nome) |
| Misc | `PauseOnMinorInjury` | regra/gosto: The game will stop to allow substitutions if your players get an injury, but not if the player picks up a minor knock. |
| Misc | `PlayersAlwaysAskForWageTheyWant` | regra/gosto (sem .txt; pelo nome) |
| Misc | `PolishFACupRemove2ndLegs` | regra/gosto: Removes all the 2nd leg games from the Polish FA Cup |
| Misc | `Portugal_No_Restrictions` | regra/gosto (sem .txt; pelo nome) |
| Misc | `raise_retirement_of_employed_managers` | regra/gosto (sem .txt; pelo nome) |
| Misc | `raise_retirement_of_unemployed_managers` | regra/gosto (sem .txt; pelo nome) |
| Misc | `ReduceFAAppealTime` | regra/gosto: As requested by Offside Trap - this reduces the number of days it takes for the FA to reply to 2 or less |
| Misc | `Remove4MonthLoanLimit` | regra/gosto: Loans for players over 18 years old are limited to 4 months. |
| Misc | `RemoveBasqueRestrictions` | regra/gosto: Some teams in the Spanish league were limited to only hiring Basque players. This patch removes that limitation. |
| Misc | `RemoveEnglandForeignPlayerRestrictions` | regra/gosto (sem .txt; pelo nome) |
| Misc | `RemoveEUOnlyLoanRestriction` | regra/gosto: Playing as an EU country you can only loan EU players. This patch removes that restriction. |
| Misc | `RemoveLoanProtection` | regra/gosto: As of Saturn v5 and onwards, Saturn has made his releases so that players that are on loan cannot be bought or sold. Thi |
| Misc | `RemoveSecondNameRequirement` | regra/gosto: When starting a new game you can now just enter a first name and leave second name blank. |
| Misc | `RetirePatchV4` | regra/gosto: This patch ensures that players will not retire until they are at least 34 |
| Misc | `SaturnSpacePatch` | infraestrutura (espaço no exe) |
| Misc | `Saturn_v8_2019_PointsDeduction` | regra/gosto (sem .txt; pelo nome) |
| Misc | `ScoreboardPatch` | regra/gosto: This changes the way the scoreboard looks. Take a look here for the difference: |
| Misc | `Secret Attributes Tighter Spacing For Italian Language` | regra/gosto: This patch makes the Show Secret Attributes option display all the text when using the Italian language option |
| Misc | `SeeAnyTeamsFinancesV2` | regra/gosto (sem .txt; pelo nome) |
| Misc | `ShowPrivateBids` | regra/gosto: Tapani's Patcher and Mine can Hide Non-Public Bids. |
| Misc | `StopYoungstersAlwaysRejectingOffers` | [discutível] regra do menor de 18 que recusa tudo (0x847ef8, shortlist_manager.cpp); comportamento de projeto |
| Misc | `SwapNationalLeagueNorthForSouthernPremierOnSaturn` | regra/gosto: This is only for Saturn patched EXEs |
| Misc | `TapaniHighlightPlayersInSearch` | regra/gosto: This patch is from Tapani's 2.21 patcher. |
| Misc | `TapaniSpacePatch` | infraestrutura (espaço no exe) |
| Misc | `Tapani_Relegation_Patch` | regra/gosto: This is from the Tapani patch and adds the relegation to leagues that did not have it. |
| Misc | `TurkishSubsForeignRestrictionsPatch` | regra/gosto (sem .txt; pelo nome) |
| Misc | `TurnOffForcedAdminRequirement` | infraestrutura (manifesto) |
| Misc | `TurnOffTapaniRegens` | regra/gosto (sem .txt; pelo nome) |
| Misc | `Uncap20s` | regra/gosto: Some intrinsic values are capped at 20, even if they would normally be higher. |
| Misc | `UseCommonNamesForBrazilRegens` | regra/gosto: This makes Brazil regens always use Brazil Common Names for the regen |
| Misc | `WalesPatch` | regra/gosto: This adds Relegation + Foreign Player Restrictions to the Welsh League |
| Misc | `World Cup first year add reputable teams` | regra/gosto: If the World Cup is seeded on the first year of play it will add additional teams in alphabetical order. |
| Misc | `Xeno National Squad Increase` | regra/gosto (sem .txt; pelo nome) |
| Misc | `Xeno Polish Foreign Restrictions Change` | regra/gosto: Poland [5 foreign -> 17 foreign; squad 5 foreign -> no restrictions] |
| Misc | `Xeno Register 30 Players for Euro Comps` | regra/gosto (sem .txt; pelo nome) |
| Misc | `Yugoslavia_To_Serbia` | regra/gosto: Should remove references to Yugoslavia (and replace with Serbia) |
| Misc | `[HIDDEN} 3.9.68 to Saturn 9 - FA Cup Changes` | regra/gosto (sem .txt; pelo nome) |
| Protection | `NoNorthernLeagueJumpPatch` | conserto de outro patch: tira um salto do Tapani (em 0x689c6e o 3.9.68 puro já tem os bytes que o patch põe) |

### Pastas Saturn e por ano (agrupadas)

Mudam competições, calendário, banco, regras ou nomes para atualizar o jogo, ou consertam o que outro patch da comunidade introduziu. Os de nome "fix", "crash" ou "error" estão explicados na tabela seguinte.

| Pasta | Qtde | Patches |
|---|---|---|
| 2020 Patches | 20 | `All Tested 2020 + Saturn Patches`, `Champions League Subs Increase`, `Copa America`, `English Fixtures Update`, `FIFA World Club Cup`, `France Restructure`, `French Subs Increase`, `German Subs Increase`, `German Super Cup`, `Greek Subs Increase`, `International Tournament Hosts Update`, `Italy Restructure`, `PlayerRestrictions`, `Recopa Sudamericana`, `Remove Mexico From Copa Libertadores`, `RenameIrishTeamsToOriginalNames`, `ShefWeds Minus 6 Points`, `Squad numbers`, `The African Cup of Nations`, `UpdateGoldenGoals` |
| 2021 Patches | 26 | `African Cup of Nations - 2021`, `All Tested 2021 + Saturn Patches`, `Allow Italian loans + January loans until the end of the season`, `Allow loans between Premier League teams`, `Allow Uefa Super Cup at major stadiums`, `Baseline`, `CONCACAF Gold Cup`, `CONCACAF Tapani Parts`, `Copa America 2024 Start`, `Derby County Minus 21 Points - Reading Minus 6 points`, `DontLoadCPUInf32`, `fairedinkum - International Hosts 2021`, `fairedinkum_ACON_Asian_Cup_Hosts`, `fairedinkum_MiscellaneousPatch`, `fairedinkum_Push Euros_Back_2_Weeks`, `FIFA World Cup 2022 groups for start year 2021 - Xeno`, `France Restructure 2021`, `Irish Leinster Senior Cup fix V2`, `Italy Restructure 2021`, `Oceania OFC Tapani Patch`, `ReplaceLimerickWithTreaty`, `ReplaceMerthyrTydfilWithTown`, `saturn_Correct_South_American_World_Cup_qualifying_schedule`, `World Cup first year add reputable teams`, `WorldCupAdjustments`, `xeno_fairedinkum_FCFWFix` |
| 2022 Patches | 10 | `All Tested 2022 + Saturn Patches`, `France Restructure 2022`, `French Ligue 1 to 4 Relegations`, `Italy Restructure 2022`, `Leinster Cup Add Aditional Leinster League Team`, `Move World Cup to 2023`, `Reverse FA Cup Changes`, `Set Euro Host 2024 To Germany`, `wc_south_american_league_744_crash`, `World Cup 2022` |
| 2023 Patches | 8 | `All Tested 2023 + Saturn Patches`, `Freeze France To 18 Teams`, `Italy Restructure 2023`, `October 2023 Update Patch`, `Set Euro Host 2024 To Germany`, `Set World Cup Host 2026 to Mexico`, `Swedish League Restructure`, `Tapani_Euro_Patch` |
| 2024 Patches | 1 | `All Tested 2024 + Saturn Patches` |
| 2025 Patches | 3 | `2025 Italy Restructure`, `All Tested 2025 + Saturn Patches`, `davdav World Cup Winners` |
| 2026 Patches | 2 | `2026 Italy Restructure`, `All Tested 2026 + Saturn Patches` |
| Saturn v1 | 32 | `2015 International Reschedule`, `Argentinian money`, `Argentinian squad restrictions`, `Australia restructure`, `Background Hires`, `CONCACAF Competitions Reactivation`, `Continental auto load`, `Disable defunct competitions`, `Display Your Star Players`, `English Loans`, `French Leagues Restructure`, `German Super Cup`, `Greek National A + B Division restructure`, `Italian Serie A + B restructure`, `Manage Any Team`, `Olympic Games 2016 entrants`, `Player Restrictions`, `Polish Second Division restructure`, `Recopa Sudamericana rename`, `Recopa Sudamericana`, `Rename 'National Leagues'`, `Republic Of Ireland fixes`, `Reserve Sides Enabled`, `Russian squad restrictions`, `Scout Role Offer`, `Second nationalities in non-European leagues`, `South Korean start date`, `Transfer Windows`, `Wembley fix`, `World Club Championship`, `World Club Cup hosting`, `Younger Regens` |
| Saturn v2 | 17 | `B teams`, `Champions League Fixtures`, `Copa América tiebreaks`, `Copa Argentina`, `Currency`, `English FA Cup Quarter Final`, `Europa League Fixtures`, `French loans`, `International Reschedule`, `Irish loans`, `Other`, `Round Name Changes`, `Turkish Foreign Players`, `USA Clubs Renaming`, `World Cup Qualifying`, `x English loans - reverted from +v1`, `x Work permits - reverted from +v1` |
| Saturn v3 | 37 | `Bosman countries`, `Belgian FA Cup`, `Brazilian Cup`, `Copa Sudamericana`, `Croatian Cup`, `English FA Trophy`, `English Vans Trophy`, `Greek Cup`, `Irish League Cup`, `Italian Cup`, `Italian Serie C Cup`, `Northern Irish Charity Shield`, `Northern Irish Cup`, `Northern Irish League Cup`, `Norwegian Cup`, `Polish Cup`, `Portuguese Cup`, `Portuguese Super Cup`, `Scottish Cup`, `Swedish Cup`, `Turkish Cup`, `USA MLS`, `Welsh League Cup`, `_Competition changes_`, `Competition changes`, `Dutch Eerste Divisie relegation place removal`, `English National League North suspensions`, `English suspensions minor edit`, `English Third Division relegations`, `Golden goals tweak (normal)`, `Job application nudge`, `MLS clubs name changes`, `Real Sociedad Spanish signings`, `Scottish playoffs`, `Squad numbers`, `Transfer windows edit`, `x Irish Leagues Restructure - reverted from +v1` |
| Saturn v4 | 33 | `2017 International competitions`, `African Cup of Nations reschedule`, `Australia to Asia`, `B Teams`, `Canadian Green Card`, `England restructure`, `English loan limit ASCII`, `English National League North rename`, `English National League North substitutes named`, `European competitions' round names`, `Eurox2`, `FAI Cup fixes`, `Fixture fixes`, `French Championnat National fix`, `French League Cup restructure`, `Host nation updates`, `International B Teams removal`, `Job application nudge edit`, `Jobs abroad boost`, `Loans boost`, `Obscene gesture suspensions`, `Portuguese ProLiga restructure`, `Puchar Polski restructure`, `Republic of Ireland loans`, `S.D. Éibar Basque removal`, `Scottish loans`, `Select Leagues ASCII`, `Spelling fixes`, `Taça de Portugal restructure`, `Tottenham Hotspur stadium move`, `Values`, `Version name`, `World Cup 2018 Qualifiers` |
| Saturn v5 | 20 | `2018 World Cup Group Stage`, `AI Loan Players Register`, `Awards fix`, `B Teams`, `CONCACAF Gold Cup`, `Copa América entrants`, `English FA Trophy team selection`, `English National League North awards`, `English National League North relegations`, `FAI Cup restructure`, `FIFA World Club Cup`, `Irish Leinster Senior Cup fix`, `Irish Munster Senior Cup fix`, `Loan_players_purchase_protection`, `Polish match restrictions`, `Regen tweak`, `Register for Europe age increase`, `Republic Of Ireland restructuring`, `Spelling fixes reversed from +v4`, `Version update` |
| Saturn v6 | 45 | `Argentina foreign players allowed`, `Brazilian Champions Cup disable`, `Champions League Saturday final`, `Confederations Cup fixes`, `Copa Libertadores tiebreaks`, `EFL Cup tiebreaks`, `French fixtures reschedule`, `French joker window removal`, `German DFB-Pokal player restriction`, `Girona FC B Team`, `Greek Cup fixtures reschedule`, `International call-ups edit`, `African friendlies`, `Asian friendlies`, `European friendlies`, `North American friendlies`, `Oceanic friendlies`, `South American friendlies`, `International friendlies`, `Ireland - Russia swap`, `FAI Cup fixtures`, `Irish Leinster Senior Cup fixtures`, `Irish Munster Senior Cup fixtures`, `League Of Ireland Cup fixtures`, `League Of Ireland First Division fixtures`, `League Of Ireland Premier Division fixtures`, `Republic Of Ireland & Russia swap`, `Republic Of Ireland game start, training & holiday`, `Russian Cup fixtures`, `Russian Football National League fixtures & restructure`, `Russian Premier Division fixtures`, `Setup`, `Irish Munster Senior Cup team selection`, `Italian Serie A & B playoffs disable`, `Japanese Emperors Cup fixture reschedule`, `Japanese J1 League Final reschedule`, `Portuguese ProLiga restructure`, `Regen CA level reset`, `Regen tweak 2`, `Russian second nationality`, `Scottish Premiership stadium requirements`, `Taça de Portugal restructure`, `Turkish competitions' substitutes`, `Under-21 caps non-binding`, `Version name update` |
| Saturn v7 | 17 | `Atlanta United nation edit`, `Awards fix update`, `EFL Cup fix`, `English FA Cup Fifth Round`, `English transfer window update`, `Goalkeeper training boost`, `Italian transfer window update`, `MLS clubs update`, `North American friendlies edit`, `Norwegian 1. Divisjon changes`, `Norwegian Eliteserien restructure`, `Republic Of Ireland contracts`, `Russian contracts`, `Saudi Arabia restriction removal`, `South American continental finals`, `Turkish TFF 2. Lig fixture changes`, `Version name update` |
| Saturn v8 | 69 | `Asian Cup fixture reschedule`, `Awards fix update`, `B Teams update`, `China & South Korea swap`, `Confederations Cup disable`, `Copa América changes`, `Coupe de Belgique replays removal`, `Croatian league play-offs disable`, `Danish transfer window`, `English competitions fixtures reschedule`, `European Championship Qualifying Playoff nations`, `European competitions fixture reschedule`, `FIFA Club World Cup fix`, `Find Players fix`, `English FA Community Shield fixture reschedule`, `English FA Cup fixture reschedule`, `English FA Trophy fixture reschedule`, `English Football Championship fixture reschedule`, `English Football League Cup fixture reschedule`, `English Football League One fixture reschedule`, `English Football League Trophy fixture reschedule`, `English Football League Two fixture reschedule`, `English National League fixture reschedule`, `English Premier League fixture reschedule`, `UEFA Champions League fixture reschedule`, `UEFA Europa League fixture reschedule`, `UEFA Europa League Qualifying fixture reschedule`, `UEFA Super Cup fixture reschedule`, `Coppa Italia fixture reschedule`, `Coppa Italia team selection adjustment`, `Italian Serie A & B playoffs disable adjustment`, `Italian Serie A fixture reschedule`, `Italian Serie B fixture reschedule & 20 team changes`, `Supercoppa Italia fixture reschedule`, `Spanish Copa del Rey fixture reschedule`, `Spanish La Liga fixture reschedule`, `Spanish Segunda División B minor fixture reschedule`, `Spanish Segunda División fixture reschedule`, `Supercopa de España fixture reschedule`, `African World Cup Qualifying fixture reschedule`, `Asian World Cup Qualifying fixture reschedule`, `Error message suppression`, `North American World Cup Qualifying fixture reschedule`, `Oceanic World Cup Qualifying fixture reschedule`, `South American World Cup Qualifying fixture reschedule`, `Greece restructuring`, `International hosts update`, `Italy changes`, `match_eng error information`, `Physical training fix`, `Polish Ekstraklasa points halving removal`, `Regens mega boost`, `Republic Of Ireland awards date sync`, `Republic Of Ireland fixtures reschedule`, `Russian awards date sync`, `Russian new game start date`, `Spanish fixture reschedule`, `Spanish new game start date change`, `Spanish Segunda División B non-EU players`, `Swedish transfer window`, `Transfer offer button sensitivity fix`, `Turkish reserve matches`, `Turkish Süper Lig minor fixture reschedule`, `Turkish TFF Lig subs`, `Turkish training period extension`, `Türkiye Kupasi update`, `UEFA Super Cup subs`, `Version name update`, `World Cup Qualifying fixture reschedule` |
| Saturn v9 | 4 | `Irish Munster Senior Cup fix`, `MLS clubs update`, `Toronto FC nation edit`, `Version name update` |

### Os de nome "fix", "crash" ou "error" nas pastas Saturn e por ano, e por que ficam fora

| Patch | Por quê |
|---|---|
| Saturn v4 `Fixture fixes` (+ os avulsos de `Fixture Fixes/`), `FAI Cup fixes`, `French Championnat National fix` | datas e formato no calendário/banco atualizado (regra) |
| Saturn v5/v7/v8 `Awards fix`, `Awards fix update` | prêmios antes do 1º ano quando o início não é 2001 (só com troca de ano) |
| Saturn v5/v9 `Irish Leinster/Munster Senior Cup fix`, 2021 `Irish Leinster Senior Cup fix V2` | nomes de clubes do banco atualizado |
| Saturn v6 `Confederations Cup fixes` | ano e sede (regra) |
| Saturn v7 `EFL Cup fix` | defeito do próprio Saturn v6 (3º jogo na semi) |
| Saturn v8 `FIFA Club World Cup fix` | queda no sorteio do Mundial de Clubes reescrito pelo Saturn v5 sobre o código da antiga Copa da Liga alemã (0x5dc532); o byte de origem não bate com o 3.9.68 |
| Saturn v8 `Find Players fix` | o 3.9.68 já tem `mov ecx, 0xb74450` em 0x5a4207: conserta o Saturn |
| Saturn v8 `Physical training fix` | 0 de 87 bytes de origem batem com o 3.9.68 (0x8b3ce3.., training_manager.cpp): conserta código do Saturn |
| Saturn v8 `Transfer offer button sensitivity fix` | chama 0x966170, rotina posta pela `idlesensitivity`: conserta a convivência com o idle |
| Saturn v8 `match_eng error information`, `Error message suppression` | diagnóstico / caixa de erro calada |
| Saturn v5 `Loan_players_purchase_protection` | regra (não comprar emprestado) |
| Saturn v1 `Wembley fix`, `Republic Of Ireland fixes` | estádio/banco atualizado |
| Saturn v2 `Other` + Saturn v5 `Regen tweak` | regra: limite 50 → 45 para regens e `jne` → `jl` em 0x7a6f48; no 3.9.68 o laço de 0x7a6f10 para no limite, então `==` e `<` dão o mesmo resultado (não há defeito) |
| 2021 `xeno_fairedinkum_FCFWFix`, `Copa America 2024 Start`, `WorldCupAdjustments`; 2022 `wc_south_american_league_744_crash`; 2025 `davdav World Cup Winners` | anos, sedes e vagas das competições internacionais com o ano de início trocado ou o banco atualizado (o 744 troca 10 − x por 8 vagas na eliminatória sul-americana, 0x920f1d) |
| 2020..2022 `France/Italy Restructure`, 2021 `African Cup of Nations - 2021` | reestruturação; calam caixas de erro (match_eng 612, club_records 794, player_stats 1753, world_cup 1438) que a própria mudança provoca |
| linha "Portugal Cup would crash if not enough teams" dos "All Tested" | dado do banco (a Taça precisa de 184 clubes portugueses); reativa clubes, sem byte no exe |
