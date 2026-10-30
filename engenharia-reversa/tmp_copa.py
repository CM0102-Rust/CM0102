"""O .tmp de uma copa, campo a campo, na ordem de Cup::save_cup_to_disk (0x5210F0).

Uso: python tmp_copa.py <save.sav> [trecho do nome]     (sem nome: todos os .tmp de copa simples)

Lê cada .tmp de copa do save, segue a ordem da gravação do original e confere que o arquivo
acaba exatamente onde a leitura acaba. Copas com estágios (grupos, play-offs) ou com dados
próprios da classe derivada (slot 10 da vtable) não fecham e são contadas à parte.
Mapa em docs/MAPA_SAVE.md, seção "O .tmp de uma copa".
"""
import struct
import sys

from prova_save import conteudo, indice


class Leitor:
    def __init__(self, dados):
        self.d, self.p = dados, 0

    def ler(self, n):
        if self.p + n > len(self.d):
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


def copa(dados):
    r = Leitor(dados)
    c = {'nome (+0x53, 0x50)': r.ler(0x50).split(b'\0')[0].decode('cp1252')}
    if r.u32():
        c['estatisticas'] = comp_stats(r)
    # aqui entra o slot 10 da vtable (+0x24): nada na base (0x48CEA0 = ret 4)
    for campo, tam in (('+0x28 premio', 4), ('+0x52', 1), ('+0x51', 1), ('+0x4b', 1), ('+0x4c', 4), ('+0x24', 4),
                       ('+0x49 banco', 1), ('+0x4a substituicoes', 1), ('+0x45', 4), ('+0xb1', 1), ('+0x3c rodadas', 2),
                       ('+0x3e times', 2), ('+0x40 ano', 2), ('+0xab', 1), ('+0xac', 1), ('+0x42 tipo', 1),
                       ('+0x3a estagio principal', 2), ('+0x1c sobe para', 4), ('+0x20 cai para', 4),
                       ('+0x18 ponteiro da media', 4), ('+0x34 linhas da media', 2)):
        c[campo] = {1: r.u8, 2: r.u16, 4: r.u32}[tam]()
    if c['+0x18 ponteiro da media']:
        c['media de pontos (0x13 cada)'] = r.ler(0x13 * c['+0x34 linhas da media'])
    c['id da competicao'] = r.u32()
    rodadas = [r.ler(0x68) for _ in range(c['+0x3c rodadas'])]
    c['rodadas (0x68)'] = rodadas
    c['inscritos (id do clube, classe, marca)'] = [struct.unpack('<iBB', r.ler(6)) for _ in range(c['+0x3e times'])]
    # confrontos de cada rodada: 0x35 bytes por jogo, quantidade em +0x1a da rodada
    c['confrontos (0x35)'] = [r.ler(0x35 * struct.unpack_from('<h', rd, 0x1A)[0]) for rd in rodadas]
    c['+0x2c estagios'], c['+0x30 estagio atual'], c['+0x44'] = r.u32(), r.u32(), r.u8()
    if c['+0x30 estagio atual'] >= 0:
        raise ValueError('copa com estágios (Cup_Stage 0x523EB0 / League_Stage 0x68CA40): fora deste leitor')
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
    for nome, pos, tam in itens:
        if not nome.endswith('.tmp') or (filtro and filtro not in nome):
            continue
        dados = conteudo(d, tipo, pos, tam)
        try:
            c, fim = copa(dados)
        except (ValueError, struct.error, UnicodeDecodeError) as e:
            falham += 1
            if filtro: print(f'{nome}: não fecha ({e})')
            continue
        if fim != len(dados):
            falham += 1
            if filtro: print(f'{nome}: sobram {len(dados) - fim} bytes')
            continue
        fecham += 1
        if filtro:
            print(f'{nome}: {len(dados)} bytes, lidos todos')
            for k, v in c.items():
                if isinstance(v, (bytes, list, dict)):
                    v = f'{len(v)} itens' if isinstance(v, (list, dict)) else f'{len(v)} bytes'
                print(f'  {k}: {v}')
            print('  inscritos:', c['inscritos (id do clube, classe, marca)'])
    print(f'{fecham} .tmp fecham pela ordem da copa simples; {falham} não (ligas, copas com estágios ou classe derivada)')


if __name__ == '__main__':
    main()
