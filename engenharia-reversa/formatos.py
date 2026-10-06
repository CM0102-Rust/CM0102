r"""Formatos dos arquivos de dados (Data\*.dat): tamanhos medidos x estruturas do CM0102Patcher.

Uso: python formatos.py <pasta Data> <saida.md>

1. Le o index.dat (8 bytes + entradas de 67: nome[51], tipo, quantidade, deslocamento, versao) e
   mede o tamanho de registro de cada tabela (tamanho do trecho / quantidade).
2. Le as estruturas [StructLayout(Pack = 1)] de referencias/CM0102Patcher/SaveChanger/Structures.cs
   e calcula deslocamento e tamanho de cada campo.
3. Confere: estrutura cujo total bate com o registro medido = confirmada.
Gera uma pagina Markdown com a tabela de arquivos e o layout de cada estrutura.
"""
import os
import re
import struct
import sys

AQUI = os.path.dirname(os.path.abspath(__file__))
ESTRUTURAS = os.path.join(AQUI, '..', 'referencias', 'CM0102Patcher', 'SaveChanger', 'Structures.cs')
TAMANHOS = {'int': 4, 'uint': 4, 'Int32': 4, 'UInt32': 4, 'short': 2, 'ushort': 2, 'Int16': 2, 'UInt16': 2,
            'byte': 1, 'sbyte': 1, 'Byte': 1, 'SByte': 1, 'char': 1, 'long': 8, 'ulong': 8, 'Int64': 8, 'UInt64': 8,
            'float': 4, 'double': 8, 'bool': 1}

# Qual estrutura descreve cada tabela do index (tipo -> nome da classe).
TABELA_ESTRUTURA = {0: 'TClub', 1: 'TClub', 3: 'TContinent', 4: 'TNation', 5: 'TStadiums', 6: 'TStaff', 9: 'TNonPlayer',
                    10: 'TPlayer', 11: 'TStaffComp', 12: 'TComp', 13: 'TNames', 14: 'TNames', 15: 'TNames', 16: 'TComp',
                    17: 'TStaffHistory', 18: 'TStaffCompHistory', 19: 'TCompHistory', 20: 'TCompHistory', 21: 'TCities'}


def ler_index(pasta):
    d = open(os.path.join(pasta, 'index.dat'), 'rb').read()
    entradas, pos = [], 8
    while pos + 67 <= len(d):
        nome = d[pos:pos + 51].split(b'\0')[0].decode('latin1')
        tipo, qtd, ofs, ver = struct.unpack_from('<4i', d, pos + 51)
        entradas.append(dict(nome=nome, tipo=tipo, qtd=qtd, ofs=ofs, ver=ver))
        pos += 67
    # Fim de cada trecho: proximo deslocamento no mesmo arquivo, ou o fim do arquivo.
    for e in entradas:
        tam_arq = os.path.getsize(os.path.join(pasta, e['nome']))
        fins = sorted({x['ofs'] for x in entradas if x['nome'] == e['nome'] and x['ofs'] > e['ofs']}) or [tam_arq]
        e['bytes'] = fins[0] - e['ofs']
        e['registro'] = e['bytes'] / e['qtd'] if e['qtd'] else None
    return entradas


def ler_estruturas():
    texto = open(ESTRUTURAS, encoding='utf-8').read()
    estruturas = {}
    for m in re.finditer(r'public class (T\w+)\s*\{', texto):
        nome, i, nivel = m.group(1), m.end(), 1
        j = i
        while nivel:
            nivel += {'{': 1, '}': -1}.get(texto[j], 0)
            j += 1
        corpo = texto[i:j - 1]
        corpo = re.sub(r'(public|private)[^;{]*\([^)]*\)\s*\{', '{', corpo)  # tira cabecalhos de metodos
        campos, profundidade, pendente = [], 0, None
        for linha in corpo.splitlines():
            s = linha.strip()
            if profundidade == 0:
                a = re.search(r'SizeConst\s*=\s*(\d+)', s)
                if a:
                    pendente = int(a.group(1))
                s = re.sub(r'^(\[[^\]]*\]\s*)+', '', s)  # atributo na mesma linha do campo
                f = re.match(r'public\s+([\w\[\]]+)\s+(\w+)\s*;', s)
                if f:
                    tipo, campo = f.groups()
                    campos.append((campo, tipo, pendente))
                    pendente = None
            profundidade += s.count('{') - s.count('}')
        estruturas[nome] = campos
    # Tamanhos (TCMDate e outras classes aninhadas resolvidas recursivamente).
    tamanhos = {}

    def tamanho(nome):
        if nome in tamanhos:
            return tamanhos[nome]
        total = 0
        for campo, tipo, n in estruturas[nome]:
            total += tamanho_campo(tipo, n)
        tamanhos[nome] = total
        return total

    def tamanho_campo(tipo, n):
        base = tipo.rstrip('[]')
        un = TAMANHOS.get(base) or (tamanho(base) if base in estruturas else None)
        if un is None:
            return 0
        return un * (n or 1) if tipo.endswith('[]') else un

    layouts = {}
    for nome in estruturas:
        ofs, linhas = 0, []
        for campo, tipo, n in estruturas[nome]:
            t = tamanho_campo(tipo, n)
            linhas.append((ofs, t, tipo + ('' if not tipo.endswith('[]') else '[%s]' % n), campo))
            ofs += t
        layouts[nome] = (ofs, linhas)
    return layouts


