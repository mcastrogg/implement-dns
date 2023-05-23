from dataclasses import dataclass
import dataclasses
import struct

@dataclass
class DNSHeader:
    id: int
    flags: int
    num_questions: int = 0
    num_answers: int = 0
    num_authorities: int = 0
    num_additionals: int = 0

@dataclass
class DNSQuestion:
    name: bytes
    type_: int 
    class_: int 

def header_to_bytes(header):
    print(f"Header: {header}")
    fields = dataclasses.astuple(header)
    print(f"Fields: {fields}")
    # there are 6 `H`s because there are 6 fields
    b = struct.pack("!HHHHHH", *fields)
    print(f"Packed: {b}")
    return b

def question_to_bytes(question):
    return question.name + struct.pack("!HH", question.type_, question.class_)


def main():
    header_to_bytes(DNSHeader(id=0x1314, flags=0, num_questions=1, num_additionals=0, num_authorities=0, num_answers=0))





if __name__ == "__main__":
    main()