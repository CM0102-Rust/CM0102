# Os prêmios argentinos: o objeto, as suas duas equipes da semana e o dia (award/argentina_awards.cpp; leitor rx13,
# rodada 6, 25/11/2026). Simulação com a Argentina carregada: construtor e init na criação do jogo, daily_update todo
# dia: LIGAS do grupo da Argentina e ATE = '2002-08-01'. Usa o molde comum do sx05b_5 (já no motor).
# - 0x40a350 ArgentinaAwards::ArgentinaAwards thiscall(arquivo) ret 4: NationAwards 0x768290, vtable 0x96748c,
#   +0x13 = 3, +0x12 = 1, +0xe = [KEY_NATION_ARGENTINA] na tabela das nações (x 0x122); com arquivo,
#   NationAwards::load 0x76a280 (lido: fim); senão init_awards 0x40a3e0(arquivo). Sem notas do mês. Prólogo push -1;
#   push imm32 (7 bytes).
# - 0x40a3e0 ArgentinaAwards::init_awards thiscall(arquivo) ret 4: +8 = 2, malloc(8) (nulo: caixa 0x5b); dois
#   TEAM_AWARD 0x8af520 (new 0x25d): (id, vaga, [0x9cfc44]/[0x9cfc48], 2000/1000, 0, [0x9cf654]/[0x9cf658], 1, 0,
#   arquivo, 1), cada push conferido. A vaga do segundo é 1 só quando o primeiro new deu certo (a cópia fixa as vagas).
#   Prólogo mov eax, fs:[0] (6).
# - 0x40a550 ArgentinaAwards::daily_update thiscall(hoje) ret 4: só NationAwards::daily_update 0x768310(hoje) no mesmo
#   ecx. Prólogo mov eax, [esp+4]; push eax (5 bytes; o call vem depois).
# Iguais no cm0102-1080.exe. Comparadores: só a original (alocam; os prêmios do mês saem uma vez), com o plano do molde
# no construtor (vtable, id 3, tipo 1, contagem 2) e no init (cada prêmio com o id e a vaga).
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
ROTINAS = [('argentina_awards_construct', 0x40a350, [0x6a, 0xff, 0x68, 0xb8, 0x36, 0x95, 0x00]),
           ('argentina_awards_init', 0x40a3e0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('argentina_awards_daily_update', 0x40a550, [0x8b, 0x44, 0x24, 0x04, 0x50])]
DOCS = ('os prêmios argentinos: o objeto, as suas duas equipes da semana e o dia',
        'the Argentine awards: the object, its two team awards and the day',
        [('argentina_awards_construct', 'ArgentinaAwards::ArgentinaAwards', '0x40a350', 'award/argentina_awards.cpp', 'thiscall, ret 4',
          'regra: os prêmios argentinos são da Argentina; de um jogo salvo são lidos, senão criados',
          'rule: the Argentine awards object is built for Argentina; from a saved game it loads its awards, otherwise it creates them'),
         ('argentina_awards_init', 'ArgentinaAwards::init_awards', '0x40a3e0', 'award/argentina_awards.cpp', 'thiscall, ret 4',
          'regra: a Argentina tem dois prêmios: a equipe da semana da primeira divisão (reputação nacional 2000) e da segunda (1000), sem reputação mundial, para jogadores',
          'rule: Argentina has two awards, the team of the week of the first division (national reputation 2000) and of the second division (1000), no world reputation, for players'),
         ('argentina_awards_daily_update', 'ArgentinaAwards::daily_update', '0x40a550', 'award/argentina_awards.cpp', 'thiscall, ret 4',
          'regra: o dia dos prêmios argentinos é só a virada do mês comum a todas as nações',
          "rule: the Argentine awards' day is only the month change of every nation's awards")])
CAMPOS = []
INTERESSA = []
