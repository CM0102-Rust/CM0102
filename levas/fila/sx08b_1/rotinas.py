# Database.cpp (leitor sx08b, rodada 5, 23/11/2026): os nomes traduzidos do banco (o .lng do idioma, em CM3_TEMP).
# - 0x5339a0: cdecl(idioma, barra_em_duas_partes), sem retorno (Config 0x5d7694 e 0x5d85bd fazem add esp, 8; a chamada
#   a si mesma empilha (0, 1)). Quadro SEH so por causa do `new` da PunditList no fim (estado 0); o Rust nao tem quadro.
#   Fim: o ret 0x5358dd e a tabela de saltos 0x5358e0 (13 entradas, idiomas 0..12; acima de 12, o padrao). Prologo
#   `mov eax, fs:[0]` (6 bytes, fronteira de instrucao, sem relativo).
# - Chamadas: 0x90bde0 cdecl(pasta, nome); 0x945b00 fopen cdecl(caminho, "rb"); 0x90c140 cdecl(arquivo) o marcador (le o
#   fim do arquivo e volta a posicao: as linhas comecam em 0x20); 0x945bfe fread cdecl(buf, tam, n, arq); 0x944e46 malloc;
#   0x9452ca free; 0x946a22 fseek cdecl(arq, pos, 0); 0x9466d0 strncpy cdecl(dst, src, n); 0x945a09 fclose; 0x66f4e0
#   langlib_sprintf cdecl(buf, texto); 0x802f80 thiscall(0xb74450, texto, passo, total, -1) ret 0x10; 0x544f30 thiscall(t)
#   e 0x944c94 delete(t); 0x7da690 thiscall(lista), 0x944cf1 new(10), 0x7da190 thiscall(lista, idioma) ret 4; a caixa de
#   erro do Database.cpp (0x944cff, 0x90d130, 0x944c9f, 0x5e8290 e [0xb67a34] = 0) = database_error_box.
# - Reaproveita eng_lng_counts_match do sx08_3 (a mesma conferencia das contas do cabecalho): integrar DEPOIS do sx08_3.
# Na carreira: a criacao poe o idioma dos nomes em 0 (0x528a30); as 3 chamadas da Config passam o idioma das
# preferencias. Com ingles (0) voltam no comeco; o corpo so roda com outro idioma nas preferencias (ver PRONTO.txt).
# Tipo: simulacao (roda na criacao do jogo, pela Config), provada pelo comparador lado a lado nas 7 tabelas de nomes.
SAVE = False
ROTINAS = [('database_load_language_names', 0x5339a0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os nomes traduzidos do banco (o .lng do idioma)',
        'the translated database names (the language .lng)',
        [('database_load_language_names', 'database_load_language_names', '0x5339a0', 'Database.cpp', 'cdecl',
          'carrega os nomes do idioma (o .lng em CM3_TEMP) nas tabelas: nada sem a tabela de nações ou se os nomes já estão nesse idioma. Inglês (0, 1, 11, 12): eng.lng, e o objeto das tabelas inglesas é liberado; outro idioma volta antes ao inglês (chamando a si mesma com 0) e lê o próprio arquivo na segunda metade da barra "Re-loading Database"; idioma desconhecido: caixa de erro e os nomes franceses. Sem o arquivo, caixa; contas do cabeçalho que não batem com as tabelas, fecha e caixa (o idioma fica). Senão, na ordem do arquivo, um passo da barra depois de cada: clubes (com o índice de nomes ingleses da criação, o clube i lê a linha índice[i]; sem nome curto no arquivo, as seleções ficam com os 26 primeiros bytes do nome; nomes espanhóis trocam o gênero "nenhum" por 1), continentes, competições de nação (no fim da tabela de competições de clube, antes das 127 reservas), nações, estádios, cidades, competições de pessoal e de clube; cada tabela lida inteira num malloc ou, sem memória, linha a linha. Fecha, grava o idioma e refaz a lista de comentaristas no novo idioma',
          'loads the names of the language (the .lng in CM3_TEMP) into the tables: nothing without the nation table or when the names are already in that language. English (0, 1, 11, 12): eng.lng, and the English tables object is freed; another language first goes back to English (calling itself with 0) and reads its own file in the second half of the "Re-loading Database" bar; an unknown language: error box and the French names. Without the file, box; header counts that do not match the tables, close and box (the language stays). Otherwise, in file order, a bar step after each: clubs (with the English-name index of the creation, club i reads row index[i]; without a short name in the file, the national teams take the first 26 bytes of the name; Spanish names turn the "none" gender into 1), continents, nation competitions (at the end of the club competition table, before the 127 reserves), nations, stadiums, cities, staff and club competitions; each table read whole into a malloc block or, without memory, row by row. Closes, sets the language and rebuilds the pundit list in the new language')])
CAMPOS = [('TClub/TNation/TComp/TStaffComp +4 nome (51), +0x37 gênero, +0x38 nome curto (26), +0x51 fim do nome curto, +0x52 gênero curto; TNation +0x53 três letras, +0x57 nacionalidade; continente (198 bytes) +4, +0x1e, +0x1f, +0x23, +0x3d (101), +0xa2, +0xa3, +0xbd; cidade +4 (26), +0x1e; estádio +4, +0x37; grava (database_load_language_names)',
           'TClub/TNation/TComp/TStaffComp +4 name (51), +0x37 gender, +0x38 short name (26), +0x51 short name end, +0x52 short gender; TNation +0x53 three letters, +0x57 nationality; continent (198 bytes) +4, +0x1e, +0x1f, +0x23, +0x3d (101), +0xa2, +0xa3, +0xbd; city +4 (26), +0x1e; stadium +4, +0x37; writes (database_load_language_names)')]
INTERESSA = []
