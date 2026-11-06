# As sedes dos torneios (host_country.cpp): a tabela padrão das sedes (HostCountry::init_default_table, 0x5f9970: 1
# chamada, no começo do jogo novo, pelo construtor 0x5f97c0, com 57 sorteios), a notícia das candidaturas e das sedes
# escolhidas (HostCountry::post_host_news, 0x5faec0: ~500 chamadas, pelo calendário das sedes 0x5fa850 já em Rust) e a
# carga do HostCountry.tmp (HostCountry::load_temp_file, 0x5fb160: 1 chamada, sem o arquivo no jogo novo; com ele na
# carga de um save).
# Iguais no cm0102-1080.exe (as três inteiras).
# A tabela das gravações da 0x5f9970 (CQO3_TABELA, CQO3_DATAS, CQO3_LACOS) foi gerada da desmontagem por um script
# (registradores acompanhados: o [esi] relido antes de cada gravação, os globais, o edi = −1, o ebp = 0, o bl = 0xfe e
# depois 0xfd), conferido contra o pseudocódigo do Ghidra.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x5f9970: pelo construtor 0x5f97c0 (0x5f9943): mov ecx, ebp; call, sem push -> thiscall(this), ret 0.
# - 0x5faec0: pelo calendário das sedes 0x5fa850 (Rust; na original push &lista; mov ecx, this) -> thiscall(this,
#   lista), ret 4.
# - 0x5fb160: pelo construtor 0x5f97c0 (0x5f98a9): mov ecx, ebp; call, sem push -> thiscall(this), ret 0.
# - dentro: 0x54a130 (Date::set) 4 push (−1, 1997, mês, dia); lea ecx, data -> thiscall, ret 0x10 (original);
#   0x90cfc0 push 0x46; add esp, 4 -> cdecl (Rust); a caixa (aviso_em: 0x944cff, 0x90d130, 0x944c9f, 0x5e8290);
#   0x68d1b0 mov ecx, lista -> thiscall, ret 0 (Rust); 0x77bbe0 push 0; push 4/5; lea ecx, notícia -> thiscall, ret 8
#   (Rust); 0x785790 push valor; push índice; lea ecx, notícia -> thiscall, ret 8 (Rust); 0x786640 push continente;
#   push &notícia; mov ecx, [0xb7431c] -> thiscall, ret 8 (Rust); 0x786780 push &notícia; mov ecx, [0xb7431c] ->
#   thiscall, ret 4 (Rust); 0x90bde0 2 push; add esp, 8 -> cdecl; 0x945b00 (fopen) 2 push (o "rb" empurrado antes),
#   add esp, 8 -> cdecl; 0x945bfe (fread) 4 push, add esp, 0x10 -> cdecl; 0x945a09 (fclose), 0x944e46 (malloc),
#   0x9452ca (free): 1 push, add esp, 4 -> cdecl.
# Comparadores: a tabela padrão lado a lado (a cópia sem contar os sorteios) pela tabela inteira, pelo estado do
# sorteador e por [0xb67a34]; a notícia (entrega, aloca) e a carga (abre o arquivo, aloca): só a original, com os
# ramos na cópia.
# Determinismo: a tabela só tem constantes e globais do banco; os sorteios do jogo.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('tabela_das_sedes', 0x5f9970, [0x6a, 0xff, 0x68, 0x5b, 0xaf, 0x95, 0x00]),
           ('noticia_das_sedes', 0x5faec0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('carregar_sedes', 0x5fb160, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as sedes dos torneios: a tabela padrão, a notícia das candidaturas e a carga do HostCountry.tmp',
        'tournament hosts: the default table, the bids news and loading HostCountry.tmp',
        [('tabela_das_sedes', 'HostCountry::init_default_table', '0x5f9970', 'host_country.cpp', 'thiscall, ret 0',
          'a tabela padrão das sedes: 57 entradas (Copa do Mundo de 1998 a 2026 de 4 em 4, Eurocopa e Olimpíadas de 2000 a 2028, Copa Africana de 2000 a 2014 de 2 em 2, Copa América de 1999 a 2009, Copa da Ásia de 2000 a 2016 e as edições avulsas), cada uma com até três candidaturas de duas nações, a escolhida (ou votação aberta/fechada, ou sede fixa) e o dia da decisão (10 de julho para a Copa do Mundo, 20 de julho ou 20 de setembro para as outras [inferência: o mês conta de 0]) mais um sorteio de −35 a +34 dias',
          'the default host table: 57 entries (World Cup 1998 to 2026 every 4 years, European Championship and Olympics 2000 to 2028, African Cup 2000 to 2014 every 2 years, Copa América 1999 to 2009, Asian Cup 2000 to 2016 and one-off editions), each with up to three bids of two nations, the chosen one (or open/closed voting, or a fixed host) and the decision day (10 July for the World Cup, 20 July or 20 September for the others [inference: the month counts from 0]) plus a random −35 to +34 days'),
         ('noticia_das_sedes', 'HostCountry::post_host_news', '0x5faec0', 'host_country.cpp', 'thiscall, ret 4',
          'a notícia das sedes que mudaram: com a votação aberta, as candidaturas (notícia 4); com a escolhida, as sedes (notícia 5); aos técnicos do continente da competição ou a todos; fora as Olimpíadas, a Copa do Mundo de 2006 na Alemanha e a escolhida da edição de daqui a 8 anos',
          'the news of changed hosts: with open voting, the bids (news 4); with a chosen bid, the hosts (news 5); to the managers of the competition\'s continent or to everyone; except the Olympics, the 2006 World Cup in Germany and the chosen bid of the edition 8 years ahead'),
         ('carregar_sedes', 'HostCountry::load_temp_file', '0x5fb160', 'host_country.cpp', 'thiscall, ret 0',
          'lê o HostCountry.tmp (a conta e as entradas de 0x22 bytes num bloco novo); sem o arquivo 0 (o jogo novo monta a tabela padrão); erro de leitura com a caixa',
          'reads HostCountry.tmp (the count and the 0x22-byte entries into a new block); without the file 0 (a new game builds the default table); read errors with the error box')])
CAMPOS = [('sedes [0xb5e134] +0 entradas de 0x22 bytes (+0 competição, +4 ano, +6/+0xa, +0xe/+0x12, +0x16/+0x1a as três candidaturas, +0x1e escolhida/estado, +0x1f dia da decisão, +0x21 intervalo), +4 conta (word); grava (tabela_das_sedes, carregar_sedes), lê (noticia_das_sedes)', 'hosts [0xb5e134] +0 0x22-byte entries (+0 competition, +4 year, +6/+0xa, +0xe/+0x12, +0x16/+0x1a the three bids, +0x1e chosen/state, +0x1f decision day, +0x21 interval), +4 count (word); writes (tabela_das_sedes, carregar_sedes), reads (noticia_das_sedes)'),
          ('[0xae2c92] ano do jogo; [0xae23d0] competições (0x6b bytes, +0x59 continente); lê', '[0xae2c92] game year; [0xae23d0] competitions (0x6b bytes, +0x59 continent); reads')]
INTERESSA = ['SGE', 'CMRTE']
