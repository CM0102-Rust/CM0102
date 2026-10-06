# Formatos dos arquivos de dados (`Data\*.dat`)

Gerado por `engenharia-reversa/formatos.py`. Tamanhos medidos nos arquivos da instalação 3.9.68; layouts das estruturas lidos de `referencias/CM0102Patcher/SaveChanger/Structures.cs` (nomes de campo do nckstwrt). Uma estrutura é marcada como **confirmada** quando o total dela bate com o registro medido.

## `index.dat`

8 bytes de cabeçalho e entradas de 67 bytes: nome (51, terminado em zero), tipo (i32), quantidade (i32), deslocamento no arquivo (i32), versão (i32). O `staff.dat` guarda três tabelas em sequência (tipos 6, 9 e 10) e uma vazia (8).

| Arquivo | Tipo | Registros | Deslocamento | Versão | Bytes/registro | Estrutura | Confere |
|---|---:|---:|---:|---:|---:|---|---|
| `club.dat` | 0 | 13214 | 0 | 2 | 581 | TClub | **sim** |
| `nat_club.dat` | 1 | 426 | 0 | 2 | 581 | TClub | **sim** |
| `colour.dat` | 2 | 34 | 0 | 1 | 58 | ? | — |
| `continent.dat` | 3 | 6 | 0 | 1 | 198 | TContinent | **sim** |
| `nation.dat` | 4 | 213 | 0 | 2 | 290 | TNation | **sim** |
| `officials.dat` | 7 | 4567 | 0 | 1 | 43 | ? | — |
| `stadium.dat` | 5 | 8917 | 0 | 1 | 78 | TStadiums | **sim** |
| `staff.dat` | 6 | 169342 | 0 | 1 | 157 | TStaff | não (110) |
| `staff.dat` | 8 | 0 | 26586694 | 1 | — | ? | — |
| `staff.dat` | 9 | 30865 | 26586694 | 2 | 68 | TNonPlayer | **sim** |
| `staff.dat` | 10 | 139769 | 28685514 | 2 | 70 | TPlayer | **sim** |
| `staff_comp.dat` | 11 | 543 | 0 | 2 | 101 | TStaffComp | **sim** |
| `city.dat` | 21 | 6682 | 0 | 1 | 56 | TCities | **sim** |
| `club_comp.dat` | 12 | 433 | 0 | 2 | 107 | TComp | **sim** |
| `nation_comp.dat` | 16 | 19 | 0 | 2 | 107 | TComp | **sim** |
| `first_names.dat` | 13 | 44281 | 0 | 1 | 60 | TNames | **sim** |
| `second_names.dat` | 14 | 104395 | 0 | 1 | 60 | TNames | **sim** |
| `common_names.dat` | 15 | 13256 | 0 | 1 | 60 | TNames | **sim** |
| `staff_history.dat` | 17 | 385846 | 0 | 1 | 17 | TStaffHistory | **sim** |
| `staff_comp_history.dat` | 18 | 2609 | 0 | 1 | 58 | TStaffCompHistory | **sim** |
| `club_comp_history.dat` | 19 | 10766 | 0 | 1 | 26 | TCompHistory | **sim** |
| `nation_comp_history.dat` | 20 | 273 | 0 | 1 | 26 | TCompHistory | **sim** |

## Pendências

- **`staff.dat` versão 1 (157 bytes por pessoa):** é a base original do 3.9.68. A referência só descreve a versão 2 das atualizações da comunidade (`TStaff`, 110 bytes, mais as preferências numa tabela à parte, tipo 22, `TPreferences` de 52 bytes). Os primeiros campos coincidem (o `YearChanger` do patcher usa nascimento em +0x10, ano de nascimento em +0x18 e datas de contrato em +0x3e/+0x46 nas duas versões). A hipótese "versão 1 = pessoa + preferências embutidas" não fecha exato (110 + 52 − 4 = 158, não 157); falta mapear os 47 bytes finais.
- **`colour.dat` (58 bytes) e `officials.dat` (43 bytes):** sem estrutura na referência. Dos árbitros, o `RefereePatcher` usa capacidade atual/potencial (i16 em +30/+32) e disciplina (byte em +37).

