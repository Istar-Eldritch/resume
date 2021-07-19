use yew::prelude::*;
use yew::virtual_dom::{VList, VNode};

#[derive(Properties, Clone, PartialEq)]
pub struct ScoreProperties {
    pub score: u8,
}
pub struct Score {
    props: ScoreProperties,
}

impl Component for Score {
    type Message = ();

    type Properties = ScoreProperties;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Score { props }
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
        let mut scores: Vec<VNode> = Vec::new();

        for i in 1..6 {
            let mut score_class = String::from("score__dot");
            if self.props.score >= i {
                score_class = score_class + " score__dot--high"
            }
            scores.push(html! {
                <div class={score_class}></div>
            })
        }
        let children = VList::new_with_children(scores, None);

        html! {
            <div class="score">
                {children}
            </div>
        }
    }
}
