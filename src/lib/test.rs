use super::*;

#[test]
fn name_chars() {
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
fn name_length() {
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
fn file_name_chars() {
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
fn file_name_length() {
    assert!(valid_filename(&"hunter2".to_string()).is_ok());  //short filenames are OK
    assert!(valid_filename(&"".to_string()).is_err()); //empty filenames are not

    //filenames over 1024 bytes are NOT ok.
    assert!(valid_filename(&"™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™XY".to_string()).is_err()); 

    //filenames up to and including 1024 bytes in length are OK
    assert!(valid_filename(&"™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™X".to_string()).is_ok());
}


#[test]
fn message_chars() {
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
fn message_length() {
    assert!(valid_message(&"I'd like to introduce myself. Hello!\0".to_string()).is_ok());  //short messages are OK

    assert!(valid_message(&"".to_string()).is_err()); //empty messages are not

    //1200 bytes (399 three byte code points, two more single byte code points and a null) are OK.
    assert!(valid_message(&"™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™AB\0".to_string()).is_ok());



    //1201 bytes (400 three byte code points,  a null) and more are NOT ok.
    assert!(valid_message(&"™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™™\0".to_string()).is_err());



}

///////////////////////////////////////////////
//  New Client Packet
///////////////////////////////////////////////

#[test]
fn new_client_packet_from_bytes() {
    let mut bytes_good = BytesMut::with_capacity(69);
    bytes_good.put_u8( IrcKind::IRC_KIND_NEW_CLIENT as u8);
    bytes_good.put_u32(64);
    bytes_good.put_slice("Bobtato".as_bytes());
    let remain = 64 - "Bobtato".len();
    bytes_good.put_bytes(b'\0',remain);

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
    bytes_short.put_bytes(b'\0',remain);

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
    bytes_lenf.put_bytes(b'\0',remain);

    let ncp_bad_lenf = NewClientPacket::from_bytes(&bytes_lenf);
    assert!(ncp_bad_lenf.is_err());
    if let Err(e) = ncp_bad_lenf {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::FieldLengthIncorrect() => true, _ => false });
    };

    let mut bytes_mismatch= BytesMut::with_capacity(69);
    bytes_mismatch.put_u8( IrcKind::IRC_KIND_ENTER_ROOM as u8); //wrong type
    bytes_mismatch.put_u32(64); 
    bytes_mismatch.put_slice("Bobtato".as_bytes());
    let remain = 64 - "Bobtato".len(); 
    bytes_mismatch.put_bytes(b'\0',remain);

    let ncp_bad_mismatch = NewClientPacket::from_bytes(&bytes_mismatch);
    assert!(ncp_bad_mismatch.is_err());
    if let Err(e) = ncp_bad_mismatch {
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

///////////////////////////////////////////////
//  Heartbeat Packet
///////////////////////////////////////////////

#[test]
fn heartbeat_packet_from_bytes() {
    let mut bytes_good = BytesMut::with_capacity(5);
    bytes_good.put_u8( IrcKind::IRC_KIND_HEARTBEAT as u8);
    bytes_good.put_u32(0);

    let hbp_good = HeartbeatPacket::from_bytes(&bytes_good);
    assert!(hbp_good.is_ok());


    let mut bytes_wrong_length = BytesMut::with_capacity(5);
    bytes_wrong_length.put_u8( IrcKind::IRC_KIND_HEARTBEAT as u8);
    bytes_wrong_length.put_u32(60);

    let hbp_bad_len = HeartbeatPacket::from_bytes(&bytes_wrong_length);
    assert!(hbp_bad_len.is_err());


    let mut bytes_wrong_type = BytesMut::with_capacity(5);
    bytes_wrong_type.put_u8( IrcKind::IRC_KIND_ERR as u8);
    bytes_wrong_type.put_u32(0);

    let hbp_bad_type = HeartbeatPacket::from_bytes(&bytes_wrong_type);
    assert!(hbp_bad_type.is_err());

}

#[test]
fn heartbeat_packet_as_bytes() {
    let hbp = HeartbeatPacket::new().unwrap();
    assert_eq!(hbp.as_bytes(), Bytes::from_static(b"\x03\0\0\0\0"));
}

///////////////////////////////////////////////
//  Error Packet
///////////////////////////////////////////////


#[test]
fn error_packet_from_bytes() {
    let mut bytes_good = BytesMut::with_capacity(6);
    bytes_good.put_u8( IrcKind::IRC_KIND_ERR as u8);
    bytes_good.put_u32(1);
    bytes_good.put_u8( IrcErrCode::IRC_ERR_ILLEGAL_LENGTH as u8);

    let erp_good = ErrorPacket::from_bytes(&bytes_good);
    assert!(erp_good.is_ok());


    let mut bytes_wrong_length = BytesMut::with_capacity(6);
    bytes_wrong_length.put_u8( IrcKind::IRC_KIND_ERR as u8);
    bytes_wrong_length.put_u32(60);
    bytes_wrong_length.put_u8( IrcErrCode::IRC_ERR_ILLEGAL_LENGTH as u8);

    let erp_bad_len = ErrorPacket::from_bytes(&bytes_wrong_length);
    assert!(erp_bad_len.is_err());


    let mut bytes_wrong_type = BytesMut::with_capacity(6);
    bytes_wrong_type.put_u8( IrcKind::IRC_KIND_HEARTBEAT as u8);
    bytes_wrong_type.put_u32(1);
    bytes_wrong_type.put_u8( IrcErrCode::IRC_ERR_ILLEGAL_LENGTH as u8);

    let erp_bad_type = ErrorPacket::from_bytes(&bytes_wrong_type);
    assert!(erp_bad_type.is_err());

    let mut bytes_invalid_errcode = BytesMut::with_capacity(6);
    bytes_invalid_errcode.put_u8( IrcKind::IRC_KIND_ERR as u8);
    bytes_invalid_errcode.put_u32(1);
    bytes_invalid_errcode.put_u8(255);

    let erp_bad_code = ErrorPacket::from_bytes(&bytes_invalid_errcode);
    assert!(erp_bad_code.is_err());

}

#[test]
fn error_packet_as_bytes() {
    let erp = ErrorPacket::new(IrcErrCode::IRC_ERR_NAME_IN_USE).unwrap();
    assert_eq!(erp.as_bytes(), Bytes::from_static(b"\x01\0\0\0\x01\x04"));
}

///////////////////////////////////////////////
//  Enter Room Packet
///////////////////////////////////////////////

#[test]
fn enter_room_packet_from_bytes() {
    let mut bytes_good = BytesMut::with_capacity(69);
    bytes_good.put_u8( IrcKind::IRC_KIND_ENTER_ROOM as u8);
    bytes_good.put_u32(64);
    bytes_good.put_slice("Bob'sroom".as_bytes());
    let remain = 64 - "Bob'sroom".len();
    bytes_good.put_bytes(b'\0',remain);

    let erp_good = EnterRoomPacket::from_bytes(&bytes_good);
    assert!(erp_good.is_ok());

    let mut bytes_short = BytesMut::with_capacity(69);
    bytes_short.put_u8( IrcKind::IRC_KIND_ENTER_ROOM as u8);
    bytes_short.put_u32(64);
    bytes_short.put_slice("Bob'sroom".as_bytes());
    let remain = 60 - "Bob'sroom".len(); //TOO SHORT
    bytes_short.put_bytes(b'\0',remain);

    let erp_bad_short = EnterRoomPacket::from_bytes(&bytes_short);
    assert!(erp_bad_short.is_err());
    if let Err(e) = erp_bad_short {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketLengthIncorrect(65,69) => true, _ => false });
    };


    let mut bytes_lenf= BytesMut::with_capacity(69);
    bytes_lenf.put_u8( IrcKind::IRC_KIND_ENTER_ROOM as u8);
    bytes_lenf.put_u32(30); //wrong length field value
    bytes_lenf.put_slice("Bob'sroom".as_bytes());
    let remain = 64 - "Bob'sroom".len(); 
    bytes_lenf.put_bytes(b'\0',remain);

    let erp_bad_lenf = EnterRoomPacket::from_bytes(&bytes_lenf);
    assert!(erp_bad_lenf.is_err());
    if let Err(e) = erp_bad_lenf {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::FieldLengthIncorrect() => true, _ => false });
    };

    let mut bytes_mismatch= BytesMut::with_capacity(69);
    bytes_mismatch.put_u8( IrcKind::IRC_KIND_NEW_CLIENT as u8); //wrong type
    bytes_mismatch.put_u32(64);
    bytes_mismatch.put_slice("Bob'sroom".as_bytes());
    let remain = 64 - "Bob'sroom".len(); 
    bytes_mismatch.put_bytes(b'\0',remain);

    let erp_bad_mismatch = EnterRoomPacket::from_bytes(&bytes_mismatch);
    assert!(erp_bad_mismatch.is_err());
    if let Err(e) = erp_bad_mismatch {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketMismatch() => true, _ => false });
    };
}

