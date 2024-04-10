use crate::domain::tab::tab_id::TabId;

#[derive(Debug)]
pub struct OpenItem {
    menu_number: usize,
    description: String,
}

#[derive(Debug, Default)]
pub struct OpenTab {
    id: TabId,
    open_items: Vec<OpenItem>,
}

// impl OpenTab {
//     pub fn new(id: TabId) -> Self {
//         Self {
//             id,
//             open_items: Vec::new(),
//         }
//     }
// }

#[cfg(test)]
mod test {
    use crate::domain::tab::tab_id::TabId;

    use super::OpenTab;

    #[test]
    fn when_new_tab_then_open_items_is_empty() {
        let tab = OpenTab::new(TabId::default());

        assert!(tab.open_items.is_empty())
    }
}
