use std::collections::HashMap;
macro_rules! silly_chars {
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = HashMap::new();
            $(map.insert($key, $value);)*
            map
        }
    };
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    // Remove path of the executable from the vec
    if !args.is_empty() {
        println!("Path {:#?}", args.remove(0)) 
    }
    
    let characters = silly_chars![
"q" => "й","w" => "ц","e" => "у","r" => "к","t" => "е","y" => "н","u" => "г","i" => "ш","o" => "щ","p" => "з","[" => "х","]" => "ъ","a" => "ф","s" => "ы","d" => "в","f" => "а","g" => "п","h" => "р","j" => "о","k" => "л","l" => "д",";" => "ж","'" => "э","z" => "я","x" => "ч","c" => "с","v" => "м","b" => "и","n" => "т","m" => "ь","," => "б","." => "ю","/" => ".","`" => "ё","Q" => "Й","W" => "Ц","E" => "У","R" => "К","T" => "Е","Y" => "Н","U" => "Г","I" => "Ш","O" => "Щ","P" => "З","{" => "Х","}" => "Ъ","A" => "Ф","S" => "Ы","D" => "В","F" => "А","G" => "П","H" => "Р","J" => "О","K" => "Л","L" => "Д",":" => "Ж","\"" => "Э","Z" => "Я","X" => "Ч","C" => "С","V" => "М","B" => "И","N" => "Т","M" => "Ь","<" => "Б",">" => "Ю","?" => ",","~" => "Ё","-" => "-","=" => "=","_" => "_","+" => "+","!" => "!","@" => "\"","#" => "№","$" => ";","%" => "%","^" => ":","&" => "?","*" => "*","(" => "(",")" => ")","\\" => "/","|" => "\\"

    ];
    
    println!("Before: {:#?}", args);
    for text in args.iter_mut() {
        for (from, to) in &characters {
            *text = text.replace(from, to);
        }
    }
    println!("After: {:#?}", args);


}