def main():
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    entradas = ler_index(sys.argv[1])
    layouts = ler_estruturas()
    out = ['# Formatos dos arquivos de dados (`Data\\*.dat`)', '',
           'Gerado por `engenharia-reversa/formatos.py`. Tamanhos medidos nos arquivos da instalação 3.9.68; '
           'layouts das estruturas lidos de `referencias/CM0102Patcher/SaveChanger/Structures.cs` (nomes de campo do '
           'nckstwrt). Uma estrutura é marcada como **confirmada** quando o total dela bate com o registro medido.', '',
           '## `index.dat`', '',
           '8 bytes de cabeçalho e entradas de 67 bytes: nome (51, terminado em zero), tipo (i32), quantidade (i32), '
           'deslocamento no arquivo (i32), versão (i32). O `staff.dat` guarda três tabelas em sequência '
           '(tipos 6, 9 e 10) e uma vazia (8).', '',
           '| Arquivo | Tipo | Registros | Deslocamento | Versão | Bytes/registro | Estrutura | Confere |',
           '|---|---:|---:|---:|---:|---:|---|---|']
    for e in entradas:
        est = TABELA_ESTRUTURA.get(e['tipo'])
        tam_est = layouts[est][0] if est in layouts else None
        reg = e['registro']
        confere = '—' if reg is None or tam_est is None else ('**sim**' if abs(reg - tam_est) < 1e-9 else 'não (%d)' % tam_est)
        out.append('| `%s` | %d | %d | %d | %d | %s | %s | %s |' % (
            e['nome'], e['tipo'], e['qtd'], e['ofs'], e['ver'], ('%g' % reg) if reg is not None else '—', est or '?', confere))
    out += ['', '## Pendências', '',
            '- **`staff.dat` versão 1 (157 bytes por pessoa):** é a base original do 3.9.68. A referência só descreve a '
            'versão 2 das atualizações da comunidade (`TStaff`, 110 bytes, mais as preferências numa tabela à parte, tipo 22, '
            '`TPreferences` de 52 bytes). Os primeiros campos coincidem (o `YearChanger` do patcher usa nascimento em +0x10, '
            'ano de nascimento em +0x18 e datas de contrato em +0x3e/+0x46 nas duas versões). A hipótese "versão 1 = pessoa '
            '+ preferências embutidas" não fecha exato (110 + 52 − 4 = 158, não 157); falta mapear os 47 bytes finais.',
            '- **`colour.dat` (58 bytes) e `officials.dat` (43 bytes):** sem estrutura na referência. Dos árbitros, o '
            '`RefereePatcher` usa capacidade atual/potencial (i16 em +30/+32) e disciplina (byte em +37).', '',
            '## Estruturas', '']
    for nome, (total, linhas) in layouts.items():
        out += ['### `%s` (%d bytes)' % (nome, total), '', '| Desl. | Bytes | Tipo | Campo |', '|---:|---:|---|---|']
        out += ['| %#x | %d | `%s` | %s |' % (o, t, ti, c) for o, t, ti, c in linhas]
        out.append('')
    open(sys.argv[2], 'w', encoding='utf-8', newline='\n').write('\n'.join(out))
    ok = sum(1 for e in entradas if e['registro'] and TABELA_ESTRUTURA.get(e['tipo']) in layouts
             and abs(e['registro'] - layouts[TABELA_ESTRUTURA[e['tipo']]][0]) < 1e-9)
    print('%d tabelas; %d conferem com a estrutura de referencia' % (len(entradas), ok))
    for e in entradas:
        est = TABELA_ESTRUTURA.get(e['tipo'])
        print('  %-24s tipo %2d registro %-8s estrutura %-18s %s' % (e['nome'], e['tipo'], ('%g' % e['registro']) if e['registro'] else '-',
              est, layouts[est][0] if est in layouts else '-'))


if __name__ == '__main__':
    main()
