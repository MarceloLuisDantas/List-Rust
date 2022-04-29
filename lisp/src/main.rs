mod sintaxe;

fn main() {
    sintaxe::parenteses_fechados("(sub (sum 2 (sub 1 2)) (mult 2 3))");
    sintaxe::parenteses_fechados("(su b s(um 2 (sub 1 2)) (mult 2 3))");
}