## Estruturas

### `TCMDate` (8 bytes)

| Desl. | Bytes | Tipo | Campo |
|---:|---:|---|---|
| 0x0 | 2 | `short` | Day |
| 0x2 | 2 | `short` | Year |
| 0x4 | 4 | `int` | LeapYear |

### `TContract` (80 bytes)

| Desl. | Bytes | Tipo | Campo |
|---:|---:|---|---|
| 0x0 | 4 | `int` | ID |
| 0x4 | 4 | `int` | Club |
| 0x8 | 4 | `int` | Unknown |
| 0xc | 4 | `int` | Wage |
| 0x10 | 4 | `int` | GoalBonus |
| 0x14 | 4 | `int` | AssistBonus |
| 0x18 | 4 | `int` | CleanSheetBonus |
| 0x1c | 1 | `byte` | NonPromotionRC |
| 0x1d | 1 | `byte` | MinimumFeeRC |
| 0x1e | 1 | `byte` | NonPlayingRC |
| 0x1f | 1 | `byte` | RelegationRC |
| 0x20 | 1 | `byte` | ManagerJobRC |
| 0x21 | 4 | `int` | ReleaseFee |
| 0x25 | 8 | `TCMDate` | DateStarted |
| 0x2d | 8 | `TCMDate` | ContractExpires |
| 0x35 | 1 | `byte` | ContractType |
| 0x36 | 8 | `UInt64` | Unknown18_1 |
| 0x3e | 8 | `UInt64` | Unknown18_2 |
| 0x46 | 2 | `short` | Unknown18_3 |
| 0x48 | 1 | `byte` | Unknown18_4 |
| 0x49 | 1 | `byte` | LeavingOnBosman |
| 0x4a | 4 | `int` | TransferArrangedFor |
| 0x4e | 1 | `byte` | TransferStatus |
| 0x4f | 1 | `byte` | SquadStatus |

### `TStaff` (110 bytes)

| Desl. | Bytes | Tipo | Campo |
|---:|---:|---|---|
| 0x0 | 4 | `int` | ID |
| 0x4 | 4 | `int` | FirstName |
| 0x8 | 4 | `int` | SecondName |
| 0xc | 4 | `int` | CommonName |
| 0x10 | 8 | `TCMDate` | DateOfBirth |
| 0x18 | 2 | `UInt16` | YearOfBirth |
| 0x1a | 4 | `int` | Nation |
| 0x1e | 4 | `int` | SecondNation |
| 0x22 | 1 | `byte` | IntApps |
| 0x23 | 1 | `byte` | IntGoals |
| 0x24 | 4 | `int` | NationalJob |
| 0x28 | 1 | `byte` | JobForNation |
| 0x29 | 8 | `TCMDate` | DateJoinedNation |
| 0x31 | 8 | `TCMDate` | DateExpiresNation |
| 0x39 | 4 | `int` | ClubJob |
| 0x3d | 1 | `byte` | JobForClub |
| 0x3e | 8 | `TCMDate` | DateJoinedClub |
| 0x46 | 8 | `TCMDate` | DateExpiresClub |
| 0x4e | 4 | `int` | Wage |
| 0x52 | 4 | `int` | Value |
| 0x56 | 1 | `byte` | Adaptability |
| 0x57 | 1 | `byte` | Ambition |
| 0x58 | 1 | `byte` | Determination |
| 0x59 | 1 | `byte` | Loyality |
| 0x5a | 1 | `byte` | Pressure |
| 0x5b | 1 | `byte` | Professionalism |
| 0x5c | 1 | `byte` | Sportsmanship |
| 0x5d | 1 | `byte` | Temperament |
| 0x5e | 1 | `byte` | PlayingSquad |
| 0x5f | 1 | `byte` | Classification |
| 0x60 | 1 | `byte` | ClubValuation |
| 0x61 | 4 | `int` | Player |
| 0x65 | 4 | `int` | StaffPreferences |
| 0x69 | 4 | `int` | NonPlayer |
| 0x6d | 1 | `byte` | SquadSelectedFor |

