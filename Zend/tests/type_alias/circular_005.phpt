--TEST--
Type alias circular reference: deep 4-way cycle
--FILE--
<?php

type A = B;
type B = C;
type C = D;
type D = A;

?>
--EXPECTF--
Fatal error: Type alias D cannot reference itself, either directly or through other type aliases in %s on line %d
