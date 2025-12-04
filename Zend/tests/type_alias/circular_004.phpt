--TEST--
Type alias circular reference: self-reference in union type
--FILE--
<?php

type A = int|A;

?>
--EXPECTF--
Fatal error: Type alias A cannot reference itself, either directly or through other type aliases in %s on line %d
