mod cities;

use cities::{Cities, Colors, Players};
use std::collections::HashMap;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let mut cities: HashMap<Cities, Players> = HashMap::new();
    cities.insert(Cities::AMSTERDAM, Players::NOONE);
    cities.insert(Cities::ANKARA, Players::NOONE);
    cities.insert(Cities::ATHENS, Players::NOONE);
    cities.insert(Cities::BARCELONA, Players::NOONE);
    cities.insert(Cities::BERLIN, Players::NOONE);
    cities.insert(Cities::BREST, Players::NOONE);
    cities.insert(Cities::BRINDISI, Players::NOONE);
    cities.insert(Cities::BRUSSELS, Players::NOONE);
    cities.insert(Cities::BUCHAREST, Players::NOONE);
    cities.insert(Cities::BUDAPEST, Players::NOONE);
    cities.insert(Cities::CADIZ, Players::NOONE);
    cities.insert(Cities::ISTANBUL, Players::NOONE);
    cities.insert(Cities::GDANSK, Players::NOONE);
    cities.insert(Cities::DIEPPE, Players::NOONE);
    cities.insert(Cities::EDINBURGH, Players::NOONE);
    cities.insert(Cities::ERZURUM, Players::NOONE);
    cities.insert(Cities::ESSEN, Players::NOONE);
    cities.insert(Cities::FRANKFURT, Players::NOONE);
    cities.insert(Cities::KHARKOV, Players::NOONE);
    cities.insert(Cities::COPENHAGEN, Players::NOONE);
    cities.insert(Cities::KIEV, Players::NOONE);
    cities.insert(Cities::LISBON, Players::NOONE);
    cities.insert(Cities::LONDON, Players::NOONE);
    cities.insert(Cities::MADRID, Players::NOONE);
    cities.insert(Cities::MARSEILLE, Players::NOONE);
    cities.insert(Cities::MOSCOW, Players::NOONE);
    cities.insert(Cities::MUNICH, Players::NOONE);
    cities.insert(Cities::PALERMO, Players::NOONE);
    cities.insert(Cities::PAMPLONA, Players::NOONE);
    cities.insert(Cities::PARIS, Players::NOONE);
    cities.insert(Cities::STPETERSBURG, Players::NOONE);
    cities.insert(Cities::RIGA, Players::NOONE);
    cities.insert(Cities::ROME, Players::NOONE);
    cities.insert(Cities::ROSTOV, Players::NOONE);
    cities.insert(Cities::SARAJEVO, Players::NOONE);
    cities.insert(Cities::SEVASTOPOL, Players::NOONE);
    cities.insert(Cities::SMOLENSK, Players::NOONE);
    cities.insert(Cities::IZMIR, Players::NOONE);
    cities.insert(Cities::SOCHI, Players::NOONE);
    cities.insert(Cities::SOFIA, Players::NOONE);
    cities.insert(Cities::STOCKHOLM, Players::NOONE);
    cities.insert(Cities::VENICE, Players::NOONE);
    cities.insert(Cities::WARSAW, Players::NOONE);
    cities.insert(Cities::VIENNA, Players::NOONE);
    cities.insert(Cities::VILNIUS, Players::NOONE);
    cities.insert(Cities::ZAGREB, Players::NOONE);
    cities.insert(Cities::ZURICH, Players::NOONE);

    let mut connections: HashMap<(Cities, Cities, Colors), (Players, u8)> = HashMap::new();

    connections.insert(
        (Cities::BARCELONA, Cities::MADRID, Colors::YELLOW),
        (Players::NOONE, 2),
    );

    connections.insert(
        (Cities::BARCELONA, Cities::PAMPLONA, Colors::GREY),
        (Players::NOONE, 2),
    );

    connections.insert(
        (Cities::CADIZ, Cities::MADRID, Colors::ORANGE),
        (Players::NOONE, 3),
    );

    connections.insert(
        (Cities::CADIZ, Cities::LISBON, Colors::BLUE),
        (Players::NOONE, 2),
    );

    connections.insert(
        (Cities::LISBON, Cities::MADRID, Colors::PINK),
        (Players::NOONE, 3),
    );

    connections.insert(
        (Cities::MADRID, Cities::PAMPLONA, Colors::BLACK),
        (Players::NOONE, 3),
    );

    connections.insert(
        (Cities::MADRID, Cities::PAMPLONA, Colors::WHITE),
        (Players::NOONE, 3),
    );

    html! {<>
        {
            connections.iter().map(|((from, to, _color), (_player, len))| {
                html!{<>
                    <div class={"card bg-neutral w-96 shadow-xl"}>
                        <div class={"card-body"}>
                            <p>{from.as_str()}{ "---" }{ len }{ "---" }{to.as_str()}</p>
                        </div>
                    </div>
                </>}
            }).collect::<Html>()
        }
    </>}
}

fn main() {
    yew::Renderer::<App>::new().render();
}
