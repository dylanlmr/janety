use chumsky::{error::Error as ChumskyError, input::Input, prelude::*};
use std::fmt::Debug;

#[allow(dead_code)]
pub trait ParserTestExt<'src, I, O, E>
where
    I: Input<'src>,
    E: ChumskyError<'src, I>,
{
    fn assert_parse(&self, given: I, expected: O) -> O
    where
        O: Debug + PartialEq,
        E: Debug;

    fn assert_err_eq(&self, given: I, expected_errors: Vec<E>)
    where
        O: Debug,
        E: Debug + PartialEq;

    fn assert_fails(&self, given: I)
    where
        O: Debug,
        E: Debug;
}

impl<'src, I, O, E, P> ParserTestExt<'src, I, O, E> for P
where
    P: Parser<'src, I, O, extra::Err<E>>,
    I: Input<'src> + Clone,
    E: ChumskyError<'src, I>,
{
    fn assert_parse(&self, given: I, expected: O) -> O
    where
        O: Debug + PartialEq,
        E: Debug,
    {
        match self.parse(given).into_result() {
            Ok(output) => {
                assert_eq!(output, expected, "Mismatch sur le résultat attendu.");
                output
            }
            Err(errors) => panic!("Échec inattendu du parsing !\n{:#?}", errors),
        }
    }

    fn assert_err_eq(&self, given: I, expected_errors: Vec<E>)
    where
        O: Debug,
        E: Debug + PartialEq,
    {
        match self.parse(given).into_result() {
            Ok(output) => panic!(
                "Le parsing a réussi alors qu'il devait échouer !\nSortie: {:#?}",
                output
            ),
            Err(errors) => assert_eq!(
                errors, expected_errors,
                "Les erreurs ne correspondent pas à ce qui était attendu."
            ),
        }
    }

    fn assert_fails(&self, given: I)
    where
        O: Debug,
        E: Debug,
    {
        if let Ok(output) = self.parse(given).into_result() {
            panic!(
                "Le parsing a réussi alors qu'il devait échouer !\nSortie: {:#?}",
                output
            );
        }
    }
}
