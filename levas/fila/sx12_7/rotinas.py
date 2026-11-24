# A carga do arquivo do hall da fama e a ordem das suas entradas (hall_of_fame.cpp; leitor sx12, rodada 5, 23/11/2026).
# HallOfFame::load 0x5efaa0 e hall_of_fame_unscramble_and_check 0x5f0680 rodam uma vez no inicio do jogo (o construtor
# 0x5ef0b0, chamado de 0x5cc773; 1 chamada cada na cobertura do mundo), se o hall_of_fame.bin existir na pasta do jogo
# (sem ele, a carga devolve 0 sem caixa e o construtor grava um novo). hall_of_fame_compare_entries 0x5f0620 e o qsort
# do try_add_entry (0x5ef5e0, ja em Rust) e do merge_entry (0x5ef970), quando um tecnico humano entra no hall da fama.
# Convencoes conferidas na desmontagem:
# - HallOfFame::load: thiscall, sem argumentos, eax 1/0; 0x945b00 fopen, 0x945bfe fread cdecl(destino, tamanho,
#   quantidade, arquivo), 0x945a09 fclose; ordem nos erros: a caixa, depois (0x1d8, 0x1e1, 0x1ea) a tabela zerada
#   (rep stosd 0x14820 dwords), depois o fclose.
# - hall_of_fame_unscramble_and_check: cdecl(buffer, tamanho, soma); i mod 0x1e0 sem sinal (div), chave mod tamanho com
#   sinal (cdq/idiv); tamanho 0 nao divide (os lacos nao rodam).
# - hall_of_fame_compare_entries: cdecl(a, b) do qsort; pontuacao sem sinal (jae/jbe), data e hora com sinal.
SAVE = False
ROTINAS = [('hall_of_fame_load', 0x5efaa0, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('hall_of_fame_unscramble_and_check', 0x5f0680, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('hall_of_fame_compare_entries', 0x5f0620, [0x56, 0x8b, 0x74, 0x24, 0x08])]
DOCS = ('a carga do hall da fama e a ordem das suas entradas',
        'loading the hall of fame and the order of its entries',
        [('hall_of_fame_load', 'HallOfFame::load', '0x5efaa0', 'hall_of_fame.cpp', 'thiscall',
          'le o hall_of_fame.bin para as 28 tabelas do hall da fama: sem o arquivo, 0 sem caixa; cabecalho curto, assinatura diferente de 0x3abfb2 ou versao diferente de 1, a caixa "Error" e 0; tabelas curtas, soma curta ou soma que nao confere depois de desembaralhar, a caixa, a tabela zerada e 0; senao 1',
          'reads hall_of_fame.bin into the 28 hall of fame tables: no file, 0 with no box; a short header, a magic other than 0x3abfb2 or a version other than 1, the "Error" box and 0; short tables, a short checksum or a checksum that does not match after unscrambling, the box, the table cleared and 0; otherwise 1'),
         ('hall_of_fame_unscramble_and_check', 'hall_of_fame_unscramble_and_check', '0x5f0680', 'hall_of_fame.cpp', 'cdecl',
          'desembaralha o buffer do hall da fama: do ultimo byte ao primeiro, troca os bytes das posicoes dadas pela chave de 480 numeros (o resto pelo tamanho); depois faz o ou-exclusivo de cada byte com a chave e soma os bytes; devolve 1 quando a soma confere; buffer nulo ou tamanho negativo, a caixa "Error" e 0',
          'unscrambles the hall of fame buffer: from the last byte to the first, swaps the bytes at the positions given by the 480-number key (the remainder by the length); then XORs each byte with the key and adds the bytes up; returns 1 when the sum matches; a null buffer or a negative length, the "Error" box and 0'),
         ('hall_of_fame_compare_entries', 'hall_of_fame_compare_entries', '0x5f0620', 'hall_of_fame.cpp', 'cdecl',
          'ordena duas entradas do hall da fama: a maior pontuacao primeiro, depois a mais antiga; nulo: o a nulo vai depois, o b nulo antes',
          'orders two hall of fame entries: the higher score first, then the older one; null: a null a goes after, a null b before')])
