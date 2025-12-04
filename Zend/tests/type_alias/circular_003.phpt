--TEST--
Type alias circular reference: 3-way indirect cycle
--FILE--
<?php

type A = B;
type B = C;
type C = A;

?>
--EXPECTF--
Fatal error: Type alias C cannot reference itself, either directly or through other type aliases in %s on line %d
