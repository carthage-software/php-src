--TEST--
Type alias with type error
--FILE--
<?php

type IntOrString = int|string;

function test(IntOrString $value): IntOrString {
    return $value;
}

try {
    test([]);
} catch (TypeError $e) {
    echo $e->getMessage(), "\n";
}

?>
--EXPECTF--
test(): Argument #1 ($value) must be of type string|int, array given, called in %s on line %d