### `TPreferences` (52 bytes)

| Desl. | Bytes | Tipo | Campo |
|---:|---:|---|---|
| 0x0 | 4 | `int` | StaffPreferencesID |
| 0x4 | 4 | `int` | StaffFavouriteClubs1 |
| 0x8 | 4 | `int` | StaffFavouriteClubs2 |
| 0xc | 4 | `int` | StaffFavouriteClubs3 |
| 0x10 | 4 | `int` | StaffDislikedClubs1 |
| 0x14 | 4 | `int` | StaffDislikedClubs2 |
| 0x18 | 4 | `int` | StaffDislikedClubs3 |
| 0x1c | 4 | `int` | StaffFavouriteStaff1 |
| 0x20 | 4 | `int` | StaffFavouriteStaff2 |
| 0x24 | 4 | `int` | StaffFavouriteStaff3 |
| 0x28 | 4 | `int` | StaffDislikedStaff1 |
| 0x2c | 4 | `int` | StaffDislikedStaff2 |
| 0x30 | 4 | `int` | StaffDislikedStaff3 |

### `TPlayer` (70 bytes)

| Desl. | Bytes | Tipo | Campo |
|---:|---:|---|---|
| 0x0 | 4 | `int` | ID |
| 0x4 | 1 | `byte` | SquadNumber |
| 0x5 | 2 | `UInt16` | CurrentAbility |
| 0x7 | 2 | `Int16` | PotentialAbility |
| 0x9 | 2 | `UInt16` | HomeReputation |
| 0xb | 2 | `UInt16` | CurrentReputation |
| 0xd | 2 | `UInt16` | WorldReputation |
| 0xf | 1 | `sbyte` | Goalkeeper |
| 0x10 | 1 | `sbyte` | Sweeper |
| 0x11 | 1 | `sbyte` | Defender |
| 0x12 | 1 | `sbyte` | DefensiveMidfielder |
| 0x13 | 1 | `sbyte` | Midfielder |
| 0x14 | 1 | `sbyte` | AttackingMidfielder |
| 0x15 | 1 | `sbyte` | Attacker |
| 0x16 | 1 | `sbyte` | WingBack |
| 0x17 | 1 | `sbyte` | RightSide |
| 0x18 | 1 | `sbyte` | LeftSide |
| 0x19 | 1 | `sbyte` | Central |
| 0x1a | 1 | `sbyte` | FreeRole |
| 0x1b | 1 | `sbyte` | Acceleration |
| 0x1c | 1 | `sbyte` | Aggression |
| 0x1d | 1 | `sbyte` | Agility |
| 0x1e | 1 | `sbyte` | Anticipation |
| 0x1f | 1 | `sbyte` | Balance |
| 0x20 | 1 | `sbyte` | Bravery |
| 0x21 | 1 | `sbyte` | Consistency |
| 0x22 | 1 | `sbyte` | Corners |
| 0x23 | 1 | `sbyte` | Crossing |
| 0x24 | 1 | `sbyte` | Decisions |
| 0x25 | 1 | `sbyte` | Dirtiness |
| 0x26 | 1 | `sbyte` | Dribbling |
| 0x27 | 1 | `sbyte` | Finishing |
| 0x28 | 1 | `sbyte` | Flair |
| 0x29 | 1 | `sbyte` | FreeKicks |
| 0x2a | 1 | `sbyte` | Handling |
| 0x2b | 1 | `sbyte` | Heading |
| 0x2c | 1 | `sbyte` | ImportantMatches |
| 0x2d | 1 | `sbyte` | InjuryProneness |
| 0x2e | 1 | `sbyte` | Jumping |
| 0x2f | 1 | `sbyte` | Leadership |
| 0x30 | 1 | `sbyte` | LeftFoot |
| 0x31 | 1 | `sbyte` | LongShots |
| 0x32 | 1 | `sbyte` | Marking |
| 0x33 | 1 | `sbyte` | Movement |
| 0x34 | 1 | `sbyte` | NaturalFitness |
| 0x35 | 1 | `sbyte` | OneOnOnes |
| 0x36 | 1 | `sbyte` | PlayerPace |
| 0x37 | 1 | `sbyte` | Passing |
| 0x38 | 1 | `sbyte` | Penalties |
| 0x39 | 1 | `sbyte` | Positioning |
| 0x3a | 1 | `sbyte` | Reflexes |
| 0x3b | 1 | `sbyte` | RightFoot |
| 0x3c | 1 | `sbyte` | Stamina |
| 0x3d | 1 | `sbyte` | Strength |
| 0x3e | 1 | `sbyte` | Tackling |
| 0x3f | 1 | `sbyte` | Teamwork |
| 0x40 | 1 | `sbyte` | Technique |
| 0x41 | 1 | `sbyte` | ThrowIns |
| 0x42 | 1 | `sbyte` | Versatility |
| 0x43 | 1 | `sbyte` | Vision |
| 0x44 | 1 | `sbyte` | WorkRate |
| 0x45 | 1 | `byte` | PlayerMorale |

