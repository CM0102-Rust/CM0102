# Do Database.cpp: a nação da Polinésia (0x53d940: as copas da Oceania 0x798220 e 0x799e30), o nome do cargo de
# treino (0x53ac40: a regeneração da comissão, player_regen 0x7a9650; texto fixo 0xae1980 e o byte do gênero quando o
# idioma é o 4) e os anos completos entre duas datas em texto (0x540340: telas de recordes do clube e das competições).
# Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no jogo real comparando. Sem SEH, sem sorteio;
# as locais de 0x200 bytes da 0x540340 só servem à mensagem da caixa (texto nulo, tamanho < 5, data nula), caminho
# que fica com a original. As chamadas (0x66f4e0, 0x66f4d0, 0x944c9f) são pelo endereço do jogo.
SAVE = False
ROTINAS = [('nacao_da_polinesia', 0x53d940, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0xd0, 0xf4, 0x9c, 0x00]),
           ('nome_do_cargo_de_treino', 0x53ac40, [0x0f, 0xbe, 0x44, 0x24, 0x04]),
           ('anos_entre_datas_em_texto', 0x540340, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a nação da Polinésia, o nome do cargo de treino e os anos entre duas datas em texto',
        'the Polynesian nation, the coaching job name and the years between two dates as text',
        [('nacao_da_polinesia', 'database_is_polynesian_nation', '0x53d940', 'Database.cpp', 'cdecl',
          '1 se a nação é uma das cinco polinésias (chaves 0x9cf4d0, 0x9cf52c, 0x9cf4b4, 0x9cf2a4, 0x9cf514)',
          '1 when the nation is one of the five Polynesian ones (keys 0x9cf4d0, 0x9cf52c, 0x9cf4b4, 0x9cf2a4, 0x9cf514)'),
         ('nome_do_cargo_de_treino', 'database_get_coaching_job_name', '0x53ac40', 'Database.cpp', 'cdecl',
          'o nome do cargo (5/12 técnico, 6/13 auxiliar, 8/15 treinador; outros: vazio) traduzido em 0xae1980; com o idioma 4 grava o gênero (4 ou 1) no segundo argumento',
          'the job name (5/12 manager, 6/13 assistant, 8/15 coach; others: empty) translated into 0xae1980; with language 4 writes the gender (4 or 1) into the second argument'),
         ('anos_entre_datas_em_texto', 'database_format_years_between_dates', '0x540340', 'Database.cpp', 'cdecl',
          'os anos completos entre as datas (ano do fim menos o do início, menos 1 se o dia do início é maior) em texto "%d" de palavra sem sinal; devolve 1',
          'the full years between the dates (end year minus start year, minus 1 when the start day is later) as "%d" text of an unsigned word; returns 1')])
CAMPOS = [('TNation +0: id; inteiro; lê (nacao_da_polinesia). Chaves 0x9cf4d0, 0x9cf52c, 0x9cf4b4, 0x9cf2a4, 0x9cf514 (Polinésia); lê',
           'TNation +0: id; integer; reads (nacao_da_polinesia). Keys 0x9cf4d0, 0x9cf52c, 0x9cf4b4, 0x9cf2a4, 0x9cf514 (Polynesia); reads'),
          ('0xae1980: texto fixo do nome do cargo; grava (nome_do_cargo_de_treino)', '0xae1980: static text of the job name; writes (nome_do_cargo_de_treino)'),
          ('data +0 dia do ano, +2 ano (palavras); lê (anos_entre_datas_em_texto)', 'date +0 day of year, +2 year (words); reads (anos_entre_datas_em_texto)')]
INTERESSA = ['SGE', 'CMRTE']
