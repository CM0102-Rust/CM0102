"""O .tmp de uma copa, campo a campo, na ordem de Cup::save_cup_to_disk (0x5210F0).

Uso: python tmp_copa.py <save.sav> [trecho do nome]     (sem nome: todos os .tmp de copa simples)

Lê cada .tmp de copa do save, segue a ordem da gravação do original e confere que o arquivo
acaba exatamente onde a leitura acaba. Os estágios (de copa e de liga) e os dados
próprios de cada classe derivada (slot 10 da vtable) entram; a classe é a variante que fecha.
Mapa em docs/MAPA_SAVE.md, seção "O .tmp de uma copa".
"""
import struct
import sys

from prova_save import conteudo, indice


class Leitor:
    def __init__(self, dados):
        self.d, self.p = dados, 0

    def ler(self, n):
        if n < 0 or self.p + n > len(self.d):
            raise ValueError(f'passou do fim em 0x{self.p:x} (+{n})')
        b = self.d[self.p:self.p + n]
        self.p += n
        return b

    def u8(self): return self.ler(1)[0]
    def u16(self): return struct.unpack('<h', self.ler(2))[0]
    def u32(self): return struct.unpack('<i', self.ler(4))[0]


def comp_stats(r):
    """Comp_Stats::save (0x4AC4E0); o objeto tem 0x5CE bytes."""
    s = {'+0x10': r.u32()}
    n = s['+0x14 times'] = r.u32()
    s['tabela times (0x76 cada)'] = r.ler(0x76 * n)          # só gravada se o ponteiro +0x0C existe; vazio => n = 0
    s['+0x1d'] = r.u32()
    n = s['+0x21 linhas'] = r.u32()
    s['tabela +0x19 (0x2d cada)'] = r.ler(0x2D * n)
    s['+0x56a (0x64)'] = r.ler(0x64)
    s['recordes +0x26 (0x269)'] = r.ler(0x269)
    s['recordes +0x28f (0x269)'] = r.ler(0x269)
    s['+0x4f8'], s['+0x4fc'], s['+0x500'] = r.u32(), r.u32(), r.u16()
    s['+0x506 (0x5c)'] = r.ler(0x5C)
    s['+0x562'] = r.u32()
    n = s['+0x04 linhas'] = r.u32()
    s['+0x08'] = r.u32()
    s['tabela +0x00 (0x1c cada)'] = r.ler(0x1C * n)
    return s


def campos(r, lista):
    return {nome: {1: r.u8, 2: r.u16, 4: r.u32}[tam]() for nome, tam in lista}


def rodadas_e_confrontos(r, c, n_rodadas, n_times):
    """Rodadas de copa (0x68), inscritos (6, clube em id) e os confrontos de cada rodada (0x35 por jogo)."""
    rodadas = [r.ler(0x68) for _ in range(n_rodadas)]
    c['rodadas (0x68)'] = rodadas
    c['inscritos (id do clube, classe, marca)'] = [struct.unpack('<iBB', r.ler(6)) for _ in range(n_times)]
    c['confrontos (0x35)'] = [r.ler(0x35 * struct.unpack_from('<h', rd, 0x1A)[0]) for rd in rodadas]


def estagio_copa(r):
    """Cup_Stage::save_to_file (0x523EB0)."""
    c = campos(r, (('+0x28 premio', 4), ('+0x51', 1), ('+0x49 banco', 1), ('+0x4a substituicoes', 1), ('+0xb1', 1),
                   ('+0x3c rodadas', 2), ('+0x3e times', 2), ('+0x40 ano', 2), ('+0xab', 1), ('+0xac', 1), ('+0x44', 1),
                   ('+0x42 tipo', 1), ('+0x3a estagio', 2), ('+0x1c', 4), ('+0x20', 4), ('id da competicao', 4)))
    rodadas_e_confrontos(r, c, c['+0x3c rodadas'], c['+0x3e times'])
    return c


def estagio_liga(r):
    """League_Stage::save_to_file (0x68CA40). Também é a parte de liga do .tmp de uma liga."""
    c = campos(r, (('+0xe1', 1), ('+0x28 premio', 4), ('+0xdd', 4), ('+0xa7 jogos por time', 2), ('+0x51', 1),
                   ('+0x49 banco', 1), ('+0x4a substituicoes', 1), ('+0x3c turnos', 2), ('+0x3e times', 2), ('+0x40 ano', 2),
                   ('+0x43', 1), ('+0x42 tipo', 1), ('+0x3a estagio', 2), ('id da competicao', 4), ('+0xa9 rodadas', 2),
                   ('+0xc2 pontos vitoria', 1), ('+0xc3 pontos empate', 1), ('+0xc4', 1), ('+0xc5 desempates', 4),
                   ('+0xbe vagas', 4), ('+0xd9 bits', 2), ('+0xdb', 2), ('+0xab', 2), ('+0x1c sobe para', 4),
                   ('+0x20 cai para', 4), ('+0x44', 1)))
    n = c['+0x3e times']
    c['rodadas (0x41)'] = r.ler(0x41 * c['+0xa9 rodadas'])
    c['+0x18 ponteiro da media'], c['+0x34 linhas da media'] = r.u32(), r.u16()
    if c['+0x18 ponteiro da media']:
        c['media de pontos (0x13 cada)'] = r.ler(0x13 * c['+0x34 linhas da media'])
    c['tabela (0x3b, clube em id)'] = [r.ler(0x3B) for _ in range(n)]
    if r.u32():                                   # cada linha aponta (+5) para n - 1 registros de 0xe (um por adversário)
        c['confronto direto (0xe por adversario)'] = [r.ler(0xE * (n - 1)) for _ in range(n)]
    if r.u32():                                   # histórico de posições por rodada (+9 de cada linha)
        tam = c['+0xa7 jogos por time'] if c['+0xa7 jogos por time'] != -1 else (n - 1) * c['+0x3c turnos']
        c['posicoes por rodada'] = [r.ler(tam) for _ in range(n)]
    return c