### `TNonPlayer` (68 bytes)

| Desl. | Bytes | Tipo | Campo |
|---:|---:|---|---|
| 0x0 | 4 | `int` | ID |
| 0x4 | 2 | `UInt16` | CurrentAbility |
| 0x6 | 2 | `UInt16` | PotentialAbility |
| 0x8 | 2 | `UInt16` | HomeReputation |
| 0xa | 2 | `UInt16` | CurrentReputation |
| 0xc | 2 | `UInt16` | WorldReputation |
| 0xe | 1 | `sbyte` | Attacking |
| 0xf | 1 | `sbyte` | Business |
| 0x10 | 1 | `sbyte` | Coaching |
| 0x11 | 1 | `sbyte` | CoachingGks |
| 0x12 | 1 | `sbyte` | CoachingTechnique |
| 0x13 | 1 | `sbyte` | Directness |
| 0x14 | 1 | `sbyte` | Discipline |
| 0x15 | 1 | `sbyte` | FreeRoles |
| 0x16 | 1 | `sbyte` | Interference |
| 0x17 | 1 | `sbyte` | Judgement |
| 0x18 | 1 | `sbyte` | JudgingPotential |
| 0x19 | 1 | `sbyte` | ManHandling |
| 0x1a | 1 | `sbyte` | Marking |
| 0x1b | 1 | `sbyte` | Motivating |
| 0x1c | 1 | `sbyte` | Offside |
| 0x1d | 1 | `sbyte` | Patience |
| 0x1e | 1 | `sbyte` | Physiotherapy |
| 0x1f | 1 | `sbyte` | Pressing |
| 0x20 | 1 | `sbyte` | Resources |
| 0x21 | 1 | `sbyte` | Tactics |
| 0x22 | 1 | `sbyte` | Youngsters |
| 0x23 | 4 | `int` | Goalkeeper |
| 0x27 | 4 | `int` | Sweeper |
| 0x2b | 4 | `int` | Defender |
| 0x2f | 4 | `int` | DefensiveMidfielder |
| 0x33 | 4 | `int` | Midfielder |
| 0x37 | 4 | `int` | AttackingMidfielder |
| 0x3b | 4 | `int` | Attacker |
| 0x3f | 4 | `int` | WingBack |
| 0x43 | 1 | `sbyte` | Formation |

### `TClub` (581 bytes)