#[test]
fn enter_room_packet_as_bytes() {
    let erp = EnterRoomPacket::new(&"ExampleName".to_string()).unwrap();
    assert_eq!(erp.as_bytes(), Bytes::from_static(b"\x04\0\0\0\x40ExampleName\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"));
}

///////////////////////////////////////////////
//  Leave Room Packet
///////////////////////////////////////////////

#[test]
fn leave_room_packet_from_bytes() {
    let mut bytes_good = BytesMut::with_capacity(69);
    bytes_good.put_u8( IrcKind::IRC_KIND_LEAVE_ROOM as u8);
    bytes_good.put_u32(64);
    bytes_good.put_slice("Bob'sroom".as_bytes());
    let remain = 64 - "Bob'sroom".len();
    bytes_good.put_bytes(b'\0',remain);

    let lrp_good = LeaveRoomPacket::from_bytes(&bytes_good);
    assert!(lrp_good.is_ok());

    let mut bytes_short = BytesMut::with_capacity(69);
    bytes_short.put_u8( IrcKind::IRC_KIND_LEAVE_ROOM as u8);
    bytes_short.put_u32(64);
    bytes_short.put_slice("Bob'sroom".as_bytes());
    let remain = 60 - "Bob'sroom".len(); //TOO SHORT
    bytes_short.put_bytes(b'\0',remain);

    let lrp_bad_short = LeaveRoomPacket::from_bytes(&bytes_short);
    assert!(lrp_bad_short.is_err());
    if let Err(e) = lrp_bad_short {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketLengthIncorrect(65,69) => true, _ => false });
    };


    let mut bytes_lenf= BytesMut::with_capacity(69);
    bytes_lenf.put_u8( IrcKind::IRC_KIND_LEAVE_ROOM as u8);
    bytes_lenf.put_u32(30); //wrong length field value
    bytes_lenf.put_slice("Bob'sroom".as_bytes());
    let remain = 64 - "Bob'sroom".len(); 
    bytes_lenf.put_bytes(b'\0',remain);

    let lrp_bad_lenf = LeaveRoomPacket::from_bytes(&bytes_lenf);
    assert!(lrp_bad_lenf.is_err());
    if let Err(e) = lrp_bad_lenf {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::FieldLengthIncorrect() => true, _ => false });
    };

    let mut bytes_mismatch= BytesMut::with_capacity(69);
    bytes_mismatch.put_u8( IrcKind::IRC_KIND_NEW_CLIENT as u8); //wrong type
    bytes_mismatch.put_u32(64);
    bytes_mismatch.put_slice("Bob'sroom".as_bytes());
    let remain = 64 - "Bob'sroom".len();
    bytes_mismatch.put_bytes(b'\0',remain);

    let lrp_bad_mismatch = LeaveRoomPacket::from_bytes(&bytes_mismatch);
    assert!(lrp_bad_mismatch.is_err());
    if let Err(e) = lrp_bad_mismatch {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketMismatch() => true, _ => false });
    };
}

#[test]
fn leave_room_packet_as_bytes() {
    let lrp = LeaveRoomPacket::new(&"ExampleName".to_string()).unwrap();
    assert_eq!(lrp.as_bytes(), Bytes::from_static(b"\x05\0\0\0\x40ExampleName\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"));
}

///////////////////////////////////////////////
//  List Rooms Packet
///////////////////////////////////////////////

#[test]
fn list_rooms_packet_from_bytes() {
    let mut bytes_good = BytesMut::with_capacity(5);
    bytes_good.put_u8( IrcKind::IRC_KIND_LIST_ROOMS as u8);
    bytes_good.put_u32(0);

    let lrp_good = ListRoomsPacket::from_bytes(&bytes_good);
    assert!(lrp_good.is_ok());


    let mut bytes_wrong_length = BytesMut::with_capacity(5);
    bytes_wrong_length.put_u8( IrcKind::IRC_KIND_LIST_ROOMS as u8);
    bytes_wrong_length.put_u32(60);

    let lrp_bad_len = ListRoomsPacket::from_bytes(&bytes_wrong_length);
    assert!(lrp_bad_len.is_err());


    let mut bytes_wrong_type = BytesMut::with_capacity(5);
    bytes_wrong_type.put_u8( IrcKind::IRC_KIND_ERR as u8);
    bytes_wrong_type.put_u32(0);

    let lrp_bad_type = ListRoomsPacket::from_bytes(&bytes_wrong_type);
    assert!(lrp_bad_type.is_err());

}

#[test]
fn list_rooms_packet_as_bytes() {
    let lrp = ListRoomsPacket::new().unwrap();
    assert_eq!(lrp.as_bytes(), Bytes::from_static(b"\x06\0\0\0\0"));
}

///////////////////////////////////////////////
//  Room Listing Packet
///////////////////////////////////////////////

#[test]
fn room_listing_packet_from_bytes() {
    let mut bytes_good1 = BytesMut::with_capacity(133);
    bytes_good1.put_u8( IrcKind::IRC_KIND_ROOM_LISTING as u8);
    bytes_good1.put_u32(64*2);
    bytes_good1.put_bytes(b'\0',64);
    bytes_good1.put_slice("Bob'sroom".as_bytes());
    let remain = 64 - "Bob'sroom".len();
    bytes_good1.put_bytes(b'\0',remain);

    let rlp_good1 = RoomListingPacket::from_bytes(&bytes_good1);
    assert!(rlp_good1.is_ok());

    let mut bytes_good3 = BytesMut::with_capacity(261);
    bytes_good3.put_u8( IrcKind::IRC_KIND_ROOM_LISTING as u8);
    bytes_good3.put_u32(64*4);
    bytes_good3.put_bytes(b'\0',64);

    bytes_good3.put_slice("Bob'sroom".as_bytes());
    let remain = 64 - "Bob'sroom".len();
    bytes_good3.put_bytes(b'\0',remain);

    bytes_good3.put_slice("Lobby".as_bytes());
    let remain = 64 - "Lobby".len();
    bytes_good3.put_bytes(b'\0',remain);

    bytes_good3.put_slice("Just_Chatting".as_bytes());
    let remain = 64 - "Just_Chatting".len();
    bytes_good3.put_bytes(b'\0',remain);

    let rlp_good3 = RoomListingPacket::from_bytes(&bytes_good3);
    assert!(rlp_good3.is_ok());
    let rlp3 = rlp_good3.unwrap();
    assert_eq!(rlp3.rooms[0], "Bob'sroom".to_string());
    assert_eq!(rlp3.rooms[1], "Lobby".to_string());
    assert_eq!(rlp3.rooms[2], "Just_Chatting".to_string());

    let mut bytes_short = BytesMut::with_capacity(133);
    bytes_short.put_u8( IrcKind::IRC_KIND_ROOM_LISTING as u8);
    bytes_short.put_u32(128);
    bytes_short.put_bytes(b'\0',64);
    bytes_short.put_slice("Bob'sroom".as_bytes());
    let remain = 60 - "Bob'sroom".len(); //TOO SHORT
    bytes_short.put_bytes(b'\0',remain);

    let rlp_bad_short = RoomListingPacket::from_bytes(&bytes_short);
    assert!(rlp_bad_short.is_err());
    if let Err(e) = rlp_bad_short {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketLengthIncorrect(_,133) => true, _ => false });
    };


    let mut bytes_lenf= BytesMut::with_capacity(133);
    bytes_lenf.put_u8( IrcKind::IRC_KIND_ROOM_LISTING as u8);
    bytes_lenf.put_u32(30); //wrong length field value
    bytes_lenf.put_bytes(b'\0',64);
    bytes_lenf.put_slice("Bob'sroom".as_bytes());
    let remain = 64 - "Bob'sroom".len(); 
    bytes_lenf.put_bytes(b'\0',remain);

    let rlp_bad_lenf = RoomListingPacket::from_bytes(&bytes_lenf);
    assert!(rlp_bad_lenf.is_err());
    if let Err(e) = rlp_bad_lenf {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e {IrcError::PacketLengthIncorrect(_,_) => true, IrcError::FieldLengthIncorrect() => true, _ => false });
    };

    let mut bytes_mismatch= BytesMut::with_capacity(133);
    bytes_mismatch.put_u8( IrcKind::IRC_KIND_NEW_CLIENT as u8); //wrong type
    bytes_mismatch.put_u32(128); 
    bytes_mismatch.put_bytes(b'\0',64);
    bytes_mismatch.put_slice("Bob'sroom".as_bytes());
    let remain = 64 - "Bob'sroom".len();
    bytes_mismatch.put_bytes(b'\0',remain);

    let rlp_bad_mismatch = RoomListingPacket::from_bytes(&bytes_mismatch);
    assert!(rlp_bad_mismatch.is_err());
    if let Err(e) = rlp_bad_mismatch {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketMismatch() => true, _ => false });
    };
}

