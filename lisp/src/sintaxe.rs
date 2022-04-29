pub fn parenteses_fechados(linha: &str) {
    let parenteses: i32 = 
        linha.split("")
             .filter(|x| x == &"(" || x == &")")
             .map(|x| {
                if x == "(" { 1 }
                else { -1 }
             })
             .sum();
    if parenteses != 0 {
        println!("Sintaxe error");
    }
    
}