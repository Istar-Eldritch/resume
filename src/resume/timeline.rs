use chrono::{Date, Utc};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct TimelineValueProps {
    pub from: Date<Utc>,
    pub to: Date<Utc>,
    pub children: Children,
}

pub struct TimeLineValue {
    pub props: TimelineValueProps,
}

impl Component for TimeLineValue {
    type Message = ();

    type Properties = TimelineValueProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        TimeLineValue { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            return true;
        }
        false
    }

    fn view(&self) -> Html {
        let to_date = self.props.to.format("%B %Y");
        let from_date = self.props.from.format("%B %Y");
        html! {
            <div class="timeline__value">
                <div class="timeline__value__time">
                    <span class="timeline__value__time__text">{to_date.to_string()}</span>
                    <span class="timeline__value__time__text">{from_date.to_string()}</span>
                </div>
                <div class="timeline__value__dot_bg"></div>
                <div class="timeline__value__dot_fg"></div>
                <div class="timeline__value__content">
                    {self.props.children.clone()}
                </div>
            </div>
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct TimeLineProps {
    pub children: Children,
}
pub struct TimeLine {
    pub props: TimeLineProps,
}

impl Component for TimeLine {
    type Message = ();

    type Properties = TimeLineProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        TimeLine { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            return true;
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="timeline">{self.props.children.clone()}</div>
        }
    }
}
