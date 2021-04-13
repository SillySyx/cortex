use yew::prelude::*;
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};
use yew::web_sys::HtmlInputElement;

use crate::components::{Button, PageHeader, Error};
use crate::services::PasswordService;

use super::page::Views;

pub enum Messages {
    ChangeView(Views, Option<String>),

    ImportClicked,
    ImportFile(File),
    ImportData(Vec<u8>),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub change_view: Callback<(Views, Option<String>)>,
}

pub struct ImportExportView {
    props: Props,
    link: ComponentLink<Self>,
    reader_service: ReaderService,
    reader_task: Option<ReaderTask>,
    upload_ref: NodeRef,
    import_error: String,
}

impl Component for ImportExportView {
    type Message = Messages;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            reader_service: ReaderService::new(),
            reader_task: None,
            upload_ref: NodeRef::default(),
            import_error: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Messages::ChangeView(view, id) => {
                self.props.change_view.emit((view, id));
                false
            },
            Messages::ImportClicked => {
                self.import_error = String::from("");
                if let Some(input) = self.upload_ref.cast::<HtmlInputElement>() {
                    let _ = input.click();
                }
                true
            },
            Messages::ImportFile(file) => {
                let callback = self.link.callback(|data: FileData| Messages::ImportData(data.content));
                let task = self.reader_service.read_file(file, callback).unwrap();
                self.reader_task = Some(task);
                false
            },
            Messages::ImportData(data) => {
                let bytes: Vec<u8> = match serde_json::from_slice(&data) {
                    Ok(bytes) => bytes,
                    Err(_) => {
                        self.import_error = String::from("Failed to read passwords file");
                        return true;
                    },
                };

                if let Ok(_) = PasswordService::import_categories(&bytes) {
                    self.props.change_view.emit((Views::ListPasswords, None));
                    return false;
                }

                self.import_error = String::from("Failed to import passwords");
                true
            },
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let encrypted_bytes = match PasswordService::export_categories() {
            Ok(value) => value,
            Err(_) => vec![],
        };
        let href = format!("data:text/plain;charset=utf-8,{:?}", encrypted_bytes);

        let file_uploaded = self.link.callback(|event: ChangeData| {
            if let ChangeData::Files(files) = event {
                if let Some(file) = files.get(0) {
                    return Messages::ImportFile(file);
                }
            }
            Messages::ImportData(vec![])
        });

        html! {
            <div class="import-export animation-fade">
                <PageHeader 
                    title="Import/Export"
                    description="Move passwords between devices by export and import, both devices needs the same master password.">
                </PageHeader>

                { match self.import_error.is_empty() {
                    true => html! {},
                    false => html! {
                        <Error text=&self.import_error />
                    },
                }}
                
                <div class="button-grid import-export-buttons">
                    <input ref=self.upload_ref.clone() type="file" onchange=file_uploaded />
                    <Button clicked=self.link.callback(|_| Messages::ImportClicked)>
                        {"Import"}
                    </Button>
                    <a class="main-button animation-grow" href=href download="passwords.cortex">
                        {"Export"}
                    </a>
                    <Button clicked=self.link.callback(|_| Messages::ChangeView(Views::ListPasswords, None))>
                        {"Back"}
                    </Button>
                </div>
            </div>
        }
    }
}