--TEST--
Type alias circular reference: 2-way indirect cycle
--FILE--
<?php

type A = B;
type B = A;

?>
--EXPECTF--
Fatal error: Type alias B cannot reference itself, either directly or through other type aliases in %s on line %d
