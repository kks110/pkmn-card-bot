use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub fn set_names() -> Vec<&'static str> {
    sets().into_keys().collect()
}

pub fn sets() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("Base", "base1"),
        ("Jungle", "base2"),
        ("Wizards Black Star Promos", "basep"),
        ("Fossil", "base3"),
        ("Base Set 2", "base4"),
        ("Team Rocket", "base5"),
        ("Gym Heroes", "gym1"),
        ("Gym Challenge", "gym2"),
        ("Neo Genesis", "neo1"),
        ("Neo Discovery", "neo2"),
        ("Southern Islands", "si1"),
        ("Neo Revelation", "neo3"),
        ("Neo Destiny", "neo4"),
        ("Legendary Collection", "base6"),
        ("Expedition Base Set", "ecard1"),
        ("Aquapolis", "ecard2"),
        ("Skyridge", "ecard3"),
        ("Ruby & Sapphire", "ex1"),
        ("Sandstorm", "ex2"),
        ("Dragon", "ex3"),
        ("Nintendo Black Star Promos", "np"),
        ("Team Magma vs Team Aqua", "ex4"),
        ("Hidden Legends", "ex5"),
        ("FireRed & LeafGreen", "ex6"),
        ("POP Series 1", "pop1"),
        ("Team Rocket Returns", "ex7"),
        ("Deoxys", "ex8"),
        ("Emerald", "ex9"),
        ("Unseen Forces", "ex10"),
        ("POP Series 2", "pop2"),
        ("Delta Species", "ex11"),
        ("Legend Maker", "ex12"),
        ("POP Series 3", "pop3"),
        ("Holon Phantoms", "ex13"),
        ("Crystal Guardians", "ex14"),
        ("POP Series 4", "pop4"),
        ("Dragon Frontiers", "ex15"),
        ("POP Series 5", "pop5"),
        ("Power Keepers", "ex16"),
        ("Diamond & Pearl", "dp1"),
        ("DP Black Star Promos", "dpp"),
        ("Mysterious Treasures", "dp2"),
        ("POP Series 6", "pop6"),
        ("Secret Wonders", "dp3"),
        ("Great Encounters", "dp4"),
        ("POP Series 7", "pop7"),
        ("Majestic Dawn", "dp5"),
        ("Legends Awakened", "dp6"),
        ("POP Series 8", "pop8"),
        ("Stormfront", "dp7"),
        ("Platinum", "pl1"),
        ("POP Series 9", "pop9"),
        ("Rising Rivals", "pl2"),
        ("Supreme Victors", "pl3"),
        ("Arceus", "pl4"),
        ("Pokémon Rumble", "ru1"),
        ("HeartGold & SoulSilver", "hgss1"),
        ("HGSS Black Star Promos", "hsp"),
        ("HS—Unleashed", "hgss2"),
        ("HS—Undaunted", "hgss3"),
        ("HS—Triumphant", "hgss4"),
        ("Call of Legends", "col1"),
        ("BW Black Star Promos", "bwp"),
        ("Black & White", "bw1"),
        ("McDonald's Collection 2011", "mcd11"),
        ("Emerging Powers", "bw2"),
        ("Noble Victories", "bw3"),
        ("Next Destinies", "bw4"),
        ("Dark Explorers", "bw5"),
        ("McDonald's Collection 2012", "mcd12"),
        ("Dragons Exalted", "bw6"),
        ("Dragon Vault", "dv1"),
        ("Boundaries Crossed", "bw7"),
        ("Plasma Storm", "bw8"),
        ("Plasma Freeze", "bw9"),
        ("Plasma Blast", "bw10"),
        ("XY Black Star Promos", "xyp"),
        ("Legendary Treasures", "bw11"),
        ("Kalos Starter Set", "xy0"),
        ("XY", "xy1"),
        ("Flashfire", "xy2"),
        ("Furious Fists", "xy3"),
        ("Phantom Forces", "xy4"),
        ("Primal Clash", "xy5"),
        ("Double Crisis", "dc1"),
        ("Roaring Skies", "xy6"),
        ("Ancient Origins", "xy7"),
        ("BREAKthrough", "xy8"),
        ("BREAKpoint", "xy9"),
        ("Generations", "g1"),
        ("Fates Collide", "xy10"),
        ("Steam Siege", "xy11"),
        ("McDonald's Collection 2016", "mcd16"),
        ("Evolutions", "xy12"),
        ("Sun & Moon", "sm1"),
        ("SM Black Star Promos", "smp"),
        ("Guardians Rising", "sm2"),
        ("Burning Shadows", "sm3"),
        ("Shining Legends", "sm35"),
        ("Crimson Invasion", "sm4"),
        ("Ultra Prism", "sm5"),
        ("Forbidden Light", "sm6"),
        ("Celestial Storm", "sm7"),
        ("Dragon Majesty", "sm75"),
        ("Lost Thunder", "sm8"),
        ("Team Up", "sm9"),
        ("Detective Pikachu", "det1"),
        ("Unbroken Bonds", "sm10"),
        ("Unified Minds", "sm11"),
        ("Hidden Fates", "sm115"),
        ("SM Shiny Vault", "sma"),
        ("McDonald's Collection 2019", "mcd19"),
        ("Cosmic Eclipse", "sm12"),
        ("SWSH Black Star Promos", "swshp"),
        ("Sword & Shield", "swsh1"),
        ("Rebel Clash", "swsh2"),
        ("Darkness Ablaze", "swsh3"),
        ("Champion's Path", "swsh35"),
        ("Vivid Voltage", "swsh4"),
        ("Shining Fates", "swsh45"),
        ("SWSH Shiny Vault", "swsh45sv"),
        ("Battle Styles", "swsh5"),
        ("Chilling Reign", "swsh6"),
        ("Evolving Skies", "swsh7"),
        ("Celebrations", "cel25"),
        ("Celebrations: Classic Collection", "cel25c"),
        ("McDonald's Collection 2014", "mcd14"),
        ("McDonald's Collection 2015", "mcd15"),
        ("McDonald's Collection 2018", "mcd18"),
        ("McDonald's Collection 2017", "mcd17"),
        ("McDonald's Collection 2021", "mcd21"),
        ("Best of Game", "bp"),
        ("Fusion Strike", "swsh8"),
        ("Pokémon Futsal Collection", "fut20"),
        ("EX Trainer Kit Latias", "tk1a"),
        ("EX Trainer Kit Latios", "tk1b"),
        ("EX Trainer Kit 2 Plusle", "tk2a"),
        ("EX Trainer Kit 2 Minun", "tk2b"),
        ("Brilliant Stars", "swsh9"),
        ("Brilliant Stars Trainer Gallery", "swsh9tg"),
        ("Astral Radiance", "swsh10"),
        ("Astral Radiance Trainer Gallery", "swsh10tg"),
        ("Pokémon GO", "pgo"),
        ("Lost Origin", "swsh11"),
        ("Lost Origin Trainer Gallery", "swsh11tg"),
        ("Silver Tempest", "swsh12"),
        ("Silver Tempest Trainer Gallery", "swsh12tg"),
        ("McDonald's Collection 2022", "mcd22"),
    ])
}

