"""Gera uma copia "portatil" do cm0102.exe (ou cm0102_GDI.exe) 3.9.68.

Tres correcoes, todas localizadas por padrao e conferidas antes de escrever:

1. CD: o exe tem 33 copias da mesma checagem (procura <unidade de CD>\\cm0102.exe
   e, se nao acha, mostra "Please insert the CM01/02 CD" e repete).
   Em cada copia, o inicio do laco vira um salto direto para o caminho de sucesso.
2. Pasta: o WinMain le HKLM\\Software\\Sports Interactive Ltd\\Championship Manager 01-02
   \\Location e faz chdir para la. Passa a fazer chdir para a pasta do proprio exe.
3. game.cfg: guarda o caminho absoluto da pasta do usuario (saves). Se a pasta antiga
   ainda existir, o jogo grava os saves nela. Passa a ignorar esse caminho e usar a pasta do jogo.

Uso: python patch_portatil.py <exe original> <exe de saida>
"""
import hashlib
import struct
import sys

import capstone
import pefile

MD5_CONHECIDOS = {
    'ed0cb3bc9bcc8ac3964dad52f6007cb8': 'cm0102.exe 3.9.68 (DirectDraw)',
    '0ece18564ace3012e67912c9f8a30000': 'cm0102_GDI.exe 3.9.68 (GDI)',
}
BASE = 0x400000

md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_32)


def falha(msg):
    sys.exit('ERRO: ' + msg)


class Exe:
    def __init__(self, dados):
        self.d = bytearray(dados)
        pe = pefile.PE(data=bytes(dados), fast_load=True)
        pe.parse_data_directories(directories=[pefile.DIRECTORY_ENTRY['IMAGE_DIRECTORY_ENTRY_IMPORT']])
        self.iat = {(imp.name or b'').decode(): imp.address for e in pe.DIRECTORY_ENTRY_IMPORT for imp in e.imports}
        texto = next(s for s in pe.sections if s.Name.startswith(b'.text'))
        # Em .text, .rdata e .data o deslocamento no arquivo e igual ao RVA.
        if texto.PointerToRawData != texto.VirtualAddress:
            falha('layout de secoes inesperado')
        self.texto_ini = BASE + texto.VirtualAddress
        self.texto_fim = self.texto_ini + texto.Misc_VirtualSize

    def off(self, va):
        return va - BASE

    def string_va(self, s):
        alvo = s.encode('ascii') + b'\0'
        pos = self.d.find(alvo)
        if pos < 0 or self.d.find(alvo, pos + 1) >= 0:
            falha('string %r ausente ou repetida' % s)
        return pos + BASE

    def achar_todos(self, padrao, ini=None, fim=None):
        ini = self.off(ini or self.texto_ini)
        fim = self.off(fim or self.texto_fim)
        res, i = [], self.d.find(padrao, ini, fim)
        while i >= 0:
            res.append(i + BASE)
            i = self.d.find(padrao, i + 1, fim)
        return res

    def instrucoes(self, va, n_bytes):
        o = self.off(va)
        return list(md.disasm(bytes(self.d[o:o + n_bytes]), va))

    def escrever(self, va, bs):
        o = self.off(va)
        self.d[o:o + len(bs)] = bs


def jmp_rel32(de, para):
    return b'\xe9' + struct.pack('<i', para - (de + 5))


def alvo_salto(ins):
    return int(ins.op_str, 16)


