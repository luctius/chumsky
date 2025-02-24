use super::*;

impl<'a, 'b, T, I, O, E> ParserSealed<'a, I, O, E> for &'b T
where
    T: Parser<'a, I, O, E>,
    I: Input<'a>,
    E: ParserExtra<'a, I>,
{
    fn go<M: Mode>(&self, inp: &mut InputRef<'a, '_, I, E>) -> PResult<M, O>
    where
        Self: Sized,
    {
        (*self).go::<M>(inp)
    }

    go_extra!(O);
}

impl<'a, 'b, T, I, O, E> ConfigParserSealed<'a, I, O, E> for &'b T
where
    T: ConfigParser<'a, I, O, E>,
    I: Input<'a>,
    E: ParserExtra<'a, I>,
{
    type Config = T::Config;

    fn go_cfg<M: Mode>(&self, inp: &mut InputRef<'a, '_, I, E>, cfg: Self::Config) -> PResult<M, O>
    where
        Self: Sized,
    {
        (*self).go_cfg::<M>(inp, cfg)
    }

    go_cfg_extra!(O);
}

impl<'a, I, O, E, P> Parser<'a, I, O, E> for P
where
    I: Input<'a>,
    E: ParserExtra<'a, I>,
    P: ParserSealed<'a, I, O, E>,
{
}

impl<'a, I, O, E, P> ConfigParser<'a, I, O, E> for P
where
    I: Input<'a>,
    E: ParserExtra<'a, I>,
    P: ConfigParserSealed<'a, I, O, E>,
{
}

impl<'a, I, O, E, P> IterParser<'a, I, O, E> for P
where
    I: Input<'a>,
    E: ParserExtra<'a, I>,
    P: IterParserSealed<'a, I, O, E>,
{
}

impl<'a, I, O, E, P> ConfigIterParser<'a, I, O, E> for P
where
    I: Input<'a>,
    E: ParserExtra<'a, I>,
    P: ConfigIterParserSealed<'a, I, O, E>,
{
}
