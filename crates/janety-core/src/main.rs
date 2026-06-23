use janety_core::{parser::file_parser, run, semantics, transpiler};
use chumsky::Parser;

fn test_top_level_parsing() {
    let input = r#"
            (:: foldl-num (-> (-> number number number) number [number] number))
            (defn foldl-num [f acc l]
              (if (empty? l)
                acc
                (foldl-num f (f acc (first l)) (slice l 1))))

            (:: map-num (-> (-> number number) [number] [number]))
            (defn map-num [f l]
              (if (empty? l)
                []
                (tuple (f (first l)) (map-num f (slice l 1)))))

            (:: add (-> number number number))
            (defn add [x y]
              (+ x y))

            (:: add-ten (-> number number))
            (defn add-ten []
              (add 10))

            (:: process-data (-> [number] number))
            (defn process-data [l]
              (foldl-num add 0 (map-num add-ten l)))

            (:: process-more (-> [number] [number]))
            (defn process-more [l]
              (map-num (add 42) l))

            (print (process-data [1 2 3 4 5]))
            (print (process-more [1 2 3 4 5]))
            
            (+ 1 (+ 2 (+ 3 (+ 4 5))))
        "#;

    let parser = file_parser();
    let result = parser.parse(input);

    match result.into_result() {
        Ok(ast) => {
            if let Err(errors) = semantics::typeck::check_types(&ast) {
                for e in errors {
                    println!("Type Error: {}", e);
                }
            } else {
                println!("Type ok! Starting transpilation...\n");

                let janet_code = transpiler::transpile(&ast);
                println!("Generated Janet code : \n");
                println!("{}", janet_code);

                let resultat = run(&janet_code, "main");
                
                println!("\n=== RÉSULTAT CAPTURÉ PAR RUST ===");
                println!("Succès : {}", resultat.success);
                
                println!("\n[CONSOLE VIRTUELLE (ce qui a été imprimé via pp)]");
                if resultat.console_output.is_empty() {
                    println!("(Vide)");
                } else {
                    println!("{}", resultat.console_output);
                }

                println!("\n[VALEUR DE RETOUR (La dernière ligne évaluée)]");
                if let Some(valeur) = resultat.output {
                    println!("--{}--", valeur);
                } else {
                    println!("(Aucune valeur retournée)");
                }
                println!("=================================\n");
            }
        }
        Err(e) => println!("Parse Error: {:?}", e),
    }
}

fn main() {
    test_top_level_parsing();
}
