// ---------- batch %%LEVA%%: the credits screen (contract_screens.cpp, part of it): the Additional Research rows ----------

/// The Additional Research names (exe texts), in the order of the original (698).
const ADDITIONAL_RESEARCH_NAMES: [u32; 698] = [
    0x9b4b0c, // Tony Persoglia
    0x9b4afc, // Jeremy Lowe
    0x9b4aec, // Andrew Marler
    0x9b4adc, // Simon Treanor
    0x9b4ac8, // Nick Vatmanidis
    0x9b4ab8, // Jerko Zuvela
    0x9b4aac, // Albert Au
    0x9b4a98, // Matthew Cheeseman
    0x9b4a88, // Wayne Dingle
    0x9b4a78, // James Doughty
    0x9b4a6c, // Derek Lau
    0x9b4a5c, // Stephen Lewis
    0x9b4a4c, // Minkey Sellers
    0x9b4a38, // Michelle Taplin
    0x9b4a24, // Bailey Thorndyke
    0x9b4a14, // Clinton Tolkin
    0x9b4a04, // Mark Thompson
    0x9b49f0, // Albert 'Ali' Kropf
    0x9b49d4, // Matthias 'Opul' Obermayr
    0x9b49bc, // Bernd 'BTC' Kronawetter
    0x9b49a0, // Alexander 'Shodan' Schnider
    0x9b4984, // Alexander 'Shadohawk' Muhr
    0x9b4974, // Tarek Mustafa
    0x9b4958, // Thomas 'Divan' Kutschera
    0x9b4940, // Yannick Boudringhien
    0x9b492c, // Christophe Boulaert
    0x9b491c, // Olivier Brackx
    0x9b490c, // Sepp De Coninck
    0x9b48fc, // Bart Deckers
    0x9b48ec, // Cédric Dejonghe
    0x9b48d8, // Petro Depraetere
    0x9b48cc, // Kim Depree
    0x9b48b8, // Diederik Dewaele
    0x9b48a8, // Pascal D'Helft
    0x9b4898, // Narcisse Genin
    0x9b4884, // Stéphane Glowacki
    0x9b4870, // Remi Keirsebilck
    0x9b4864, // Juan Hamers
    0x9b4854, // Benoît Humbled
    0x9b4840, // Emmanuel Hoornaert
    0x9b4830, // John Jespers
    0x9b4824, // Paul Jacobs
    0x9b4818, // Tom Jottier
    0x9b4804, // Christoph Lavens
    0x9b47f4, // Thierry Lepage
    0x9b47e0, // Olivier Mathurin
    0x9b47cc, // Jessie Meuleman
    0x9b47c0, // Yves Meyen
    0x9b47b0, // Nicolas Muller
    0x9b47a0, // Frank Nielandt
    0x9b4790, // Jonas Onkelinx
    0x9b4784, // Bart Packo
    0x9b4770, // David Philippart
    0x9b4760, // Jeroen Pots
    0x9b474c, // Frederik Rambout
    0x9b4740, // Tim Rollé
    0x9b4730, // Dave Stynen
    0x9b4720, // Luc Sysmans
    0x9b470c, // Vincent Tibackx
    0x9b46f8, // Jan Valkenborgh
    0x9b46e4, // Ronan Van Aubel
    0x9b46cc, // Franky Vandenberghe
    0x9b46b4, // Yves Van Den Berghe
    0x9b469c, // Jom Vangoidtsenhoven
    0x9b4688, // Kris Vanmunster
    0x9b4674, // Tim Van Speybroeck
    0x9b4660, // Pieter Vantieghem
    0x9b464c, // Mathias Vissers
    0x9b463c, // Stefan Wauters
    0x9b462c, // Gilles Wilmart
    0x9b4618, // Evert Winkelmans
    0x9b4608, // Marc Van Lierop
    0x9b45f0, // Thiago 'Azulas' de Rose
    0x9b45d8, // Paulo 'PMLF' Freitas
    0x9b45c8, // Thiago Kuceki
    0x9b45b4, // Rodrigo 'Biro' Osna
    0x9b4598, // Thiago 'Lee Hodges' Porto
    0x9b458c, // Alex Amaral
    0x9b4580, // Igor Jordão
    0x9b4570, // Gustavo Militão
    0x9b4564, // Filipe Melo
    0x9b4554, // Alan Capitanio
    0x9b4548, // Genaro Vaz
    0x9b453c, // Léo Fauth
    0x9b452c, // Matheus Antunes
    0x9b451c, // Eduardo Pacheco
    0x9b450c, // Juliano Lorenz
    0x9b4500, // Luiz Guidi
    0x9b44f0, // Daniel Cardoso
    0x9b44e4, // Liu Xing
    0x9b44d8, // Wang Zhixin
    0x9b44cc, // Zhang Zhen
    0x9b44bc, // Zhou Zhanxiang
    0x9b44ac, // Zhuang Xiaofei
    0x9b4494, // Matthew Li (Hong Kong)
    0x9b4484, // Joso Puljic
    0x9b4470, // Berislav Jagustin
    0x9b4464, // Alen Brkic
    0x9b4450, // Tomislav Jakopanec
    0x9b4440, // Dario Karnik
    0x9b442c, // Antonio Kuzmanic
    0x9b4420, // Orsat Lasic
    0x9b440c, // Ivan Prgomet Selak
    0x9b4400, // Ivan Sunic
    0x9b43f0, // Sinisa Topalov
    0x9b43e0, // Ivan Buconjic
    0x9b43d4, // Nino Burek
    0x9b43c8, // Dario Kozul
    0x9b43b8, // Zoran Mokrovcak
    0x9b43ac, // Josip Music
    0x9b4394, // Jari-Pekka Aarrekivi
    0x9b437c, // Sebastian Aniszewski
    0x9b436c, // Jani Arponen
    0x9b435c, // Mikko Aspelin
    0x9b434c, // Jörgen Bolin
    0x9b4338, // Joakim Eriksson
    0x9b4328, // Thomas Forsten
    0x9b4318, // Kalle Helin
    0x9b4304, // Patrik Herrgård
    0x9b42f4, // Kim Ittonen
    0x9b42e4, // Juho Kauppi
    0x9b42d4, // Juha Korhonen
    0x9b42c0, // Teemu Kärenlampi
    0x9b42b0, // Jarno Laiho
    0x9b429c, // Raine Lindqvist
    0x9b428c, // Antti Luode
    0x9b4280, // Esko Lämsä
    0x9b426c, // Tuomas Makkonen
    0x9b425c, // Jukka Mononen
    0x9b4248, // Andrei Neuvonen
    0x9b4238, // Jari Okkonen
    0x9b4228, // Patrik Olin
    0x9b4218, // Tapani Olkku
    0x9b4204, // Hannu Piiparinen
    0x9b41f4, // Jussi Pirinen
    0x9b41e4, // Johan Prest
    0x9b41d0, // Kimmo Rouhiainen
    0x9b41c0, // Mikko Salonen
    0x9b41a8, // Jari-Pekka Siivonen
    0x9b4198, // Sakari Similä
    0x9b4184, // Mikael Sorjonen
    0x9b4174, // Patrick Suves
    0x9b4164, // Tuomas Vaalto
    0x9b4154, // Risto Kuustera
    0x9b4140, // Arnaud Guelfucci
    0x9b4134, // B@ptiste
    0x9b4128, // B.Chalut
    0x9b411c, // Barthfan
    0x9b410c, // CharlieRoach
    0x9b40f8, // Finton Sirockin
    0x9b40e8, // Forza Tolosa
    0x9b40d8, // Jordan Dessoye
    0x9b40d0, // Romjet
    0x9b40c4, // Striker
    0x9b40b4, // Thomas Croux
    0x9b40a4, // Ronald Demohn
    0x9b4094, // Robert Demohn
    0x9b4084, // Günther Demohn
    0x9b4070, // Alisan Tasdelen
    0x9b405c, // Christian Meyer
    0x9b404c, // Stefan Hack
    0x9b403c, // Wilhelm Döbel
    0x9b4028, // Giorgos Tartaris
    0x9b4018, // Tasos Varsos
    0x9b4000, // Nikolaos Giannoutsos
    0x9b3fec, // Riccardo Accossato
    0x9b3fd8, // Gianpaolo Altamura
    0x9b3fc0, // Ugo 'Grifone' Amoretti
    0x9b3fa4, // Stefano 'Stark' Andreoli
    0x9b3f94, // Luca Argiolas
    0x9b3f84, // Davide Atria
    0x9b3f70, // Stefano Benericetti
    0x9b3f5c, // Pierpaolo Benvoluti
    0x9b3f50, // Luca Biondi
    0x9b3f44, // Andrea Boni
    0x9b3f2c, // Alessandro Borgonovo
    0x9b3f18, // Saverio Bozzolan
    0x9b3f08, // Franco Bramante
    0x9b3ef4, // Daniele Brunengo
    0x9b3edc, // Diego 'El Cholo' Bruno
    0x9b3ecc, // Emanuele Burri
    0x9b3ec0, // Luca Calia
    0x9b3eb4, // Enzo Cameli
    0x9b3ea0, // Alessandro Camera
    0x9b3e88, // Alessandro Cammarota
    0x9b3e74, // Davide Camporese
    0x9b3e64, // Canavese Calcio
    0x9b3e54, // Sabrina Caralli
    0x9b3e44, // Umberto Carbone
    0x9b3e30, // Walter Cavalcante
    0x9b3e20, // Santolo Ceglia
    0x9b3e10, // Fabrizio Celani
    0x9b3df4, // Cristiano 'Zoran' Cinacchio
    0x9b3de8, // Simone Cola
    0x9b3dd4, // Nicola Colasante
    0x9b3dc4, // Ivano Colombo
    0x9b3db4, // Andrea Consonni
    0x9b3da8, // Mario Costa
    0x9b3d98, // Ernesto Covino
    0x9b3d78, // Lorenzo 'Lord Lorenz' Cozzella
    0x9b3d68, // Niccolò Curti
    0x9b3d58, // Daniele Curzi
    0x9b3d44, // Diego Dalla Costa
    0x9b3d20, // Simone 'OlivoAscolano' Dalle Crode
    0x9b3d00, // Severina 'Mrs Panoz' Daniele
    0x9b3ce8, // Alessandro De Cillis
    0x9b3cd8, // Gianni De Gese
    0x9b3cc4, // Vincenzo D'Amico
    0x9b3ca8, // Nicola 'Santos81' D'Egidio
    0x9b3c94, // Enzo Della Morte
    0x9b3c80, // Andrea Della Valle
    0x9b3c6c, // Juza Delle Nuvole
    0x9b3c54, // Alessandro Desiderio
    0x9b3c44, // Franco Di Carlo
    0x9b3c24, // Federico 'Uccello' Di Cesare
    0x9b3c04, // Francesco 'Armida' Di Giovanni
    0x9b3bf4, // Luigi Di Guilmi
    0x9b3bd4, // Cristiano 'Emerald' Dionello
    0x9b3bbc, // Carlo 'Nerone' D'Orazio
    0x9b3bac, // Massimo Fagotto
    0x9b3b9c, // Andrea Ferrarin
    0x9b3b80, // Marco 'SuperMarco' Ferro
    0x9b3b68, // Michele 'Nagol' Fidati
    0x9b3b58, // Luca Filipasso
    0x9b3b48, // Matteo Foresti
    0x9b3b38, // Giorgio Forlano
    0x9b3b28, // Alberto Forte
    0x9b3b18, // Andrea Fortina
    0x9b3b00, // Valerio Francavillese
    0x9b3ae8, // Maria Gemma Galliano
    0x9b3ad4, // Ivan Garberoglio
    0x9b3ac0, // Angelo Gennarelli
    0x9b3aa0, // Fabio 'wrightpatterson' Gentile
    0x9b3a90, // Fabio Germanà
    0x9b3a80, // Fabio Ghiglione
    0x9b3a6c, // Alberto Giacomel
    0x9b3a5c, // Andrea Gilioli
    0x9b3a4c, // Gianpaolo Giolo
    0x9b3a3c, // Tommaso Grandi
    0x9b3a2c, // Luca Graniero
    0x9b3a1c, // Helmut Isella
    0x9b3a0c, // Andrea Ladas
    0x9b39f8, // Raffaele Larovere
    0x9b39e8, // Ettore Leali
    0x9b39cc, // Luca 'Fantaluca' Lezzeni
    0x9b39bc, // Marco Lombardo
    0x9b39ac, // Fabio Malerba
    0x9b399c, // Valerio Malerba
    0x9b398c, // Andrea Manetti
    0x9b397c, // Filippo Marroni
    0x9b396c, // Riccardo Marzi
    0x9b3954, // Guido 'Fox' Marzucchi
    0x9b3940, // Gabriele Mattiussi
    0x9b3928, // Mauro 'Ronald' Mercuri
    0x9b3914, // Emanuele Messina
    0x9b3904, // Nevio Millia
    0x9b38e8, // Marco 'Elvis' Montermini
    0x9b38d0, // Matteo 'Il Moro' Morigi
    0x9b38c0, // Davide Muraro
    0x9b38b4, // Theo Nuvoli
    0x9b389c, // Alvaro 'Alvios' Odierna
    0x9b388c, // Andreas Pagani
    0x9b3874, // Andrea 'Xain' Pandolfo
    0x9b3864, // Michele Panno
    0x9b3854, // Antonio Pantano
    0x9b383c, // Marco 'Panzer' Paone
    0x9b3820, // Stefano 'Bobanteo' Parola
    0x9b3810, // Enzo Petrassi
    0x9b3800, // Ivan Piastra
    0x9b37f4, // Manuel Pini
    0x9b37e4, // Giuseppe Piva
    0x9b37d8, // Mauro Polli
    0x9b37c4, // Daniele Pompignoli
    0x9b37a8, // Giulio 'last_one' Pompili
    0x9b3794, // Alessandro Porta
    0x9b3784, // Enrico Pozzi
    0x9b3768, // Massimo 'Joanlui' Prosperi
    0x9b3754, // Gianluca Purrello
    0x9b3740, // Michele Purrello
    0x9b3730, // Corrado Putzu
    0x9b371c, // Davide Quagliuolo
    0x9b3704, // Salvatore Quattrocchi
    0x9b36f4, // Enrico Rimondot
    0x9b36dc, // Stefania 'Oliva' Rinero
    0x9b36d0, // Bruno Röhl
    0x9b36bc, // Massimo Romagnoli
    0x9b36b0, // Mauro Rosa
    0x9b36a0, // Davide Rotunno
    0x9b3694, // Gianni Sala
    0x9b3680, // Enrico Santonastaso
    0x9b3670, // Marco Santucci
    0x9b3658, // Christian Scarpellini
    0x9b3648, // Daniele Scotta
    0x9b3638, // Virgilio Scotta
    0x9b362c, // Mauro Sgoku
    0x9b361c, // Sergio Solli
    0x9b360c, // David Stefanini
    0x9b35ec, // Giampietro 'Jumpiter' Stocco
    0x9b35d8, // Lorenzo Tamborrino
    0x9b35c4, // Giammarco Tammaro
    0x9b35b4, // Massimo Tavazzi
    0x9b35a4, // Riccardo Tesei
    0x9b3590, // Stefano Tiraferri
    0x9b357c, // Giuseppe Todisco
    0x9b356c, // Claudio Tomaino
    0x9b355c, // Daniele Tosatto
    0x9b3548, // Alessandro Vario
    0x9b3538, // Davide Vecchini
    0x9b3524, // Leonardo Venturelli
    0x9b3508, // Alessandro '8,666' Vetta
    0x9b34f8, // Andrea Vezzoso
    0x9b34e8, // Vanni Volponi
    0x9b34d4, // Enrico Zaccherelli
    0x9b34c4, // Andrew Dolan
    0x9b34b8, // David Owens
    0x9b34a8, // Eamon Donohoe
    0x9b3494, // Conor Fitzgerald
    0x9b3488, // Fran Hoey
    0x9b347c, // Ian Tilson
    0x9b345c, // Johnny Ward (eircomleague.net)
    0x9b344c, // Donal Cahalan
    0x9b343c, // Makoto Karibe
    0x9b3428, // Kenji Fukushima
    0x9b3414, // Takamichi Imazu
    0x9b33fc, // Haruyoshi Kitaguchi
    0x9b33e4, // Masatoshi Shiraishi
    0x9b33d4, // Andrés Aguayo
    0x9b33c0, // Valeria Brabata
    0x9b33b0, // Eddie McMullan
    0x9b33a4, // Mark Hill
    0x9b3398, // Greg Boyd
    0x9b338c, // Tim Duff
    0x9b337c, // David Kennedy
    0x9b3368, // Johnny McKinstry
    0x9b3358, // David Finlay
    0x9b3348, // Stuart Gilmore
    0x9b333c, // Jim Doran
    0x9b332c, // Bertie Fulton
    0x9b331c, // Steven Vennard
    0x9b3308, // Brian McWilliams
    0x9b32fc, // Per Wie
    0x9b32e8, // Cato Engebakken
    0x9b32d8, // Anders Mittet
    0x9b32c8, // Hein Henriksen
    0x9b32b8, // Christian Rein
    0x9b32a8, // Tord Martinsen
    0x9b3298, // Sigurd Eggen
    0x9b3284, // Hans Martin Nakkim
    0x9b3274, // Jørgen Nyhus
    0x9b325c, // Torkel Haugan Hansen
    0x9b324c, // Terje Larsen
    0x9b3234, // Chris Robin Johansen
    0x9b3228, // Jan Moe
    0x9b3218, // Henning Tveit
    0x9b3208, // Jørgen Reitan
    0x9b31f4, // Kristian Råsberg
    0x9b31d8, // Lars Christian Bjørknes
    0x9b31c8, // Vegard Enlid
    0x9b31b8, // Tor Mogstad
    0x9b31a0, // Pål Richard Lilleseth
    0x9b3190, // Trond Alsvik
    0x9b317c, // Bjørnar Thomassen
    0x9b3168, // Alexander Solum
    0x9b3158, // John Slinning
    0x9b314c, // Knut Lima
    0x9b3134, // Steinar Hanche Aker
    0x9b3124, // Espen Johansen
    0x9b3114, // Sandro Kvernmo
    0x9b3100, // Kim Kalden López
    0x9b30f0, // Sandro Kvernmo
    0x9b30e0, // Håvard Valved
    0x9b30d8, // LooZ^
    0x9b30d0, // KDAN
    0x9b30b8, // Krzysztof Bartoszek
    0x9b30a8, // Sir Cycorus
    0x9b3098, // Karol Cygan
    0x9b3088, // Adrian Dampc
    0x9b3074, // Albert Demidowski
    0x9b3060, // Michal Florczyk
    0x9b3048, // Marcin Harasimowicz
    0x9b3038, // Pawel Hochstim
    0x9b3024, // Damian Jamrozik
    0x9b3014, // Dag Janowski
    0x9b3000, // Przemyslaw Kania
    0x9b2ff0, // Wojtek Kuczera
    0x9b2fdc, // Kamil Kurzynoga
    0x9b2fcc, // Tomasz Marek
    0x9b2fb4, // Tadeusz Placzkiewicz
    0x9b2fa4, // Kamil Przegon
    0x9b2f94, // Rafal Rezler
    0x9b2f84, // Bartosz Slaby
    0x9b2f74, // Bartosz Syrek
    0x9b2f60, // Lucas Szaniawski
    0x9b2f4c, // Marek Tuszewicki
    0x9b2f3c, // Piotr Wawulski
    0x9b2f2c, // Marcin Wesolek
    0x9b2f18, // Marcin Wisniewski
    0x9b2f04, // Arkadiusz Wojna
    0x9b2ef4, // Eddy Zielinski
    0x9b2ee0, // Zelislaw Zyzynski
    0x9b2ed0, // Konrad Gawron
    0x9b2ec4, // João Costa
    0x9b2eb4, // André Narciso
    0x9b2ea4, // António Lopes
    0x9b2e94, // António Pedro
    0x9b2e80, // Bruno Magalhães
    0x9b2e70, // Carlos Carloto
    0x9b2e60, // Carlos Couto
    0x9b2e50, // Carlos Pereira
    0x9b2e40, // Carlos Veiga
    0x9b2e30, // Davide Ribeiro
    0x9b2e20, // Duarte Barbosa
    0x9b2e14, // Fábio Poço
    0x9b2e00, // Fernando Veloso
    0x9b2dec, // Gabriel Satxibala
    0x9b2ddc, // Gonçalo Durão
    0x9b2dcc, // João Campos
    0x9b2dbc, // João Inverno
    0x9b2dac, // João Marques
    0x9b2d9c, // Jorge Almeida
    0x9b2d8c, // Luis Filipe
    0x9b2d7c, // Marco Antonio
    0x9b2d68, // Nélson Patriarca
    0x9b2d58, // Nuno Caetano
    0x9b2d48, // Nuno Machado
    0x9b2d3c, // Nuno Lobo
    0x9b2d24, // Paulo Alexandre Pinho
    0x9b2d14, // Paulo Roberto
    0x9b2d00, // Paulo Sacramento
    0x9b2ce8, // Pedro Brandão Correia
    0x9b2ccc, // Pedro Silvério Gonçalves
    0x9b2cb8, // Pedro Marcelino
    0x9b2ca0, // Pedro Miguel Ribeiro
    0x9b2c88, // Pedro Santana Cepeda
    0x9b2c78, // Pedro Sousa
    0x9b2c64, // Ricardo Gouveia
    0x9b2c50, // Ricardo Rodrigues
    0x9b2c40, // Rui Boieiro
    0x9b2c30, // Rui Couceiro
    0x9b2c20, // Rui Malheiro
    0x9b2c10, // Rui Parrula
    0x9b2c04, // Rui Silva
    0x9b2bf4, // Sandro Cruz
    0x9b2be8, // Tiago Cid
    0x9b2bd8, // Tiago Sousa
    0x9b2bc4, // Vasco Ravasqueira
    0x9b2bb0, // Vítor Aires Quintã
    0x9b2b9c, // Andrew Kravtchenko
    0x9b2b88, // Mikhail Chistyakov
    0x9b2b78, // Maxim Chumakov
    0x9b2b64, // Sergey Gerassimov
    0x9b2b54, // Ilia Kovalenko
    0x9b2b40, // Alexandr Kremez
    0x9b2b30, // Alexey Moiseev
    0x9b2b20, // Ivan Rundqvist
    0x9b2b0c, // Vitaly Sklyarov
    0x9b2af8, // Antony Tutunnik
    0x9b2ae8, // Vadim Vasiljev
    0x9b2ad8, // Steven Torney
    0x9b2acc, // Craig Feroz
    0x9b2abc, // Gavin Maxwell
    0x9b2aa8, // Iain Bartholomew
    0x9b2a98, // Paul O'Neill
    0x9b2a88, // Keith Johnston
    0x9b2a78, // Iain Caldwell
    0x9b2a6c, // Jimmy Bell
    0x9b2a60, // Alan Hunter
    0x9b2a54, // Alan Thomas
    0x9b2a44, // Gary Henderson
    0x9b2a30, // Stephen McConnachie
    0x9b2a24, // Ross Wilson
    0x9b2a18, // Allan Ross
    0x9b2a08, // Stuart Douglas
    0x9b29fc, // Scott Airth
    0x9b29ec, // Jamie Pendrigh
    0x9b29dc, // Robin Karfoot
    0x9b29cc, // Craig Buchanan
    0x9b29bc, // Robert Chappell
    0x9b29ac, // Andrew Harkes
    0x9b29a0, // Allan Hosey
    0x9b2990, // David McCann
    0x9b2984, // Paul Ferrie
    0x9b2974, // Jamie MacKay
    0x9b2968, // David Burns
    0x9b2958, // Alan Stewart
    0x9b294c, // Bruce Hale
    0x9b2940, // Jamie Wire
    0x9b292c, // Scottie MacLennan
    0x9b291c, // Osiris Navarro
    0x9b290c, // J.A. Vidania
    0x9b28ec, // Alfredo del Castillo Morgado
    0x9b28dc, // Domingo García
    0x9b28cc, // Eduardo Corral
    0x9b28b8, // David Franco Vilar
    0x9b28a8, // Pablo Blázquez
    0x9b2890, // Raúl Moreno Domínguez
    0x9b287c, // Ross van der Linde
    0x9b2868, // Andreas Koshiaris
    0x9b285c, // Sean Scott
    0x9b2848, // Johan Pettersson
    0x9b2838, // Jonas Dermark
    0x9b2828, // Per Antonsson
    0x9b2814, // Fredrik Gustafsson
    0x9b2804, // Joel Skjutar
    0x9b27f4, // Karl Alfredsson
    0x9b27e0, // Fredrik Ljunggren
    0x9b27d4, // Johan Frost
    0x9b27c0, // Andreas Ljungqvist
    0x9b27ac, // Markus Niklasson
    0x9b279c, // Staffan Runemo
    0x9b2788, // Tobias Andersson
    0x9b2778, // Luigi Riccio
    0x9b2768, // Boris Jelenic
    0x9b2758, // Branko Jelenic
    0x9b2744, // Moreno Invernizzi
    0x9b2734, // Mehmet Erikler
    0x9b2720, // Serdar Yesilyurt
    0x9b270c, // Selçuk Yesilyurt
    0x9b26fc, // Murat Odabasi
    0x9b26ec, // Orhun Çetin
    0x9b26d8, // Inci Tastabanoglu
    0x9b26c4, // Burçay Tiftikçi
    0x9b26b4, // Emre Özcanv
    0x9b26a4, // Hemi Behmoaras
    0x9b2694, // Simge Ölçer
    0x9b2684, // Okan Yurdakul
    0x9b2674, // Tolga Kaprol
    0x9b265c, // Mehmet Hakan Gökdogan
    0x9b264c, // Bülent Çelik
    0x9b2640, // Dogan Evin
    0x9b262c, // Halim Gürvardar
    0x9b261c, // Tahsin Peker
    0x9b2608, // Irfan Saraloglu
    0x9b25fc, // Faik Demir
    0x9b25f0, // Efe Avsar
    0x9b25e4, // Murat Özder
    0x9b25d8, // Onur Eyibas
    0x9b25c8, // Mustafa Budak
    0x9b25bc, // Emir Tonguç
    0x9b25ac, // Baris Konuklar
    0x9b25a0, // Chris Jones
    0x9b2594, // Tony Sharp
    0x9b2584, // Andy Glester
    0x9b2570, // Graham Armsworth
    0x9b2560, // Glenn Watkin
    0x9b2554, // Daniel Fox
    0x9b2544, // Russ Kelsall
    0x9b2534, // Kieran Robson
    0x9b2520, // Martin Pritchard
    0x9b2514, // Ian Evans
    0x9b2504, // Stuart Barnes
    0x9b24f8, // Dave Austin
    0x9b24e8, // John Holding
    0x9b24d8, // Jonathan Maltby
    0x9b24c8, // Alan Biggins
    0x9b24b8, // Robert Trand
    0x9b24ac, // Malc Buss
    0x9b249c, // Rob Fernandes
    0x9b2490, // Antony Addy
    0x9b2480, // James Parker
    0x9b2470, // Darrien Bold
    0x9b245c, // Andrea Thrussell
    0x9b244c, // Wayne Spackman
    0x9b243c, // Graeme Dixon
    0x9b2430, // Robert Rea
    0x9b2420, // Mark Halliwell
    0x9b2410, // Steve Wilkes
    0x9b2404, // Mark Denman
    0x9b23f4, // Daniel Whymark
    0x9b23e4, // Alan Roberts
    0x9b23d4, // Simon Davies
    0x9b23c4, // Steve Baxter
    0x9b23b4, // Mark Buckingham
    0x9b23a4, // Steve Bishop
    0x9b2398, // Jim Wheeler
    0x9b238c, // Steve Rose
    0x9b2380, // Peter Zeeuw
    0x9b236c, // Doug Kirkpatrick
    0x9b235c, // Stuart Townsend
    0x9b2350, // Jeff Trice
    0x9b2344, // Mark Ollard
    0x9b2334, // Colin Hamilton
    0x9b2328, // Peter Gowan
    0x9b231c, // John Lawson
    0x9b2310, // Mark Clough
    0x9b2304, // Steve Hales
    0x9b22f4, // Ian Fitzgerald
    0x9b22e4, // Richard Walker
    0x9b22d0, // David Garthwaite
    0x9b22c4, // Rich Self
    0x9b22b8, // Nick Bell
    0x9b22ac, // Ed Prosser
    0x9b229c, // Andy Medcalf
    0x9b2290, // Jim Bristow
    0x9b2280, // James Gardner
    0x9b2274, // Adam Lees
    0x9b2260, // Steven Petryszyn
    0x9b2250, // Brian Spittles
    0x9b2244, // Matt Peers
    0x9b2238, // Mark Waters
    0x9b222c, // Simon Mills
    0x9b221c, // Paul Edwards
    0x9b220c, // Jonathan Miller
    0x9b2200, // Tommy Rance
    0x9b21f4, // Pete Gresty
    0x9b21e8, // Sam Vardy
    0x9b21dc, // Dai Nelmes
    0x9b21d0, // Bob Fischer
    0x9b21c4, // Chris Rice
    0x9b21b4, // Paul Johnson
    0x9b21a4, // David Moscrop
    0x9b2198, // Kim Griggs
    0x9b218c, // Ian Scholes
    0x9b217c, // Ian Marriott
    0x9b216c, // Gary Spencer
    0x9b215c, // Alex Metcalfe
    0x9b214c, // Damon Farrant
    0x9b2140, // Ian Hesketh
    0x9b2130, // Matt Walklett
    0x9b2120, // Jonathan Oakes
    0x9b210c, // Andrew Stillwell
    0x9b2100, // James Black
    0x9b20f4, // Steve James
    0x9b20e8, // Ray Curry
    0x9b20d8, // Elinor Cooper
    0x9b20c8, // Andrew Hawkins
    0x9b20bc, // Dave Albon
    0x9b20b0, // Nick Clarke
    0x9b20a0, // Andy Britcliffe
    0x9b2090, // John Simpkins
    0x9b2084, // Rob O'Neill
    0x9b2074, // Ian Whitfield
    0x9b2064, // Paul Manzotti
    0x9b2054, // Paul Boreham
    0x9b2048, // Tom Lewis
    0x9b2038, // Ben Laughlin
    0x9b2028, // Stuart Govier
    0x9b201c, // Andy Barlow
    0x9b200c, // Mark Bradshaw
    0x9b2000, // Dave Birks
    0x9b1ff0, // Iain Watkins
    0x9b1fe4, // Ian Drayton
    0x9b1fd4, // Richard Hughes
    0x9b1fc8, // Noel Pearce
    0x9b1fbc, // Ian John
    0x9b1fac, // Andy Fellows
    0x9b1f9c, // Graham Jones
    0x9b1f90, // Mark Currie
    0x9b1f80, // Steve Barlow
    0x9b1f74, // Mike Carey
    0x9b1f64, // Chris Goodwin
    0x9b1f50, // Malcolm Williams
    0x9b1f40, // Daniel Butcher
    0x9b1f30, // Mark Williams
    0x9b1f20, // Paul Knapper
    0x9b1f10, // Richard Gibson
    0x9b1f00, // Will Thornton
    0x9b1ef4, // Sam Watt
    0x9b1ee4, // Andy Wilkinson
    0x9b1ed8, // Tom Povey
    0x9b1ec8, // Colin Cavanah
    0x9b1ebc, // Craig Buck
    0x9b1ea8, // Graham Bambrough
    0x9b1e94, // Adam Etherington
    0x9b1e88, // Danny Gee
    0x9b1e7c, // Mel Thomas
    0x9b1e70, // Mike Smith
    0x9b1e5c, // Christopher Evans
    0x9b1e44, // Rhidian 'Egg' Jenkins
    0x9b1e34, // Milos Tasic
    0x9b1e20, // Sinisa Novkovic
    0x9b1e08, // Aleksandar Miladinovic
    0x9b1dfc, // Ivan Simic
    0x9b1dec, // Driss Merabtene
    0x9b1dd8, // Giusepe Imbugria
    0x9b1dc4, // Tarek El Shaarani
    0x9b1db8, // Reda Maher
    0x9b1da0, // Amandeep Singh Mahal
    0x9b1d90, // Jacques Foetz
    0x9b1d80, // Mart Jyrisoo
    0x9b1d70, // Marek Ivanson
    0x9b1d60, // Peter Kucharik
    0x9b1d50, // Pedro Campos
    0x9b1d3c, // Davorin Stevanovic
    0x9b1d2c, // Faris Al Bugami
    0x9b1d1c, // Nedko Kyuchukov
    0x9b1d08, // Freyr Kolbeinsson
    0x9b1cf8, // Tony Gamatin
    0x9b1ce8, // Ruslan Ulbajev
    0x9b1cd8, // Béres  Endre
    0x9b1cc8, // Laith Majali
    0x9b1cb4, // Dmitriy Belcencov
    0x9b1ca8, // Ali Yasan
    0x9b1c98, // Andrey Muravyov
    0x9b1c8c, // Manav Puree
    0x9b1c80, // Lica Sever
    0x9b1c74, // Mark Muscat
    0x9b1c60, // Andreas Michaelides
];

