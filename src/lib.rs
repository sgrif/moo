use std::borrow::Cow;

pub trait Moo {
    fn moo(&self) -> ! {
        panic!("Moooooooooo!")
    }
}

impl<'a, T> Moo for Cow<'a, T>
where
    T: ToOwned + ?Sized
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Moooooooooo!")]
    fn it_moos() {
        Cow::Borrowed("lol").moo()
    }
}
