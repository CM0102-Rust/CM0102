# O construtor e a virada da Copa da Holanda (hol_cup.cpp; leitor sbF, rodada 5, 24/11/2026). Competicoes da Holanda;
# ATE 2002-08-01 (o construtor na criacao do jogo; a virada no fim da temporada 2001/02, em junho/julho de 2002).
# - 0x5f3450 HolCup::HolCup thiscall(ano, registro) ret 8: sobre League::League (0x682200) o registro, a vtable
#   0x96b7b8, o ano, +0xee = 0, 3 substituicoes, 7 reservas, a disciplina HOLLAND (0xf), importancia 0xc; sem estado
#   gravado (0x687b10): flags 0x28, a fase principal, a fase atual -1, 20 fases (malloc 0x50), as estatisticas (caixa
#   0x54), as regras (HolCup::set_up_rules 0x5f3e70 direto; caixa 0x5b) e +0x45, +0x3e e +0xb1 zerados (os clubes vem
#   no primeiro dia completo, HolCup::update_for_date do sbF_7). SEH so para o new das estatisticas.
# - 0x5f3f10 HolCup::start_new_season thiscall() ret 0: +0x4c = 0, a tabela (0x687970(0)), as datas (+0xba), o
#   arquivo das estatisticas (0x4a1c50(1)), a lista de inscritos liberada e +0xee/+0xf2 zerados, as fases 0..atual
#   destruidas, o ano + 1, a fase -1, as regras (a virtual 0x8c; caixa 0x204, depois a qual o original devolve o eax
#   da caixa e a copia 0) e +0x45 = 0.
# Prologos: push -1; push 0x95ac99 (7 bytes); sub esp, 0x200 (6 bytes). Iguais no cm0102-1080.exe.
# Comparadores: so a original, com o plano (os campos fixos; o ano + 1).
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_hol_cup', 0x5f3450, [0x6a, 0xff, 0x68, 0x99, 0xac, 0x95, 0x00]),
           ('hol_cup_start_new_season', 0x5f3f10, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o construtor e a virada da Copa da Holanda',
        'the Dutch Cup constructor and turnover',
        [('construct_hol_cup', 'HolCup::HolCup', '0x5f3450', 'hol_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa da Holanda é uma copa jogada como liga com o grupo da própria copa, 19 fases de grupo e o mata-mata (20 vagas de fase), 7 reservas no banco e 3 substituições, sob a disciplina holandesa; sem estado gravado, as estatísticas e as regras, e a copa fica vazia (sem clubes nem tabela) até o primeiro dia completo',
          'rule: the Dutch Cup is a cup played as a league with the cup\'s own group, 19 group stages and the knockout (20 stage slots), 7 substitutes on the bench and 3 substitutions, under the Dutch discipline rules; with nothing saved, the statistics and the rules, and the cup stays empty (no clubs, no table) until its first full day'),
         ('hol_cup_start_new_season', 'HolCup::start_new_season', '0x5f3f10', 'hol_cup.cpp', 'thiscall, ret 0',
          'regra: na virada a Copa da Holanda é esvaziada: a tabela, as datas e os inscritos da edição anterior liberados, as estatísticas arquivadas, as fases destruídas, o ano +1 e as regras de novo; os clubes vêm no primeiro dia completo',
          'rule: at the season turn the Dutch Cup is emptied: last edition\'s table, dates and entrants freed, the statistics archived, the stages destroyed, the year +1 and the rules again; the clubs come on the first full day')])
CAMPOS = []
INTERESSA = []