/// Rule: a two-column name list that ended on the right-hand column is closed: the half row counts.
///
/// When `tel32_credits::COLUMN` is not 0 (the last name went into the left column on the shown page), the item
/// number goes up by 1 and the row counter by 1; otherwise nothing.
unsafe fn credits_end_name_columns() {
    if read_u32(tel32_credits::COLUMN) != 0 {
        covered!("tel33_1 the half row closed");
        tel32_next_item();
        tel32_bump_row(1);
    }
}

/// Rule: the Additional Research section: the "Additional Research" heading (translated) and the 698 researchers'
/// names in two columns, closing the last half row.
///
/// The text "Additional Research" goes through `langlib_sprintf` into the screens' text buffer (`JOB_TEXT`); then
/// the heading (`tel32_heading`: highlighted when the item number is screen parameter 0 on the shown page), the names
/// (`tel32_name_columns`: on the shown page two to a row, left 0x82..0x1b8 and right 0x1c2..0x2f8, in the 0xae3184
/// colour; on the other page only the rows are counted, one per name) and `credits_end_name_columns`. Unlike the
/// other sections there is no empty row first. The names are the exe's own texts, passed by address as the
/// original does; the original is one straight 132 KB run of the same row code per name.
/// Original: `credits_add_additional_research_rows` (0x4eda90, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_add_additional_research_rows() {
    covered!("tel33_1 additional research rows");
    comp_lang_format()(JOB_TEXT, 0x9b4b1cusize); // "Additional Research"
    tel32_heading();
    tel32_name_columns(&ADDITIONAL_RESEARCH_NAMES);
    credits_end_name_columns();
}

/// Comparer for `credits_add_additional_research_rows`: side by side over the display list
/// (`compare_match_screen_build`) plus the credits build state (`tel32_credits`).
pub(crate) unsafe extern "C" fn compare_credits_add_additional_research_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    compare_match_screen_build("credits Additional Research rows", &[(tel32_credits::ROW, tel32_credits::STATE_SIZE)], || o(),
        || credits_add_additional_research_rows());
}
