use std::println;
extern crate bincode;
extern crate byteorder;
extern crate serde;

use bincode::Infinite;
use byteorder::BigEndian;
use serde::{Serialize, Deserialize};

// https://implement-dns.wizardzines.com/book/part_1.html

#[derive(Debug)]
struct DNSHeader {
    id: i16,
    flags: i16,
    num_questions: i16,
    num_answers: i16,
    num_authorities: i16,
    num_additional: i16,
}

impl DNSHeader {
    fn new(id: i16, flags: i16, num_questions: i16, num_answers: i16, num_authorities: i16, num_additional: i16) -> DNSHeader {
        return DNSHeader{
            id: id,
            flags: flags,
            num_questions: num_questions,
            num_answers: num_answers,
            num_authorities: num_authorities,
            num_additional: num_additional
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let fields = self.to_tuple();
        println!("{:?}", fields);
        let vec = pack(&fields);
        println!("{:?}", vec);

        let mut byte_literal = String::new();
        // if we don't use &, the loop takes control of vec and moves the vale
        for b in &vec {
            byte_literal.push_str(&format!("\\x{:02X}", b));
        }
    
        println!("Packed: b'{}", byte_literal); 

        return vec;
    }

    fn to_tuple(&self) -> (i16,i16,i16,i16,i16,i16){
        return (self.id, self.flags, self.num_questions, self.num_answers, self.num_authorities, self.num_additional);
    }

    // fn to_array(&self) -> [i32; 6] {
    //     return [self.id, self.flags, self.num_questions, self.num_answers, self.num_authorities, self.num_additional];
    // }

}

fn pack<T: Serialize>(data: &T) -> Vec<u8> {
    use bincode::endian_choice::serialize;
    let bytes = serialize::<_, _, BigEndian>(data, Infinite);
    return bytes.unwrap();
}

fn unpack<T: Deserialize>(bytes: &[u8]) -> T {
    use bincode::endian_choice::deserialize;
    let data = deserialize::<_, BigEndian>(bytes);
    return data.unwrap();
}

fn encode_dns_name(domain: &str) -> Vec<u8> {
    let mut encoded = Vec::new();
    for label in domain.split('.') {
        encoded.push(label.len() as u8);
        encoded.extend_from_slice(label.as_bytes());
    }
    encoded.push(0);

    // for byte in &encoded {
    //     print!("\\x{:02X}", byte);
    // }
    // println!("");

    return encoded;
}

fn build_query(domain: &str) -> Vec<u8> {
    let domain_encoded = encode_dns_name(domain);
    let id = random.randint(0, 65535);
    let RECURSION_DESIRED = 1 << 8
    let header = DNSHeader::new(4884, 0, 1, 0, 0, 0);
}

#[derive(Debug)]
struct DNSQuestion {
    name: u8,
    type_: i32,
    class_: i32
}

impl DNSQuestion {
    fn to_bytes(&self) -> Vec<u8> {
        //     return question.name + struct.pack("!HH", question.type_, question.class_)
        return Vec::new();
    }

    fn to_tuple(&self) -> (u8,i32,i32) {
        return (self.name, self.type_, self.class_);
    }
}



fn main() {
    let header = DNSHeader::new(4884, 0, 1, 0, 0, 0);
    header.to_bytes();

    let domain = "google.com";
    let encoded = encode_dns_name(domain);
    println!("{:?}", encoded);
}
