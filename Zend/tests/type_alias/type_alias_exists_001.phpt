--TEST--
type_alias_exists() function
--FILE--
<?php

type MyAlias = int|string;

var_dump(type_alias_exists('MyAlias'));
var_dump(type_alias_exists('NonExistent'));
var_dump(type_alias_exists('NonExistent', false));

// Classes should not be considered type aliases
class MyClass {}
var_dump(type_alias_exists('MyClass'));

?>
--EXPECT--
bool(true)
bool(false)
bool(false)
bool(false)
