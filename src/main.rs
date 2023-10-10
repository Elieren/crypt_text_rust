use std::io::{self, Write};

mod inner {

    pub struct Encode {
        full: Vec<char>,
        text: String,
        base: Vec<String>
    }

    pub struct Create {
    }
    
    impl Encode {
        
        pub fn new(text: &str) -> Encode {
            let mut rus = vec!['А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ё', 'Ж', 'З',
               'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П', 'Р',
               'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ',
               'Ъ', 'Ы', 'Ь', 'Э', 'Ю', 'Я'];

            let mut eng = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G',
               'H', 'I', 'J', 'K', 'L', 'M', 'N',
               'O', 'P', 'Q', 'R', 'S', 'T', 'U',
               'V', 'W', 'X', 'Y', 'Z'];
            
            let mut numbers = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

            let mut probel = vec![' '];

            let mut znaki = vec![',', '.', '!', '?', '\"', '@', '№', '#', '$',
                        ';', '%', '^', ':', '&', '*', '(', ')', '-',
                        '_', '+', '=', '{', '}', '[', ']', '\'', '>',
                        '<', '/', '\\'];
            

            let mut full: Vec<char> = Vec::new();

            let mut new_rus: Vec<char> = Vec::new();
            for i in &rus {
                let upper_case: String = i.to_lowercase().collect();
                let i: char = {
                    upper_case.chars().next().unwrap()
                };
                new_rus.push(i);
            }
            rus.append(&mut new_rus);


            let mut new_eng: Vec<char> = Vec::new();
            for i in &eng {
                let upper_case: String = i.to_lowercase().collect();
                let i: char = {
                    upper_case.chars().next().unwrap()
                };
                new_eng.push(i);
            }
            eng.append(&mut new_eng);

            full.append(&mut rus);
            full.append(&mut eng);
            full.append(&mut numbers);
            full.append(&mut probel);
            full.append(&mut znaki);
            
            let base: Vec<String> = Self::__file_read();

            Encode { full, text: text.to_string(), base}
        }


        pub fn encode(&self) -> String {
            let code_list: String = Self::__encode(&self);
            let text_len: String = code_list.len().to_string();
            let number_var: String = Self::__encode_len(&self, text_len);
            let text: String = format!("{}|{}", code_list, number_var);
            text

        }


        pub fn decode(&self, length_key: u8) -> String {
            let text: Vec<String> = self.text.split('|').map(|s| s.to_string()).collect();

            let text_0: &String = &text[0];
            let len_text: String = text[0].len().to_string();
            let key: &String = &text[1];

            let text: String = Self::__decode(&self, text_0.clone(), len_text, key.clone(), length_key.clone());
            text


        }


        fn __encode(&self) -> String {
            let mut k: Vec<usize> = Vec::new();
            for tar in self.text.chars() {
                if let Some(index) = self.full.iter().position(|&x| x == tar) {
                    k.push(index)
                } else {
                    k.push(usize::MAX)
                }
            }

            let mut code_list: Vec<String> = Vec::new();
            for i in k.iter() {
                let y: &String = &self.base[*i];
                code_list.push(y.to_string());
            }
            code_list.join("")
        }

        fn __encode_len(&self, text_len: String) -> String {
            let mut k: Vec<usize> = Vec::new();

            for i in text_len.chars() {
                if let Some(index) = self.full.iter().position(|&x| x == i) {
                    k.push(index)
                } else {
                    k.push(usize::MAX)
                }
            }

            let mut code_numbers: Vec<String> = Vec::new();
            for i in k.iter() {
                let y: &String = &self.base[*i];
                code_numbers.push(y.to_string());
            }
            code_numbers.join("")

            }
        

