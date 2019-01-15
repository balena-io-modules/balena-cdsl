use std::collections::HashMap;

use crate::dsl::schema::DocumentRoot;
use crate::dsl::schema::Schema;
use crate::dsl::schema::SchemaList;
use crate::output::UiObject;
use crate::output::UiObjectProperty;

impl From<DocumentRoot> for UiObject {
    fn from(schema: DocumentRoot) -> Self {
        match schema.schema {
            Some(schema) => match schema.children {
                Some(list) => list.into(),
                None => UiObject(HashMap::new()),
            },
            None => UiObject(HashMap::new()),
        }
    }
}

impl From<SchemaList> for UiObject {
    fn from(list: SchemaList) -> Self {
        let ui_object_entries: Vec<(String, UiObjectProperty)> = list
            .entries()
            .iter()
            .filter_map(|entry| {
                let property: UiObjectProperty = entry.schema.clone().into();
                if !property.is_empty() {
                    Some((entry.name.clone(), entry.schema.clone().into()))
                } else {
                    None
                }
            })
            .collect();

        let ui_object_entries: HashMap<String, UiObjectProperty> = ui_object_entries.into_iter().collect();
        UiObject(ui_object_entries)
    }
}

impl From<Schema> for UiObjectProperty {
    fn from(schema: Schema) -> Self {
        let help = schema.annotations.help;
        let warning = schema.annotations.warning;
        let description = schema.annotations.description;
        let widget = schema.annotations.widget;

        let children = schema.children.map(|children| children.into());

        UiObjectProperty {
            help,
            warning,
            description,
            widget,
            children,
        }
    }
}
