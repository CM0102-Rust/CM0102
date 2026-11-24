# Os premios antigos da Finlandia: o objeto, os onze premios e o dia dos premios (award/old_finland_awards.cpp;
# leitor sx04c, rodada 5, 23/11/2026). Usa os auxiliares do sx05b_5 e do sx03_2: integrar depois deles.
# NAO RODAM com a base do jogo: o AWARD_MANAGER::create_nation_awards (0x4135b0, 0x4138d4) so monta a classe antiga
# quando a chave do premio novo vale -1 (base de dados antiga); 0 chamadas na cobertura do mundo. Entram pelo "exe
# inteiro" (Lucas, 23/11), como simulacao, sem LIGAS e sem mutacoes; os comparadores conferem se um dia rodarem.
# Convencoes conferidas na desmontagem:
# - 0x79d540 thiscall(arquivo) ret 4: vtable 0x96e180, [KEY_NATION_FINLAND], +0x13 = 8, +0x12 = 1; notas do mes
#   (linha 0x4d);
# - 0x79d680 thiscall(arquivo) ret 4: o ano local; +8 = 0xb, malloc(0x2c) (nulo: linha 0x75); tipo 0 em todos; o
#   premio do mes de maio (4) a outubro (9) do mesmo ano; equipes com anos 0 e modo 0;
# - 0x79db30 thiscall(hoje) ret 4: Date::Date (0xf, 0xa, ano, -1); tres 0x769380 com os filtros como empilhados;
#   0x768940 (Premier, premio 3), 0x768cf0 (Premier, premio 4, estatistica 2), 0x7698f0 (premio 6), 0x768cf0 (First,
#   premio 8), 0x7698f0 (premio 9); no dia do fim da temporada 0x7672e0 no premio 5; 0x768310.
# Comparadores: construtor e init so a original e o objeto conferido; daily_update so a original.
SAVE = False
ROTINAS = [('old_finland_awards_construct', 0x79d540, [0x6a, 0xff, 0x68, 0x49, 0xec, 0x95, 0x00]),
           ('old_finland_awards_init', 0x79d680, [0x6a, 0xff, 0x68, 0xfa, 0xec, 0x95, 0x00]),
           ('old_finland_awards_daily_update', 0x79db30, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios antigos da Finlândia',
        'the old Finnish awards',
        [('old_finland_awards_construct', 'OldFinlandAwards::OldFinlandAwards', '0x79d540', 'award/old_finland_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios antigos da Finlândia (só com uma base de dados sem as chaves dos prêmios novos): a base com a tabela virtual da classe, a Finlândia, o id 8 e o tipo 1; com arquivo, os salvos; senão as notas do mês e os onze prêmios',
          'builds the old Finnish awards (only with a database lacking the new award keys): the base with the class vtable, Finland, id 8 and type 1; with a file, the saved ones; otherwise the month ratings and the eleven awards'),
         ('old_finland_awards_init', 'OldFinlandAwards::init_awards', '0x79d680', 'award/old_finland_awards.cpp', 'thiscall, ret 4',
          'cria os onze prêmios: jogador, jovem, goleiro e técnico finlandês do ano; da Premier Division o artilheiro, o jogador do mês (de maio a outubro), a equipe do ano e a da semana; da First Division o artilheiro, a equipe do ano e a da semana',
          'creates the eleven awards: the Finnish player, young player, goalkeeper and manager of the year; the Premier Division top goalscorer, player of the month (May to October), team of the year and team of the week; the First Division top goalscorer, team of the year and team of the week'),
         ('old_finland_awards_daily_update', 'OldFinlandAwards::daily_update', '0x79db30', 'award/old_finland_awards.cpp', 'thiscall, ret 4',
          'em 15 de novembro, jogador, jovem e goleiro do ano pelos três melhores das notas do ano, o técnico do ano, o artilheiro e a equipe da temporada de cada divisão; no dia do fim da temporada, o prêmio do mês passa à seguinte; depois, todo dia, a atualização da base',
          'on 15 November, player, young player and goalkeeper of the year from the year ratings top three, the manager of the year, the top goalscorer and team of the season of each division; on the season end day, the month award moves on; then, every day, the base daily update')])
CAMPOS = []
INTERESSA = []
