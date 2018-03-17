macro_rules! declare_event {
    ($doc:expr) => {
        #[doc = $doc]
        #[derive(Clone)]
        pub struct Event(super::Event);

        impl From<Event> for super::Event {
            #[inline]
            fn from(subclass: Event) -> Self { subclass.0 }
        }

        impl ::std::ops::Deref for Event {
            type Target = super::Event;

            #[inline]
            fn deref(&self) -> &Self::Target { &self.0 }
        }

        impl ::std::ops::DerefMut for Event {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }

        impl AsRef<super::Event> for Event {
            #[inline]
            fn as_ref(&self) -> &super::Event { self }
        }

        impl AsMut<super::Event> for Event {
            #[inline]
            fn as_mut(&mut self) -> &mut super::Event { self }
        }
    }
}
