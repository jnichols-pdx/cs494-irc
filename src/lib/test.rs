use super::*;

#[test]
fn reject_name_chars() {
    assert!(valid_name(&"blah".to_string()).is_ok());  //text is ok

    //ascii low control chars are not
    assert!(valid_name(&"bla\x00h".to_string()).is_err());
    assert!(valid_name(&"bla\x01h".to_string()).is_err());
    assert!(valid_name(&"bla\x02h".to_string()).is_err());
    assert!(valid_name(&"bla\x03h".to_string()).is_err());
    assert!(valid_name(&"bla\x04h".to_string()).is_err());
    assert!(valid_name(&"bla\x05h".to_string()).is_err());
    assert!(valid_name(&"bla\x06h".to_string()).is_err());
    assert!(valid_name(&"bla\x07h".to_string()).is_err());
    assert!(valid_name(&"bla\x08h".to_string()).is_err());
    assert!(valid_name(&"bla\x09h".to_string()).is_err());
    assert!(valid_name(&"bla\x0Ah".to_string()).is_err());
    assert!(valid_name(&"bla\x0Bh".to_string()).is_err());
    assert!(valid_name(&"bla\x0Ch".to_string()).is_err());
    assert!(valid_name(&"bla\x0Dh".to_string()).is_err());
    assert!(valid_name(&"bla\x0Eh".to_string()).is_err());
    assert!(valid_name(&"bla\x0Fh".to_string()).is_err());
    assert!(valid_name(&"bla\x10h".to_string()).is_err());
    assert!(valid_name(&"bla\x11h".to_string()).is_err());
    assert!(valid_name(&"bla\x12h".to_string()).is_err());
    assert!(valid_name(&"bla\x13h".to_string()).is_err());
    assert!(valid_name(&"bla\x14h".to_string()).is_err());
    assert!(valid_name(&"bla\x15h".to_string()).is_err());
    assert!(valid_name(&"bla\x16h".to_string()).is_err());
    assert!(valid_name(&"bla\x17h".to_string()).is_err());
    assert!(valid_name(&"bla\x18h".to_string()).is_err());
    assert!(valid_name(&"bla\x19h".to_string()).is_err());
    assert!(valid_name(&"bla\x1Ah".to_string()).is_err());
    assert!(valid_name(&"bla\x1Bh".to_string()).is_err());
    assert!(valid_name(&"bla\x1Ch".to_string()).is_err());
    assert!(valid_name(&"bla\x1Dh".to_string()).is_err());
    assert!(valid_name(&"bla\x1Eh".to_string()).is_err());
    assert!(valid_name(&"bla\x1Fh".to_string()).is_err());


    //spaces are NOT ok
    assert!(valid_name(&"bla h".to_string()).is_err());
    assert!(valid_name(&"bla\x20h".to_string()).is_err());

    //bidi go byebye
    assert!(valid_name(&"bla\u{061C}h".to_string()).is_err());

    assert!(valid_name(&"bla\u{200E}h".to_string()).is_err());
    assert!(valid_name(&"bla\u{200F}h".to_string()).is_err());

    assert!(valid_name(&"bla\u{202A}h".to_string()).is_err());
    assert!(valid_name(&"bla\u{202B}h".to_string()).is_err());
    assert!(valid_name(&"bla\u{202C}h".to_string()).is_err());
    assert!(valid_name(&"bla\u{202D}h".to_string()).is_err());
    assert!(valid_name(&"bla\u{202E}h".to_string()).is_err());

    assert!(valid_name(&"bla\u{2066}h".to_string()).is_err());
    assert!(valid_name(&"bla\u{2067}h".to_string()).is_err());
    assert!(valid_name(&"bla\u{2068}h".to_string()).is_err());
    assert!(valid_name(&"bla\u{2069}h".to_string()).is_err());
}

