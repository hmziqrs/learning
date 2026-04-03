use std::collections::HashMap;

use icewow_engine::HttpMethod;

use crate::model::{BodyType, RequestTab, ResponseData, ResponseTab, Tab, TabId};
use crate::state::tree::NodeId;

/// TabStore with O(1) lookup via HashMap + ordered display via Vec<TabId>.
#[derive(Debug, Clone)]
pub struct TabStore {
    tabs: HashMap<TabId, Tab>,
    order: Vec<TabId>,
    active: Option<TabId>,
    next_id: TabId,
}

impl TabStore {
    pub fn new() -> Self {
        Self {
            tabs: HashMap::new(),
            order: Vec::new(),
            active: None,
            next_id: 1,
        }
    }

    pub fn alloc_id(&mut self) -> TabId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    // ── Read operations ────────────────────────────────────────

    pub fn active(&self) -> Option<&Tab> {
        self.active.and_then(|id| self.tabs.get(&id))
    }

    pub fn active_mut(&mut self) -> Option<&mut Tab> {
        self.active.and_then(|id| self.tabs.get_mut(&id))
    }

    pub fn active_id(&self) -> Option<TabId> {
        self.active
    }

    pub fn get(&self, id: TabId) -> Option<&Tab> {
        self.tabs.get(&id)
    }

    pub fn get_mut(&mut self, id: TabId) -> Option<&mut Tab> {
        self.tabs.get_mut(&id)
    }

    pub fn ordered(&self) -> impl Iterator<Item = &Tab> {
        self.order.iter().filter_map(|id| self.tabs.get(id))
    }

    pub fn ordered_enumerate(&self) -> impl Iterator<Item = (usize, TabId, &Tab)> {
        self.order
            .iter()
            .enumerate()
            .filter_map(|(i, &id)| self.tabs.get(&id).map(|tab| (i, id, tab)))
    }

    pub fn len(&self) -> usize {
        self.order.len()
    }

    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }

    /// Find a tab by its associated request ID.
    pub fn find_by_request(&self, request_id: NodeId) -> Option<TabId> {
        self.tabs
            .values()
            .find(|tab| tab.request_id == Some(request_id))
            .map(|tab| tab.id)
    }

    /// Find the position index of a tab in the ordered list.
    pub fn position(&self, tab_id: TabId) -> Option<usize> {
        self.order.iter().position(|&id| id == tab_id)
    }

    // ── Write operations ───────────────────────────────────────

    /// Open a tab for the given request. If already open, activates it.
    pub fn open_for_request(&mut self, request_id: NodeId, name: String, url: String, method: HttpMethod) -> TabId {
        // Check if already open
        if let Some(existing_id) = self.find_by_request(request_id) {
            self.active = Some(existing_id);
            return existing_id;
        }

        // Create new tab
        let id = self.alloc_id();
        let tab = Tab {
            id,
            request_id: Some(request_id),
            title: name,
            url_input: url,
            method,
            body_type: BodyType::None,
            body_text: String::new(),
            form_pairs: Vec::new(),
            headers: Vec::new(),
            active_request_tab: RequestTab::Params,
            query_params: Vec::new(),
            response: None,
            loading: false,
            active_response_tab: ResponseTab::Body,
            dirty: false,
        };

        self.tabs.insert(id, tab);
        self.order.push(id);
        self.active = Some(id);
        id
    }

    /// Create a new standalone tab (no associated request).
    pub fn new_tab(&mut self) -> TabId {
        let id = self.alloc_id();
        let tab = Tab {
            id,
            request_id: None,
            title: format!("New Tab {id}"),
            url_input: String::new(),
            method: HttpMethod::Get,
            body_type: BodyType::None,
            body_text: String::new(),
            form_pairs: Vec::new(),
            headers: Vec::new(),
            active_request_tab: RequestTab::Params,
            query_params: Vec::new(),
            response: None,
            loading: false,
            active_response_tab: ResponseTab::Body,
            dirty: false,
        };

        self.tabs.insert(id, tab);
        self.order.push(id);
        self.active = Some(id);
        id
    }

    /// Close a tab by its ID.
    pub fn close_by_tab(&mut self, tab_id: TabId) {
        self.order.retain(|&id| id != tab_id);
        self.tabs.remove(&tab_id);
        self.fallback_active();
    }

    /// Close any tab referencing the given request ID.
    pub fn close_by_request(&mut self, request_id: NodeId) {
        self.order.retain(|&id| {
            self.tabs
                .get(&id)
                .is_none_or(|tab| tab.request_id != Some(request_id))
        });
        self.tabs.retain(|_, tab| tab.request_id != Some(request_id));
        self.fallback_active();
    }

    /// Close all tabs referencing any of the given request IDs.
    pub fn close_by_requests(&mut self, request_ids: &[NodeId]) {
        self.order.retain(|&id| {
            self.tabs
                .get(&id)
                .is_none_or(|tab| !tab.request_id.is_some_and(|rid| request_ids.contains(&rid)))
        });
        self.tabs.retain(|_, tab| {
            !tab.request_id.is_some_and(|rid| request_ids.contains(&rid))
        });
        self.fallback_active();
    }

    /// Reorder a tab: move it to the target index.
    pub fn reorder(&mut self, tab_id: TabId, target_index: usize) {
        let Some(source_index) = self.order.iter().position(|&id| id == tab_id) else {
            return;
        };
        let mut target = target_index.min(self.order.len());
        let tab = self.order.remove(source_index);
        if source_index < target {
            target = target.saturating_sub(1);
        }
        self.order.insert(target, tab);
    }

    /// Set the active tab.
    pub fn set_active(&mut self, tab_id: TabId) {
        if self.tabs.contains_key(&tab_id) {
            self.active = Some(tab_id);
        }
    }

    /// If active tab is gone, fall back to first tab.
    pub fn fallback_active(&mut self) {
        if let Some(active_id) = self.active {
            if self.tabs.contains_key(&active_id) {
                return;
            }
        }
        self.active = self.order.first().copied();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reorder_tabs_keeps_active_tab_identity() {
        let mut store = TabStore::new();
        let t1 = store.new_tab();
        let t2 = store.new_tab();
        let t3 = store.new_tab();

        // t1 is at index 0, t2 at 1, t3 at 2. Active is t3 (last created).
        // Move t2 (index 1) to after t3 (target index 3)
        store.reorder(t2, 3);

        let ids: Vec<TabId> = store.order.clone();
        assert_eq!(ids, vec![t1, t3, t2]);
        assert_eq!(store.active_id(), Some(t3));
    }

    #[test]
    fn open_for_request_reuses_existing_tab() {
        let mut store = TabStore::new();
        let first = store.open_for_request(42, "Req".into(), "http://x".into(), HttpMethod::Get);
        store.new_tab(); // changes active
        let second = store.open_for_request(42, "Req".into(), "http://x".into(), HttpMethod::Get);
        assert_eq!(first, second);
        assert_eq!(store.active_id(), Some(first));
    }

    #[test]
    fn close_by_request_removes_matching_tab() {
        let mut store = TabStore::new();
        let t1 = store.open_for_request(1, "A".into(), "u".into(), HttpMethod::Get);
        let _t2 = store.new_tab();
        assert_eq!(store.len(), 2);

        store.close_by_request(1);
        assert!(!store.tabs.contains_key(&t1));
        assert_eq!(store.len(), 1);
    }
}