#[test]
fn room_listing_push_room() {
    let mut rlp = RoomListingPacket::new().unwrap();
    assert_eq!(rlp.rooms.len(), 0);

    assert!(rlp.push(&"ExampleName".to_string()).is_ok());
    assert_eq!(rlp.rooms.len(), 1);

    assert!(rlp.push(&"Exam\x09pleName".to_string()).is_err());
    assert_eq!(rlp.rooms.len(), 1);
}

#[test]
fn room_listing_packet_as_bytes() {
    let mut rlp = RoomListingPacket::new().unwrap();
    assert!(rlp.push(&"ExampleName".to_string()).is_ok());
    assert_eq!(rlp.as_bytes(), Bytes::from_static(b"\x07\0\0\0\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0ExampleName\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"));

    assert!(rlp.push(&"SecondName".to_string()).is_ok());
    assert_eq!(rlp.as_bytes(), Bytes::from_static(b"\x07\0\0\0\xC0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0ExampleName\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0SecondName\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"));

    let mut rooms_vec: Vec<String> = Vec::new();
    rooms_vec.push("first".to_string());
    rooms_vec.push("second".to_string());
    rooms_vec.push("third".to_string());
    let mut rlpfv = RoomListingPacket::from_vec(&rooms_vec).unwrap();

    assert_eq!(rlpfv.as_bytes(), Bytes::from_static(b"\x07\0\0\x01\x00\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0first\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0second\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0third\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"));
}

///////////////////////////////////////////////
//  User Listing Packet
///////////////////////////////////////////////

#[test]
fn user_listing_packet_from_bytes() {
    let mut bytes_good1 = BytesMut::with_capacity(133);
    bytes_good1.put_u8( IrcKind::IRC_KIND_USER_LISTING as u8);
    bytes_good1.put_u32(128);

    //room identifier
    bytes_good1.put_slice("Lobby".as_bytes());
    let remain = 64 - "Lobby".len();
    bytes_good1.put_bytes(b'\0', remain);

    //first user
    bytes_good1.put_slice("Bob'suser".as_bytes());
    let remain = 64 - "Bob'suser".len();
    bytes_good1.put_bytes(b'\0', remain);

    let ulp_good1 = UserListingPacket::from_bytes(&bytes_good1);
    assert!(ulp_good1.is_ok());

    let mut bytes_good3 = BytesMut::with_capacity(261);
    bytes_good3.put_u8( IrcKind::IRC_KIND_USER_LISTING as u8);
    bytes_good3.put_u32(64*4);

    bytes_good3.put_slice("Just_Chatting".as_bytes());
    let remain = 64 - "Just_Chatting".len();
    bytes_good3.put_bytes(b'\0', remain);

    bytes_good3.put_slice("Franklin".as_bytes());
    let remain = 64 - "Franklin".len();
    bytes_good3.put_bytes(b'\0', remain);

    bytes_good3.put_slice("Thomas".as_bytes());
    let remain = 64 - "Thomas".len();
    bytes_good3.put_bytes(b'\0', remain);

    bytes_good3.put_slice("JohnJonaJameson".as_bytes());
    let remain = 64 - "JohnJonaJameson".len();
    bytes_good3.put_bytes(b'\0', remain);

    let ulp_good3 = UserListingPacket::from_bytes(&bytes_good3);
    assert!(ulp_good3.is_ok());
    let ulp3 = ulp_good3.unwrap();
    assert_eq!(ulp3.room, "Just_Chatting".to_string());
    assert_eq!(ulp3.users[0], "Franklin".to_string());
    assert_eq!(ulp3.users[1], "Thomas".to_string());
    assert_eq!(ulp3.users[2], "JohnJonaJameson".to_string());

    let mut bytes_short = BytesMut::with_capacity(133);
    bytes_short.put_u8( IrcKind::IRC_KIND_USER_LISTING as u8);
    bytes_short.put_u32(128);
    bytes_short.put_slice("OtherRoom".as_bytes());
    let remain = 64 - "OtherRoom".len();
    bytes_short.put_bytes(b'\0', remain);
    bytes_short.put_slice("Franklin".as_bytes());
    let remain = 60 - "Franklin".len(); //TOO SHORT
    bytes_short.put_bytes(b'\0', remain);

    let ulp_bad_short = UserListingPacket::from_bytes(&bytes_short);
    assert!(ulp_bad_short.is_err());
    if let Err(e) = ulp_bad_short {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketLengthIncorrect(_,133) => true, _ => false });
    };


    let mut bytes_lenf= BytesMut::with_capacity(133);
    bytes_lenf.put_u8( IrcKind::IRC_KIND_USER_LISTING as u8);
    bytes_lenf.put_u32(30); //wrong length field value
    bytes_lenf.put_slice("OtherRoom".as_bytes());
    let remain = 64 - "OtherRoom".len();
    bytes_lenf.put_bytes(b'\0', remain);
    bytes_lenf.put_slice("Franklin".as_bytes());
    let remain = 64 - "Franklin".len();
    bytes_lenf.put_bytes(b'\0',64);

    let ulp_bad_lenf = UserListingPacket::from_bytes(&bytes_lenf);
    assert!(ulp_bad_lenf.is_err());
    if let Err(e) = ulp_bad_lenf {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e {IrcError::PacketLengthIncorrect(_,_) => true, IrcError::FieldLengthIncorrect() => true, _ => false });
    };

    let mut bytes_mismatch= BytesMut::with_capacity(133);
    bytes_mismatch.put_u8( IrcKind::IRC_KIND_NEW_CLIENT as u8); //wrong type
    bytes_mismatch.put_u32(128);
    bytes_good3.put_slice("OtherRoom".as_bytes());
    let remain = 64 - "OtherRoom".len();
    bytes_good3.put_bytes(b'\0', remain);
    bytes_mismatch.put_slice("Franklin".as_bytes());
    let remain = 64 - "Franklin".len();
    bytes_mismatch.put_bytes(b'\0',64);

    let ulp_bad_mismatch = UserListingPacket::from_bytes(&bytes_mismatch);
    assert!(ulp_bad_mismatch.is_err());
    if let Err(e) = ulp_bad_mismatch {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketMismatch() => true, _ => false });
    };
}

#[test]
fn user_listing_push_user() {
    let mut ulp = UserListingPacket::new().unwrap();
    assert_eq!(ulp.users.len(), 0);

    assert!(ulp.push(&"ExampleName".to_string()).is_ok());
    assert_eq!(ulp.users.len(), 1);

    assert!(ulp.push(&"Exam\x09pleName".to_string()).is_err());
    assert_eq!(ulp.users.len(), 1);
}

#[test]
fn user_listing_packet_as_bytes() {
    let mut ulp = UserListingPacket::new().unwrap();
    assert!(ulp.set_room(&"Channel54".to_string()).is_ok());
    assert!(ulp.push(&"ExampleName".to_string()).is_ok());
    assert_eq!(ulp.as_bytes(), Bytes::from_static(b"\x08\0\0\0\x80Channel54\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0ExampleName\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"));

    assert!(ulp.push(&"SecondName".to_string()).is_ok());
    assert_eq!(ulp.as_bytes(), Bytes::from_static(b"\x08\0\0\0\xC0Channel54\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0ExampleName\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0SecondName\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"));

    let mut users_vec: Vec<String> = Vec::new();
    users_vec.push("first".to_string());
    users_vec.push("second".to_string());
    users_vec.push("third".to_string());
    let mut ulpfv = UserListingPacket::from_room_and_vec(&"r/IRC".to_string(), &users_vec).unwrap();

    assert_eq!(ulpfv.as_bytes(), Bytes::from_static(b"\x08\0\0\x01\x00r/IRC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0first\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0second\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0third\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"));
}

