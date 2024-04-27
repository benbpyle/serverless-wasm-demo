mod player;

use yew::prelude::*;


#[function_component(App)]
fn app() -> Html {
    let players = player::generate_players().iter().map(|p| html! {
        <tr>
            <td>{p.get_id()}</td>
            <td>{p.get_first_name()}</td>
            <td>{p.get_last_name()}</td>
            <td>{p.get_country()}</td>
        </tr>
    }).collect::<Html>();

    html! {
        <div>
            <h1>{ "PGA Tour Players" }</h1>
            <table>
                <thead>
                    <tr>
                        <td>{ "ID" }</td>
                        <td>{ "First Name"}</td>
                        <td>{ "Last Name" }</td>
                        <td>{ "Country" }</td>
                    </tr>
                </thead>
                <tbody>
                    {players}
                </tbody>
            </table>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}