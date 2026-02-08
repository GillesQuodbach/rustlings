fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // Si optional_target est du motif Some(word) alors execute le bloc avec world lié
        // a la valeur
        // TODO: Make this an if-let statement whose value is `Some`.
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        // chiffre 10
        let range = 10;
        // vector d'Option initialisé a None
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        // pour l'interval de 1 a 10
        for i in 1..=range {
            // ajoute i dans le vector
            optional_integers.push(Some(i));
        }

        // valeur mutable = a range donc 10
        let mut cursor = range;

        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.
        // TANT QUE le methode pop du vector optional_integers est un integer
        // execute ce bloc
        while let Some(integer) = optional_integers.pop() {

            if let Some(integer) = integer {
                // on verifie que integer et = a cursor
                assert_eq!(integer, cursor);
                // on decremente cursor de 1
                cursor -= 1;
            }

        }

        assert_eq!(cursor, 0);
    }
}
