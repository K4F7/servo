/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crossbeam_channel::Sender;
use log::info;
use malloc_size_of_derive::MallocSizeOf;
use webrender_api::crossbeam_channel;

use crate::FragmentTree;

#[derive(MallocSizeOf)]
pub(crate) struct AccessibilityTree {}

pub(crate) struct AccessibilityThread {
    sender: Sender<Arc<FragmentTree>>,
    join_handle: JoinHandle<()>,
}

impl AccessibilityThread {
    pub(crate) fn new() -> Self {
        let (sender, receiver) = crossbeam_channel::bounded(0);
        Self {
            sender,
            join_handle: thread::spawn(move || {
                while let Ok(fragment_tree) = receiver.recv() {
                    info!("Thinking...");
                    std::thread::sleep(Duration::from_secs(1));
                    info!(
                        "Fragment tree has {} top-level fragments",
                        fragment_tree.root_fragments.len()
                    );
                }
            }),
        }
    }

    pub(crate) fn send(&self, fragment_tree: Arc<FragmentTree>) {
        // If this fails, the channel is either full (so the thread is busy)
        // or disconnected (so the thread is gone).
        let _ = self.sender.try_send(fragment_tree);
    }
}
