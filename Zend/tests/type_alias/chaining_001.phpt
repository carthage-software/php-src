--TEST--
Type alias chaining
--FILE--
<?php

type MyString = string;
type IntOrMyString = int|MyString;

function test(IntOrMyString $value): IntOrMyString {
    return $value;
}

var_dump(test(42));
var_dump(test("hello"));

?>
--EXPECT--
int(42)
string(5) "hello"