///////////////////////////////////////////////
//  Query User Packet
///////////////////////////////////////////////

#[test]
fn query_user_packet_from_bytes() {
    let mut bytes_good = BytesMut::with_capacity(70);
    bytes_good.put_u8( IrcKind::IRC_KIND_QUERY_USER as u8);
    bytes_good.put_u32(65);

    //first user
    bytes_good.put_slice("Bob'suser".as_bytes());
    let remain = 64 - "Bob'suser".len();
    bytes_good.put_bytes(b'\0', remain);
    bytes_good.put_u8(2);

    let qup_good = QueryUserPacket::from_bytes(&bytes_good);
    assert!(qup_good.is_ok());
    let qup = qup_good.unwrap();
    assert_eq!(qup.user_name, "Bob'suser".to_string());

    let mut bytes_short = BytesMut::with_capacity(70);
    bytes_short.put_u8( IrcKind::IRC_KIND_QUERY_USER as u8);
    bytes_short.put_u32(65);
    bytes_short.put_slice("Franklin".as_bytes());
    let remain = 60 - "Franklin".len(); //TOO SHORT
    bytes_short.put_bytes(b'\0', remain);
    bytes_short.put_u8(1);

    let qup_bad_short = QueryUserPacket::from_bytes(&bytes_short);
    assert!(qup_bad_short.is_err());
    if let Err(e) = qup_bad_short {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketLengthIncorrect(_,70) => true, _ => false });
    };

    let mut bytes_lenf= BytesMut::with_capacity(133);
    bytes_lenf.put_u8( IrcKind::IRC_KIND_QUERY_USER as u8);
    bytes_lenf.put_u32(30); //wrong length field value
    bytes_lenf.put_slice("Franklin".as_bytes());
    let remain = 64 - "Franklin".len();
    bytes_lenf.put_bytes(b'\0',64);
    bytes_lenf.put_u8(0);

    let qup_bad_lenf = QueryUserPacket::from_bytes(&bytes_lenf);
    assert!(qup_bad_lenf.is_err());
    if let Err(e) = qup_bad_lenf {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e {IrcError::PacketLengthIncorrect(_,70) => true, IrcError::FieldLengthIncorrect() => true, _ => false });
    };

    let mut bytes_mismatch= BytesMut::with_capacity(133);
    bytes_mismatch.put_u8( IrcKind::IRC_KIND_NEW_CLIENT as u8); //wrong type
    bytes_mismatch.put_u32(65);
    bytes_mismatch.put_slice("Franklin".as_bytes());
    let remain = 64 - "Franklin".len();
    bytes_mismatch.put_bytes(b'\0',64);
    bytes_mismatch.put_u8(2);

    let qup_bad_mismatch = QueryUserPacket::from_bytes(&bytes_mismatch);
    assert!(qup_bad_mismatch.is_err());
    if let Err(e) = qup_bad_mismatch {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        println!("{:?}",e);
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketMismatch() => true, _ => false });
    };
}

#[test]
fn query_user_set_status() {
    let mut qup = QueryUserPacket::new(&"Pete11231@".to_string()).unwrap();
    assert_eq!(qup.status, UserStatus::Request);

    qup.set_online();
    assert_eq!(qup.status, UserStatus::Online);

    qup.set_offline();
    assert_eq!(qup.status, UserStatus::Offline);

    qup.set_query();
    assert_eq!(qup.status, UserStatus::Request);

}

#[test]
fn query_user_packet_as_bytes() {
    let mut qup = QueryUserPacket::new(&"Charley42".to_string()).unwrap();
    qup.set_online();
    assert_eq!(qup.as_bytes(), Bytes::from_static(b"\x09\0\0\0\x41Charley42\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01"));

    qup.set_offline();
    assert_eq!(qup.as_bytes(), Bytes::from_static(b"\x09\0\0\0\x41Charley42\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x00"));

    qup.set_query();
    assert_eq!(qup.as_bytes(), Bytes::from_static(b"\x09\0\0\0\x41Charley42\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02"));

}

///////////////////////////////////////////////
//  Send Message Packet
///////////////////////////////////////////////

#[test]
fn send_message_packet_from_bytes() {
    let mut bytes_good = BytesMut::with_capacity(91);
    bytes_good.put_u8( IrcKind::IRC_KIND_SEND_MESSAGE as u8);
    bytes_good.put_u32(86);

    bytes_good.put_slice("Bob's_room".as_bytes());
    let remain = 64 - "Bob's_room".len();
    bytes_good.put_bytes(b'\0', remain);
    bytes_good.put_slice("Dude, where'd you go?\0".as_bytes());

    let smp_good = SendMessagePacket::from_bytes(&bytes_good);
    assert!(smp_good.is_ok());
    let smp = smp_good.unwrap();
    assert_eq!(smp.get_message(), "Dude, where'd you go?".to_string());

    let mut bytes_short = BytesMut::with_capacity(81);
    bytes_short.put_u8( IrcKind::IRC_KIND_SEND_MESSAGE as u8);
    bytes_short.put_u32(76);
    bytes_short.put_slice("PSU".as_bytes());
    let remain = 60 - "PSU".len(); //TOO SHORT
    bytes_short.put_bytes(b'\0', remain);
    bytes_short.put_slice(b"messagebody\0");

    let smp_bad_short = SendMessagePacket::from_bytes(&bytes_short);
    assert!(smp_bad_short.is_err());
    if let Err(e) = smp_bad_short {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketLengthIncorrect(_,81) => true, _ => false });
    };

    let mut bytes_lenf= BytesMut::with_capacity(145);
    bytes_lenf.put_u8( IrcKind::IRC_KIND_SEND_MESSAGE as u8);
    bytes_lenf.put_u32(30); //wrong length field value
    bytes_lenf.put_slice("News&Rumours".as_bytes());
    let remain = 64 - "News&Rumours".len();
    bytes_lenf.put_bytes(b'\0',64);
    bytes_lenf.put_slice(b"Our records show your car's warranty is almost expired! If you'd like to...\0");

    let smp_bad_lenf = SendMessagePacket::from_bytes(&bytes_lenf);
    assert!(smp_bad_lenf.is_err());
    if let Err(e) = smp_bad_lenf {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e {IrcError::PacketLengthIncorrect(_,145) => true, IrcError::FieldLengthIncorrect() => true, _ => false });
    };

    let mut bytes_mismatch= BytesMut::with_capacity(73);
    bytes_mismatch.put_u8( IrcKind::IRC_KIND_NEW_CLIENT as u8); //wrong type
    bytes_mismatch.put_u32(68);
    bytes_mismatch.put_slice("Cars".as_bytes());
    let remain = 64 - "Cars".len();
    bytes_mismatch.put_bytes(b'\0',64);
    bytes_mismatch.put_slice("yo!\0".as_bytes());

    let smp_bad_mismatch = SendMessagePacket::from_bytes(&bytes_mismatch);
    assert!(smp_bad_mismatch.is_err());
    if let Err(e) = smp_bad_mismatch {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketMismatch() => true, _ => false });
    };
    
}
#[test]
fn send_message() {
    let smpwrap = SendMessagePacket::new(&"RTSGaming".to_string(), &"This should be good.\0".to_string());
    assert!(smpwrap.is_ok());
    let smp = smpwrap.unwrap();
    assert_eq!(smp.room, "RTSGaming");
    assert_eq!(smp.message, "This should be good.\0");
    assert_eq!(smp.get_message(), "This should be good.");

    let mut smpwrap = SendMessagePacket::new(&"RTSGaming".to_string(), &"AHH! You scared me!".to_string());
    assert!(smpwrap.is_ok());
    let smp = smpwrap.unwrap();
    assert_eq!(smp.room, "RTSGaming");
    assert_eq!(smp.message, "AHH! You scared me!\0");
    assert_eq!(smp.get_message(), "AHH! You scared me!");

    let mut smp_fail = SendMessagePacket::new(&"RTSGaming".to_string(), &"AHH! \0You scared me!".to_string());
    assert!(smp_fail.is_err());
}

