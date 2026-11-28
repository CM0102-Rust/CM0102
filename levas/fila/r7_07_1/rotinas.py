# A aplicacao do player_setup.cfg na criacao do jogo (player_regen.cpp; leitor r7_07, rodada 7, 28/11/2026): estava
# adiada no levas/fila/rx10_15/FORA.md e no sobras.md (sx16) por pedir um player_setup.cfg nos dados. O jogo_testes tem
# o do jogo (Data/player_setup.cfg, 21151 bytes: 184 INJURY, 49 INT_RETIREMENT, 56 LOAN e 1 MISCONDUCT), lido pela
# criacao do jogo (0x5cc87c.., mascaras 0x100, 0x1000, 0x800, 0x200; 0x5ccc9c mascara 0x203b; 0x81b9e2/0x81b9ed
# 0x80 e 0x400; a do -verifyplcfg, 0x828349, mascara 0x203b com report 1). Simulacao: a carreira de teste cria o jogo do
# zero e aplica as lesoes, os emprestimos, a suspensao e as aposentadorias internacionais.
# - 0x7a3a10 player_setup_apply_line (5760 bytes, ate o ret em 0x7a5085; quadro SEH com estados para as datas locais,
#   sem destrutor): le uma linha (utils_read_config_line) e a aplica pelo tipo. Inteira num desvio, em pedacos com nome
#   (decodificar a linha numa SetupAction, depois executar).
# - 0x7a30c0 player_setup_cfg_load (128 bytes, ate o ret em 0x7a3137): abre CM3_DATA\player_setup.cfg e aplica cada
#   linha ate o fim.
# Prologos: 0x7a3a10 push -1 / push 0x95f134 (7 bytes), 0x7a30c0 sub esp, 0x3e8 (6 bytes); sem relativas. Iguais no
# cm0102-1080.exe (as duas inteiras, conferido byte a byte).
# Convencoes conferidas na desmontagem:
#   0x7a3a10 cdecl(arquivo, linha, mascara, report) -> al (0 no fim do arquivo, 1 nos outros); chamada so pela 0x7a30c0
#   (0x7a3105, 0x7a3119, add esp, 0x10). 0x7a30c0 cdecl(mascara, report), sem retorno usado.
#   Dentro: 0x90cba0 cdecl(arquivo, buf); 0x7a5090/0x7a3140/0x7a50f0/0x7a32e0 (ja Rust, chamadas direto); 0x946fd8
#   sscanf variadica; 0x945a60 strstr; 0x944c9f sprintf + 0x5e8290 cdecl("Error", texto, 0); 0x54a110/0x549f70/0x54cdb0/
#   0x417c10 (Date); 0x556940 add_ban, 0x62d250 add_injury, 0x7a8510 add_retirement, 0x8bf400 add_config_loan,
#   0x419840 awol_start_absence, 0x69f7c0 set_board_ratings, 0x547e70 staff_add_disliked_club e 0x53b3d0 (ja Rust,
#   chamadas direto); 0x8bf060 thiscall([0xde1f54], pessoa, clube, data, taxa, salario, anos) ret 0x18 (pelo endereco).
# O original passa como dword valores lidos por %hd (o dia, mes e ano a Date::Date, que le a palavra; o valor do
# MANAGER_CONF, que vira u16; os anos de contrato, lidos como byte) e o tipo da lesao no al: as partes de cima sao lixo
# de pilha. A copia passa os valores limpos.
# Leitura de memoria: a linha, as pessoas, os clubes, os registros de 79 bytes; os locais sao todos gravados antes de
# lidos (fora os de sscanf que encontra o fim da linha, ver DEFEITOS).
# Comparadores: 0x7a3a10 le a mesma linha antes (ftell, utils_read_config_line, fseek de volta), decide com o
# decodificador da copia, roda a original e confere a linha lida e o efeito decidido (marcas, as marcas de regen, a
# suspensao no vetor do Discipline, a lesao no registro fisico, as notas da diretoria, a aposentadoria pendente e o
# estado "anunciada", o emprestimo); no report so a original. 0x7a30c0 so a original (abre o arquivo).
SAVE = False
ROTINAS = [('player_setup_apply_line', 0x7a3a10, [0x6a, 0xff, 0x68, 0x34, 0xf1, 0x95, 0x00]),
           ('player_setup_cfg_load', 0x7a30c0, [0x81, 0xec, 0xe8, 0x03, 0x00, 0x00])]