        fn __decode(&self, text: String, len_text: String, key: String, length_key: u8) -> String{
            let length_key: usize = length_key as usize;
            let decode_key: Vec<_> = key.as_bytes().chunks(length_key).map(|chunk| String::from_utf8(chunk.to_vec()).unwrap()).collect();
            
            let mut numbers: String = String::new();
            for i in decode_key.iter() {
                if let Some(index) = self.base.iter().position(|x| x == i) {
                    numbers.push(self.full[index])
                } else {
                }
            }


            let mut text_decode: String = String::new();
            if len_text == numbers {
                let decode_text: Vec<_> = text.as_bytes().chunks(length_key).map(|chunk| String::from_utf8(chunk.to_vec()).unwrap()).collect();

                for i in decode_text.iter() {
                    if let Some(index) = self.base.iter().position(|x| x == i) {
                        text_decode.push(self.full[index])
                    } else {
                        
                    }
                }

            } else {
               text_decode.push_str("Error. The key doesn't fit.")
            }

            text_decode

        }


        fn __file_read() -> Vec<String> {
            // Читаем данные из файла
            use std::fs::File;
            use std::io::Read;
            
            let mut file = File::open("data.json").expect("Ошибка при открытии файла");
            let mut json_contents = String::new();
            file.read_to_string(&mut json_contents).expect("Ошибка при чтении данных из файла");

            // Десериализуем JSON обратно в вектор
            let vec2: Vec<String> = serde_json::from_str(&json_contents).expect("Ошибка при десериализации данных");

            // Выводим вектор
            vec2
        }

    
    }

    impl Create {
        pub fn new() {
            println!("+");
            let code: Vec<String> = Self::__crate_code();
            println!("{:?}", code);
            Self::__file_write(code);
        }

        fn __crate_code() -> Vec<String> {
            use rand::Rng;

            let mut key: Vec<String> = Vec::new();
            let length = 5;

            let letters_and_digits: Vec<u8> = (b'a'..=b'z').chain(b'A'..=b'Z').chain(b'0'..=b'9').collect();

            for _ in 0..163 {
                let rand_string: String = (0..length)
                    .map(|_| {
                        let idx = rand::thread_rng().gen_range(0..letters_and_digits.len());
                        letters_and_digits[idx] as char
                    })
                    .collect();

                key.push(rand_string);
            }

            // Выводим ключи
            key
        }

        fn __file_write(vec: Vec<String>) {
            use std::fs::File;
            use std::io::Write;

            // Сериализуем вектор в JSON
            let json_data = serde_json::to_string(&vec).expect("Ошибка при сериализации данных");

            // Записываем JSON данные в файл
            let mut file = File::create("data.json").expect("Ошибка при создании файла");
            file.write_all(json_data.as_bytes()).expect("Ошибка при записи данных в файл");

            println!("Данные сохранены в файл");
        }

    }

}


fn main() {

    let banner = "
███████╗ ██████╗██████╗  █████╗ ███╗   ███╗██████╗ ██╗     ███████╗██████╗
██╔════╝██╔════╝██╔══██╗██╔══██╗████╗ ████║██╔══██╗██║     ██╔════╝██╔══██╗
███████╗██║     ██████╔╝███████║██╔████╔██║██████╔╝██║     █████╗  ██████╔╝
╚════██║██║     ██╔══██╗██╔══██║██║╚██╔╝██║██╔══██╗██║     ██╔══╝  ██╔══██╗
███████║╚██████╗██║  ██║██║  ██║██║ ╚═╝ ██║██████╔╝███████╗███████╗██║  ██║
╚══════╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝     ╚═╝╚═════╝ ╚══════╝╚══════╝╚═╝  ╚═╝
    ";

    println!("{}", banner);

    print!("(S) - Scrambler, (D) - Decoder, (C) - Create key: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("Вы ввели: {}", input.trim());
    
    if input.trim() == "S".to_string() {
        print!(": ");
        io::stdout().flush().unwrap();

        let mut text: String = String::new();
        io::stdin().read_line(&mut text).unwrap();
        let text: &str = text.as_str().trim();
        let enc = inner::Encode::new(text);
        let code: String = enc.encode();
        println!("{}", code);
    } else if input.trim() == "D".to_string() {
        print!(": ");
        io::stdout().flush().unwrap();

        let mut text: String = String::new();
        io::stdin().read_line(&mut text).unwrap();
        let text: &str = text.as_str().trim();
        let dec = inner::Encode::new(text);
        let text: String = dec.decode(5);
        println!("{}", text);

    } else if input.trim() == "C".to_string() {
        inner::Create::new();
    } else {

    }
}