#![feature(type_alias_impl_trait)]

use core::marker::PhantomData;

type WithEmplacableForFn<'a> = impl EmplacableFn + 'a;

fn with_emplacable_for<'a, F, R>(mut f: F) -> R
where
    F: for<'b> FnMut(Emplacable<WithEmplacableForFn<'b>>) -> R,
{
    fn with_emplacable_for_inner<'a, R>(
        _: &'a (),
        _: &mut dyn FnMut(Emplacable<WithEmplacableForFn<'a>>) -> R,
    ) -> R {
        fn _constrain(_: &mut ()) -> WithEmplacableForFn<'_> {
            ()
        }
        loop {}
    }

    with_emplacable_for_inner(&(), &mut f)
}

trait EmplacableFn {}

impl EmplacableFn for () {}

struct Emplacable<F>
where
    F: EmplacableFn,
{
    phantom: PhantomData<F>,
}

fn main() {
    with_emplacable_for(|_| {});
}