#[test]
fn reject_name_length() {
    assert!(valid_name(&"hunter2".to_string()).is_ok());  //short names are OK

    assert!(valid_name(&"abcdefghijklmnopqrstuvwxyz1234567890".to_string()).is_err()); //long names are not

    assert!(valid_name(&"12345678901234567890123456789012".to_string()).is_ok()); //max length is OK
    assert!(valid_name(&"123456789012345678901234567890123".to_string()).is_err()); //one more is not
    assert!(valid_name(&"".to_string()).is_err()); //empty strings are not
    assert!(valid_name(&"123456789012345678901234567890™™".to_string()).is_ok()); //multibyte unicode count as one and are OK
    assert!(valid_name(&"123456789012345678901234567890™™™".to_string()).is_err()); //multibyte unicode count as one, but may still push us over the limit.

    assert!(valid_name(&"™™™™™™™™™™™™™™™™™™™™™".to_string()).is_ok()); //63 bytes is OK
    assert!(valid_name(&"™™™™™™™™™™™™™™™™™™™™™A".to_string()).is_ok()); //64 bytes is OK
    assert!(valid_name(&"™™™™™™™™™™™™™™™™™™™™™AB".to_string()).is_err()); //65 bytes is NOT ok
    assert!(valid_name(&"™™™™™™™™™™™™™™™™™™™™™™".to_string()).is_err()); //66 bytes is NOT ok 


}

#[test]
fn reject_file_name_chars() {
    assert!(valid_filename(&"blah".to_string()).is_ok());  //text is ok

    //ascii low control chars are not
    assert!(valid_filename(&"bla\x00h".to_string()).is_err());
    assert!(valid_filename(&"bla\x01h".to_string()).is_err());
    assert!(valid_filename(&"bla\x02h".to_string()).is_err());
    assert!(valid_filename(&"bla\x03h".to_string()).is_err());
    assert!(valid_filename(&"bla\x04h".to_string()).is_err());
    assert!(valid_filename(&"bla\x05h".to_string()).is_err());
    assert!(valid_filename(&"bla\x06h".to_string()).is_err());
    assert!(valid_filename(&"bla\x07h".to_string()).is_err());
    assert!(valid_filename(&"bla\x08h".to_string()).is_err());
    assert!(valid_filename(&"bla\x09h".to_string()).is_err());
    assert!(valid_filename(&"bla\x0Ah".to_string()).is_err());
    assert!(valid_filename(&"bla\x0Bh".to_string()).is_err());
    assert!(valid_filename(&"bla\x0Ch".to_string()).is_err());
    assert!(valid_filename(&"bla\x0Dh".to_string()).is_err());
    assert!(valid_filename(&"bla\x0Eh".to_string()).is_err());
    assert!(valid_filename(&"bla\x0Fh".to_string()).is_err());
    assert!(valid_filename(&"bla\x10h".to_string()).is_err());
    assert!(valid_filename(&"bla\x11h".to_string()).is_err());
    assert!(valid_filename(&"bla\x12h".to_string()).is_err());
    assert!(valid_filename(&"bla\x13h".to_string()).is_err());
    assert!(valid_filename(&"bla\x14h".to_string()).is_err());
    assert!(valid_filename(&"bla\x15h".to_string()).is_err());
    assert!(valid_filename(&"bla\x16h".to_string()).is_err());
    assert!(valid_filename(&"bla\x17h".to_string()).is_err());
    assert!(valid_filename(&"bla\x18h".to_string()).is_err());
    assert!(valid_filename(&"bla\x19h".to_string()).is_err());
    assert!(valid_filename(&"bla\x1Ah".to_string()).is_err());
    assert!(valid_filename(&"bla\x1Bh".to_string()).is_err());
    assert!(valid_filename(&"bla\x1Ch".to_string()).is_err());
    assert!(valid_filename(&"bla\x1Dh".to_string()).is_err());
    assert!(valid_filename(&"bla\x1Eh".to_string()).is_err());
    assert!(valid_filename(&"bla\x1Fh".to_string()).is_err());


    //spaces are ok inside the file name
    assert!(valid_filename(&"blah blah".to_string()).is_ok());
    assert!(valid_filename(&"blah\x20blah".to_string()).is_ok());

    //spaces are NOT ok at the start or end
    assert!(valid_filename(&"blah ".to_string()).is_err());
    assert!(valid_filename(&" blah".to_string()).is_err());


    //bidi go byebye
    assert!(valid_filename(&"bla\u{061C}h".to_string()).is_err());

    assert!(valid_filename(&"bla\u{200E}h".to_string()).is_err());
    assert!(valid_filename(&"bla\u{200F}h".to_string()).is_err());

    assert!(valid_filename(&"bla\u{202A}h".to_string()).is_err());
    assert!(valid_filename(&"bla\u{202B}h".to_string()).is_err());
    assert!(valid_filename(&"bla\u{202C}h".to_string()).is_err());
    assert!(valid_filename(&"bla\u{202D}h".to_string()).is_err());
    assert!(valid_filename(&"bla\u{202E}h".to_string()).is_err());

    assert!(valid_filename(&"bla\u{2066}h".to_string()).is_err());
    assert!(valid_filename(&"bla\u{2067}h".to_string()).is_err());
    assert!(valid_filename(&"bla\u{2068}h".to_string()).is_err());
    assert!(valid_filename(&"bla\u{2069}h".to_string()).is_err());
}