#[test]
fn send_message_packet_as_bytes() {
    let mut smp = SendMessagePacket::new(&"Channel42".to_string(), &"Hello".to_string()).unwrap();
    assert_eq!(smp.as_bytes(), Bytes::from_static(b"\x0A\0\0\0\x46Channel42\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Hello\0"));

}

///////////////////////////////////////////////
//  Broadcast Message Packet
///////////////////////////////////////////////

#[test]
fn broadcast_message_packet_from_bytes() {
    let mut bytes_good = BytesMut::with_capacity(27);
    bytes_good.put_u8( IrcKind::IRC_KIND_BROADCAST_MESSAGE as u8);
    bytes_good.put_u32(22);
    bytes_good.put_slice("Dude, where'd you go?\0".as_bytes());

    let bmp_good = BroadcastMessagePacket::from_bytes(&bytes_good);
    assert!(bmp_good.is_ok());
    let bmp = bmp_good.unwrap();
    assert_eq!(bmp.get_message(), "Dude, where'd you go?".to_string());

    let mut bytes_lenf= BytesMut::with_capacity(81);
    bytes_lenf.put_u8( IrcKind::IRC_KIND_BROADCAST_MESSAGE as u8);
    bytes_lenf.put_u32(30); //wrong length field value
    bytes_lenf.put_slice(b"Our records show your car's warranty is almost expired! If you'd like to...\0");

    let bmp_bad_lenf = BroadcastMessagePacket::from_bytes(&bytes_lenf);
    assert!(bmp_bad_lenf.is_err());
    if let Err(e) = bmp_bad_lenf {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e {IrcError::PacketLengthIncorrect(81,_) => true, IrcError::FieldLengthIncorrect() => true, _ => false });
    };

    let mut bytes_mismatch= BytesMut::with_capacity(9);
    bytes_mismatch.put_u8( IrcKind::IRC_KIND_NEW_CLIENT as u8); //wrong type
    bytes_mismatch.put_u32(4);
    bytes_mismatch.put_slice("yo!\0".as_bytes());

    let bmp_bad_mismatch = BroadcastMessagePacket::from_bytes(&bytes_mismatch);
    assert!(bmp_bad_mismatch.is_err());
    if let Err(e) = bmp_bad_mismatch {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketMismatch() => true, _ => false });
    };
    
}
#[test]
fn broadcast_message() {
    let bmpwrap = BroadcastMessagePacket::new(&"This should be good.\0".to_string());
    assert!(bmpwrap.is_ok());
    let bmp = bmpwrap.unwrap();
    assert_eq!(bmp.message, "This should be good.\0");
    assert_eq!(bmp.get_message(), "This should be good.");

    let mut bmpwrap = BroadcastMessagePacket::new(&"AHH! You scared me!".to_string());
    assert!(bmpwrap.is_ok());
    let bmp = bmpwrap.unwrap();
    assert_eq!(bmp.message, "AHH! You scared me!\0");
    assert_eq!(bmp.get_message(), "AHH! You scared me!");

    let mut bmp_fail = BroadcastMessagePacket::new(&"AHH! \0You scared me!".to_string());
    assert!(bmp_fail.is_err());
}

#[test]
fn broadcast_message_packet_as_bytes() {
    let mut bmp = BroadcastMessagePacket::new(&"Hello".to_string()).unwrap();
    assert_eq!(bmp.as_bytes(), Bytes::from_static(b"\x0B\0\0\0\x06Hello\0"));

}

///////////////////////////////////////////////
//  Post Message Packet
///////////////////////////////////////////////

#[test]
fn post_message_packet_from_bytes() {
    let mut bytes_good = BytesMut::with_capacity(155);
    bytes_good.put_u8( IrcKind::IRC_KIND_POST_MESSAGE as u8);
    bytes_good.put_u32(150);

    bytes_good.put_slice("Bob's_room".as_bytes());
    let remain = 64 - "Bob's_room".len();
    bytes_good.put_bytes(b'\0', remain);
    bytes_good.put_slice("Bob".as_bytes());
    let remain = 64 - "Bob".len();
    bytes_good.put_bytes(b'\0', remain);
    bytes_good.put_slice("Dude, where'd you go?\0".as_bytes());

    let pmp_good = PostMessagePacket::from_bytes(&bytes_good);
    assert!(pmp_good.is_ok());
    let pmp = pmp_good.unwrap();
    assert_eq!(pmp.get_message(), "Dude, where'd you go?".to_string());

    let mut bytes_short = BytesMut::with_capacity(145);
    bytes_short.put_u8( IrcKind::IRC_KIND_POST_MESSAGE as u8);
    bytes_short.put_u32(140);
    bytes_short.put_slice("PSU".as_bytes());
    let remain = 60 - "PSU".len(); //TOO SHORT
    bytes_short.put_bytes(b'\0', remain);
    bytes_good.put_slice("ProffesorSnape".as_bytes());
    let remain = 64 - "ProfessorSnape".len();
    bytes_good.put_bytes(b'\0', remain);
    bytes_short.put_slice(b"messagebody\0");

    let pmp_bad_short = PostMessagePacket::from_bytes(&bytes_short);
    assert!(pmp_bad_short.is_err());
    if let Err(e) = pmp_bad_short {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketLengthIncorrect(_,_) => true, _ => false });
    };

    let mut bytes_lenf= BytesMut::with_capacity(209);
    bytes_lenf.put_u8( IrcKind::IRC_KIND_POST_MESSAGE as u8);
    bytes_lenf.put_u32(30); //wrong length field value
    bytes_lenf.put_slice("News&Rumours".as_bytes());
    let remain = 64 - "News&Rumours".len();
    bytes_lenf.put_bytes(b'\0',64);
    bytes_lenf.put_slice("SpamCaller".as_bytes());
    let remain = 64 - "SpamCaller".len();
    bytes_lenf.put_bytes(b'\0',64);
    bytes_lenf.put_slice(b"Our records show your car's warranty is almost expired! If you'd like to...\0");

    let pmp_bad_lenf = PostMessagePacket::from_bytes(&bytes_lenf);
    assert!(pmp_bad_lenf.is_err());
    if let Err(e) = pmp_bad_lenf {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        println!("{:?}",e);
        //does NOT implement PartialEq
        assert!(match e {IrcError::PacketLengthIncorrect(_,145) => true, IrcError::FieldLengthIncorrect() => true, _ => false });
    };

    let mut bytes_mismatch= BytesMut::with_capacity(137);
    bytes_mismatch.put_u8( IrcKind::IRC_KIND_NEW_CLIENT as u8); //wrong type
    bytes_mismatch.put_u32(132);
    bytes_mismatch.put_slice("Cars".as_bytes());
    let remain = 64 - "Cars".len();
    bytes_mismatch.put_bytes(b'\0',64);
    bytes_mismatch.put_slice("DudeBro".as_bytes());
    let remain = 64 - "DudeBro".len();
    bytes_mismatch.put_bytes(b'\0',64);
    bytes_mismatch.put_slice("yo!\0".as_bytes());

    let pmp_bad_mismatch = PostMessagePacket::from_bytes(&bytes_mismatch);
    assert!(pmp_bad_mismatch.is_err());
    if let Err(e) = pmp_bad_mismatch {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketMismatch() => true, _ => false });
    };
    
}
#[test]
fn post_message() {
    let pmpwrap = PostMessagePacket::new(&"RTSGaming".to_string(), &"blah_user".to_string(), &"This should be good.\0".to_string());
    assert!(pmpwrap.is_ok());
    let pmp = pmpwrap.unwrap();
    assert_eq!(pmp.room, "RTSGaming");
    assert_eq!(pmp.sender, "blah_user");
    assert_eq!(pmp.message, "This should be good.\0");
    assert_eq!(pmp.get_message(), "This should be good.");

    //Without pre-appending a null to the message
    let mut pmpwrap = PostMessagePacket::new(&"RTSGaming".to_string(), &"SCV429".to_string(), &"AHH! You scared me!".to_string());
    assert!(pmpwrap.is_ok());
    let pmp = pmpwrap.unwrap();
    assert_eq!(pmp.room, "RTSGaming");
    assert_eq!(pmp.sender, "SCV429");
    assert_eq!(pmp.message, "AHH! You scared me!\0");
    assert_eq!(pmp.get_message(), "AHH! You scared me!");

    let mut pmp_fail = PostMessagePacket::new(&"RTSGaming".to_string(),&"SCV429".to_string(), &"AHH! \0You scared me!".to_string());
    assert!(pmp_fail.is_err());
}

