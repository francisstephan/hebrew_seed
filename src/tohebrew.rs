use std::collections::HashMap; 

lazy_static! {
    static ref LATIVR: HashMap<char, char> = {
        let lat: Vec<char> = vec!('\'','b','g','d','h','w','z','H','T','y','k','l','m','n','c','A','p','S','q','r','s','t');
        let ivr: Vec<char> = vec!('\u{05D0}','\u{05D1}','\u{05D2}','\u{05D3}','\u{05D4}','\u{05D5}','\u{05D6}','\u{05D7}','\u{05D8}','\u{05D9}','\u{05DB}','\u{05DC}','\u{05DE}','\u{05E0}','\u{05E1}','\u{05E2}','\u{05E4}','\u{05E6}','\u{05E7}','\u{05E8}','\u{05E9}','\u{05EA}');
        let map: HashMap<char,char> = lat.into_iter().zip(ivr).into_iter().collect();
        map
    };

    static ref LATIVR2: HashMap<char, char> = {
        let lat: Vec<char> = vec!('k','m','n','p','S');
        let ivr: Vec<char> = vec!('\u{05DA}','\u{05DD}','\u{05DF}','\u{05E3}','\u{05E5}');
        let map: HashMap<char,char> = lat.into_iter().zip(ivr).into_iter().collect();
        map
    };
}

fn subst(c:char, m: &HashMap<char,char>)->char{
    match m.get(&c) {
        Some(car) => *car,
        None => c 
    }
}
fn subgen(c:char,suc:char) -> char{
    match suc {
        ' ' =>  match &LATIVR2.get(&c) {
                    Some(car) => **car,
                    None => subst(c, &LATIVR)
                },
        _   => subst(c, &LATIVR)
     }
}

fn szip(chaine : &str)-> Vec<(char,char)>{
    let char_vec: Vec<char> = chaine.chars().collect();
    let mut v0 : Vec<char> = (&char_vec[1..]).to_vec();
    v0.push(' ');
    // println!("v0={:?}",v0);
    char_vec.into_iter().zip(v0).into_iter().collect()
}

pub fn transl(chaine : &str) -> String{
    if chaine.len() == 0  { return "".to_string() ;}
    let couples : Vec<(char,char)> = szip(chaine);
    couples.iter().map(|(k,v)| subgen(*k,*v)).collect()
}