#[test]
fn reject_file_name_length() {
    assert!(valid_filename(&"hunter2".to_string()).is_ok());  //short filenames are OK
    assert!(valid_filename(&"".to_string()).is_err()); //empty filenames are not

    //filenames over 1024 bytes are NOT ok.
    assert!(valid_filename(&"™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™XY".to_string()).is_err()); 

    //filenames up to and including 1024 bytes in length are OK
    assert!(valid_filename(&"™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™X".to_string()).is_ok()); //long names are not
}


#[test]
fn reject_message_chars() {
    assert!(valid_message(&"blah\0".to_string()).is_ok());  //text is ok

    //ascii low control chars are not
    assert!(valid_message(&"bla\x00h\0".to_string()).is_err()); //early null bytes are not permitted
    assert!(valid_message(&"bla\x01h\0".to_string()).is_err());
    assert!(valid_message(&"bla\x02h\0".to_string()).is_err());
    assert!(valid_message(&"bla\x03h\0".to_string()).is_err());
    assert!(valid_message(&"bla\x04h\0".to_string()).is_err());
    assert!(valid_message(&"bla\x05h\0".to_string()).is_err());
    assert!(valid_message(&"bla\x06h\0".to_string()).is_err());
    assert!(valid_message(&"bla\x07h\0".to_string()).is_err());
    assert!(valid_message(&"bla\x08h\0".to_string()).is_err());
    assert!(valid_message(&"bla\x09h\0".to_string()).is_ok()); //Tabs are allowed in messages
    assert!(valid_message(&"bla\x0Ah\0".to_string()).is_err());
    assert!(valid_message(&"bla\x0Bh\0".to_string()).is_err());
    assert!(valid_message(&"bla\x0Ch\0".to_string()).is_err());
    assert!(valid_message(&"bla\x0Dh\0".to_string()).is_err());
    assert!(valid_message(&"bla\x0Eh\0".to_string()).is_err());
    assert!(valid_message(&"bla\x0Fh\0".to_string()).is_err());
    assert!(valid_message(&"bla\x10h\0".to_string()).is_err());
    assert!(valid_message(&"bla\x11h\0".to_string()).is_err());
    assert!(valid_message(&"bla\x12h\0".to_string()).is_err());
    assert!(valid_message(&"bla\x13h\0".to_string()).is_err());
    assert!(valid_message(&"bla\x14h\0".to_string()).is_err());
    assert!(valid_message(&"bla\x15h\0".to_string()).is_err());
    assert!(valid_message(&"bla\x16h\0".to_string()).is_err());
    assert!(valid_message(&"bla\x17h\0".to_string()).is_err());
    assert!(valid_message(&"bla\x18h\0".to_string()).is_err());
    assert!(valid_message(&"bla\x19h\0".to_string()).is_err());
    assert!(valid_message(&"bla\x1Ah\0".to_string()).is_err());
    assert!(valid_message(&"bla\x1Bh\0".to_string()).is_err());
    assert!(valid_message(&"bla\x1Ch\0".to_string()).is_err());
    assert!(valid_message(&"bla\x1Dh\0".to_string()).is_err());
    assert!(valid_message(&"bla\x1Eh\0".to_string()).is_err());
    assert!(valid_message(&"bla\x1Fh\0".to_string()).is_err());


    //spaces are ok
    assert!(valid_message(&"bla h\0".to_string()).is_ok());
    assert!(valid_message(&"bla\x20h\0".to_string()).is_ok());

    //bidi go byebye
    assert!(valid_message(&"bla\u{061C}h\0".to_string()).is_err());

    assert!(valid_message(&"bla\u{200E}h\0".to_string()).is_err());
    assert!(valid_message(&"bla\u{200F}h\0".to_string()).is_err());

    assert!(valid_message(&"bla\u{202A}h\0".to_string()).is_err());
    assert!(valid_message(&"bla\u{202B}h\0".to_string()).is_err());
    assert!(valid_message(&"bla\u{202C}h\0".to_string()).is_err());
    assert!(valid_message(&"bla\u{202D}h\0".to_string()).is_err());
    assert!(valid_message(&"bla\u{202E}h\0".to_string()).is_err());

    assert!(valid_message(&"bla\u{2066}h\0".to_string()).is_err());
    assert!(valid_message(&"bla\u{2067}h\0".to_string()).is_err());
    assert!(valid_message(&"bla\u{2068}h\0".to_string()).is_err());
    assert!(valid_message(&"bla\u{2069}h\0".to_string()).is_err());
}