#[test]
fn post_message_packet_as_bytes() {
    let mut pmp = PostMessagePacket::new(&"Channel42".to_string(), &"New_User".to_string(), &"Hello".to_string()).unwrap();
    assert_eq!(pmp.as_bytes(), Bytes::from_static(b"\x0C\0\0\0\x86Channel42\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0New_User\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Hello\0"));

}

///////////////////////////////////////////////
//  Direct Message Packet
///////////////////////////////////////////////

#[test]
fn direct_message_packet_from_bytes() {
    let mut bytes_good = BytesMut::with_capacity(91);
    bytes_good.put_u8( IrcKind::IRC_KIND_DIRECT_MESSAGE as u8);
    bytes_good.put_u32(86);

    bytes_good.put_slice("Bob's_room".as_bytes());
    let remain = 64 - "Bob's_room".len();
    bytes_good.put_bytes(b'\0', remain);
    bytes_good.put_slice("Dude, where'd you go?\0".as_bytes());

    let dmp_good = DirectMessagePacket::from_bytes(&bytes_good);
    assert!(dmp_good.is_ok());
    let dmp = dmp_good.unwrap();
    assert_eq!(dmp.get_message(), "Dude, where'd you go?".to_string());

    let mut bytes_short = BytesMut::with_capacity(81);
    bytes_short.put_u8( IrcKind::IRC_KIND_DIRECT_MESSAGE as u8);
    bytes_short.put_u32(76);
    bytes_short.put_slice("PSU".as_bytes());
    let remain = 60 - "PSU".len(); //TOO SHORT
    bytes_short.put_bytes(b'\0', remain);
    bytes_short.put_slice(b"messagebody\0");

    let dmp_bad_short = DirectMessagePacket::from_bytes(&bytes_short);
    assert!(dmp_bad_short.is_err());
    if let Err(e) = dmp_bad_short {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketLengthIncorrect(_,81) => true, _ => false });
    };

    let mut bytes_lenf= BytesMut::with_capacity(145);
    bytes_lenf.put_u8( IrcKind::IRC_KIND_DIRECT_MESSAGE as u8);
    bytes_lenf.put_u32(30); //wrong length field value
    bytes_lenf.put_slice("News&Rumours".as_bytes());
    let remain = 64 - "News&Rumours".len();
    bytes_lenf.put_bytes(b'\0',64);
    bytes_lenf.put_slice(b"Our records show your car's warranty is almost expired! If you'd like to...\0");

    let dmp_bad_lenf = DirectMessagePacket::from_bytes(&bytes_lenf);
    assert!(dmp_bad_lenf.is_err());
    if let Err(e) = dmp_bad_lenf {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e {IrcError::PacketLengthIncorrect(_,145) => true, IrcError::FieldLengthIncorrect() => true, _ => false });
    };

    let mut bytes_mismatch= BytesMut::with_capacity(73);
    bytes_mismatch.put_u8( IrcKind::IRC_KIND_NEW_CLIENT as u8); //wrong type
    bytes_mismatch.put_u32(68);
    bytes_mismatch.put_slice("Cars".as_bytes());
    let remain = 64 - "Cars".len();
    bytes_mismatch.put_bytes(b'\0',64);
    bytes_mismatch.put_slice("yo!\0".as_bytes());

    let dmp_bad_mismatch = DirectMessagePacket::from_bytes(&bytes_mismatch);
    assert!(dmp_bad_mismatch.is_err());
    if let Err(e) = dmp_bad_mismatch {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketMismatch() => true, _ => false });
    };
}

#[test]
fn direct_message() {
    let dmpwrap = DirectMessagePacket::new(&"RTSGaming".to_string(), &"This should be good.\0".to_string());
    assert!(dmpwrap.is_ok());
    let dmp = dmpwrap.unwrap();
    assert_eq!(dmp.room, "RTSGaming");
    assert_eq!(dmp.message, "This should be good.\0");
    assert_eq!(dmp.get_message(), "This should be good.");

    let mut dmpwrap = DirectMessagePacket::new(&"RTSGaming".to_string(), &"AHH! You scared me!".to_string());
    assert!(dmpwrap.is_ok());
    let dmp = dmpwrap.unwrap();
    assert_eq!(dmp.room, "RTSGaming");
    assert_eq!(dmp.message, "AHH! You scared me!\0");
    assert_eq!(dmp.get_message(), "AHH! You scared me!");

    let mut dmp_fail = DirectMessagePacket::new(&"RTSGaming".to_string(), &"AHH! \0You scared me!".to_string());
    assert!(dmp_fail.is_err());
}

#[test]
fn direct_message_packet_as_bytes() {
    let mut dmp = DirectMessagePacket::new(&"Channel42".to_string(), &"Hello".to_string()).unwrap();
    assert_eq!(dmp.as_bytes(), Bytes::from_static(b"\x0D\0\0\0\x46Channel42\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Hello\0"));

}

///////////////////////////////////////////////
//  Offer File Packet
///////////////////////////////////////////////

#[test]
fn offer_file_packet_from_bytes() {
    let mut bytes_good = BytesMut::with_capacity(91);
    bytes_good.put_u8( IrcKind::IRC_KIND_OFFER_FILE as u8);
    bytes_good.put_u32(145);

//to,from,id,size,filename
//Frank, Bob, 0, 512, example.txt  < 11 bytes , 64+64+2+4+11 = 145

    bytes_good.put_slice("Frank".as_bytes());
    let remain = 64 - "Frank".len();
    bytes_good.put_bytes(b'\0', remain);

    bytes_good.put_slice("Bob".as_bytes());
    let remain = 64 - "Bob".len();
    bytes_good.put_bytes(b'\0', remain);
    bytes_good.put_u16(0);
    bytes_good.put_u32(512);
    bytes_good.put_slice("example.txt".as_bytes());

    let ofp_good = OfferFilePacket::from_bytes(&bytes_good);
    if let Err(e) = &ofp_good {
        println!("{:?}",e);
    }
    assert!(ofp_good.is_ok());
    let ofp = ofp_good.unwrap();
    assert_eq!(ofp.get_file_name(), "example.txt".to_string());


    let mut bytes_short = BytesMut::with_capacity(81);
    bytes_short.put_u8( IrcKind::IRC_KIND_OFFER_FILE as u8);
    bytes_short.put_u32(145);
    bytes_short.put_slice("Frank".as_bytes());
    let remain = 60 - "Frank".len(); //too short
    bytes_short.put_bytes(b'\0', remain);

    bytes_short.put_slice("Bob".as_bytes());
    let remain = 64 - "Bob".len();
    bytes_short.put_bytes(b'\0', remain);

    bytes_short.put_u16(0);
    bytes_short.put_u32(512);
    bytes_short.put_slice("example.txt".as_bytes());

    let ofp_bad_short = OfferFilePacket::from_bytes(&bytes_short);
    assert!(ofp_bad_short.is_err());
    if let Err(e) = ofp_bad_short {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketLengthIncorrect(_,_) => true, _ => false });
    };

    let mut bytes_lenf= BytesMut::with_capacity(145);
    bytes_lenf.put_u8( IrcKind::IRC_KIND_OFFER_FILE as u8);
    bytes_lenf.put_u32(30); //wrong length field value

    bytes_lenf.put_slice("Frank".as_bytes());
    let remain = 60 - "Frank".len(); //too short
    bytes_lenf.put_bytes(b'\0', remain);

    bytes_lenf.put_slice("Bob".as_bytes());
    let remain = 64 - "Bob".len();
    bytes_lenf.put_bytes(b'\0', remain);

    bytes_lenf.put_u16(0);
    bytes_lenf.put_u32(512);
    bytes_lenf.put_slice("example.txt".as_bytes());

    let ofp_bad_lenf = OfferFilePacket::from_bytes(&bytes_lenf);
    assert!(ofp_bad_lenf.is_err());
    if let Err(e) = ofp_bad_lenf {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e {IrcError::PacketLengthIncorrect(_,145) => true, IrcError::FieldLengthIncorrect() => true, _ => false });
    };


    let mut bytes_mismatch= BytesMut::with_capacity(73);
    bytes_mismatch.put_u8( IrcKind::IRC_KIND_NEW_CLIENT as u8); //wrong type
    bytes_mismatch.put_u32(145);
    bytes_mismatch.put_slice("Frank".as_bytes());
    let remain = 60 - "Frank".len(); //too short
    bytes_mismatch.put_bytes(b'\0', remain);

    bytes_mismatch.put_slice("Bob".as_bytes());
    let remain = 64 - "Bob".len();
    bytes_mismatch.put_bytes(b'\0', remain);

    bytes_mismatch.put_u16(0);
    bytes_mismatch.put_u32(512);
    bytes_mismatch.put_slice("example.txt".as_bytes());

    let ofp_bad_mismatch = OfferFilePacket::from_bytes(&bytes_mismatch);
    assert!(ofp_bad_mismatch.is_err());
    if let Err(e) = ofp_bad_mismatch {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketMismatch() => true, _ => false });
    };
}