pub fn pokemon() -> Vec<&'static str> {
    vec![
        "Bulbasaur",
        "Ivysaur",
        "Venusaur",
        "Charmander",
        "Charmeleon",
        "Charizard",
        "Squirtle",
        "Wartortle",
        "Blastoise",
        "Caterpie",
        "Metapod",
        "Butterfree",
        "Weedle",
        "Kakuna",
        "Beedrill",
        "Pidgey",
        "Pidgeotto",
        "Pidgeot",
        "Rattata",
        "Raticate",
        "Spearow",
        "Fearow",
        "Ekans",
        "Arbok",
        "Pikachu",
        "Raichu",
        "Sandshrew",
        "Sandslash",
        "Nidoran ♀",
        "Nidorina",
        "Nidoqueen",
        "Nidoran ♂",
        "Nidorino",
        "Nidoking",
        "Clefairy",
        "Clefable",
        "Vulpix",
        "Ninetales",
        "Jigglypuff",
        "Wigglytuff",
        "Zubat",
        "Golbat",
        "Oddish",
        "Gloom",
        "Vileplume",
        "Paras",
        "Parasect",
        "Venonat",
        "Venomoth",
        "Diglett",
        "Dugtrio",
        "Meowth",
        "Persian",
        "Psyduck",
        "Golduck",
        "Mankey",
        "Primeape",
        "Growlithe",
        "Arcanine",
        "Poliwag",
        "Poliwhirl",
        "Poliwrath",
        "Abra",
        "Kadabra",
        "Alakazam",
        "Machop",
        "Machoke",
        "Machamp",
        "Bellsprout",
        "Weepinbell",
        "Victreebel",
        "Tentacool",
        "Tentacruel",
        "Geodude",
        "Graveler",
        "Golem",
        "Ponyta",
        "Rapidash",
        "Slowpoke",
        "Slowbro",
        "Magnemite",
        "Magneton",
        "Farfetchd",
        "Doduo",
        "Dodrio",
        "Seel",
        "Dewgong",
        "Grimer",
        "Muk",
        "Shellder",
        "Cloyster",
        "Gastly",
        "Haunter",
        "Gengar",
        "Onix",
        "Drowzee",
        "Hypno",
        "Krabby",
        "Kingler",
        "Voltorb",
        "Electrode",
        "Exeggcute",
        "Exeggutor",
        "Cubone",
        "Marowak",
        "Hitmonlee",
        "Hitmonchan",
        "Lickitung",
        "Koffing",
        "Weezing",
        "Rhyhorn",
        "Rhydon",
        "Chansey",
        "Tangela",
        "Kangaskhan",
        "Horsea",
        "Seadra",
        "Goldeen",
        "Seaking",
        "Staryu",
        "Starmie",
        "Mr. mime",
        "Scyther",
        "Jynx",
        "Electabuzz",
        "Magmar",
        "Pinsir",
        "Tauros",
        "Magikarp",
        "Gyarados",
        "Lapras",
        "Ditto",
        "Eevee",
        "Vaporeon",
        "Jolteon",
        "Flareon",
        "Porygon",
        "Omanyte",
        "Omastar",
        "Kabuto",
        "Kabutops",
        "Aerodactyl",
        "Snorlax",
        "Articuno",
        "Zapdos",
        "Moltres",
        "Dratini",
        "Dragonair",
        "Dragonite",
        "Mewtwo",
        "Mew",
        "Chikorita",
        "Bayleef",
        "Meganium",
        "Cyndaquil",
        "Quilava",
        "Typhlosion",
        "Totodile",
        "Croconaw",
        "Feraligatr",
        "Sentret",
        "Furret",
        "Hoothoot",
        "Noctowl",
        "Ledyba",
        "Ledian",
        "Spinarak",
        "Ariados",
        "Crobat",
        "Chinchou",
        "Lanturn",
        "Pichu",
        "Cleffa",
        "Igglybuff",
        "Togepi",
        "Togetic",
        "Natu",
        "Xatu",
        "Mareep",
        "Flaaffy",
        "Ampharos",
        "Bellossom",
        "Marill",
        "Azumarill",
        "Sudowoodo",
        "Politoed",
        "Hoppip",
        "Skiploom",
        "Jumpluff",
        "Aipom",
        "Sunkern",
        "Sunflora",
        "Yanma",
        "Wooper",
        "Quagsire",
        "Espeon",
        "Umbreon",
        "Murkrow",
        "Slowking",
        "Misdreavus",
        "Unown",
        "Wobbuffet",
        "Girafarig",
        "Pineco",
        "Forretress",
        "Dunsparce",
        "Gligar",
        "Steelix",
        "Snubbull",
        "Granbull",
        "Qwilfish",
        "Scizor",
        "Shuckle",
        "Heracross",
        "Sneasel",
        "Teddiursa",
        "Ursaring",
        "Slugma",
        "Magcargo",
        "Swinub",
        "Piloswine",
        "Corsola",
        "Remoraid",
        "Octillery",
        "Delibird",
        "Mantine",
        "Skarmory",
        "Houndour",
        "Houndoom",
        "Kingdra",
        "Phanpy",
        "Donphan",
        "Porygon2",
        "Stantler",
        "Smeargle",
        "Tyrogue",
        "Hitmontop",
        "Smoochum",
        "Elekid",
        "Magby",
        "Miltank",
        "Blissey",
        "Raikou",
        "Entei",
        "Suicune",
        "Larvitar",
        "Pupitar",
        "Tyranitar",
        "Lugia",
        "Ho-oh",
        "Celebi",
        "Treecko",
        "Grovyle",
        "Sceptile",
        "Torchic",
        "Combusken",
        "Blaziken",
        "Mudkip",
        "Marshtomp",
        "Swampert",
        "Poochyena",
        "Mightyena",
        "Zigzagoon",
        "Linoone",
        "Wurmple",
        "Silcoon",
        "Beautifly",
        "Cascoon",
        "Dustox",
        "Lotad",
        "Lombre",
        "Ludicolo",
        "Seedot",
        "Nuzleaf",
        "Shiftry",
        "Taillow",
        "Swellow",
        "Wingull",
        "Pelipper",
        "Ralts",
        "Kirlia",
        "Gardevoir",
        "Surskit",
        "Masquerain",
        "Shroomish",
        "Breloom",
        "Slakoth",
        "Vigoroth",
        "Slaking",
        "Nincada",
        "Ninjask",
        "Shedinja",
        "Whismur",
        "Loudred",
        "Exploud",
        "Makuhita",
        "Hariyama",
        "Azurill",
        "Nosepass",
        "Skitty",
        "Delcatty",
        "Sableye",
        "Mawile",
        "Aron",
        "Lairon",
        "Aggron",
        "Meditite",
        "Medicham",
        "Electrike",
        "Manectric",
        "Plusle",
        "Minun",
        "Volbeat",
        "Illumise",
        "Roselia",
        "Gulpin",
        "Swalot",
        "Carvanha",
        "Sharpedo",
        "Wailmer",
        "Wailord",
        "Numel",
        "Camerupt",
        "Torkoal",
        "Spoink",
        "Grumpig",
        "Spinda",
        "Trapinch",
        "Vibrava",
        "Flygon",
        "Cacnea",
        "Cacturne",
        "Swablu",
        "Altaria",
        "Zangoose",
        "Seviper",
        "Lunatone",
        "Solrock",
        "Barboach",
        "Whiscash",
        "Corphish",
        "Crawdaunt",
        "Baltoy",
        "Claydol",
        "Lileep",
        "Cradily",
        "Anorith",
        "Armaldo",
        "Feebas",
        "Milotic",
        "Castform",
        "Kecleon",
        "Shuppet",
        "Banette",
        "Duskull",
        "Dusclops",
        "Tropius",
        "Chimecho",
        "Absol",
        "Wynaut",
        "Snorunt",
        "Glalie",
        "Spheal",
        "Sealeo",
        "Walrein",
        "Clamperl",
        "Huntail",
        "Gorebyss",
        "Relicanth",
        "Luvdisc",
        "Bagon",
        "Shelgon",
        "Salamence",
        "Beldum",
        "Metang",
        "Metagross",
        "Regirock",
        "Regice",
        "Registeel",
        "Latias",
        "Latios",
        "Kyogre",
        "Groudon",
        "Rayquaza",
        "Jirachi",
        "Deoxys",
        "Turtwig",
        "Grotle",
        "Torterra",
        "Chimchar",
        "Monferno",
        "Infernape",
        "Piplup",
        "Prinplup",
        "Empoleon",
        "Starly",
        "Staravia",
        "Staraptor",
        "Bidoof",
        "Bibarel",
        "Kricketot",
        "Kricketune",
        "Shinx",
        "Luxio",
        "Luxray",
        "Budew",
        "Roserade",
        "Cranidos",
        "Rampardos",
        "Shieldon",
        "Bastiodon",
        "Burmy",
        "Wormadam",
        "Mothim",
        "Combee",
        "Vespiquen",
        "Pachirisu",
        "Buizel",
        "Floatzel",
        "Cherubi",
        "Cherrim",
        "Shellos",
        "Gastrodon",
        "Ambipom",
        "Drifloon",
        "Drifblim",
        "Buneary",
        "Lopunny",
        "Mismagius",
        "Honchkrow",
        "Glameow",
        "Purugly",
        "Chingling",
        "Stunky",
        "Skuntank",
        "Bronzor",
        "Bronzong",
        "Bonsly",
        "Mime jr.",
        "Happiny",
        "Chatot",
        "Spiritomb",
        "Gible",
        "Gabite",
        "Garchomp",
        "Munchlax",
        "Riolu",
        "Lucario",
        "Hippopotas",
        "Hippowdon",
        "Skorupi",
        "Drapion",
        "Croagunk",
        "Toxicroak",
        "Carnivine",
        "Finneon",
        "Lumineon",
        "Mantyke",
        "Snover",
        "Abomasnow",
        "Weavile",
        "Magnezone",
        "Lickilicky",
        "Rhyperior",
        "Tangrowth",
        "Electivire",
        "Magmortar",
        "Togekiss",
        "Yanmega",
        "Leafeon",
        "Glaceon",
        "Gliscor",
        "Mamoswine",
        "Porygon-z",
        "Gallade",
        "Probopass",
        "Dusknoir",
        "Froslass",
        "Rotom",
        "Uxie",
        "Mesprit",
        "Azelf",
        "Dialga",
        "Palkia",
        "Heatran",
        "Regigigas",
        "Giratina",
        "Cresselia",
        "Phione",
        "Manaphy",
        "Darkrai",
        "Shaymin",
        "Arceus",
        "Victini",
        "Snivy",
        "Servine",
        "Serperior",
        "Tepig",
        "Pignite",
        "Emboar",
        "Oshawott",
        "Dewott",
        "Samurott",
        "Patrat",
        "Watchog",
        "Lillipup",
        "Herdier",
        "Stoutland",
        "Purrloin",
        "Liepard",
        "Pansage",
        "Simisage",
        "Pansear",
        "Simisear",
        "Panpour",
        "Simipour",
        "Munna",
        "Musharna",
        "Pidove",
        "Tranquill",
        "Unfezant",
        "Blitzle",
        "Zebstrika",
        "Roggenrola",
        "Boldore",
        "Gigalith",
        "Woobat",
        "Swoobat",
        "Drilbur",
        "Excadrill",
        "Audino",
        "Timburr",
        "Gurdurr",
        "Conkeldurr",
        "Tympole",
        "Palpitoad",
        "Seismitoad",
        "Throh",
        "Sawk",
        "Sewaddle",
        "Swadloon",
        "Leavanny",
        "Venipede",
        "Whirlipede",
        "Scolipede",
        "Cottonee",
        "Whimsicott",
        "Petilil",
        "Lilligant",
        "Basculin",
        "Sandile",
        "Krokorok",
        "Krookodile",
        "Darumaka",
        "Darmanitan",
        "Maractus",
        "Dwebble",
        "Crustle",
        "Scraggy",
        "Scrafty",
        "Sigilyph",
        "Yamask",
        "Cofagrigus",
        "Tirtouga",
        "Carracosta",
        "Archen",
        "Archeops",
        "Trubbish",
        "Garbodor",
        "Zorua",
        "Zoroark",
        "Minccino",
        "Cinccino",
        "Gothita",
        "Gothorita",
        "Gothitelle",
        "Solosis",
        "Duosion",
        "Reuniclus",
        "Ducklett",
        "Swanna",
        "Vanillite",
        "Vanillish",
        "Vanilluxe",
        "Deerling",
        "Sawsbuck",
        "Emolga",
        "Karrablast",
        "Escavalier",
        "Foongus",
        "Amoonguss",
        "Frillish",
        "Jellicent",
        "Alomomola",
        "Joltik",
        "Galvantula",
        "Ferroseed",
        "Ferrothorn",
        "Klink",
        "Klang",
        "Klinklang",
        "Tynamo",
        "Eelektrik",
        "Eelektross",
        "Elgyem",
        "Beheeyem",
        "Litwick",
        "Lampent",
        "Chandelure",
        "Axew",
        "Fraxure",
        "Haxorus",
        "Cubchoo",
        "Beartic",
        "Cryogonal",
        "Shelmet",
        "Accelgor",
        "Stunfisk",
        "Mienfoo",
        "Mienshao",
        "Druddigon",
        "Golett",
        "Golurk",
        "Pawniard",
        "Bisharp",
        "Bouffalant",
        "Rufflet",
        "Braviary",
        "Vullaby",
        "Mandibuzz",
        "Heatmor",
        "Durant",
        "Deino",
        "Zweilous",
        "Hydreigon",
        "Larvesta",
        "Volcarona",
        "Cobalion",
        "Terrakion",
        "Virizion",
        "Tornadus",
        "Thundurus",
        "Reshiram",
        "Zekrom",
        "Landorus",
        "Kyurem",
        "Keldeo",
        "Meloetta",
        "Genesect",
        "Chespin",
        "Quilladin",
        "Chesnaught",
        "Fennekin",
        "Braixen",
        "Delphox",
        "Froakie",
        "Frogadier",
        "Greninja",
        "Bunnelby",
        "Diggersby",
        "Fletchling",
        "Fletchinder",
        "Talonflame",
        "Scatterbug",
        "Spewpa",
        "Vivillon",
        "Litleo",
        "Pyroar",
        "Flabebe",
        "Floette",
        "Florges",
        "Skiddo",
        "Gogoat",
        "Pancham",
        "Pangoro",
        "Furfrou",
        "Espurr",
        "Meowstic",
        "Honedge",
        "Doublade",
        "Aegislash",
        "Spritzee",
        "Aromatisse",
        "Swirlix",
        "Slurpuff",
        "Inkay",
        "Malamar",
        "Binacle",
        "Barbaracle",
        "Skrelp",
        "Dragalge",
        "Clauncher",
        "Clawitzer",
        "Helioptile",
        "Heliolisk",
        "Tyrunt",
        "Tyrantrum",
        "Amaura",
        "Aurorus",
        "Sylveon",
        "Hawlucha",
        "Dedenne",
        "Carbink",
        "Goomy",
        "Sliggoo",
        "Goodra",
        "Klefki",
        "Phantump",
        "Trevenant",
        "Pumpkaboo",
        "Gourgeist",
        "Bergmite",
        "Avalugg",
        "Noibat",
        "Noivern",
        "Xerneas",
        "Yveltal",
        "Zygarde",
        "Diancie",
        "Hoopa",
        "Volcanion",
        "Rowlet",
        "Dartrix",
        "Decidueye",
        "Litten",
        "Torracat",
        "Incineroar",
        "Popplio",
        "Brionne",
        "Primarina",
        "Pikipek",
        "Trumbeak",
        "Toucannon",
        "Yungoos",
        "Gumshoos",
        "Grubbin",
        "Charjabug",
        "Vikavolt",
        "Crabrawler",
        "Crabominable",
        "Oricorio",
        "Cutiefly",
        "Ribombee",
        "Rockruff",
        "Lycanroc",
        "Wishiwashi",
        "Mareanie",
        "Toxapex",
        "Mudbray",
        "Mudsdale",
        "Dewpider",
        "Araquanid",
        "Fomantis",
        "Lurantis",
        "Morelull",
        "Shiinotic",
        "Salandit",
        "Salazzle",
        "Stufful",
        "Bewear",
        "Bounsweet",
        "Steenee",
        "Tsareena",
        "Comfey",
        "Oranguru",
        "Passimian",
        "Wimpod",
        "Golisopod",
        "Sandygast",
        "Palossand",
        "Pyukumuku",
        "Type: null",
        "Silvally",
        "Minior",
        "Komala",
        "Turtonator",
        "Togedemaru",
        "Mimikyu",
        "Bruxish",
        "Drampa",
        "Dhelmise",
        "Jangmo-o",
        "Hakamo-o",
        "Kommo-o",
        "Tapu-koko",
        "Tapu-lele",
        "Tapu-bulu",
        "Tapu-fini",
        "Cosmog",
        "Cosmoem",
        "Solgaleo",
        "Lunala",
        "Nihilego",
        "Buzzwole",
        "Pheromosa",
        "Xurkitree",
        "Celesteela",
        "Kartana",
        "Guzzlord",
        "Necrozma",
        "Magearna",
        "Marshadow",
        "Poipole",
        "Naganadel",
        "Stakataka",
        "Blacephalon",
        "Zeraora",
        "Meltan",
        "Melmetal",
        "Grookey",
        "Thwackey",
        "Rillaboom",
        "Scorbunny",
        "Raboot",
        "Cinderace",
        "Sobble",
        "Drizzile",
        "Inteleon",
        "Skwovet",
        "Greedent",
        "Rookidee",
        "Corvisquire",
        "Corviknight",
        "Blipbug",
        "Dottler",
        "Orbeetle",
        "Nickit",
        "Thievul",
        "Gossifleur",
        "Eldegoss",
        "Wooloo",
        "Dubwool",
        "Chewtle",
        "Drednaw",
        "Yamper",
        "Boltund",
        "Rolycoly",
        "Carkol",
        "Coalossal",
        "Applin",
        "Flapple",
        "Appletun",
        "Silicobra",
        "Sandaconda",
        "Cramorant",
        "Arrokuda",
        "Barraskewda",
        "Toxel",
        "Toxtricity",
        "Sizzlipede",
        "Centiskorch",
        "Clobbopus",
        "Grapploct",
        "Sinistea",
        "Polteageist",
        "Hatenna",
        "Hattrem",
        "Hatterene",
        "Impidimp",
        "Morgrem",
        "Grimmsnarl",
        "Obstagoon",
        "Perrserker",
        "Cursola",
        "Sirfetchd",
        "Mr. rime",
        "Runerigus",
        "Milcery",
        "Alcremie",
        "Falinks",
        "Pincurchin",
        "Snom",
        "Frosmoth",
        "Stonjourner",
        "Eiscue",
        "Indeedee",
        "Morpeko",
        "Cufant",
        "Copperajah",
        "Dracozolt",
        "Arctozolt",
        "Dracovish",
        "Arctovish",
        "Duraludon",
        "Dreepy",
        "Drakloak",
        "Dragapult",
        "Zacian",
        "Zamazenta",
        "Eternatus",
        "Kubfu",
        "Urshifu",
        "Zarude",
        "Regieleki",
        "Regidrago",
        "Glastrier",
        "Spectrier",
        "Calyrex",
        "Wyrdeer",
        "Kleavor",
        "Ursaluna",
        "Basculegion",
        "Sneasler",
        "Overqwil",
        "Enamorus",
    ]
}

pub fn colour_map(card_type: &str) -> i32 {
    match card_type {
        "Grass" => 0x7DB808,
        "Fire" => 0xE24242,
        "Water" => 0x5BC7E5,
        "Lightning" => 0xFAB536,
        "Fighting" => 0xB0382E,
        "Psychic" => 0xA65E9A,
        "Colorless" => 0xE5D6D0,
        "Darkness" => 0x2C2E2B,
        "Metal" => 0xCFCCC2,
        "Dragon" => 0xC6A114,
        "Fairy" => 0xE03A83,
        _ => 0xE5D6D0
    }
}
