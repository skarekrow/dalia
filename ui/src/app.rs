use yew::prelude::*;

pub struct App {}

pub struct WeatherCard {
    day: String,
    temperature: String,
}
#[derive(Clone, Properties)]
pub struct WeatherCardProps {
    pub day: String,
    pub temperature: String,
}

impl Component for WeatherCard {
    type Message = ();
    type Properties = WeatherCardProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        WeatherCard { day: props.day, temperature: props.temperature }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="weather-card flex gap-2 rounded border-2 backdrop-filter backdrop-blur p-4 shadow lg:flex-col">
                <img
                src="http://cdn.weatherapi.com/weather/64x64/day/116.png"
                alt="Weather Icon"
                class="h-12 w-12"
                />
                <div>
                <h2 class="text-xl">{ self.day.clone() }</h2>
                <p>{ self.temperature.clone() }{ "°" }</p>
                </div>
            </div>
        }
    }
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="min-h-screen text-white">
                <div class="container mx-auto px-4">
                <p class="absolute top-0 right-0 text-gray-500 p-4">
                    { "Zip: 12345" }
                </p>
                <div class="flex min-h-screen flex-col items-center justify-center">
                    <div class="mb-2 flex items-center space-x-4">
                    <img
                        src="http://cdn.weatherapi.com/weather/64x64/day/116.png"
                        alt="Weather Icon"
                        class="h-28 w-28"
                    />
                    <h1 class="text-6xl">{ "33°" }</h1>
                    </div>
                    <div class="mb-6">
                    <h2 class="text-4xl">{ "Raleigh, NC" }</h2>
                    </div>
                    <div class="grid grid-cols-1 gap-2 sm:grid-cols-1 lg:grid-cols-7">
                    <WeatherCard day="Tomorrow" temperature="33" />
                    <WeatherCard day="Tuesday" temperature="33" />
                    <WeatherCard day="Wednesday" temperature="33" />
                    <WeatherCard day="Thursday" temperature="33" />
                    <WeatherCard day="Friday" temperature="33" />
                    <WeatherCard day="Saturday" temperature="33" />
                    <WeatherCard day="Sunday" temperature="33" />
                    </div>
                </div>
                </div>
            </div>
        }
    }
}