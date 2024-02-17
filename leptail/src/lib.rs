use leptos::*;

pub mod callback;
pub mod icon;
pub mod input;
pub mod navigation;
pub mod overlay;
pub mod surfaces;
pub mod theme;


pub use input::Switch;
pub use input::SwitchBaseTheme;
pub use input::SwitchTheme;
pub use navigation::Drawer;
pub use navigation::DrawerTheme;
pub use overlay::Overlay; 
pub use overlay::OverlayTheme; 
pub use surfaces::Appbar;
pub use surfaces::AppbarTheme;
pub use theme::AppTheme;
use prelude::Consumer;
// pub use theme::ThemeVariant;


#[derive(Debug, Clone)]
pub struct OptionalSignal<T: 'static>(Option<Signal<T>>);

impl<T> Default for OptionalSignal<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T: 'static, I: Into<Signal<T>>> From<I> for OptionalSignal<T> {
    fn from(value: I) -> Self {
        Self(Some(value.into()))
    }
}

#[derive(Debug, Clone)]
pub struct OptionalMaybeSignal<T: 'static>(Option<MaybeSignal<T>>);

impl<T: Clone> OptionalMaybeSignal<T> {
    pub fn or<D: Into<MaybeSignal<T>>>(self, default: D) -> MaybeSignal<T> {
        match self.0 {
            Some(maybe_signal) => maybe_signal,
            None => default.into(),
        }
    }

    pub fn or_else<D: Into<MaybeSignal<T>>, F: Fn() -> D>(self, callback: F) -> MaybeSignal<T> {
        match self.0 {
            Some(maybe_signal) => maybe_signal,
            None => callback().into(),
        }
    }

    pub fn or_default(self) -> MaybeSignal<T>
    where
        T: Default,
    {
        match self.0 {
            Some(maybe_signal) => maybe_signal,
            None => MaybeSignal::Static(T::default()),
        }
    }

    pub fn map<U: 'static, F: Fn(T) -> U + 'static>(self, map: F) -> OptionalMaybeSignal<U> {
        match self.0 {
            Some(maybe_signal) => match maybe_signal {
                MaybeSignal::Static(v) => MaybeSignal::Static(map(v)).into(),
                MaybeSignal::Dynamic(sig) => {
                    MaybeSignal::Dynamic(Signal::derive(move || map(sig.get()))).into()
                }
            },
            None => OptionalMaybeSignal(None),
        }
    }
}

impl<T: Copy> Copy for OptionalMaybeSignal<T> {}

impl<T> Default for OptionalMaybeSignal<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T: 'static, I: Into<MaybeSignal<T>>> From<I> for OptionalMaybeSignal<T> {
    fn from(value: I) -> Self {
        Self(Some(value.into()))
    }
}

impl<T: Clone + Default> SignalGet for OptionalMaybeSignal<T> {
    type Value = T;

    fn get(&self) -> T {
        match &self.0 {
            Some(signal) => signal.get(),
            None => T::default(),
        }
    }

    fn try_get(&self) -> Option<T> {
        match &self.0 {
            Some(signal) => signal.try_get(),
            None => Some(T::default()),
        }
    }
}

impl<T: Clone + Default> SignalGetUntracked for OptionalMaybeSignal<T> {
    type Value = T;

    fn get_untracked(&self) -> T {
        match &self.0 {
            Some(signal) => signal.get_untracked(),
            None => T::default(),
        }
    }

    fn try_get_untracked(&self) -> Option<T> {
        match &self.0 {
            Some(signal) => signal.try_get_untracked(),
            None => Some(T::default()),
        }
    }
}

impl<T: IntoAttribute + Clone> IntoAttribute for OptionalMaybeSignal<T> {
    fn into_attribute(self) -> Attribute {
        match self.0 {
            Some(t) => t.into_attribute(), // Requires T to be Clone!
            None => Attribute::Option(None),
        }
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        match self.0 {
            Some(t) => t.into_attribute(), // Requires T to be Clone!
            None => Attribute::Option(None),
        }
    }
}

pub trait MaybeSignalExt<T: 'static> {
    fn map<U: 'static, F: Fn(T) -> U + 'static>(self, mapper: F) -> MaybeSignal<U>;
}

impl<T: Clone + 'static> MaybeSignalExt<T> for MaybeSignal<T> {
    fn map<U: 'static, F: Fn(T) -> U + 'static>(self, mapper: F) -> MaybeSignal<U> {
        match self {
            MaybeSignal::Static(v) => MaybeSignal::Static(mapper(v)),
            MaybeSignal::Dynamic(sig) => {
                MaybeSignal::Dynamic(Signal::derive(move || mapper(sig.get())))
            }
        }
    }
}

pub mod prelude {
    // pub use super::theme::ThemeVariant;
    pub use super::callback::consumer;
    pub use super::callback::Consumer;
    pub use super::callback::producer;
    pub use super::callback::Producer;
    pub use super::callback::ViewCallback;
    pub use super::callback::ViewProducer;
    pub use super::icon::Icon;
    pub use super::input::Switch;
    pub use super::input::SwitchBaseTheme;
    pub use super::input::SwitchTheme;
    pub use super::navigation::Drawer;
    pub use super::navigation::DrawerTheme;
    pub use super::OptionalMaybeSignal; 
    pub use super::OptionalSignal; 
    pub use super::Out;
    pub use super::overlay::Overlay;
    pub use super::overlay::OverlayTheme;
    pub use super::surfaces::Appbar;
    pub use super::surfaces::AppbarTheme;
    pub use super::theme::AppTheme;
    pub use super::theme::HasThemeVariant;
}



pub enum Out<O: 'static> {
    Consumer(Consumer<O>),
    Callback(Callback<O, ()>),
    WriteSignal(WriteSignal<O>),
}

impl<O: 'static> Copy for Out<O> {}

impl<O: 'static> Clone for Out<O> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<O: 'static> Out<O> {
    pub fn set(&self, new_value: O) {
        match self {
            Out::Consumer(consumer) => consumer.consume(new_value),
            Out::Callback(callback) => callback.call(new_value),
            Out::WriteSignal(write_signal) => write_signal.set(new_value),
        }
    }
}

// impl<T: 'static, F: Fn(T) -> () + 'static> From<F> for Out<T> {
//     fn from(fun: F) -> Self {
//         Out::Consumer(fun.into())
//     }
// }

impl<O: 'static> From<Consumer<O>> for Out<O> {
    fn from(consumer: Consumer<O>) -> Self {
        Out::Consumer(consumer)
    }
}

impl<O: 'static> From<Callback<O, ()>> for Out<O> {
    fn from(callback: Callback<O, ()>) -> Self {
        Out::Callback(callback)
    }
}

impl<O: 'static> From<WriteSignal<O>> for Out<O> {
    fn from(write_signal: WriteSignal<O>) -> Self {
        Out::WriteSignal(write_signal)
    }
}



