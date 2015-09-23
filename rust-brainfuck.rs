//@ympons
trait Runnable {
    fn run(&mut self);
}

struct Engine {
    source: Vec<char>,
    ip: usize,
    tape: Vec<u8>,
    p: usize,
}

fn readchar() -> Option<char> {
    use std::io::Read;

    let input: Option<char> = std::io::stdin()
        .bytes() 
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as char);
    return input;
}

impl Runnable for Engine {
    fn run(&mut self) {
        while self.source.get(self.ip)!=None {
            let op = self.source[self.ip];
            self.ip += 1;
            match op {
                '>' => {
                    self.p += 1;
                },
                '<' => {
                    self.p -= 1;
                },
                '+' => {
                    self.tape[self.p] += 1;
                },
                '-' => {
                    self.tape[self.p] -= 1;
                },
                '.' => {
                    let item = self.tape[self.p] as char;
                    print!("{}", item);
                },
                ',' => {
                    match readchar() {
                        Some(item) => {
                            self.tape[self.p] = if item == '\x04' { 0 } else { item as u8 };
                        },
                        _ => {},
                    };
                },
                '[' => {
                    if self.tape[self.p] == 0 {
                        let mut nesting = 1;
                        while nesting != 0 {
                            let item = self.source[self.ip];
                            self.ip += 1;
                            if item == '[' { nesting += 1; }
                            if item == ']' { nesting -= 1; }
                        }
                    }    
                },
                ']' => {
                    self.ip -= 1;
                    let mut nesting = 1;
                    while nesting != 0 {
                        self.ip -= 1;
                        let item = self.source[self.ip];
                        if item == ']' { nesting += 1; }
                        if item == '[' { nesting -= 1; }
                    }                
                },
                _ => { break;}
            };      
        }
    }
}

fn main() {
  let code = ">+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.>>>++++++++[<++++>-]<.>>>++++++++++[<+++++++++>-]<---.<<<<.+++.------.--------.>>+.";
  let mut e = Engine{source: code.chars().collect(), ip: 0, p: 0, tape: vec![0; 3000]};
  e.run(); 
}