def _ids(r, c):
    n = c['+0xb6 quantos'] = r.u32()
    c['+0xb2 clubes (id)'] = [r.u32() for _ in range(n)]
    return c


# dados próprios da classe derivada (slot 10 da vtable, +0x24); na base é vazio (0x48CEA0 = ret 4)
EXTRAS = {
    'Cup': lambda r: {},
    'UefaCup (0x903340; tambem EngCcCup, ScoLgeCup)': lambda r: {'+0xb2': r.u8()},
    'IntertotoCup (0x633F50)': lambda r: {'+0xb2': r.u32()},
    'EngFaCup (0x5708A0)': lambda r: {'+0xb3': r.u8(), '+0xb4': r.u8(), '+0xb2': r.u8()},
    'MiniCup (0x7664C0)': lambda r: {'+0xb2 (8)': r.ler(8), '+0xba': r.u16()},
    'WcAfricanCup (0x911110)': lambda r: _ids(r, {'+0xba': r.u8()}),
    'WcConcacafCup (0x917CD0)': lambda r: _ids(r, {'+0xba': r.u8(), '+0xbb': r.u8(), '+0xbc': r.u8()}),
}


def copa(dados, extra=EXTRAS['Cup']):
    r = Leitor(dados)
    c = {'nome (+0x53, 0x50)': r.ler(0x50).split(b'\0')[0].decode('cp1252')}
    if r.u32():
        c['estatisticas'] = comp_stats(r)
    c['dados da classe (slot 10)'] = extra(r)
    for campo, tam in (('+0x28 premio', 4), ('+0x52', 1), ('+0x51', 1), ('+0x4b', 1), ('+0x4c', 4), ('+0x24', 4),
                       ('+0x49 banco', 1), ('+0x4a substituicoes', 1), ('+0x45', 4), ('+0xb1', 1), ('+0x3c rodadas', 2),
                       ('+0x3e times', 2), ('+0x40 ano', 2), ('+0xab', 1), ('+0xac', 1), ('+0x42 tipo', 1),
                       ('+0x3a estagio principal', 2), ('+0x1c sobe para', 4), ('+0x20 cai para', 4),
                       ('+0x18 ponteiro da media', 4), ('+0x34 linhas da media', 2)):
        c[campo] = {1: r.u8, 2: r.u16, 4: r.u32}[tam]()
    if c['+0x18 ponteiro da media']:
        c['media de pontos (0x13 cada)'] = r.ler(0x13 * c['+0x34 linhas da media'])
    c['id da competicao'] = r.u32()
    rodadas_e_confrontos(r, c, c['+0x3c rodadas'], c['+0x3e times'])
    c['+0x2c estagios'], c['+0x30 estagio atual'], c['+0x44'] = r.u32(), r.u32(), r.u8()
    c['estagios'] = []
    for _ in range(c['+0x30 estagio atual'] + 1):  # do 0 até o atual, inclusive
        tipo = r.u8()                                # +0x43 do estágio
        if tipo == 1:
            n_times, n_rodadas = r.u16(), r.u16()
            jogos = [r.u16() for _ in range(n_rodadas)]   # o +0x1a de cada rodada, de novo
            c['estagios'].append(('copa', n_times, jogos, estagio_copa(r)))
        elif tipo == 2:
            c['estagios'].append(('liga', r.u16(), r.u16(), estagio_liga(r)))
        else:
            raise ValueError(f'estágio de tipo {tipo}')
    c['+0x38 capacidade do livro'], c['+0x36 inscritos no livro'] = r.u16(), r.u16()
    c['livro (0x14, 6 cada)'] = [struct.unpack('<iBB', r.ler(6)) for _ in range(c['+0x38 capacidade do livro'])]
    return c, r.p


def main():
    if len(sys.argv) < 2:
        sys.exit(__doc__)
    d = open(sys.argv[1], 'rb').read()
    tipo, _, itens = indice(d)
    filtro = sys.argv[2] if len(sys.argv) > 2 else None
    fecham = falham = 0
    por_classe = {}
    for nome, pos, tam in itens:
        if not nome.endswith('.tmp') or (filtro and filtro not in nome):
            continue
        dados = conteudo(d, tipo, pos, tam)
        classe = None
        for classe, extra in EXTRAS.items():      # vale a variante que fecha exatamente no fim do arquivo
            try:
                c, fim = copa(dados, extra)
            except (ValueError, struct.error, UnicodeDecodeError):
                continue
            if fim == len(dados):
                break
        else:
            falham += 1
            if filtro: print(f'{nome}: não fecha por nenhuma classe de copa')
            continue
        fecham += 1
        por_classe[classe] = por_classe.get(classe, 0) + 1
        if filtro:
            print(f'{nome}: {len(dados)} bytes, lidos todos, classe {classe}')
            for k, v in c.items():
                if isinstance(v, (bytes, list, dict)):
                    v = f'{len(v)} itens' if isinstance(v, (list, dict)) else f'{len(v)} bytes'
                print(f'  {k}: {v}')
            print('  inscritos:', c['inscritos (id do clube, classe, marca)'])
    print(f'{fecham} .tmp fecham pela ordem da copa; {falham} não (ligas e tabelas globais)')
    for classe, n in por_classe.items():
        print(f'  {classe}: {n}')


if __name__ == '__main__':
    main()
