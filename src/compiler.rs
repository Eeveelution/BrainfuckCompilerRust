pub mod compiler {
    use std::{fmt::format, fs::{self, File}, io::Write};

    use crate::CompilerArguments;

    struct Entry {
        operation: char,
        amount: i32,
    }

    pub fn compile(args: CompilerArguments) {
        let _input_contents: String = fs::read_to_string(args.filename)
            .expect("Failed to read input file!");
        
        let mut entries: Vec<Entry> = vec![];

        let mut count_right: i32 = 0; 
        let mut count_left: i32 = 0;

        let mut out_file: String = format!("int main() {{ unsigned char buf[<.$buffer_size>]; int ptr = 0;");

        let mut last_char: char = '\r';
        let mut op_amount: i32 = 0;

        println!("Log:: Parsing Operations..");

        for op in _input_contents.chars() {
            //Special case for > and <, as they affect $buffer_size

            if op == '>' {
                count_right += 1;
            }
            else if op == '<' {
                count_left += 1;
            }

            if op != last_char && last_char != '\r' {
                let new_entry= Entry {
                    operation: last_char,
                    amount: op_amount
                };

                entries.push(new_entry);

                op_amount = 1;
            } else {
                op_amount += 1;
            }

            last_char = op;
        }

        let new_entry= Entry {
            operation: last_char,
            amount: op_amount
        };

        entries.push(new_entry);

        println!("Log:: Figuring out Buffer Size..");

        let buffer_size = 1024;

        out_file = out_file.replace("<.$buffer_size>", &buffer_size.to_string().to_owned());

        println!("Log:: Writing Operations..");

        for entry in entries {
            if entry.operation == '+' {

                out_file.push_str(
                    format!("{}{};", "buf[ptr] += ", entry.amount)
                    .as_str()
                );
            } 
            else if entry.operation == '-' {
                out_file.push_str(
                    format!("{}{};", "buf[ptr] -= ", entry.amount)
                    .as_str()
                );
            } 
            else if entry.operation == '>' {
                out_file.push_str(
                    format!("{}{};", "ptr += ", entry.amount)
                    .as_str()
                );
            } 
            else if entry.operation == '<' {
                out_file.push_str(
                    format!("{}{};", "ptr -= ", entry.amount)
                    .as_str()
                );
            } 
            else if entry.operation == '[' {
                for i in 0..entry.amount {
                    out_file.push_str("while(buf[ptr] != 0){");
                }
            } 
            else if entry.operation == ']' {
                for i in 0..entry.amount {
                    out_file.push_str("}");
                }
            } 
            else if entry.operation == '.' {
                for i in 0..entry.amount {
                    out_file.push_str("putchar(buf[ptr]);");
                }
            } 
            else if entry.operation == '.' {
                for i in 0..entry.amount {
                    out_file.push_str("buf[ptr]=getchar();");
                }
            }
        }

        out_file = format!("{}{}", out_file, "return 0;}");

        println!("Log:: Writing File..");

        let mut file = File::create(args.out_filename)
            .expect("Failed to create output file!");

        file.write_all(out_file.as_bytes())
            .expect("Faile to write output to File!");
    }
}