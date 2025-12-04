--TEST--
Basic type alias declaration and usage
--FILE--
<?php

type IntOrString = int|string;

function test(IntOrString $value): IntOrString {
    return $value;
}

var_dump(test(42));
var_dump(test("hello"));

?>
--EXPECT--
int(42)
string(5) "hello"