| Desl. | Bytes | Tipo | Campo |
|---:|---:|---|---|
| 0x0 | 4 | `int` | ID |
| 0x4 | 51 | `byte[][51]` | Name |
| 0x37 | 1 | `byte` | GenderName |
| 0x38 | 26 | `byte[][26]` | ShortName |
| 0x52 | 1 | `byte` | ShortGenderName |
| 0x53 | 4 | `int` | Nation |
| 0x57 | 4 | `int` | Division |
| 0x5b | 4 | `int` | LastDivision |
| 0x5f | 1 | `byte` | LastPosition |
| 0x60 | 4 | `int` | ReserveDivision |
| 0x64 | 1 | `byte` | ProfessionalStatus |
| 0x65 | 4 | `int` | Cash |
| 0x69 | 4 | `int` | Stadium |
| 0x6d | 1 | `byte` | OwnStadium |
| 0x6e | 4 | `int` | ReserveStadium |
| 0x72 | 1 | `byte` | MatchDay |
| 0x73 | 4 | `int` | Attendance |
| 0x77 | 4 | `int` | MinAttendance |
| 0x7b | 4 | `int` | MaxAttendance |
| 0x7f | 1 | `byte` | Training |
| 0x80 | 2 | `UInt16` | Reputation |
| 0x82 | 1 | `byte` | PLC |
| 0x83 | 4 | `int` | ForeColour1 |
| 0x87 | 4 | `int` | BackColour1 |
| 0x8b | 4 | `int` | ForeColour2 |
| 0x8f | 4 | `int` | BackColour2 |
| 0x93 | 4 | `int` | ForeColour3 |
| 0x97 | 4 | `int` | BackColour3 |
| 0x9b | 4 | `int` | FavStaff1 |
| 0x9f | 4 | `int` | FavStaff2 |
| 0xa3 | 4 | `int` | FavStaff3 |
| 0xa7 | 4 | `int` | DisStaff1 |
| 0xab | 4 | `int` | DisStaff2 |
| 0xaf | 4 | `int` | DisStaff3 |
| 0xb3 | 4 | `int` | Rival1 |
| 0xb7 | 4 | `int` | Rival2 |
| 0xbb | 4 | `int` | Rival3 |
| 0xbf | 4 | `int` | Chairman |
| 0xc3 | 12 | `int[][3]` | Directors |
| 0xcf | 4 | `int` | Manager |
| 0xd3 | 4 | `int` | AssistantManager |
| 0xd7 | 200 | `int[][50]` | Squad |
| 0x19f | 20 | `int[][5]` | Coaches |
| 0x1b3 | 28 | `int[][7]` | Scouts |
| 0x1cf | 12 | `int[][3]` | Physios |
| 0x1db | 4 | `int` | EuroFlag |
| 0x1df | 1 | `byte` | EuroSeeding |
| 0x1e0 | 80 | `int[][20]` | TeamSelected |
| 0x230 | 16 | `int[][4]` | TacticTraining |
| 0x240 | 4 | `int` | TacticSelected |
| 0x244 | 1 | `byte` | HasLinkedClub |

### `TComp` (107 bytes)

| Desl. | Bytes | Tipo | Campo |
|---:|---:|---|---|
| 0x0 | 4 | `int` | ID |
| 0x4 | 51 | `byte[][51]` | Name |
| 0x37 | 1 | `byte` | GenderName |
| 0x38 | 26 | `byte[][26]` | ShortName |
| 0x52 | 1 | `byte` | ShortGenderName |
| 0x53 | 4 | `byte[][4]` | ThreeLetterName |
| 0x57 | 1 | `sbyte` | ClubCompScope |
| 0x58 | 1 | `sbyte` | ClubCompSelected |
| 0x59 | 4 | `int` | ClubCompContinent |
| 0x5d | 4 | `int` | ClubCompNation |
| 0x61 | 4 | `int` | ClubCompForegroundColour |
| 0x65 | 4 | `int` | ClubCompBackgroundColour |
| 0x69 | 2 | `Int16` | ClubCompReputation |

### `TStaffComp` (101 bytes)

| Desl. | Bytes | Tipo | Campo |
|---:|---:|---|---|
| 0x0 | 4 | `int` | ID |
| 0x4 | 51 | `byte[][51]` | Name |
| 0x37 | 1 | `byte` | GenderName |
| 0x38 | 26 | `byte[][26]` | ShortName |
| 0x52 | 1 | `sbyte` | GenderNameShort |
| 0x53 | 4 | `int` | Continent |
| 0x57 | 4 | `int` | Nation |
| 0x5b | 4 | `int` | ForegroundColour |
| 0x5f | 4 | `int` | BackgroundColour |
| 0x63 | 2 | `Int16` | StaffCompReputation |

### `TCompHistory` (26 bytes)