#[test]
fn offer_file() {
    let ofpwrap = OfferFilePacket::new(&"Frank".to_string(), &"Bob".to_string(), 512, &"Example.txt".to_string());
    assert!(ofpwrap.is_ok());
    let ofp = ofpwrap.unwrap();
    assert_eq!(ofp.get_to(), "Frank");
    assert_eq!(ofp.get_from(), "Bob");
    assert_eq!(ofp.get_size(), 512);
    assert_eq!(ofp.get_file_name(), "Example.txt");

    let mut ofp_fail = OfferFilePacket::new(&"Frank".to_string(), &"Bob".to_string(), 512, &"Exa:mple.txt".to_string());
    assert!(ofp_fail.is_err());
}

#[test]
fn offer_file_packet_as_bytes() {
    let mut ofp = OfferFilePacket::new(&"Frank".to_string(), &"Bob".to_string(), 512, &"example.txt".to_string()).unwrap();
    assert_eq!(ofp.as_bytes(), Bytes::from_static(b"\x0E\0\0\0\x96Frank\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Bob\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x00\x00\0\0\x02\x00example.txt"));
}

///////////////////////////////////////////////
//  Accept File Packet
///////////////////////////////////////////////

#[test]
fn accept_file_packet_from_bytes() {
    let mut bytes_good = BytesMut::with_capacity(91);
    bytes_good.put_u8( IrcKind::IRC_KIND_ACCEPT_FILE as u8);
    bytes_good.put_u32(145);

//to,from,id,size,filename
//Frank, Bob, 0, 512, example.txt  < 11 bytes , 64+64+2+4+11 = 145

    bytes_good.put_slice("Frank".as_bytes());
    let remain = 64 - "Frank".len();
    bytes_good.put_bytes(b'\0', remain);

    bytes_good.put_slice("Bob".as_bytes());
    let remain = 64 - "Bob".len();
    bytes_good.put_bytes(b'\0', remain);
    bytes_good.put_u16(11);
    bytes_good.put_u32(512);
    bytes_good.put_slice("example.txt".as_bytes());

    let afp_good = AcceptFilePacket::from_bytes(&bytes_good);
    if let Err(e) = &afp_good {
        println!("{:?}",e);
    }
    assert!(afp_good.is_ok());
    let afp = afp_good.unwrap();
    assert_eq!(afp.get_file_name(), "example.txt".to_string());


    let mut bytes_short = BytesMut::with_capacity(81);
    bytes_short.put_u8( IrcKind::IRC_KIND_ACCEPT_FILE as u8);
    bytes_short.put_u32(145);
    bytes_short.put_slice("Frank".as_bytes());
    let remain = 60 - "Frank".len(); //too short
    bytes_short.put_bytes(b'\0', remain);

    bytes_short.put_slice("Bob".as_bytes());
    let remain = 64 - "Bob".len();
    bytes_short.put_bytes(b'\0', remain);

    bytes_short.put_u16(11);
    bytes_short.put_u32(512);
    bytes_short.put_slice("example.txt".as_bytes());

    let afp_bad_short = AcceptFilePacket::from_bytes(&bytes_short);
    assert!(afp_bad_short.is_err());
    if let Err(e) = afp_bad_short {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketLengthIncorrect(_,_) => true, _ => false });
    };

    let mut bytes_lenf= BytesMut::with_capacity(145);
    bytes_lenf.put_u8( IrcKind::IRC_KIND_ACCEPT_FILE as u8);
    bytes_lenf.put_u32(30); //wrong length field value

    bytes_lenf.put_slice("Frank".as_bytes());
    let remain = 60 - "Frank".len(); //too short
    bytes_lenf.put_bytes(b'\0', remain);

    bytes_lenf.put_slice("Bob".as_bytes());
    let remain = 64 - "Bob".len();
    bytes_lenf.put_bytes(b'\0', remain);

    bytes_lenf.put_u16(11);
    bytes_lenf.put_u32(512);
    bytes_lenf.put_slice("example.txt".as_bytes());

    let afp_bad_lenf = AcceptFilePacket::from_bytes(&bytes_lenf);
    assert!(afp_bad_lenf.is_err());
    if let Err(e) = afp_bad_lenf {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e {IrcError::PacketLengthIncorrect(_,145) => true, IrcError::FieldLengthIncorrect() => true, _ => false });
    };


    let mut bytes_mismatch= BytesMut::with_capacity(73);
    bytes_mismatch.put_u8( IrcKind::IRC_KIND_NEW_CLIENT as u8); //wrong type
    bytes_mismatch.put_u32(145);
    bytes_mismatch.put_slice("Frank".as_bytes());
    let remain = 60 - "Frank".len(); //too short
    bytes_mismatch.put_bytes(b'\0', remain);

    bytes_mismatch.put_slice("Bob".as_bytes());
    let remain = 64 - "Bob".len();
    bytes_mismatch.put_bytes(b'\0', remain);

    bytes_mismatch.put_u16(11);
    bytes_mismatch.put_u32(512);
    bytes_mismatch.put_slice("example.txt".as_bytes());

    let afp_bad_mismatch = AcceptFilePacket::from_bytes(&bytes_mismatch);
    assert!(afp_bad_mismatch.is_err());
    if let Err(e) = afp_bad_mismatch {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketMismatch() => true, _ => false });
    };
}

#[test]
fn accept_file() {
    let afpwrap = AcceptFilePacket::new(&"Frank".to_string(), &"Bob".to_string(), 11, 512, &"Example.txt".to_string());
    assert!(afpwrap.is_ok());
    let afp = afpwrap.unwrap();
    assert_eq!(afp.get_to(), "Frank");
    assert_eq!(afp.get_from(), "Bob");
    assert_eq!(afp.get_transfer_id(), 11);
    assert_eq!(afp.get_size(), 512);
    assert_eq!(afp.get_file_name(), "Example.txt");

    let mut afp_fail = AcceptFilePacket::new(&"Frank".to_string(), &"Bob".to_string(),11,  512, &"Exa:mple.txt".to_string());
    assert!(afp_fail.is_err());
}

#[test]
fn accept_file_packet_as_bytes() {
    let mut afp = AcceptFilePacket::new(&"Frank".to_string(), &"Bob".to_string(),11, 512, &"example.txt".to_string()).unwrap();
    assert_eq!(afp.as_bytes(), Bytes::from_static(b"\x0F\0\0\0\x96Frank\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Bob\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x00\x0B\0\0\x02\x00example.txt"));
}

///////////////////////////////////////////////
//  Reject File Packet
///////////////////////////////////////////////

