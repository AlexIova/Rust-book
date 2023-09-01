fn main() {
    // let y = 6;      // statemant
    // let x = (let y = 6);    // questo non compila perché let non ritorna un valore come in C

    let y = {
        let x = 3;
        x + 1           // questo ritorna il valore 4 che verrà poi dato come risultato ad y. NOTA: non ha il `;`
        // Senza il `;` è una espressione, con il `;` invece è una istruzione
    };

    println!("The value of y is: {y}");

}