DOCS = ('o player_setup.cfg aplicado na criação do jogo: cada linha pelo tipo (lesão, suspensão, aposentadoria, transferência, empréstimo, fuga e as marcas)',
        'the player_setup.cfg applied on game creation: each line by type (injury, ban, retirement, transfer, loan, absence and the flags)',
        [('player_setup_apply_line', 'player_setup_apply_line', '0x7a3a10', 'player_regen.cpp', 'cdecl',
          'lê a próxima linha do player_setup.cfg (sem os comentários) e a aplica quando o tipo está na máscara, ou sempre no modo de verificação (-verifyplcfg); devolve 0 no fim do arquivo. A pessoa pelos três nomes no elenco e no técnico do clube nomeado (o MANAGER_CONF só no técnico); não achada, só na verificação, uma caixa "Error" dizendo em que clube estão os nomes ou que não existem. AWOL: a pessoa foge (também na verificação). PRESET_TACTICS, COLOURBLIND e GERMAN: a marca de cada um. ARSE: a marca de regen em todas as pessoas. DRUGS/NANDROLONE, MISCONDUCT e FALSE_PASSPORT: uma suspensão por dias até a data (motivos 5, 3 e 6, a regra da FIFA), ano até 2010 e não antes do atual, também na verificação. INJURY, só jogadores e fora da verificação: a lesão pelo nome, ou "PERMANENT_<nome> n" com o n, ou "FUTURE_PERMANENT_<nome> n" só como recorrência. MANAGER_CONF: as quatro notas da diretoria do clube quando a pessoa é o técnico dele. RETIREMENT: a aposentadoria pendente na data (motivo 0x20, anunciada hoje) e o estado "anunciada". INT_RETIREMENT: a seleção da pessoa entre os clubes de que ela não gosta (forçado). FUTURE_TRANSFER, só jogadores: a transferência para o clube na data com taxa, salário e anos (até 10). LOAN: o empréstimo ao clube entre as duas datas. Outro tipo: a caixa "unknown type". Datas "dia mês ano", ano de dois dígitos 20xx abaixo de 10 e 19xx abaixo de 100, anos de 1980 a 2010',
          'reads the next player_setup.cfg line (comments skipped) and applies it when its type is in the mask, or always in the verify mode (-verifyplcfg); returns 0 at the end of the file. The person by the three names in the named club\'s squad and manager (MANAGER_CONF only the manager); not found, only when verifying, an "Error" box saying in which club the names are or that they are nowhere. AWOL: the person goes absent (in the verify mode too). PRESET_TACTICS, COLOURBLIND and GERMAN: their flag. ARSE: the regen mark on every person. DRUGS/NANDROLONE, MISCONDUCT and FALSE_PASSPORT: a ban by days until the date (reasons 5, 3 and 6, the FIFA rule), year up to 2010 and not before the current one, in the verify mode too. INJURY, players only and not when verifying: the injury by name, or "PERMANENT_<name> n" with n, or "FUTURE_PERMANENT_<name> n" as a recurrence only. MANAGER_CONF: the club\'s four board ratings when the person is its manager. RETIREMENT: the pending retirement on the date (reason 0x20, announced today) and the "announced" state. INT_RETIREMENT: the person\'s national side among his disliked clubs (forced). FUTURE_TRANSFER, players only: the transfer to the club on the date with fee, wages and years (up to 10). LOAN: the loan to the club between the two dates. Any other type: the "unknown type" box. Dates "day month year", two-digit years 20xx below 10 and 19xx below 100, years 1980 to 2010'),
         ('player_setup_cfg_load', 'player_setup_cfg_load', '0x7a30c0', 'player_regen.cpp', 'cdecl',
          'abre CM3_DATA\\player_setup.cfg e aplica cada linha cujo tipo está na máscara até o fim do arquivo; sem o arquivo, nada',
          'opens CM3_DATA\\player_setup.cfg and applies every line whose type is in the mask until the end of the file; without the file, nothing')])
CAMPOS = [('registro de 79 bytes ([0xae2d0c]) +0xb: 0x40 marca de regen (ARSE), 0x30 estado da aposentadoria (RETIREMENT: 0x10 anunciada, 0x20 tirado); grava',
           '79-byte record ([0xae2d0c]) +0xb: 0x40 regen mark (ARSE), 0x30 retirement state (RETIREMENT: 0x10 announced, 0x20 cleared); writes'),
          ('marcas do player_setup.cfg: 0xb7434a PRESET_TACTICS, 0xb74349 COLOURBLIND, 0xb74348 GERMAN; byte; grava 1',
           'player_setup.cfg flags: 0xb7434a PRESET_TACTICS, 0xb74349 COLOURBLIND, 0xb74348 GERMAN; byte; writes 1')]
INTERESSA = ['CMRTE', 'SGE']
