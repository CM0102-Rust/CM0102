# O gestor dos premios: a passagem diaria e a de cada partida, e o construtor da base de um premio (award_manager.cpp;
# leitor sx03, rodada 5, 23/11/2026). Rodam na carreira ENG: process_date todo dia (G_AWARD_MANAGER, rotina diaria),
# process_fixture no fim de cada partida (match_day.rs: 0x4134c0 na lista dos que recebem a partida), AWARD::AWARD na
# criacao dos premios (inicio da carreira) pelos construtores dos premios do mes, do ano e de equipe.
# Convencoes conferidas na desmontagem:
# - 0x4134c0: thiscall(gestor, partida, registro), ret 8; virtual +8 do objeto da nacao (thiscall, ret 8);
#   0x9315d0 YEAR_RATINGS::record_match_ratings thiscall(partida, registro, filtro), ret 0xc (Rust, desviada);
# - 0x413540: thiscall(gestor, hoje), ret 4; virtual +4 (thiscall, ret 4); 0x931aa0 thiscall(hoje), ret 4;
# - 0x412e40: thiscall com 8 argumentos, ret 0x20; o setimo nao e lido.
# Comparadores: process_fixture e process_date so a original (os premios alocam, mandam noticia e mexem em reputacao);
# AWARD::AWARD lado a lado nos 0x1d bytes da base.
SAVE = False
ROTINAS = [
    ('award_manager_process_fixture', 0x4134c0, [0x53, 0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14]),
    ('award_manager_process_date', 0x413540, [0x53, 0x55, 0x8b, 0x6c, 0x24, 0x0c]),
    ('construct_award', 0x412e40, [0x8b, 0xc1, 0x56, 0x8b, 0x4c, 0x24, 0x10]),
]
DOCS = ('o gestor dos prêmios: a passagem diária, a de cada partida e a base de um prêmio',
        "the award manager: the daily pass, the per-match pass and an award's base",
        [('award_manager_process_fixture', 'award_manager_process_fixture', '0x4134c0', 'award_manager.cpp', 'thiscall, ret 8',
          'regra: amistoso (categoria 1) não conta; a nação é a da competição da partida (nenhuma sem partida, competição ou nação); o primeiro dos 30 objetos de prêmios de nação cuja nação é essa recebe a partida pela virtual +8 (as notas do mês); sem nação, o primeiro objeto sem nação (o internacional). Depois as notas do ano mundiais (+0x10, jogadores com reputação atual acima de 2000) e as das nações escolhidas (+0xc: Dinamarca, Bélgica, França, Holanda, Suécia), cada uma se existir',
          "rule: a friendly (category 1) counts for nothing; the nation is the match competition's (none without a fixture, competition or nation); the first of the 30 nation award objects whose nation is that one gets the match through its virtual +8 (the month ratings); with no nation, the first object with no nation (the international one). Then the world year ratings (+0x10, players above 2000 of current reputation) and the selected nations' ones (+0xc: Denmark, Belgium, France, Holland, Sweden), each when it exists"),
         ('award_manager_process_date', 'award_manager_process_date', '0x413540', 'award_manager.cpp', 'thiscall, ret 4',
          'regra: cada um dos 30 objetos de prêmios de nação que existe roda a sua atualização diária (virtual +4), na ordem das vagas; com a marca +0xdc, o bloco de estado dos prêmios passados (+0x1c, 0x30 dwords) e a marca são zerados; depois as duas notas do ano zeram se o ano é o delas',
          "rule: each of the 30 nation award objects that exists runs its daily update (virtual +4), in slot order; with the flag +0xdc, the past award state block (+0x1c, 0x30 dwords) and the flag are cleared; then both year ratings reset when the year is theirs"),
         ('construct_award', 'construct_award', '0x412e40', 'award_manager.cpp', 'thiscall, ret 0x20',
          'regra: a base de um prêmio: vtable 0x967864; +5 a competição de pessoas (registros de 101 bytes, sem conferir −1); +9 a competição de clubes (registros de 107 bytes) ou 0 para −1; +0xd e +0x11 os dois valores da classe; reputação nacional (+0x15) e mundial (+0x17), palavras baixas; +0x19 a marca de prêmio de jogador; o sétimo argumento não é lido',
          "rule: an award's base: vtable 0x967864; +5 the staff competition (101-byte records, no check for −1); +9 the club competition (107-byte records) or 0 for −1; +0xd and +0x11 the class's two values; national (+0x15) and world (+0x17) reputation, low words; +0x19 the player award flag; the seventh argument is not read")])
CAMPOS = []
INTERESSA = []
