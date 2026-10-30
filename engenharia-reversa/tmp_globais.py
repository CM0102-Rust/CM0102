"""Os .tmp globais do save (os que não são de uma competição), campo a campo, na ordem da gravação do original.

Uso: python tmp_globais.py <save.sav> [trecho do nome]

Cada leitor segue a rotina que grava o arquivo e confere que a leitura acaba exatamente no fim.
Mapa em docs/MAPA_SAVE.md, seção 7. Os .tmp de competição ficam com tmp_competicao.py.
"""
import re
import struct
import sys

from prova_save import conteudo, indice
from tmp_competicao import Leitor

# Históricos em lista ligada no arquivo (FILE_LLIST, 0x58E280): cada nó é o registro seguido do
# deslocamento do próximo nó (i32, -1 no último). Tamanho do registro: o 2º argumento do construtor.
LISTAS = {
    'past_year_awards.tmp': 0x4A,          # AWARD_MANAGER 0x413180
    'past_team_awards.tmp': 0x205,         # AWARD_MANAGER 0x413180
    'club league history.tmp': 0x15,       # Club_History 0x442AB0
    'club cup history.tmp': 0x0B,          # Club_History 0x442AB0
    'club records history.tmp': 0x38D,     # Club_Records 0x444520
    'comp records history.tmp': 0x26B,     # Comp_Stats 0x49EE70
    'ban_history.tmp': 0x14,               # Discipline 0x5554A0
    'injury_history.tmp': 0x2C,            # Injury 0x62A680
    'player stats history.tmp': 0x2B,      # PlayerStats 0x7BFD50
    'staff history.tmp': 0x35,             # Staff_Records 0x868390
    'club history.tmp': 0x31,              # Staff_Records 0x868390
}


def lista_ligada(r, tam):
    nos = []
    while r.p < len(r.d):
        registro, prox = r.ler(tam), r.u32()
        if prox != -1 and (prox % (tam + 4) or prox >= len(r.d)):
            raise ValueError(f'próximo nó fora da grade em 0x{r.p - 4:x}')
        nos.append((registro, prox))
    return {'nos': nos}


def host_country(r):
    """HostCountry::save_temp_file (0x5FB320): quantos (u16) e 0x22 bytes por anfitrião."""
    n = r.u16()
    return {'anfitrioes (0x22)': [r.ler(0x22) for _ in range(n)]}