#[test]
fn reject_message_length() {
    assert!(valid_message(&"I'd like to introduce myself. Hello!\0".to_string()).is_ok());  //short messages are OK

    assert!(valid_message(&"".to_string()).is_err()); //empty messages are not

    //1200 bytes (399 three byte code points, two more single byte code points and a null) are OK.
    assert!(valid_message(&"™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™AB\0".to_string()).is_ok());



    //1201 bytes (400 three byte code points,  a null) are NOT ok.
    assert!(valid_message(&"™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™\0".to_string()).is_err());



}


#[test]
fn new_client_packet_from_bytes() {
    let mut bytes_good = BytesMut::with_capacity(69);
    bytes_good.put_u8( IrcKind::IRC_KIND_NEW_CLIENT as u8);
    bytes_good.put_u32(64);
    bytes_good.put_slice("Bobtato".as_bytes());
    let remain = 64 - "Bobtato".len();
    for x in 1..remain+1 {
        println!("{}",x);
        bytes_good.put_u8(b'\0');
    }

    let ncp_good = NewClientPacket::from_bytes(&bytes_good);
    assert!(ncp_good.is_ok());
    /*(let res = match ncp_good {
      Ok(x) => Ok(x),
      Err(e) => {println!("{:?}",e); Err(e)}
      };
      assert!(res.is_ok());*/



    let mut bytes_short = BytesMut::with_capacity(69);
    bytes_short.put_u8( IrcKind::IRC_KIND_NEW_CLIENT as u8);
    bytes_short.put_u32(64);
    bytes_short.put_slice("Bobtato".as_bytes());
    let remain = 60 - "Bobtato".len(); //TOO SHORT
    for x in 1..remain+1 {
        println!("{}",x);
        bytes_short.put_u8(b'\0');
    }

    let ncp_bad_short = NewClientPacket::from_bytes(&bytes_short);
    assert!(ncp_bad_short.is_err());
    if let Err(e) = ncp_bad_short {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketLengthIncorrect(65,69) => true, _ => false });
    };


    let mut bytes_lenf= BytesMut::with_capacity(69);
    bytes_lenf.put_u8( IrcKind::IRC_KIND_NEW_CLIENT as u8);
    bytes_lenf.put_u32(30); //wrong length field value
    bytes_lenf.put_slice("Bobtato".as_bytes());
    let remain = 64 - "Bobtato".len(); 
    for x in 1..remain+1 {
        println!("{}",x);
        bytes_lenf.put_u8(b'\0');
    }

    let ncp_bad_short = NewClientPacket::from_bytes(&bytes_lenf);
    assert!(ncp_bad_short.is_err());
    if let Err(e) = ncp_bad_short {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::FieldLengthIncorrect() => true, _ => false });
    };

    let mut bytes_mismatch= BytesMut::with_capacity(69);
    bytes_mismatch.put_u8( IrcKind::IRC_KIND_ENTER_ROOM as u8); //wrong type
    bytes_mismatch.put_u32(64); 
    bytes_mismatch.put_slice("Bobtato".as_bytes());
    let remain = 64 - "Bobtato".len(); 
    for x in 1..remain+1 {
        println!("{}",x);
        bytes_mismatch.put_u8(b'\0');
    }

    let ncp_bad_short = NewClientPacket::from_bytes(&bytes_mismatch);
    assert!(ncp_bad_short.is_err());
    if let Err(e) = ncp_bad_short {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketMismatch() => true, _ => false });
        //assert_eq!(e, IrcError::PacketMismatch());
    };
}

#[test]
fn new_client_packet_as_bytes() {
    let ncp = NewClientPacket::new(&"ExampleName".to_string()).unwrap();
    assert_eq!(ncp.as_bytes(), Bytes::from_static(b"\x02\0\0\0\x40ExampleName\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"));
}