| Desl. | Bytes | Tipo | Campo |
|---:|---:|---|---|
| 0x0 | 4 | `int` | ID |
| 0x4 | 4 | `int` | Comp |
| 0x8 | 2 | `Int16` | Year |
| 0xa | 4 | `int` | Winners |
| 0xe | 4 | `int` | RunnersUp |
| 0x12 | 4 | `int` | ThirdPlace |
| 0x16 | 4 | `int` | Host |

### `TStaffCompHistory` (58 bytes)

| Desl. | Bytes | Tipo | Campo |
|---:|---:|---|---|
| 0x0 | 4 | `int` | ID |
| 0x4 | 4 | `int` | Comp |
| 0x8 | 2 | `Int16` | Year |
| 0xa | 4 | `int` | WinnersFirstName |
| 0xe | 4 | `int` | WinnersSecondName |
| 0x12 | 4 | `int` | WinnerID |
| 0x16 | 4 | `int` | WinnerInfo |
| 0x1a | 4 | `int` | RunnersUpFirstName |
| 0x1e | 4 | `int` | RunnersUpSecondName |
| 0x22 | 4 | `int` | RunnersUpID |
| 0x26 | 4 | `int` | RunnersUpInfo |
| 0x2a | 4 | `int` | ThirdPlaceFirstName |
| 0x2e | 4 | `int` | ThirdPlaceSecondName |
| 0x32 | 4 | `int` | ThirdPlaceID |
| 0x36 | 4 | `int` | ThirdPlaceInfo |

### `TStaffHistory` (17 bytes)

| Desl. | Bytes | Tipo | Campo |
|---:|---:|---|---|
| 0x0 | 4 | `int` | ID |
| 0x4 | 4 | `int` | StaffID |
| 0x8 | 2 | `Int16` | Year |
| 0xa | 4 | `int` | ClubID |
| 0xe | 1 | `sbyte` | OnLoan |
| 0xf | 1 | `sbyte` | Apps |
| 0x10 | 1 | `sbyte` | Goals |

### `TNames` (60 bytes)

| Desl. | Bytes | Tipo | Campo |
|---:|---:|---|---|
| 0x0 | 51 | `byte[][51]` | Name |
| 0x33 | 4 | `int` | ID |
| 0x37 | 4 | `int` | Nation |
| 0x3b | 1 | `sbyte` | Count |

### `TStadiums` (78 bytes)

| Desl. | Bytes | Tipo | Campo |
|---:|---:|---|---|
| 0x0 | 4 | `int` | ID |
| 0x4 | 51 | `byte[][51]` | Name |
| 0x37 | 1 | `byte` | StadiumGenderName |
| 0x38 | 4 | `int` | StadiumCity |
| 0x3c | 4 | `int` | StadiumCapacity |
| 0x40 | 4 | `int` | StadiumSeatingCapacity |
| 0x44 | 4 | `int` | StadiumExpansionCapacity |
| 0x48 | 4 | `int` | StadiumNearbyStadium |
| 0x4c | 1 | `byte` | StadiumCovered |
| 0x4d | 1 | `byte` | StadiumUnderSoilHeating |

### `TCities` (56 bytes)

| Desl. | Bytes | Tipo | Campo |
|---:|---:|---|---|
| 0x0 | 4 | `int` | ID |
| 0x4 | 26 | `byte[][26]` | Name |
| 0x1e | 1 | `char` | CityGenderName |
| 0x1f | 4 | `int` | CityNation |
| 0x23 | 8 | `double` | CityLatitude |
| 0x2b | 8 | `double` | CityLongitude |
| 0x33 | 1 | `char` | CityAttraction |
| 0x34 | 4 | `int` | CityWeather |

### `TNation` (290 bytes)

