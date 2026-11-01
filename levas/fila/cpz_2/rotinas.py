# As divisões inglesas na tabela (eng_second.cpp, eng_first.cpp, eng_third.cpp; a família de league.cpp está quase
# esgotada e as ligas inglesas não têm dono na fila): EngSecond::update_staff_records_for_status (0x576c50, a virtual
# +0x44 da First, da Second e da Third, chamada pelo anúncio da situação 0x686670 quando a 0x684cd0 decide o título,
# o acesso, a vaga no mata-mata ou o rebaixamento, de março a maio de 2002, e pelo mata-mata), EngFirst::set_club_progress
# (0x573260, a virtual +0x68 da First e da Second) e EngThird::set_club_progress (0x578660, a da Third), chamadas pela
# 0x684cd0 a cada linha da tabela em toda atualização. Sem SEH; iguais no cm0102-1080.exe.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x868c50 (add_league_champions_record): push comp; push clube; ecx = [0xdd7f00]; sem add esp -> thiscall(reg,
#   clube, comp), ret 8.
# - 0x869480 (add_promotion_record): push 0x64 (ou 0x32); push comp; push clube; ecx = [0xdd7f00] -> thiscall(reg,
#   clube, comp, peso), ret 0xc.
# - 0x86bdd0 (add_qualified_for_round_record): push 0x1e; push 0x82; push 0xa0; push comp; push clube; ecx =
#   [0xdd7f00] -> thiscall(reg, clube, comp, fase, rodada, peso), ret 0x14.
# - 0x86a1c0 (add_relegation_record): push comp; push clube; ecx = [0xdd7f00] -> thiscall(reg, clube, comp), ret 8.
# - 0x4a4850 (Comp_Stats::find_club_entry): push clube; ecx = [this + 8]; sem add esp -> thiscall(stats, clube), ret 4.
# Comparadores: a 0x576c50 só a original (os registros gravam o histórico no arquivo, sorteiam o moral e renovam
# contratos); o plano confere o que ela mesma grava no mata-mata (+0x37 da linha do clube e o `saiu`). As duas
# set_club_progress lado a lado nos 3 bytes da entrada do clube (+0x73..+0x75), achada antes.
SAVE = False
ROTINAS = [('registros_pela_situacao_inglesa', 0x576c50, [0x8a, 0x44, 0x24, 0x0c, 0x53]),
           ('situacao_na_first', 0x573260, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('situacao_na_third', 0x578660, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('as divisões inglesas na tabela: os registros pela situação e a situação do clube nas estatísticas da First e da Third',
        'the English divisions in the table: the records by status and the club progress in the First and Third stats',
        [('registros_pela_situacao_inglesa', 'EngSecond::update_staff_records_for_status', '0x576c50', 'eng_second.cpp', 'thiscall, ret 0x18',
          'na liga (fase 0xff): título (5) 0x868c50, acesso (0) 0x869480 com peso 0x64, vaga no mata-mata (1) 0x86bdd0 (0xa0, 0x82, 0x1e), rebaixamento (3) 0x86a1c0; no mata-mata, na linha do clube: venceu (1) o acesso com 0x32, +0x37 = 0 e saiu = 1; passou (0) 0x86bdd0 (0xa0, 0x96, 0xf); caiu (0xff, 2) +0x37 = 0xfe; devolve 0',
          'in the league (stage 0xff): title (5) 0x868c50, promotion (0) 0x869480 with weight 0x64, play-off place (1) 0x86bdd0 (0xa0, 0x82, 0x1e), relegation (3) 0x86a1c0; in the play-offs, on the club row: won (1) promotion with 0x32, +0x37 = 0 and removed = 1; went through (0) 0x86bdd0 (0xa0, 0x96, 0xf); out (0xff, 2) +0x37 = 0xfe; returns 0'),
         ('situacao_na_first', 'EngFirst::set_club_progress', '0x573260', 'eng_first.cpp', 'thiscall, ret 0x14',
          'grava fase, melhor e pior na entrada do clube nas estatísticas (+0x73..+0x75): na liga, melhor de 3 a 6 vira 3 e pior de 3 a 6 vira 6; no mata-mata (fase 0), 1, 2 e 3 viram 3, 4 e 5',
          'writes stage, best and worst in the club stats entry (+0x73..+0x75): in the league, best 3 to 6 becomes 3 and worst 3 to 6 becomes 6; in the play-offs (stage 0), 1, 2 and 3 become 3, 4 and 5'),
         ('situacao_na_third', 'EngThird::set_club_progress', '0x578660', 'eng_third.cpp', 'thiscall, ret 0x14',
          'como a da First, com as vagas 4 a 7 (4 e 7; no mata-mata, 4, 5 e 6)',
          'like the First one, with places 4 to 7 (4 and 7; in the play-offs, 4, 5 and 6)')])
CAMPOS = [('liga +0xb1 / +0x3e: a tabela (0x3b bytes; +0 o clube, +0x37 a situação) e as linhas; ponteiro e valor; lê, grava +0x37 (registros_pela_situacao_inglesa)',
           'league +0xb1 / +0x3e: the table (0x3b bytes; +0 club, +0x37 status) and the row count; pointer and value; reads, writes +0x37 (registros_pela_situacao_inglesa)'),
          ('liga +4: o registro da competição; ponteiro; lê (registros_pela_situacao_inglesa)',
           'league +4: the competition record; pointer; reads (registros_pela_situacao_inglesa)'),
          ('liga +8: as estatísticas da competição (Comp_Stats); ponteiro; lê (situacao_na_first, situacao_na_third)',
           'league +8: the competition stats (Comp_Stats); pointer; reads (situacao_na_first, situacao_na_third)'),
          ('entrada do clube nas estatísticas +0x73 / +0x74 / +0x75: a fase, a melhor e a pior situação; valor; grava (situacao_na_first, situacao_na_third)',
           'club stats entry +0x73 / +0x74 / +0x75: the stage, best and worst progress; value; writes (situacao_na_first, situacao_na_third)'),
          ('[0xdd7f00] os registros de staff; ponteiro; lê (registros_pela_situacao_inglesa)',
           '[0xdd7f00] the staff records; pointer; reads (registros_pela_situacao_inglesa)')]
INTERESSA = ['CMRTE']
