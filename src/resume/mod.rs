mod score;
mod timeline;

use chrono::{Date as UtcDate, NaiveDate, Utc};
use js_sys::Date;
use score::Score;
use timeline::{TimeLine, TimeLineValue};
use yew::prelude::*;
use yew::virtual_dom::VNode;
pub struct Resume {}

impl Component for Resume {
    type Message = ();

    type Properties = ();

    fn create(_props: Self::Properties, _link: yew::ComponentLink<Self>) -> Self {
        Resume {}
    }

    fn update(&mut self, _msg: Self::Message) -> yew::ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        let present = Date::new_0();

        let present: UtcDate<Utc> = UtcDate::from_utc(
            NaiveDate::from_ymd(
                present.get_utc_full_year() as i32,
                present.get_utc_month(),
                present.get_utc_date(),
            ),
            Utc {},
        );

        html! {
            <div class="resume">
                <div class="resume__header">
                    <div class="resume__header__intro">
                        <span class="resume__header__intro__name">{"Ruben Paz"}</span>
                        <span class="resume__header__intro__title">{"Software Developer"}</span>
                    </div>
                    <div class="resume__header__contact">
                        <Contact title="Address">
                            <span class="resume__header__contact__item">{"4 Rachel Close"}</span>
                            <span class="resume__header__contact__item">{"CB5 8TP"}</span>
                            <span class="resume__header__contact__item">{"Cambridge UK"}</span>
                        </Contact>
                        <Contact title="Contact">
                            <a class="resume__header__contact__item" href="mailto:me@ruben.io">{"me@ruben.io"}</a>
                            <a class="resume__header__contact__item" href="https://github.com/Istar-Eldritch">{"GitHub"}</a>
                            <a class="resume__header__contact__item" href="https://linkedin.com/in/rubenpaz/">{"LinkedIn"}</a>
                            <span class="resume__header__contact__item">{"+44 7470522734"}</span>
                        </Contact>
                    </div>
                </div>
                <div class="resume__content">
                    <div class="resume__content__row">
                        <Section title="PERSONAL STATEMENT">
                            <p class="statement">{"I'm a software developer with experience in mobile,
                            web development, tooling and infrastructure."}</p>
                            <p class="statement">{"I have in depth knowledge of the software development process
                            from beginning to end and I helped define the agile process
                            based on scrum that is used at Repositive. I architected
                            microservices systems from scratch using DDD, CQRS and Event
                            Sourcing and I have experience designing both public and
                            private APIs using REST & RPC."}</p>
                            <p class="statement">{"I create environments that provide high autonomy and I have
                            experience coaching team members, my leadership style is
                            based on trust and communicating what is important clearly. I'm
                            a team builder, I learned how to define the hiring strategy at
                            source{d} and I proved it at Repositive by taking an active role
                            in the hiring of my own team."}</p>
                        </Section>
                        <Section title="STRENGTHS">
                            <div class="strengths">
                                <div class="strengths__strength">
                                    <Image image="./assets/software_development.svg" text="Software Development"/>
                                </div>
                                <div class="strengths__strength strengths__strength--low">
                                    <Image image="./assets/project_management.svg" text="Project Management"/>
                                </div>
                                <div class="strengths__strength">
                                    <Image image="./assets/leadership.svg" text="Leadership"/>
                                </div>
                            </div>
                        </Section>
                        <Section title="WORK EXPERIENCE">
                            <TimeLine>

                                <TimeLineValue
                                    from={UtcDate::from_utc(NaiveDate::from_ymd(2019, 10, 1), Utc {})}
                                    to={present
                                }>
                                    <Job title="Lead Engineer" company={html!{<a href="https://www.fluidic.com/">{"Fluidic Analytics"}</a>}}>
                                        <ul>
                                            <li>{"Implemented the cloud architecture from scratch, services in async Rust & Python, Azure Blob storage, Postgres, Kubernetes"}</li>
                                            <li>{"Designed and implemented the data synchronization mechanism between instruments and cloud using JSON-RPC"}</li>
                                            <li>{"Provided tooling for provisioning and generation of certificates & credentials for new instruments in the production line"}</li>
                                            <li>{"Designed and implemented the experiment management, instrument management and permission modules"}</li>
                                        </ul>
                                    </Job>
                                </TimeLineValue>

                                <TimeLineValue
                                    from={UtcDate::from_utc(NaiveDate::from_ymd(2018, 10, 15), Utc {})}
                                    to={UtcDate::from_utc(NaiveDate::from_ymd(2019, 10, 1), Utc {})
                                }>
                                    <Job title="Tech Lead" company=html!{<a href="https://web.archive.org/web/20180830162534/https://repositive.io/">{"Repositive"}</a>}>
                                        <ul>
                                            <li>{"Line management,technical direction and vision"}</li>
                                            <li>{"Managed the technial and architecture backlog"}</li>
                                            <li>{"Achieved a tight iteration loop of two weeks between releases that included atomated testing and manual QA"}</li>
                                            <li>{"Ensured high quality tooling was in place, new team members were able to push code in a couple days without prior introduction to the system architecture"}</li>
                                        </ul>
                                    </Job>
                                </TimeLineValue>

                                <TimeLineValue
                                from={UtcDate::from_utc(NaiveDate::from_ymd(2016, 6, 15), Utc {})}
                                to={UtcDate::from_utc(NaiveDate::from_ymd(2018, 10, 1), Utc {})
                                }>
                                    <Job title="Backend Engineer" company=html!{<a href="https://web.archive.org/web/20180830162534/https://repositive.io/">{"Repositive"}</a>}>
                                        <ul>
                                        <li>{"Designed and implmented the first iterations of the Discover platform, migrated the monolitic application to microservices in Rust & Typescript"}</li>
                                        <li>{"Implemented a fine grained attribute based permission system (ABAC)"}</li>
                                        <li>{"Implemented a custom DSL for search based on PEGs, with backends to SQL & ElasticSearch"}</li>
                                        <li>{"Trialed and managed the transition to Kubernetes on GCP and AWS"}</li>
                                        </ul>
                                    </Job>
                                </TimeLineValue>

                                <TimeLineValue
                                from={UtcDate::from_utc(NaiveDate::from_ymd(2015, 11, 1), Utc {})}
                                to={UtcDate::from_utc(NaiveDate::from_ymd(2016, 5, 1), Utc {})
                                }>
                                    <Job title="Developer Relations" company=html!{<a href="https://web.archive.org/web/20191031123107/https://sourced.tech/">{"source{d}"}</a>}>
                                        <ul>
                                        <li>{"Developed tooling for BI using BigQuery"}</li>
                                        <li>{"Interviewed a large number of software engineers (+500)"}</li>
                                        <li>{"Provided technical input and feedback on clasification algorithms for ML on Code"}</li>
                                        </ul>
                                    </Job>
                                </TimeLineValue>

                                <TimeLineValue
                                from={UtcDate::from_utc(NaiveDate::from_ymd(2014, 5, 1), Utc {})}
                                to={UtcDate::from_utc(NaiveDate::from_ymd(2015, 11, 1), Utc {})
                                }>
                                    <Job title="Software Engineer" company=html!{<a href="https://www.glofox.com/">{"Glofox"}</a>}>
                                        <ul>
                                        <li>{"Developed the first iteration of the Glofox product"}</li>
                                        <li>{"Helped design and implement the notifications system using MongoDB & Redis"}</li>
                                        <li>{"Developed a fullstack (Scala, Angular.js) dashboard system"}</li>
                                        <li>{"Developed a CI/CD system for Android & iOS"}</li>
                                        </ul>
                                    </Job>
                                </TimeLineValue>

                            </TimeLine>
                        </Section>
                    </div>
                    <div class="resume__content__row">
                        <Section title="SKILLS">
                            <div class="skills_explanation">
                                <div class="skills_explanation__explanation">
                                    <Score score=1/>
                                    <span class="skills_explanation__explanation__text">{"Used on academic context or pet projects"}</span>
                                </div>
                                <div class="skills_explanation__explanation">
                                    <Score score=2/>
                                    <span class="skills_explanation__explanation__text">{"Used at least in a professional project"}</span>
                                </div>
                                <div class="skills_explanation__explanation">
                                    <Score score=3/>
                                    <span class="skills_explanation__explanation__text">{"At least one year of professional experience"}</span>
                                </div>
                                <div class="skills_explanation__explanation">
                                    <Score score=4/>
                                    <span class="skills_explanation__explanation__text">{"At least three years of professional experience"}</span>
                                </div>
                                <div class="skills_explanation__explanation">
                                    <Score score=5/>
                                    <span class="skills_explanation__explanation__text">{"Knowledge beyond common professional requirements"}</span>
                                </div>
                            </div>

                            <div class="skill_section">
                                <span class="skill_section__header">{"Software Development"}</span>
                                <div class="skill_section__content">
                                    <Skill score=4 name="Rust"/>
                                    <Skill score=4 name="JavaScript"/>
                                    <Skill score=4 name="TypeScript"/>
                                    <Skill score=4 name="Node.js"/>
                                    <Skill score=4 name="React"/>
                                    <Skill score=3 name="Java"/>
                                    <Skill score=3 name="Android"/>
                                    <Skill score=2 name="Scala"/>
                                    <Skill score=2 name="Play Framework"/>
                                    <Skill score=1 name="Haskell"/>
                                    <Skill score=1 name="Lua"/>
                                </div>
                            </div>

                            <div class="skill_section">
                                <span class="skill_section__header">{"Infrastructure & Devops"}</span>
                                <div class="skill_section__content">
                                    <Skill score=4 name="Kubernetes"/>
                                    <Skill score=4 name="Docker"/>
                                    <Skill score=2 name="Terraform"/>
                                    <Skill score=4 name="Bash"/>
                                    <Skill score=3 name="AWS"/>
                                    <Skill score=3 name="GCP"/>
                                    <Skill score=4 name="CI/CD"/>
                                </div>
                            </div>

                            <div class="skill_section">
                                <span class="skill_section__header">{"Databases"}</span>
                                <div class="skill_section__content">
                                    <Skill score=4 name="Postgres"/>
                                    <Skill score=4 name="Redis"/>
                                    <Skill score=3 name="Elasticsearch"/>
                                    <Skill score=2 name="MongoDB"/>
                                    <Skill score=1 name="CrocoroachDB"/>
                                    <Skill score=1 name="ArangoDB"/>
                                </div>
                            </div>


                            <div class="skill_section">
                                <span class="skill_section__header">{"Paradigms & Patterns"}</span>
                                <div class="skill_section__content">
                                    <Skill score=4 name="Object Oriented"/>
                                    <Skill score=4 name="Functional"/>
                                    <Skill score=3 name="CQRS"/>
                                    <Skill score=3 name="Event Sourcing"/>
                                    <Skill score=4 name="Microservices"/>
                                </div>
                            </div>
                        </Section>
                        <Section title="INTERESTS">
                            <div class="interests">
                                <Image image="./assets/open-source.svg" text="Open Source" class=classes!("interests__image")/>
                                <Image image="./assets/environment.svg" text="Sustainability" class=classes!("interests__image")/>
                                <Image image="./assets/learning.svg" text="Lifelong Learning" class=classes!("interests__image")/>
                                <Image image="./assets/reading.svg" text="Reading" class=classes!("interests__image")/>
                                <Image image="./assets/electronics.svg" text="Electronics" class=classes!("interests__image")/>
                                <Image image="./assets/lambda.svg" text="Programming Languages" class=classes!("interests__image")/>
                            </div>
                        </Section>
                    </div>
                </div>
            </div>
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
struct ContactProps {
    title: String,
    #[prop_or_default]
    class: Classes,
    children: Children,
}
struct Contact {
    props: ContactProps,
}

impl Component for Contact {
    type Message = ();

    type Properties = ContactProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Contact { props }
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
        let mut css = self.props.class.clone();
        css.push("contact");
        html! {
            <div class=css>
                <div class="contact__title_box">
                    <span class="contact__title_box__title">{&self.props.title}</span>
                </div>
                <div class="contact__content">{self.props.children.clone()}</div>
            </div>
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
struct SectionProps {
    title: String,
    #[prop_or_default]
    class: Classes,
    children: Children,
}

struct Section {
    props: SectionProps,
}

impl Component for Section {
    type Message = ();

    type Properties = SectionProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Section { props }
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
        let mut css = self.props.class.clone();
        css.push("section");
        html! {
            <div class=css>
                <div class="section__header">{&self.props.title}</div>
                <div class="section__content">{self.props.children.clone()}</div>
            </div>
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
struct JobProperties {
    pub title: String,
    pub company: VNode,
    pub children: Children,
}

struct Job {
    props: JobProperties,
}

impl Component for Job {
    type Message = ();
    type Properties = JobProperties;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Job { props }
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
        html! {
            <div class="job">
                <span class="job__title">{self.props.title.clone()}</span>
                <span class="job__company">{self.props.company.clone()}</span>
                <div class="job__detail">
                    {self.props.children.clone()}
                </div>
            </div>
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
struct SkillProperties {
    score: u8,
    name: String,
}

struct Skill {
    props: SkillProperties,
}

impl Component for Skill {
    type Message = ();

    type Properties = SkillProperties;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Skill { props }
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
        html! {
            <div class="skill">
                <span class="skill__text">{self.props.name.clone()}</span>
                <Score score=self.props.score/>
            </div>
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
struct ImageProperties {
    image: String,
    text: String,
    #[prop_or_default]
    class: Classes,
}

struct Image {
    props: ImageProperties,
}

impl Component for Image {
    type Message = ();

    type Properties = ImageProperties;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Image { props }
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
        let mut c = self.props.class.clone();
        c.push("image");
        html! {
            <div class=c>
            <img src={self.props.image.clone()} class="image__img"/>
            <span class="image__text">{self.props.text.clone()}</span>
        </div>
        }
    }
}