def club_dates(r):
    """fix_man_save_club_dates (0x5AF040): um u16 por clube, de [0xAE2A5C]; quantos = [0xAE235C]."""
    return {'datas (u16 por clube)': [r.u16() for _ in range(len(r.d) // 2)]}


def check_fixture_legs(r):
    """fix_man_save_fixture_leg_checks (0x5AF4A0): quantos (u16) e um registro de 0x0F por item da lista."""
    n = r.u16()
    return {'itens (0x0f)': [r.ler(0x0F) for _ in range(n)]}


def conmebol_seeding(r):
    """ConmebolSeeding::save_to_temp_file (0x4C6B70): quantos (+0x6E, u8), 0x0A por país, +0x6F (u16), 4 bytes."""
    n = r.u8()
    return {'paises (0x0a)': [r.ler(0x0A) for _ in range(n)], '+0x6f': r.u16(), 'final (4)': r.ler(4)}


def uefa_seeding(r):
    """UefaSeeding::save_to_temp_file (0x905510): países (+0x438, u8) de 0x12 bytes (UEFA_SEEDINGS), +0x439 (u16),
    quantos (+0x43F, u8) e essa quantidade de u32 de [+0x43B]; por fim 50 registros fixos de 0x0C de +0x440 (0x905706)."""
    n = r.u8()
    c = {'paises (0x12)': [r.ler(0x12) for _ in range(n)], '+0x439': r.u16()}
    m = r.u8()
    c['+0x43b (u32)'] = [r.u32() for _ in range(m)]
    c['tabela final (50 x 0x0c)'] = [r.ler(0x0C) for _ in range(0x32)]
    return c


def stadium(r):
    """Stadium::save_to_temp_file (0x85BEA0)."""
    c = {'+0x08': r.u32()}
    n = r.u32()
    c['+0x00 (0x0c cada)'] = [r.ler(0x0C) for _ in range(n)]
    c['+0x14'] = r.u32()
    m = r.u32()
    c['+0x18'] = r.u8()
    c['+0x0c (0x10 cada, clube em id)'] = [r.ler(0x10) for _ in range(m)]
    c['[0xA835A4]'] = r.u32()
    return c


def _contada(r, tam):
    """Lista com a contagem (u32) gravada antes e regravada no fim por fseek."""
    return [r.ler(tam) for _ in range(r.u32())]


def friendly(r):
    """Friendly::save_to_temp_file (0x5C9650)."""
    c = {'+0xa7 (8)': r.ler(8), '+0xaf': r.u16(), '+0x34dc': r.u32()}
    n = r.u32()
    c['+0x34e4'] = r.u32()
    c['+0x34d8 (0x14 cada)'] = [r.ler(0x14) for _ in range(n)]
    c['+0xb1 (127 x 0x69, clubes em id)'] = [r.ler(0x69) for _ in range(0x7F)]
    c['lista +0x34c8 (0x19)'] = _contada(r, 0x19)
    c['lista +0x34d0 (+0x64 do item, 1 byte)'] = _contada(r, 1)
    c['lista (8 + 4)'] = _contada(r, 12)
    c['lista +0x34cc (0x107)'] = _contada(r, 0x107)
    c['+0x34f4'] = r.u16()
    por_nacao = []                                  # uma lista por nação ([0xAE2350] nações): quantos (u16) e u32
    while r.p < len(r.d):
        por_nacao.append([r.u32() for _ in range(r.u16())])
    c['listas por nacao'] = por_nacao
    return c


def fixtures(r):
    """FIXTURE_MANAGER::save_fixtures (0x5AC060): quantos (u32, regravado no fim) e um registro de partida de 0x4F
    por jogo do ano (o MATCH_DATA da NickDLL)."""
    return {'partidas (0x4f)': _contada(r, 0x4F)}


def eventos(r):
    """Match_Events::save_event_log (0x6D98B0): arquivo rotativo. Cabeçalho de 3 u32 (o 3º, em +8, é regravado a
    cada partida anexada) e os eventos de 0x0E bytes de cada partida, anexados no fim; onde começa cada partida
    (deslocamento, quantos eventos, qual dos dois arquivos) fica na memória do jogo, não no arquivo."""
    c = {'cabecalho': [r.u32(), r.u32(), r.u32()]}
    if (len(r.d) - r.p) % 0x0E:
        raise ValueError('eventos fora da grade de 0x0e')
    c['eventos (0x0e)'] = [r.ler(0x0E) for _ in range((len(r.d) - r.p) // 0x0E)]
    return c


def match_stats_matches(r):
    """Match_Stats (0x729000): 10000 vagas fixas de 0xD7A bytes, lidas e gravadas por índice (0x729600/0x729820);
    o índice fica no match_stats.dat."""
    return {'vagas (0xd7a)': [r.ler(0xD7A) for _ in range(10000)]}


LEITORES = {
    'fixtures_%d.tmp': fixtures,
    'computer events1.tmp': eventos,
    'computer events2.tmp': eventos,
    'human events1.tmp': eventos,
    'human events2.tmp': eventos,
    'match_stats_matches.tmp': match_stats_matches,
    'uefa_seeding.tmp': uefa_seeding,
    'Stadium.tmp': stadium,
    'Friendly.tmp': friendly,
    'HostCountry.tmp': host_country,
    'club_dates.tmp': club_dates,
    'check_fixture_legs.tmp': check_fixture_legs,
    'conmebol_seeding.tmp': conmebol_seeding,
}


def ler(nome, dados):
    r = Leitor(dados)
    if nome in LISTAS:
        c = lista_ligada(r, LISTAS[nome])
    else:
        c = LEITORES[nome](r)
    return c, r.p


def main():
    if len(sys.argv) < 2:
        sys.exit(__doc__)
    d = open(sys.argv[1], 'rb').read()
    tipo, _, itens = indice(d)
    filtro = sys.argv[2] if len(sys.argv) > 2 else None
    fecham, falham, sem_leitor = [], [], []
    for nome, pos, tam in itens:
        if not nome.endswith('.tmp') or (filtro and filtro not in nome):
            continue
        chave = re.sub(r'^(fixtures|transfers)_\d+\.tmp$', lambda m: m.group(1) + '_%d.tmp', nome)
        if chave not in LISTAS and chave not in LEITORES:
            if chave != nome or not re.search(r'_\d+\.tmp$', nome):   # os de competição ficam com tmp_competicao.py
                sem_leitor.append(nome)
            continue
        dados = conteudo(d, tipo, pos, tam)
        try:
            c, fim = ler(chave, dados)
        except (ValueError, struct.error) as e:
            falham.append(f'{nome} ({e})')
            continue
        if fim != len(dados):
            falham.append(f'{nome} (sobram {len(dados) - fim})')
            continue
        fecham.append(nome)
        if filtro:
            print(f'{nome}: {len(dados)} bytes, lidos todos')
            for k, v in c.items():
                print(f'  {k}: {len(v) if isinstance(v, (list, bytes)) else v}')
    print(f'{len(fecham)} .tmp globais fecham; {len(falham)} não; {len(sem_leitor)} sem leitor')
    for f in falham:
        print('  não fecha:', f)
    for f in sem_leitor:
        print('  sem leitor:', f)


if __name__ == '__main__':
    main()