| Desl. | Bytes | Tipo | Campo |
|---:|---:|---|---|
| 0x0 | 4 | `int` | ID |
| 0x4 | 51 | `byte[][51]` | Name |
| 0x37 | 1 | `byte` | GenderName |
| 0x38 | 26 | `byte[][26]` | ShortName |
| 0x52 | 1 | `byte` | ShortGenderName |
| 0x53 | 4 | `byte[][4]` | ThreeLetterName |
| 0x57 | 26 | `byte[][26]` | Nationality |
| 0x71 | 4 | `int` | Continent |
| 0x75 | 1 | `sbyte` | Region |
| 0x76 | 1 | `sbyte` | ActualRegion |
| 0x77 | 1 | `sbyte` | FirstLanguage |
| 0x78 | 1 | `sbyte` | SecondLanguage |
| 0x79 | 1 | `sbyte` | ThirdLanguage |
| 0x7a | 4 | `int` | CapitalCity |
| 0x7e | 1 | `sbyte` | StateOfDevelopment |
| 0x7f | 1 | `sbyte` | GroupMembership |
| 0x80 | 4 | `int` | NationalStadium |
| 0x84 | 1 | `sbyte` | GameImportance |
| 0x85 | 1 | `sbyte` | LeagueStandard |
| 0x86 | 2 | `Int16` | NumberClubs |
| 0x88 | 4 | `int` | NumberStaff |
| 0x8c | 2 | `Int16` | SeasonUpdateDay |
| 0x8e | 2 | `Int16` | Reputation |
| 0x90 | 4 | `int` | ForegroundColour1 |
| 0x94 | 4 | `int` | BackgroundColour1 |
| 0x98 | 4 | `int` | ForegroundColour2 |
| 0x9c | 4 | `int` | BackgroundColour2 |
| 0xa0 | 4 | `int` | ForegroundColour3 |
| 0xa4 | 4 | `int` | BackgroundColour3 |
| 0xa8 | 8 | `double` | FIFACoefficient |
| 0xb0 | 8 | `double` | FIFACoefficient91 |
| 0xb8 | 8 | `double` | FIFACoefficient92 |
| 0xc0 | 8 | `double` | FIFACoefficient93 |
| 0xc8 | 8 | `double` | FIFACoefficient94 |
| 0xd0 | 8 | `double` | FIFACoefficient95 |
| 0xd8 | 8 | `double` | FIFACoefficient96 |
| 0xe0 | 8 | `double` | UEFACoefficient91 |
| 0xe8 | 8 | `double` | UEFACoefficient92 |
| 0xf0 | 8 | `double` | UEFACoefficient93 |
| 0xf8 | 8 | `double` | UEFACoefficient94 |
| 0x100 | 8 | `double` | UEFACoefficient95 |
| 0x108 | 8 | `double` | UEFACoefficient96 |
| 0x110 | 4 | `int` | Rivals1 |
| 0x114 | 4 | `int` | Rivals2 |
| 0x118 | 4 | `int` | Rivals3 |
| 0x11c | 1 | `sbyte` | NationLeagueSelected |
| 0x11d | 4 | `int` | NationShortlistOffset |
| 0x121 | 1 | `sbyte` | NationGamesPlayed |

### `TContinent` (198 bytes)

| Desl. | Bytes | Tipo | Campo |
|---:|---:|---|---|
| 0x0 | 4 | `int` | ContinentID |
| 0x4 | 26 | `byte[][26]` | ContinentName |
| 0x1e | 1 | `char` | ContinentGenderName |
| 0x1f | 4 | `byte[][4]` | ContinentNameThreeLetter |
| 0x23 | 26 | `byte[][26]` | ContinentNameContinentality |
| 0x3d | 101 | `byte[][101]` | ContinentFederationName |
| 0xa2 | 1 | `char` | ContinentGenderFederationName |
| 0xa3 | 26 | `byte[][26]` | ContinentFederationNameShort |
| 0xbd | 1 | `char` | ContinentGenderFederationNameShort |
| 0xbe | 8 | `double` | ContinentRegionalStrength |

### `TIndex` (67 bytes)

| Desl. | Bytes | Tipo | Campo |
|---:|---:|---|---|
| 0x0 | 51 | `byte[][51]` | Name |
| 0x33 | 4 | `int` | FileType |
| 0x37 | 4 | `int` | Count |
| 0x3b | 4 | `int` | Offset |
| 0x3f | 4 | `int` | Version |
