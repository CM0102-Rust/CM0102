# Os arquivos de formacao e os nomes das posicoes nas telas (formation.cpp; leitor sx10, rodada 5, 23/11/2026).
# Cada rotina provada pela desmontagem (inicio no prologo, fim no ultimo ret antes do enchimento, sem tabelas de
# saltos). Chamadores:
# - check_duplicate_coordinate 0x5b4ee0: tactics_screens 0x89edaa, 0x89f265;
# - save_to_disk 0x5b6900: tactics_screens 0x89fc72, 0x8a9826, 0x8a9862 (gravar a tatica num .pct);
# - formation_get_position_name 0x5b7580: tactics_screens 0x8aa697;
# - formation_get_position_abbreviation 0x5b7810: club_screens 0x459ae7, 0x45af50, 0x45b072 e tactics_screens 0x8aa475;
# - formation_get_position_flag_name 0x5b7ab0: training_screens 0x8ba195, 0x8ba251, 0x8ba43c, 0x8ba4f8;
# - formation_read_file_password 0x5b8600: as duas abaixo e tactics_screens (0x89f917, 0x89fb37, 0x89fd24, 0x89fe88,
#   0x8a0649, 0x8a0da7);
# - formation_check_file_password 0x5b87e0 e _obfuscated 0x5b88a0: so por ponteiro (absolutas em tactics_screens
#   0x89f88e, 0x8a013b..., 0x8a0583, 0x8a08a7), o filtro da lista de arquivos de taticas.
# Nenhuma roda na carreira automatica (cobertura do mundo: 0 chamadas).
# Convencoes conferidas na desmontagem: check_duplicate_coordinate thiscall(vaga, fase, zona, *saida) ret 0x10;
# save_to_disk thiscall(caminho, quieto, ofuscado) ret 0xc; os nomes cdecl(codigo, buf, tamanho[, estrito ou
# so_linha]) -> 1/0; read_file_password cdecl(caminho, senha, criador, ofuscado) -> 1/0; check_file_password(_obf.)
# cdecl(caminho) -> 2/1/9/5. Chamadas: fopen 0x945b00, fwrite 0x945ce6, fread 0x945bfe, fclose 0x945a09,
# _splitpath 0x944cff, sprintf 0x944c9f, langlib_sprintf 0x66f4e0, utils_read_file_format_marker 0x90c140, a
# gravacao do marcador 0x90c1a0 cdecl(arquivo, marcador), HUMAN_MANAGER::get_password 0x5fde40 thiscall([0xb63c98], 0).
# Comparadores: save_to_disk so a original (grava arquivo) com o plano conferido depois; os demais lado a lado (os de
# senha so leem o arquivo, entao rodam duas vezes sem efeito); argumento que da a caixa de erro: so a original.
TELAS = True
ROTINAS = [('formation_check_duplicate_coordinate', 0x5b4ee0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('formation_save_to_disk', 0x5b6900, [0x81, 0xec, 0xd4, 0x02, 0x00, 0x00]),
           ('formation_position_name', 0x5b7580, [0x81, 0xec, 0xc8, 0x02, 0x00, 0x00]),
           ('formation_position_abbreviation', 0x5b7810, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00]),
           ('formation_position_flag_name', 0x5b7ab0, [0x8b, 0x54, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('formation_read_file_password', 0x5b8600, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('formation_check_file_password', 0x5b87e0, [0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0xfc, 0x00, 0x00, 0x00]),
           ('formation_check_file_password_obfuscated', 0x5b88a0, [0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0xfc, 0x00, 0x00, 0x00])]
DOCS = ('os arquivos de formação e os nomes das posições',
        'formation files and position names',
        [('formation_check_duplicate_coordinate', 'Formation::check_duplicate_coordinate', '0x5b4ee0', 'formation.cpp', 'thiscall',
          'procura outra vaga no mesmo ponto do campo numa fase e zona: uma vaga posterior marca a saída e dá 1, uma anterior dá 0; nenhuma, 1',
          'looks for another slot on the same pitch point in one phase and zone: a later slot sets the flag and gives 1, an earlier one gives 0; none, 1'),
         ('formation_save_to_disk', 'Formation::save_to_disk', '0x5b6900', 'formation.cpp', 'thiscall',
          'grava a formação num .pct: o identificador (ofuscado se pedido), a senha invertida, o criador, a máscara, as posições gerais, a ordem de exibição, as extras, as coordenadas (falha nessas: fecha e dá 0), as instruções do time e dos jogadores e a marcação (falha: só a caixa), o marcador do formato; a formação toma o nome do arquivo e fica gravada; os designados não vão para o arquivo',
          'writes the formation to a .pct: the identifier (obfuscated if asked), the inverted password, the creator, the mask, the general positions, the display order, the extra ones, the coordinates (a failure there closes and gives 0), the team and player instructions and the marking (a failure only shows the box), the format marker; the formation takes the file name and is marked saved; the designated people are not written'),
         ('formation_position_name', 'formation_get_position_name', '0x5b7580', 'formation.cpp', 'cdecl',
          'o nome longo de um código de posição com o lado ("Defender Centre Left"); o goleiro só "Goalkeeper"; sem linha ou lado, vazio, ou a caixa no modo estrito',
          'the long name of a position code with its side ("Defender Centre Left"); the goalkeeper only "Goalkeeper"; without line or side, empty, or the box in strict mode'),
         ('formation_position_abbreviation', 'formation_get_position_abbreviation', '0x5b7810', 'formation.cpp', 'cdecl',
          'a abreviatura de um código de posição (GK, DL, AMC...); com "só a linha", uma linha de duas letras (SW, DM, AM) basta',
          'the abbreviation of a position code (GK, DL, AMC...); with "line only", a two-letter line (SW, DM, AM) is enough'),
         ('formation_position_flag_name', 'formation_get_position_flag_name', '0x5b7ab0', 'formation.cpp', 'cdecl',
          'o nome de um único bit de linha ou de lado, ou "Free Role" para 0x1000; outro valor, vazio',
          'the name of a single line or side bit, or "Free Role" for 0x1000; any other value, empty'),
         ('formation_read_file_password', 'formation_read_file_password', '0x5b8600', 'formation.cpp', 'cdecl',
          'lê de um .pct a senha (desinvertida) e o criador, se o identificador for da versão 0x98ec59 em diante',
          'reads a .pct file\'s password (un-inverted) and creator, when the identifier is version 0x98ec59 or later'),
         ('formation_check_file_password', 'formation_check_file_password', '0x5b87e0', 'formation.cpp', 'cdecl',
          'confere a senha de um .pct com a do técnico humano atual: 2 ilegível, 1 sem senha, 9 igual, 5 diferente',
          'checks a .pct file\'s password against the current human manager\'s: 2 unreadable, 1 no password, 9 equal, 5 different'),
         ('formation_check_file_password_obfuscated', 'formation_check_file_password_obfuscated', '0x5b88a0', 'formation.cpp', 'cdecl',
          'o mesmo para um .pct ofuscado',
          'the same for an obfuscated .pct file')])
