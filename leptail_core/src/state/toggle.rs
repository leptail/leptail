/*
 * Copyright 2024 Leptail.
 * MIT License
 *
 * Portions of this file are based on code from react-spectrum and solid aria.
 * Copyright 2020 Adobe. All rights reserved.
 *
 * This file is licensed to you under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License. You may obtain a copy
 * of the License at http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software distributed under
 * the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR REPRESENTATIONS
 * OF ANY KIND, either express or implied. See the License for the specific language
 * governing permissions and limitations under the License.
 */

use leptos::prelude::*;

use super::controllable::ControllableSignal;

pub struct ToggleStateProps {
    // TODO: check if this is correct.
    // from solid aria: defaultSelected?: MaybeAccessor<boolean | undefined>;
    /// Whether the element should be selected (uncontrolled).
    default_selected: MaybeProp<bool>,

    /// Whether the element should be selected (controlled).
    is_selected: MaybeProp<bool>,

    /// Whether the element can be selected but not changed by the user.
    is_read_only: MaybeProp<bool>,
    // Handler that is called when the element's selection state changes.
    onChange: Option<Callback<bool>>,
}

pub struct ToggleState {
    is_selected: ReadSignal<bool>,
    set_selected: WriteSignal<bool>,
    effect: Option<Effect>,
    is_read_only: bool,
}

impl ToggleState {
    pub fn new(
        value: Option<RwSignal<bool>>,
        default_value: bool,
        is_read_only: bool,
        // on_change: Option<Callback<bool>>,
    ) -> Self {
        let (is_selected, set_selected, effect) =
            ControllableSignal::new(value, default_value, None).split();

        ToggleState {
            is_selected,
            set_selected,
            effect,
            is_read_only,
        }
    }

    // pub fn split(self) -> (ReadSignal<bool>, WriteSignal<bool>, Option<Effect>) {
    //     (self.is_selected, self.set_selected, self.effect)
    // }

    pub fn set(&self, value: bool) {
        if !&self.is_read_only {
            self.set_selected.set(value);
        }
    }

    pub fn toggle(&self) {
        if !&self.is_read_only {
            self.set_selected.update(|v| *v = !(*v));
        }
    }
}
