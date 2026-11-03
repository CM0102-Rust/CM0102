# A auditoria das 8 ligas (02/11/2026, Lucas): todas as levas publicadas pelo modo comparando, com as ligas da
# carreira ampla, e as carreiras Rust x2 + detector e a original. Roda em duas bancadas em paralelo (a 4 e do Lucas).
# Uso: powershell -File levas\carreira_ampla\auditoria.ps1   (depois de publicar; as bancadas sao sincronizadas aqui)
$ErrorActionPreference = 'Continue'
$env:CM_VAGAS = '4'
$R = 'D:\Sistemas\CM0102'
$LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
$dia = Get-Date -Format 'yyyy-MM-dd'
$saida = "$R\levas\carreira_ampla\auditoria_$dia"
New-Item -ItemType Directory -Force $saida | Out-Null

# as bancadas com a principal (o motor.rs publicado) e a DLL compilada no jogo_testes de cada uma
foreach ($k in 2, 3) {
    python "$R\engenharia-reversa\bancada.py" sincronizar $k *>> "$saida\sincronizar.txt"
    $b = "$R\copias\bancada$k"
    (Get-Item "$b\rust\src\motor.rs").LastWriteTime = Get-Date  # o cargo recompila (a copia mantem a data)
    Push-Location "$b\rust"; cargo build --release *>> "$saida\sincronizar.txt"; Pop-Location
    Copy-Item "$b\rust\target\i686-pc-windows-msvc\release\ddraw.dll" "$b\jogo_testes\ddraw.dll" -Force
}

# bancada 3: a comparacao rotina a rotina com as 8 ligas (a parte longa)
$b3 = Start-Job -ArgumentList $R, $LIGAS, $saida -ScriptBlock {
    param($R, $LIGAS, $saida)
    $env:CM_VAGAS = '4'
    Set-Location "$R\copias\bancada3\engenharia-reversa"
    "=== COMPARANDO 8 LIGAS ate 2002-06-01 ($LIGAS)" | Out-File "$saida\comparar_8ligas.txt" -Encoding utf8
    python -u simular.py --ate 2002-06-01 --motor sim --modo comparar --ligas $LIGAS --limite 30000 *>> "$saida\comparar_8ligas.txt"
    Copy-Item ..\jogo_testes\ddraw.log "$saida\comparar_8ligas_ddraw.log" -Force
    "=== COMPARANDO ENG SCO IRL ITA ESP ate 2002-06-01" | Out-File "$saida\comparar_sco_irl.txt" -Encoding utf8
    python -u simular.py --ate 2002-06-01 --motor sim --modo comparar --ligas 'ENG SCO IRL ITA ESP' --limite 20000 *>> "$saida\comparar_sco_irl.txt"
    Copy-Item ..\jogo_testes\ddraw.log "$saida\comparar_sco_irl_ddraw.log" -Force
}

# bancada 2: as carreiras com as 8 ligas ate 2003-01-31 (Rust x2 + detector, depois a original)
$b2 = Start-Job -ArgumentList $R, $LIGAS, $saida -ScriptBlock {
    param($R, $LIGAS, $saida)
    $env:CM_VAGAS = '4'
    Set-Location "$R\copias\bancada2\engenharia-reversa"
    "=== CARREIRAS 8 LIGAS ate 2003-01-31 ($LIGAS)" | Out-File "$saida\carreiras_8ligas.txt" -Encoding utf8
    python -u simular.py --ate 2003-01-31 --motor sim --modo rust --ligas $LIGAS --repetir 2 --detector --limite 7200 *>> "$saida\carreiras_8ligas.txt"
    python -u simular.py --ate 2003-01-31 --motor sim --modo original --ligas $LIGAS --limite 7200 *>> "$saida\carreiras_8ligas.txt"
}

Wait-Job $b2, $b3 | Out-Null
Receive-Job $b2, $b3 *>> "$saida\jobs.txt"
"=== FIM $(Get-Date -Format 'HH:mm')" | Out-File "$saida\FIM.txt" -Encoding utf8
