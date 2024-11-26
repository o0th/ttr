pub enum Players {
    NOONE,
    BLACK,
    RED,
    GREEN,
    YELLOW,
    BLUE,
}

#[derive(Eq, Hash, PartialEq)]
pub enum Colors {
    GREY,
    BLACK,
    ORANGE,
    PINK,
    WHITE,
    YELLOW,
    BLUE,
    RED,
}

impl Colors {
    pub fn as_str(&self) -> &str {
        match self {
            Colors::GREY => "",
            Colors::BLACK => "",
            Colors::ORANGE => "",
            Colors::PINK => "",
            Colors::WHITE => "",
            Colors::YELLOW => "",
            Colors::BLUE => "",
            Colors::RED => "",
        }
    }
}

#[derive(Hash, Eq, PartialEq, PartialOrd)]
pub enum Cities {
    AMSTERDAM,
    ANKARA,
    ATHENS,
    BARCELONA,
    BERLIN,
    BREST,
    BRINDISI,
    BRUSSELS,
    BUCHAREST,
    BUDAPEST,
    CADIZ,
    ISTANBUL,
    GDANSK,
    DIEPPE,
    EDINBURGH,
    ERZURUM,
    ESSEN,
    FRANKFURT,
    KHARKOV,
    COPENHAGEN,
    KIEV,
    LISBON,
    LONDON,
    MADRID,
    MARSEILLE,
    MOSCOW,
    MUNICH,
    PALERMO,
    PAMPLONA,
    PARIS,
    STPETERSBURG,
    RIGA,
    ROME,
    ROSTOV,
    SARAJEVO,
    SEVASTOPOL,
    SMOLENSK,
    IZMIR,
    SOCHI,
    SOFIA,
    STOCKHOLM,
    VENICE,
    WARSAW,
    VIENNA,
    VILNIUS,
    ZAGREB,
    ZURICH,
}

impl Cities {
    pub fn as_str(&self) -> &str {
        match self {
            Cities::AMSTERDAM => "Amsterdam",
            Cities::ANKARA => "Ankara",
            Cities::ATHENS => "Athens",
            Cities::BARCELONA => "Barcelona",
            Cities::BERLIN => "Berlin",
            Cities::BREST => "Brest",
            Cities::BRINDISI => "Brindisi",
            Cities::BRUSSELS => "Brussels",
            Cities::BUCHAREST => "Bucharest",
            Cities::BUDAPEST => "Budapest",
            Cities::CADIZ => "Cadiz",
            Cities::ISTANBUL => "Istanbul",
            Cities::GDANSK => "Gdansk",
            Cities::DIEPPE => "Dieppe",
            Cities::EDINBURGH => "Edinburgh",
            Cities::ERZURUM => "Erzurum",
            Cities::ESSEN => "Essen",
            Cities::FRANKFURT => "Frankfurt",
            Cities::KHARKOV => "Kharkov",
            Cities::COPENHAGEN => "Copenhagen",
            Cities::KIEV => "Kiev",
            Cities::LISBON => "Lisbon",
            Cities::LONDON => "London",
            Cities::MADRID => "Madrid",
            Cities::MARSEILLE => "Marseille",
            Cities::MOSCOW => "Moscow",
            Cities::MUNICH => "Munich",
            Cities::PALERMO => "Palermo",
            Cities::PAMPLONA => "Pamplona",
            Cities::PARIS => "Paris",
            Cities::STPETERSBURG => "St. Petersburg",
            Cities::RIGA => "Riga",
            Cities::ROME => "Rome",
            Cities::ROSTOV => "Rostov",
            Cities::SARAJEVO => "Sarajevo",
            Cities::SEVASTOPOL => "Sevastopol",
            Cities::SMOLENSK => "Smolensk",
            Cities::IZMIR => "Izmir",
            Cities::SOCHI => "Sochi",
            Cities::SOFIA => "Sofia",
            Cities::STOCKHOLM => "Stockholm",
            Cities::VENICE => "Venice",
            Cities::WARSAW => "Warsaw",
            Cities::VIENNA => "Vienna",
            Cities::VILNIUS => "Vilnius",
            Cities::ZAGREB => "Zagreb",
            Cities::ZURICH => "Zurich",
        }
    }

    pub fn to_iter() -> impl Iterator<Item = &'static str> {
        static CITIES: [Cities; 47] = [
            Cities::AMSTERDAM,
            Cities::ANKARA,
            Cities::ATHENS,
            Cities::BARCELONA,
            Cities::BERLIN,
            Cities::BREST,
            Cities::BRINDISI,
            Cities::BRUSSELS,
            Cities::BUCHAREST,
            Cities::BUDAPEST,
            Cities::CADIZ,
            Cities::ISTANBUL,
            Cities::GDANSK,
            Cities::DIEPPE,
            Cities::EDINBURGH,
            Cities::ERZURUM,
            Cities::ESSEN,
            Cities::FRANKFURT,
            Cities::KHARKOV,
            Cities::COPENHAGEN,
            Cities::KIEV,
            Cities::LISBON,
            Cities::LONDON,
            Cities::MADRID,
            Cities::MARSEILLE,
            Cities::MOSCOW,
            Cities::MUNICH,
            Cities::PALERMO,
            Cities::PAMPLONA,
            Cities::PARIS,
            Cities::STPETERSBURG,
            Cities::RIGA,
            Cities::ROME,
            Cities::ROSTOV,
            Cities::SARAJEVO,
            Cities::SEVASTOPOL,
            Cities::SMOLENSK,
            Cities::IZMIR,
            Cities::SOCHI,
            Cities::SOFIA,
            Cities::STOCKHOLM,
            Cities::VENICE,
            Cities::WARSAW,
            Cities::VIENNA,
            Cities::VILNIUS,
            Cities::ZAGREB,
            Cities::ZURICH,
        ];

        return CITIES.iter().map(|city| city.as_str());
    }
}
