// // The problem is to loop through a string and print out the number of 
// // times it appears before the letter. For example, aaabbbcccdd=3a3b3c2d

pub fn run() {
        let test = String::from("aaabbcc");
        let test2= String:: from("RReeesssqq");
        println!("{}", encrypt(test));
        println!("{}", encrypt(test2));
    }
    
    fn encrypt(word: String) -> String {
        let mut result = String::new(); //empty growable string to hold variable
        let mut count = 1;
        //Assigns previous letter as the first letter in the beggining
        let mut previous_letter = word.chars().next(); 
    
        for letter in word.chars().skip(1) {
            if let Some(p) = previous_letter {
                if p == letter {
                    count += 1;
                } else {
                    result.push_str(&count.to_string());
                    result.push(p);
                    count = 1;
                }
                previous_letter = Some(letter);
                
            } else {
                println!("The string is empty");
                return result;
            };
        }
    
        if let Some(p)=previous_letter {
            result.push_str(&count.to_string());
            result.push(p);
        }
    
        result
        
       
    }
    
    // //aaabbcc=3a2b2c
    