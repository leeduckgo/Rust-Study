use state::{Entry, Filter, State};
use strum::IntoEnumIterator;
use yew::format::Json;
use yew::web_sys::HtmlInputElement as InputElement;
use yew::{classes, html, Component, ComponentLink, Html, InputData, NodeRef, ShouldRender};
use yew::{events::KeyboardEvent, Classes};
use yew_services::storage::{Area, StorageService};
#[macro_use] extern crate log;

use base64::{encode, decode};

mod state;

const KEY: &str = "yew.todomvc.self";

pub enum Msg {
    Add,
    Edit(usize),
    Update(String),
    UpdateEdit(String),
    Remove(usize),
    SetFilter(Filter),
    ToggleAll,
    ToggleEdit(usize),
    Toggle(usize),
    ClearCompleted,
    Focus,
}

pub struct Model {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
    focus_ref: NodeRef,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        let entries = {
            if let Json(Ok(restored_model)) = storage.restore(KEY) {
                restored_model
            } else {
                Vec::new()
            }
        };
        let state = State {
            entries,
            filter: Filter::All,
            value: "".into(),
            edit_value: "".into(),
        };
        let focus_ref = NodeRef::default();
        Self {
            link,
            storage,
            state,
            focus_ref,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                info!("add things");
                // mut variable
                let description = self.state.value.trim();
                let description_handled = format!("{}: {}", description, encode(description.to_string()));

                if !description.is_empty() {
                    let entry = Entry {
                        description: description_handled,
                        completed: false,
                        editing: false,
                    };
                    info!("{}", entry.description);
                    self.state.entries.push(entry);
                }
                self.state.value = "".to_string();
            }
            Msg::Edit(idx) => {
                let edit_value = self.state.edit_value.trim().to_string();
                self.state.complete_edit(idx, edit_value);
                self.state.edit_value = "".to_string();
            }
            Msg::Update(val) => {
                println!("Input: {}", val);
                self.state.value = val;
            }
            Msg::UpdateEdit(val) => {
                println!("Input: {}", val);
                self.state.edit_value = val;
            }
            Msg::Remove(idx) => {
                self.state.remove(idx);
            }
            Msg::SetFilter(filter) => {
                self.state.filter = filter;
            }
            Msg::ToggleEdit(idx) => {
                self.state.edit_value = self.state.entries[idx].description.clone();
                self.state.clear_all_edit();
                self.state.toggle_edit(idx);
            }
            Msg::ToggleAll => {
                let status = !self.state.is_all_completed();
                self.state.toggle_all(status);
            }
            Msg::Toggle(idx) => {
                self.state.toggle(idx);
            }
            Msg::ClearCompleted => {
                self.state.clear_completed();
            }
            Msg::Focus => {
                if let Some(input) = self.focus_ref.cast::<InputElement>() {
                    input.focus().unwrap();
                }
            // Msg::Encode => {     
            //     info!("encoding!");
            //     let description = self.state.value.trim();
            //     if !description.is_empty() {
            //         let entry = Entry {
            //             description: encode(description.to_string()),
            //             completed: false,
            //             editing: false,
            //         };
            //         info!("{}", entry);
            //         self.state.entries.push(entry);
            //     }
            //     self.state.value = "".to_string();     
            // }
            }
        }
        self.storage.store(KEY, Json(&self.state.entries));
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let hidden_class = if self.state.entries.is_empty() {
            "hidden"
        } else {
            ""
        };
        html! {
            <div class="todomvc-wrapper">
                <h1>{ "encode/decode" }</h1>
                { self.view_input() }
                <section class=classes!("main", hidden_class)>
                    <ul class="todo-list">
                        { for self.state.entries.iter().filter(|e| self.state.filter.fits(e)).enumerate().map(|e| self.view_entry(e)) }
                    </ul>
                </section>
            </div>
        }
    }
}

impl Model {
    fn view_filter(&self, filter: Filter) -> Html {
        let cls = if self.state.filter == filter {
            "selected"
        } else {
            "not-selected"
        };
        html! {
            <li>
                <a class=cls
                   href=filter.as_href()
                   onclick=self.link.callback(move |_| Msg::SetFilter(filter))
                >
                    { filter }
                </a>
            </li>
        }
    }

    fn view_input(&self) -> Html {
        html! {
            // You can use standard Rust comments. One line:
            // <li></li>
            <input
                class="new-todo"
                placeholder="What needs to be encode/decode?"
                value=&self.state.value
                oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                onkeypress=self.link.batch_callback(|e: KeyboardEvent| {
                    if e.key() == "Enter" { Some(Msg::Add) } else { None }
                })
            />
            /* Or multiline:
            <ul>
                <li></li>
            </ul>
            */
        }
    }

    fn view_entry(&self, (idx, entry): (usize, &Entry)) -> Html {
        let mut class = Classes::from("todo");
        if entry.editing {
            class.push(" editing");
        }
        if entry.completed {
            class.push(" completed");
        }
        html! {
            <li class=class>
                <div class="view">
                    <input
                        type="checkbox"
                        class="toggle"
                        checked=entry.completed
                        onclick=self.link.callback(move |_| Msg::Toggle(idx))
                    />
                    <label ondblclick=self.link.callback(move |_| Msg::ToggleEdit(idx))>{ &entry.description }</label>
                    <button class="destroy" onclick=self.link.callback(move |_| Msg::Remove(idx)) />
                </div>
                { self.view_entry_edit_input((idx, &entry)) }
            </li>
        }
    }

    fn view_entry_edit_input(&self, (idx, entry): (usize, &Entry)) -> Html {
        if entry.editing {
            html! {
                <input
                    class="edit"
                    type="text"
                    ref=self.focus_ref.clone()
                    value=&self.state.edit_value
                    onmouseover=self.link.callback(|_| Msg::Focus)
                    oninput=self.link.callback(|e: InputData| Msg::UpdateEdit(e.value))
                    onblur=self.link.callback(move |_| Msg::Edit(idx))
                    onkeypress=self.link.batch_callback(move |e: KeyboardEvent| {
                        if e.key() == "Enter" { Some(Msg::Edit(idx)) } else { None }
                    })
                />
            }
        } else {
            html! { <input type="hidden" /> }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
