fn main(){
    let sentence:String =  String::from("My name is Paarish");
    let first_word:String = get_first_word(sentence);
    println!("First word is: {}",first_word);

    fn get_first_word(sentence:String) -> String{
        let mut ans = String::from("");
        for char in sentence.chars(){
            ans.push_str(&char.to_string());
            if char == ' '{
                break;
            }
        }
        return ans;
    }
}