# As vagas de técnico e o agendamento das ofertas (manager_manager.cpp, transfer_manager.cpp): abrir a vaga dos clubes
# sem técnico e sem vaga (MANAGER_MANAGER::fix_missing_jobs 0x69d060, 16 chamadas na carreira até 2002-06-01), a melhor
# nota de candidato a técnico de um clube fora o atual (MANAGER_MANAGER::best_manager_candidate_score 0x69c540, 14; a
# 0x69b0c0 a compara com a nota do técnico atual + 450) e agendar a oferta de transferência no balde do dia e da fatia
# (TRANSFER_MANAGER::schedule_offer 0x8c6220, 4634). Sem SEH. Chamam pelo endereço rotinas ainda em C: 0x69ba50 (abre a
# vaga), 0x69d1b0 e 0x69f810 (a nota e a aceitação do candidato), 0x54c5b0, 0x54cdb0 e 0x417c10 (datas), 0x68d0d0 (a
# lista, aloca). Trocas: a 0x8c91e0 (as cláusulas da transferência anterior) ficou de fora: os registros que ela paga
# nascem 2 e 14 vezes na carreira (as mutações escapariam). Iguais no cm0102-1080.exe. Comparadores: as vagas lado a
# lado quando não há vaga a abrir (senão só a original); a nota lado a lado a partir do mesmo estado do sorteio; o
# agendamento pela original conferida com o plano.
SAVE = False
ROTINAS = [('abrir_vagas_que_faltam', 0x69d060, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('melhor_nota_de_candidato_a_tecnico', 0x69c540, [0x83, 0xec, 0x08, 0x53, 0x55]),
           ('agendar_oferta', 0x8c6220, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00])]
DOCS = ('as vagas de técnico e o agendamento das ofertas: abrir as vagas que faltam, a melhor nota de candidato a técnico e agendar a oferta',
        'manager jobs and offer scheduling: opening the missing jobs, the best manager candidate score and scheduling the offer',
        [('abrir_vagas_que_faltam', 'MANAGER_MANAGER::fix_missing_jobs', '0x69d060', 'manager_manager.cpp', 'thiscall, ret 0',
          'cada clube sem técnico (0x53ca90), dirigível (0x544820), que não é time B e sem vaga na lista [this+8] ganha o aviso "Job not found" e a vaga (0x69ba50)',
          'each club without a manager (0x53ca90), manageable (0x544820), not a B team and without an entry in the [this+8] list gets the "Job not found" note and a job vacancy (0x69ba50)'),
         ('melhor_nota_de_candidato_a_tecnico', 'MANAGER_MANAGER::best_manager_candidate_score', '0x69c540', 'manager_manager.cpp', 'thiscall, ret 8',
          'a maior nota de técnico (0x69d1b0, cargo 5) entre as pessoas da comissão disponíveis e aceitas para o clube (nação, continente, reputação, contrato, 0x69f810), fora o técnico atual; ninguém: −10000',
          'the highest manager score (0x69d1b0, job 5) among the available staff accepted for the club (nation, continent, reputation, contract, 0x69f810), ignoring the current manager; nobody: −10000'),
         ('agendar_oferta', 'TRANSFER_MANAGER::schedule_offer', '0x8c6220', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'dá à oferta a data de hoje + dias e a fatia do dia (0 a 2, cada 3 fatias mais um dia) e a põe no balde (dias desde a data base + começo, módulo 50, × 3 + fatia) das listas de +0x18',
          'gives the offer today\'s date + days and the day slice (0 to 2, every 3 slices one more day) and puts it in the bucket (days since the base date + start, modulo 50, × 3 + slice) of the +0x18 lists')])
CAMPOS = [('MANAGER_MANAGER +8: a lista das vagas de técnico (+0 conta, +2 cabeça, +0xa cursor; vaga +0xe o id do clube); ponteiro; lê e grava o cursor (abrir_vagas_que_faltam, melhor_nota_de_candidato_a_tecnico)',
           'MANAGER_MANAGER +8: the manager vacancy list (+0 count, +2 head, +0xa cursor; entry +0xe the club id); pointer; reads and writes the cursor (abrir_vagas_que_faltam, melhor_nota_de_candidato_a_tecnico)'),
          ('clube +4: o nome; texto; lê (abrir_vagas_que_faltam)', 'club +4: the name; text; reads (abrir_vagas_que_faltam)'),
          ('clube +0x53 / +0x80 e nação +0x75: a nação, a reputação e o continente; ponteiro e valor; lê (melhor_nota_de_candidato_a_tecnico)',
           'club +0x53 / +0x80 and nation +0x75: nation, reputation and continent; pointer and value; reads (melhor_nota_de_candidato_a_tecnico)'),
          ('staff +0x69 → +0xa / +0xc: as reputações da ficha da comissão; valor; lê (melhor_nota_de_candidato_a_tecnico)',
           'staff +0x69 → +0xa / +0xc: the staff data reputations; value; reads (melhor_nota_de_candidato_a_tecnico)'),
          ('staff +0x5f, +0x1a, +0x39, +0x3d, +0x61: humano, nação, clube, cargo e ficha de jogador; valor e ponteiro; lê (melhor_nota_de_candidato_a_tecnico)',
           'staff +0x5f, +0x1a, +0x39, +0x3d, +0x61: human, nation, club, job and player data; value and pointer; reads (melhor_nota_de_candidato_a_tecnico)'),
          ('[0xae2d0c] registro de 79 bytes +0xb: as marcas 0x30; valor; lê (melhor_nota_de_candidato_a_tecnico)',
           '[0xae2d0c] 79-byte record +0xb: the 0x30 flags; value; reads (melhor_nota_de_candidato_a_tecnico)'),
          ('contrato +0x49: a marca que impede o candidato; valor; lê (melhor_nota_de_candidato_a_tecnico)',
           'contract +0x49: the flag that bars the candidate; value; reads (melhor_nota_de_candidato_a_tecnico)'),
          ('TRANSFER_MANAGER +0x84c / +0x84d / +0x855: o começo do anel, a data base (ano +0x84f) e a fatia base; valor; lê e grava (agendar_oferta)',
           'TRANSFER_MANAGER +0x84c / +0x84d / +0x855: the ring start, the base date (year +0x84f) and the base slice; value; reads and writes (agendar_oferta)'),
          ('TRANSFER_MANAGER +0x18: as 150 listas de 14 bytes (50 dias × 3 fatias) das ofertas agendadas; ponteiro; grava (agendar_oferta)',
           'TRANSFER_MANAGER +0x18: the 150 14-byte lists (50 days × 3 slices) of scheduled offers; pointer; writes (agendar_oferta)'),
          ('oferta +0x12 / +0x1a / +0x20: a data e a fatia do processamento e o valor que entra na lista; valor; grava e lê (agendar_oferta)',
           'offer +0x12 / +0x1a / +0x20: processing date and slice and the value put in the list; value; writes and reads (agendar_oferta)'),
          ('[0xae2c90] / [0xae2c88]: hoje e a fatia de hoje; valor; lê (agendar_oferta)',
           '[0xae2c90] / [0xae2c88]: today and today\'s slice; value; reads (agendar_oferta)')]
INTERESSA = ['CMRTE']