#[test]
fn reject_file_packet_from_bytes() {
    let mut bytes_good = BytesMut::with_capacity(91);
    bytes_good.put_u8( IrcKind::IRC_KIND_REJECT_FILE as u8);
    bytes_good.put_u32(145);

//to,from,id,size,filename
//Frank, Bob, 0, 512, example.txt  < 11 bytes , 64+64+2+4+11 = 145

    bytes_good.put_slice("Frank".as_bytes());
    let remain = 64 - "Frank".len();
    bytes_good.put_bytes(b'\0', remain);

    bytes_good.put_slice("Bob".as_bytes());
    let remain = 64 - "Bob".len();
    bytes_good.put_bytes(b'\0', remain);
    bytes_good.put_u16(11);
    bytes_good.put_u32(512);
    bytes_good.put_slice("example.txt".as_bytes());

    let rfp_good = RejectFilePacket::from_bytes(&bytes_good);
    if let Err(e) = &rfp_good {
        println!("{:?}",e);
    }
    assert!(rfp_good.is_ok());
    let rfp = rfp_good.unwrap();
    assert_eq!(rfp.get_file_name(), "example.txt".to_string());


    let mut bytes_short = BytesMut::with_capacity(81);
    bytes_short.put_u8( IrcKind::IRC_KIND_REJECT_FILE as u8);
    bytes_short.put_u32(145);
    bytes_short.put_slice("Frank".as_bytes());
    let remain = 60 - "Frank".len(); //too short
    bytes_short.put_bytes(b'\0', remain);

    bytes_short.put_slice("Bob".as_bytes());
    let remain = 64 - "Bob".len();
    bytes_short.put_bytes(b'\0', remain);

    bytes_short.put_u16(11);
    bytes_short.put_u32(512);
    bytes_short.put_slice("example.txt".as_bytes());

    let rfp_bad_short = RejectFilePacket::from_bytes(&bytes_short);
    assert!(rfp_bad_short.is_err());
    if let Err(e) = rfp_bad_short {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketLengthIncorrect(_,_) => true, _ => false });
    };

    let mut bytes_lenf= BytesMut::with_capacity(145);
    bytes_lenf.put_u8( IrcKind::IRC_KIND_REJECT_FILE as u8);
    bytes_lenf.put_u32(30); //wrong length field value

    bytes_lenf.put_slice("Frank".as_bytes());
    let remain = 60 - "Frank".len(); //too short
    bytes_lenf.put_bytes(b'\0', remain);

    bytes_lenf.put_slice("Bob".as_bytes());
    let remain = 64 - "Bob".len();
    bytes_lenf.put_bytes(b'\0', remain);

    bytes_lenf.put_u16(11);
    bytes_lenf.put_u32(512);
    bytes_lenf.put_slice("example.txt".as_bytes());

    let rfp_bad_lenf = RejectFilePacket::from_bytes(&bytes_lenf);
    assert!(rfp_bad_lenf.is_err());
    if let Err(e) = rfp_bad_lenf {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e {IrcError::PacketLengthIncorrect(_,145) => true, IrcError::FieldLengthIncorrect() => true, _ => false });
    };


    let mut bytes_mismatch= BytesMut::with_capacity(73);
    bytes_mismatch.put_u8( IrcKind::IRC_KIND_NEW_CLIENT as u8); //wrong type
    bytes_mismatch.put_u32(145);
    bytes_mismatch.put_slice("Frank".as_bytes());
    let remain = 60 - "Frank".len(); //too short
    bytes_mismatch.put_bytes(b'\0', remain);

    bytes_mismatch.put_slice("Bob".as_bytes());
    let remain = 64 - "Bob".len();
    bytes_mismatch.put_bytes(b'\0', remain);

    bytes_mismatch.put_u16(11);
    bytes_mismatch.put_u32(512);
    bytes_mismatch.put_slice("example.txt".as_bytes());

    let rfp_bad_mismatch = RejectFilePacket::from_bytes(&bytes_mismatch);
    assert!(rfp_bad_mismatch.is_err());
    if let Err(e) = rfp_bad_mismatch {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketMismatch() => true, _ => false });
    };
}

#[test]
fn reject_file() {
    let rfpwrap = RejectFilePacket::new(&"Frank".to_string(), &"Bob".to_string(), 11, 512, &"Example.txt".to_string());
    assert!(rfpwrap.is_ok());
    let rfp = rfpwrap.unwrap();
    assert_eq!(rfp.get_to(), "Frank");
    assert_eq!(rfp.get_from(), "Bob");
    assert_eq!(rfp.get_transfer_id(), 11);
    assert_eq!(rfp.get_size(), 512);
    assert_eq!(rfp.get_file_name(), "Example.txt");

    let mut rfp_fail = RejectFilePacket::new(&"Frank".to_string(), &"Bob".to_string(),11,  512, &"Exa:mple.txt".to_string());
    assert!(rfp_fail.is_err());
}

#[test]
fn reject_file_packet_as_bytes() {
    let mut rfp = RejectFilePacket::new(&"Frank".to_string(), &"Bob".to_string(),11, 512, &"example.txt".to_string()).unwrap();
    assert_eq!(rfp.as_bytes(), Bytes::from_static(b"\x10\0\0\0\x96Frank\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Bob\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x00\x0B\0\0\x02\x00example.txt"));
}

///////////////////////////////////////////////
//  File Transfer Packet
///////////////////////////////////////////////

#[test]
fn file_transfer_packet_from_bytes() {
    let mut bytes_good = BytesMut::with_capacity(91);
    bytes_good.put_u8( IrcKind::IRC_KIND_FILE_TRANSFER as u8);
    bytes_good.put_u32(24);
    bytes_good.put_u16(65534);
    bytes_good.put_u8(0);
    bytes_good.put_slice(b"Dude, where'd you go?");

    let ftp_good = FileTransferPacket::from_bytes(&bytes_good);
    if let Err(e) = &ftp_good {
        println!("{:?}", e);
    }
    assert!(ftp_good.is_ok());
    let ftp = ftp_good.unwrap();
    assert!(ftp.transfer_id == 65534);
    assert!(ftp.finished == false);
    assert_eq!(ftp.data, Bytes::from_static(b"Dude, where'd you go?"));

    let mut bytes_good_end = BytesMut::with_capacity(91);
    bytes_good_end.put_u8( IrcKind::IRC_KIND_FILE_TRANSFER as u8);
    bytes_good_end.put_u32(18);
    bytes_good_end.put_u16(604);
    bytes_good_end.put_u8(1);
    bytes_good_end.put_slice(b"To Albuquerque!");

    let ftp_good_end = FileTransferPacket::from_bytes(&bytes_good_end);
    if let Err(e) = &ftp_good_end {
        println!("{:?}", e);
    }
    assert!(ftp_good_end.is_ok());
    let ftp_end = ftp_good_end.unwrap();
    assert!(ftp_end.transfer_id == 604);
    assert!(ftp_end.finished == true);
    assert_eq!(ftp_end.data, Bytes::from_static(b"To Albuquerque!"));

    let mut bytes_lenf= BytesMut::with_capacity(145);
    bytes_lenf.put_u8( IrcKind::IRC_KIND_FILE_TRANSFER as u8);
    bytes_lenf.put_u32(30); //wrong length field value
    bytes_lenf.put_u16(13);
    bytes_lenf.put_u8(0);
    bytes_lenf.put_slice(b"Our records show your car's warranty is almost expired! If you'd like to...\0");

    let ftp_bad_lenf = FileTransferPacket::from_bytes(&bytes_lenf);
    assert!(ftp_bad_lenf.is_err());
    if let Err(e) = ftp_bad_lenf {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e {IrcError::PacketLengthIncorrect(_,35) => true, IrcError::FieldLengthIncorrect() => true, _ => false });
    };

    let mut bytes_mismatch= BytesMut::with_capacity(73);
    bytes_mismatch.put_u8( IrcKind::IRC_KIND_NEW_CLIENT as u8); //wrong type
    bytes_mismatch.put_u32(33);
    bytes_mismatch.put_u16(255);
    bytes_mismatch.put_u8(1);
    bytes_mismatch.put_slice("It was not the end, but it was AN end.".as_bytes());

    let ftp_bad_mismatch = FileTransferPacket::from_bytes(&bytes_mismatch);
    assert!(ftp_bad_mismatch.is_err());
    if let Err(e) = ftp_bad_mismatch {
        //workaround - unable to derive PartialEq on IrcError as it can contain io::Error which
        //does NOT implement PartialEq
        assert!(match e { IrcError::PacketMismatch() => true, _ => false });
    };
}

#[test]
fn file_transfer_packet_as_bytes() {
    let mut ftp = FileTransferPacket::new(11, false, Bytes::from_static(b"asdbvadfavasdfasdfijasdifnmalsdikf")).unwrap();
    assert_eq!(ftp.as_bytes(), Bytes::from_static(b"\x11\0\0\0\x25\x00\x0B\x00asdbvadfavasdfasdfijasdifnmalsdikf"));

    let mut ftp = FileTransferPacket::new(14, true, Bytes::from_static(b"And thus spoke micheal: the end.\"")).unwrap();
    assert_eq!(ftp.as_bytes(), Bytes::from_static(b"\x11\0\0\0\x24\x00\x0E\x01And thus spoke micheal: the end.\""));

}