def corrigir_cd(exe):
    """Cada checagem: [laco] GetLogicalDriveStringsA ... _findfirst ... je SUCESSO ... "Insert CD" ... jmp laco."""
    msg = exe.string_va('Insert CD')
    achar_primeiro = None  # o mesmo _findfirst em todas as copias
    sitios = exe.achar_todos(b'\x68' + struct.pack('<I', msg))
    if len(sitios) != 33:
        falha('esperava 33 checagens de CD, achei %d' % len(sitios))
    for push_msg in sitios:
        # Do push da mensagem ate o "jmp laco" que reinicia a checagem.
        laco = None
        for ins in exe.instrucoes(push_msg, 0x60):
            if ins.mnemonic == 'jmp':
                laco = alvo_salto(ins)
                break
        if laco is None or not (push_msg - 0x120 < laco < push_msg):
            falha('nao achei o inicio do laco perto de %#x' % push_msg)

        corpo = exe.instrucoes(laco, push_msg - laco)
        alvos_internos = {alvo_salto(i) for i in corpo if i.mnemonic.startswith('j') and i.op_str.startswith('0x')}
        if any(laco < a < laco + 5 for a in alvos_internos):
            falha('ha salto para dentro dos 5 bytes do laco em %#x' % laco)
        # O laco abre com GetLogicalDriveStringsA(0xc8, buf), direto pela IAT ou por um
        # registrador carregado antes ("call ebp"), e testa GetDriveTypeA == 5 (CD-ROM).
        abre = [(a, b) for a, b in zip(corpo, corpo[1:])
                if (a.mnemonic, a.op_str) == ('push', '0xc8') and b.mnemonic == 'call']
        if len(abre) != 1 or abre[0][0].address - laco > 0x10 or \
                not any((i.mnemonic, i.op_str) == ('cmp', 'eax, 5') for i in corpo):
            falha('laco em %#x nao tem a forma da checagem de CD' % laco)
        sucesso = None
        for a, b, c, d in zip(corpo, corpo[1:], corpo[2:], corpo[3:]):
            if a.mnemonic == 'call' and (b.mnemonic, b.op_str) == ('add', 'esp, 0x40') \
                    and (c.mnemonic, c.op_str) == ('test', 'eax, eax') and d.mnemonic == 'je':
                if achar_primeiro is None:
                    achar_primeiro = a.op_str
                elif a.op_str != achar_primeiro:
                    falha('_findfirst diferente em %#x' % a.address)
                sucesso = alvo_salto(d)
        if sucesso is None or sucesso <= push_msg:
            falha('nao achei o caminho de sucesso do laco em %#x' % laco)
        exe.escrever(laco, jmp_rel32(laco, sucesso))
    return len(sitios)


def corrigir_pasta(exe):
    """Troca o bloco RegOpenKeyExA/RegQueryValueExA("Location")/_chdir por _chdir(<pasta do exe>)."""
    chave = exe.string_va(r'Software\Sports Interactive Ltd\Championship Manager 01-02')
    inicio_pad = b'\x8d\x4c\x24\x00\x51\x68\x3f\x00\x0f\x00\x6a\x00\x68' + struct.pack('<I', chave)
    inicios = exe.achar_todos(inicio_pad)
    if len(inicios) != 1:
        falha('bloco do registro: esperava 1, achei %d' % len(inicios))
    ini = inicios[0]
    corpo = exe.instrucoes(ini, 0x90)
    # Fim do bloco: alvo do "jne" logo apos RegOpenKeyExA.
    fim = next(alvo_salto(i) for i in corpo if i.mnemonic == 'jne')
    corpo = [i for i in corpo if i.address < fim]
    # O original usa [esp+0xc] como buffer do caminho e chama _chdir nele.
    chdir = None
    for a, b, c in zip(corpo, corpo[1:], corpo[2:]):
        if (a.mnemonic, a.op_str) == ('lea', 'eax, [esp + 0xc]') and b.mnemonic == 'push' and c.mnemonic == 'call':
            chdir = alvo_salto(c)
    if chdir is None:
        falha('nao achei a chamada a _chdir no bloco do registro')

    gmfn = exe.iat['GetModuleFileNameA']
    # Buffer em [esp+0xc], 0x104 bytes (o proximo buffer local esta em [esp+0x110]).
    # Saltos curtos com rotulo: ('jcc', opcode, rotulo); o deslocamento e resolvido abaixo.
    prog = [
        b'\x8d\x44\x24\x0c',                    # lea  eax, [esp+0xc]
        b'\x68\x04\x01\x00\x00',                # push 0x104
        b'\x50',                                # push eax
        b'\x6a\x00',                            # push 0            ; hModule = NULL (o proprio exe)
        b'\xff\x15' + struct.pack('<I', gmfn),  # call [GetModuleFileNameA]
        b'\x85\xc0',                            # test eax, eax
        ('jcc', 0x74, 'fora'),                  # je   fora         ; falhou: fica na pasta atual
        b'\x8d\x4c\x24\x0c',                    # lea  ecx, [esp+0xc]
        b'\x31\xd2',                            # xor  edx, edx     ; ultima '\' achada
        'laco',
        b'\x8a\x01',                            # mov  al, [ecx]
        b'\x84\xc0',                            # test al, al
        ('jcc', 0x74, 'achou'),                 # je   achou
        b'\x3c\x5c',                            # cmp  al, '\\'
        ('jcc', 0x75, 'prox'),                  # jne  prox
        b'\x89\xca',                            # mov  edx, ecx
        'prox',
        b'\x41',                                # inc  ecx
        ('jcc', 0xeb, 'laco'),                  # jmp  laco
        'achou',
        b'\x85\xd2',                            # test edx, edx
        ('jcc', 0x74, 'fora'),                  # je   fora
        b'\xc6\x42\x01\x00',                    # mov  byte [edx+1], 0   ; mantem a '\' final ("D:\" continua valido)
        b'\x8d\x44\x24\x0c',                    # lea  eax, [esp+0xc]
        b'\x50',                                # push eax
        ('call', chdir),                        # call _chdir
        b'\x83\xc4\x04',                        # add  esp, 4
        'fora',
        ('jmp', fim),                           # jmp  fim do bloco original
    ]
    tamanho = {bytes: len, str: lambda x: 0, tuple: lambda x: 2 if x[0] == 'jcc' else 5}
    rotulos, pos = {}, ini
    for item in prog:
        if isinstance(item, str):
            rotulos[item] = pos
        pos += tamanho[type(item)](item)
    c, pos = bytearray(), ini
    for item in prog:
        if isinstance(item, bytes):
            c += item
        elif isinstance(item, tuple):
            if item[0] == 'jcc':
                c += bytes([item[1]]) + struct.pack('<b', rotulos[item[2]] - (pos + 2))
            else:
                c += (b'\xe8' if item[0] == 'call' else b'\xe9') + struct.pack('<i', item[1] - (pos + 5))
        pos = ini + len(c)
    if len(c) > fim - ini:
        falha('codigo novo nao cabe no bloco do registro')
    c += b'\x90' * (fim - ini - len(c))
    exe.escrever(ini, bytes(c))
    return ini, fim


