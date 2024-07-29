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

use leptos::{callback, prelude::*};

/// In the case of ControlledSignal, the state is controlled by the user.
/// However, the ControlledSignal notifies on change of the value to the callback provide.    
pub struct ControlledSignal<T>
where
    T: Send + Sync + Clone + PartialEq + 'static,
{
    value: RwSignal<T>,
    // default_value: T,
    // on_change: Option<Callback<T>>,
    effect: Option<Effect>,
}
impl<T> ControlledSignal<T>
where
    T: Send + Sync + Clone + PartialEq + 'static,
{
    fn new(value: RwSignal<T>, on_change: Option<Callback<T>>) -> Self {
        // return the passsed value as it is,
        // Also, trigger the onChange if it is registered in case of controlled.
        // And only trigger if the it value is changed, like a memorised.

        // TODO: in the 0.7, the watch function is not yet implemented. Once that is implemented,
        // can be swithed back to watch. Until the use memo and hook
        // let sub = match on_change {
        //     Some(on_change) => {
        //         let stop = leptos_reactive::watch(
        //             move || value.get(),
        //             move |current, prev, _| {
        //                 if (current != prev) {
        //                     on_change.call(current)
        //                 }
        //                 // log::debug!("Number: {}; Prev: {:?}", num, prev_num);
        //             },
        //             false,
        //         );
        //         Some(stop)
        //     }
        //     None => None,
        // };

        let sub = match on_change {
            Some(on_change) => {
                let cloned_value = value.clone();
                let memorized = Memo::new(move |_| cloned_value.get());
                let effect = Effect::new(move |_| {
                    on_change(memorized.get());
                });
                Some(effect)
            }
            None => None,
        };

        Self {
            value,
            // default_value,
            effect: sub,
        }
    }

    fn split(self) -> (ReadSignal<T>, WriteSignal<T>, Option<Effect>) {
        let (read, write) = self.value.split();
        (read, write, self.effect)
    }
}

/// UncontrolledSignal manages the internal state to the componet.
/// You can optionally subscribe for on change effect
pub struct UncontrolledSignal<T>
where
    T: Send + Sync + Clone + PartialEq + 'static,
{
    // default_value: T,
    state: ReadSignal<T>,
    set_state: WriteSignal<T>,
    effect: Option<Effect>,
}

impl<T> UncontrolledSignal<T>
where
    T: Send + Sync + Clone + PartialEq + 'static,
{
    fn new(default_value: T, on_change: Option<Callback<T>>) -> Self {
        // if the passed value is not present, therefore we have to manage the state and it is uncontrolled
        // Manage the state internally. use the internal signal for the value.
        // Also, trigger the onChange only if the the state gets change.
        // use memo to do it!
        let (state, set_state) = RwSignal::new(default_value).split();

        let sub = match on_change {
            Some(on_change) => {
                // let cloned_value = state.clone();
                let memorized = Memo::new(move |_| state.get());
                let effect = Effect::new(move |_| {
                    on_change(memorized.get());
                });
                Some(effect)
            }
            None => None,
        };

        Self {
            state,
            set_state,
            effect: sub,
        }
    }

    fn split(self) -> (ReadSignal<T>, WriteSignal<T>, Option<Effect>) {
        (self.state, self.set_state, self.effect)
    }
}

/// Wrapper hook for managed and unmanaged signals.
/// It provides a convinient way in which to manage the state regardles if the user would like to
/// control the state or let be controlled by the components internally.
pub enum ControllableSignal<T>
where
    T: Send + Sync + Clone + PartialEq + 'static,
{
    Controlled(ControlledSignal<T>),
    Uncontrolled(UncontrolledSignal<T>),
}

impl<T> ControllableSignal<T>
where
    T: Send + Sync + Clone + PartialEq + 'static,
{
    pub fn new(
        value: Option<RwSignal<T>>,
        default_value: T,
        on_change: Option<Callback<T>>, // TODO: make it MaybeCallback!
    ) -> Self {
        match value {
            Some(value) => ControllableSignal::Controlled(ControlledSignal::new(value, on_change)),
            None => {
                ControllableSignal::Uncontrolled(UncontrolledSignal::new(default_value, on_change))
            }
        }
    }

    pub fn split(self) -> (ReadSignal<T>, WriteSignal<T>, Option<Effect>) {
        match self {
            ControllableSignal::Controlled(sig) => sig.split(),
            ControllableSignal::Uncontrolled(sig) => sig.split(),
        }
    }
}