def corrigir_pasta_usuario(exe):
    """No carregamento do game.cfg, nunca copiar a pasta do usuario gravada (fica a pasta atual)."""
    # Sequencia: ... repne scasb / not ecx / dec ecx / je PULA / lea eax,[esp+0xf8] / push eax / call existe_pasta
    pad = b'\xf2\xae\xf7\xd1\x49\x74'
    candidatos = []
    for va in exe.achar_todos(pad):
        je = va + 5
        seg = exe.instrucoes(je + 2, 16)
        if len(seg) >= 3 and (seg[0].mnemonic, seg[0].op_str) == ('lea', 'eax, [esp + 0xf8]') \
                and seg[1].mnemonic == 'push' and seg[2].mnemonic == 'call':
            candidatos.append(je)
    if len(candidatos) != 1:
        falha('pasta do usuario no game.cfg: esperava 1 ponto, achei %d' % len(candidatos))
    je = candidatos[0]
    exe.escrever(je, b'\xeb')  # je -> jmp
    return je


def main():
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    origem, destino = sys.argv[1], sys.argv[2]
    dados = open(origem, 'rb').read()
    md5 = hashlib.md5(dados).hexdigest()
    if md5 not in MD5_CONHECIDOS:
        falha('%s nao e um exe 3.9.68 original conhecido (md5 %s)' % (origem, md5))
    print('origem:', MD5_CONHECIDOS[md5])
    exe = Exe(dados)
    n = corrigir_cd(exe)
    print('CD: %d checagens desligadas' % n)
    ini, fim = corrigir_pasta(exe)
    print('pasta: chdir para a pasta do exe em %#x-%#x' % (ini, fim))
    je = corrigir_pasta_usuario(exe)
    print('game.cfg: pasta do usuario gravada ignorada em %#x' % je)
    open(destino, 'wb').write(exe.d)
    print('gravado', destino, hashlib.md5(exe.d).hexdigest())


if __name__ == '__main__':
    main()
